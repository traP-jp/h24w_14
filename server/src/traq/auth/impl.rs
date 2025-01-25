use futures::FutureExt;
use oauth2::{
    basic::BasicClient, AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, Scope,
    TokenResponse, TokenUrl,
};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, MySqlPool};
use uuid::Uuid;

use crate::session::SaveParams;

#[inline]
fn traq_oauth_auth_url(host: &str) -> String {
    format!("https://{host}/api/v3/oauth2/authorize")
}
#[inline]
fn traq_oauth_token_url(host: &str) -> String {
    format!("https://{host}/api/v3/oauth2/token")
}

#[derive(Debug, Clone, Deserialize, Serialize, FromRow)]
pub struct TokenRow {
    pub traq_user_id: Uuid,
    pub token: String,
}

impl<Context> super::TraqAuthService<Context> for super::TraqAuthServiceImpl
where
    Context: AsRef<MySqlPool>
        + AsRef<Client>
        + AsRef<super::TraqOauthClientConfig>
        + AsRef<crate::traq::TraqHost>
        + crate::traq::user::ProvideTraqUserService
        + crate::session::ProvideSessionService,
{
    type Error = super::Error;

    fn oauth2_entrypoint_uri<'a>(
        &'a self,
        ctx: &'a Context,
        _params: super::OAuth2EntrypointUriParams,
    ) -> futures::future::BoxFuture<'a, Result<String, Self::Error>> {
        let config: &super::TraqOauthClientConfig = ctx.as_ref();
        let host: &crate::traq::TraqHost = ctx.as_ref();
        let client = create_oauth_client(&host.0, config);

        oauth2_entrypoint_uri(client).boxed()
    }

    fn oauth2_handle_redirect<'a>(
        &'a self,
        ctx: &'a Context,
        req: http::Request<()>,
    ) -> futures::future::BoxFuture<'a, Result<super::AuthorizedUser, Self::Error>> {
        let config: &super::TraqOauthClientConfig = ctx.as_ref();
        let host: &crate::traq::TraqHost = ctx.as_ref();
        let client = create_oauth_client(&host.0, config);
        let req_client: &reqwest::Client = ctx.as_ref();
        let pool = ctx.as_ref();
        let traq_host: &crate::traq::TraqHost = ctx.as_ref();

        oauth2_handle_redirect(client, req_client.clone(), req, ctx, pool, &traq_host.0).boxed()
    }

    fn check_authorized<'a>(
        &'a self,
        ctx: &'a Context,
        user: crate::traq::user::TraqUserId,
    ) -> futures::future::BoxFuture<'a, Result<Option<super::AuthorizedUser>, Self::Error>> {
        let pool = ctx.as_ref();

        check_authorized(pool, user).boxed()
    }

    fn build_request_as_authorized_user<'a>(
        &'a self,
        ctx: &'a Context,
        params: super::BuildRequestAsAuthorizedUserParams<'a>,
    ) -> futures::future::BoxFuture<'a, Result<reqwest::RequestBuilder, Self::Error>> {
        let pool = ctx.as_ref();
        let client = ctx.as_ref();

        build_request_as_authorized_user(pool, client, params).boxed()
    }
}

async fn oauth2_entrypoint_uri(client: OauthClient) -> Result<String, super::Error> {
    let (url, _) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("read".to_string()))
        .add_scope(Scope::new("write".to_string()))
        .url();

    Ok(url.to_string())
}

async fn oauth2_handle_redirect<Context>(
    client: OauthClient,
    req_client: reqwest::Client,
    req_: http::Request<()>,
    context: &Context,
    pool: &MySqlPool,
    traq_host: &str,
) -> Result<super::AuthorizedUser, super::Error>
where
    Context: crate::traq::user::ProvideTraqUserService + crate::session::ProvideSessionService,
{
    let code = req_
        .uri()
        .query()
        .ok_or(super::Error::InvalidRequest("missing query".to_string()))?
        .split('&')
        .find(|s| s.starts_with("code="))
        .ok_or(super::Error::InvalidRequest("missing code".to_string()))?
        .split('=')
        .nth(1)
        .ok_or(super::Error::InvalidRequest("missing code".to_string()))?;

    let token = client
        .exchange_code(AuthorizationCode::new(code.to_string()))
        .request_async(&req_client)
        .await
        .map_err(|e| match e {
            oauth2::RequestTokenError::Request(oauth2::HttpClientError::Reqwest(err)) => {
                super::Error::Reqwest(*err)
            }
            oauth2::RequestTokenError::ServerResponse(err) => {
                super::Error::InvalidRequest(err.to_string())
            }
            oauth2::RequestTokenError::Parse(err, _) => {
                super::Error::InvalidRequest(err.to_string())
            }
            _ => super::Error::Unknown,
        })?;
    let access_token = token.access_token().secret();

    let uri = format!("https://{traq_host}/api/v3/users/me");
    let request = req_client.get(&uri).bearer_auth(access_token);
    let response = request
        .send()
        .await
        .map_err(super::Error::Reqwest)?
        .error_for_status()
        .map_err(super::Error::Reqwest)?
        .json::<serde_json::Value>()
        .await?;

    let response = response.as_object().ok_or(super::Error::Unknown)?;
    let id: Uuid = response
        .get("id")
        .ok_or(super::Error::Unknown)?
        .as_str()
        .ok_or(super::Error::Unknown)?
        .parse()
        .map_err(|_| super::Error::Unknown)?;

    let (user, app_user_id) = get_or_register_user(context, id).await?;

    sqlx::query(
        r#"
            INSERT INTO `traq_token` (`traq_user_id`, `token`) VALUES (?, ?) ON DUPLICATE KEY UPDATE `token` = ?
        "#
    )
        .bind(user.user_id.0)
        .bind(access_token)
        .bind(access_token)
        .execute(pool)
        .await
        .map_err(super::Error::Sqlx)?;

    let _ = context
        .save(SaveParams {
            user_id: app_user_id,
            header_map: req_.headers(),
        })
        .await
        .map_err(|e| super::Error::Other(e.into()))?;

    Ok(user)
}

async fn get_or_register_user<Context>(
    context: &Context,
    id: Uuid,
) -> Result<(super::AuthorizedUser, crate::user::UserId), super::Error>
where
    Context: crate::traq::user::ProvideTraqUserService,
{
    let user = context
        .find_traq_user(crate::traq::user::FindTraqUserParams {
            id: crate::traq::user::TraqUserId(id),
        })
        .await
        .map_err(|e| super::Error::Other(e.into()))?
        .map(|user| (super::AuthorizedUser { user_id: user.id }, user.inner.id));

    if let Some(user) = user {
        return Ok(user);
    }

    context
        .register_traq_user(crate::traq::user::RegisterTraqUserParams {
            id: crate::traq::user::TraqUserId(id),
        })
        .await
        .map_err(|e| super::Error::Other(e.into()))
        .map(|user| (super::AuthorizedUser { user_id: user.id }, user.inner.id))
}

async fn check_authorized(
    pool: &MySqlPool,
    user_id: crate::traq::user::TraqUserId,
) -> Result<Option<super::AuthorizedUser>, super::Error> {
    sqlx::query_as::<_, TokenRow>("SELECT * FROM `traq_token` WHERE `traq_user_id` = ?")
        .bind(user_id.0)
        .fetch_optional(pool)
        .await
        .map(|row| row.map(|_| super::AuthorizedUser { user_id }))
        .map_err(super::Error::Sqlx)
}

async fn build_request_as_authorized_user<'a>(
    pool: &'a MySqlPool,
    client: &'a reqwest::Client,
    params: super::BuildRequestAsAuthorizedUserParams<'a>,
) -> Result<reqwest::RequestBuilder, super::Error> {
    let token =
        sqlx::query_as::<_, TokenRow>("SELECT * FROM `traq_token` WHERE `traq_user_id` = ?")
            .bind(params.user.user_id.0)
            .fetch_one(pool)
            .await
            .map_err(super::Error::Sqlx)?;

    Ok(client
        .request(params.method, params.uri)
        .bearer_auth(token.token))
}

fn create_oauth_client(host: &str, config: &super::TraqOauthClientConfig) -> OauthClient {
    BasicClient::new(ClientId::new(config.client_id.clone()))
        .set_client_secret(ClientSecret::new(config.client_secret.clone()))
        .set_auth_uri(AuthUrl::new(traq_oauth_auth_url(host)).unwrap())
        .set_token_uri(TokenUrl::new(traq_oauth_token_url(host)).unwrap())
}

type OauthClient = oauth2::Client<
    oauth2::StandardErrorResponse<oauth2::basic::BasicErrorResponseType>,
    oauth2::StandardTokenResponse<oauth2::EmptyExtraTokenFields, oauth2::basic::BasicTokenType>,
    oauth2::StandardTokenIntrospectionResponse<
        oauth2::EmptyExtraTokenFields,
        oauth2::basic::BasicTokenType,
    >,
    oauth2::StandardRevocableToken,
    oauth2::StandardErrorResponse<oauth2::RevocationErrorResponseType>,
    oauth2::EndpointSet,
    oauth2::EndpointNotSet,
    oauth2::EndpointNotSet,
    oauth2::EndpointNotSet,
    oauth2::EndpointSet,
>;
