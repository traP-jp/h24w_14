use axum_extra::extract::{
    cookie::{Cookie, Key},
    PrivateCookieJar,
};
use futures::FutureExt as _;

use super::{CookieDomain, SessionName};

impl<Context> super::SessionService<Context> for super::SessionServiceImpl
where
    Context: AsRef<Key> + AsRef<SessionName> + AsRef<CookieDomain>,
{
    type Error = super::Error;

    fn extract<'a>(
        &'a self,
        ctx: &'a Context,
        params: super::ExtractParams<'a>,
    ) -> futures::future::BoxFuture<'a, Result<super::Session, Self::Error>> {
        let key: &Key = ctx.as_ref();
        let session_name: &SessionName = ctx.as_ref();
        let jar = PrivateCookieJar::from_headers(params.0, key.clone());

        extract(jar, &session_name.0).boxed()
    }

    fn save<'a>(
        &'a self,
        ctx: &'a Context,
        params: super::SaveParams,
    ) -> futures::future::BoxFuture<'a, Result<PrivateCookieJar, Self::Error>> {
        let key: &Key = ctx.as_ref();
        let session_name: &SessionName = ctx.as_ref();
        let domain: &CookieDomain = ctx.as_ref();
        let jar = PrivateCookieJar::from_headers(params.header_map, key.clone());

        save(jar, &session_name.0, &domain.0, params.user_id).boxed()
    }
}

async fn extract(
    cookie_jar: PrivateCookieJar,
    session_name: &str,
) -> Result<super::Session, super::Error> {
    let user_id = cookie_jar
        .get(session_name)
        .and_then(|cookie| cookie.value().parse().ok())
        .map(crate::user::UserId)
        .ok_or(super::Error::Unauthorized)?;

    Ok(super::Session { user_id })
}

async fn save(
    cookie_jar: PrivateCookieJar,
    session_name: impl Into<String>,
    domain: impl Into<String>,
    user_id: crate::user::UserId,
) -> Result<PrivateCookieJar, super::Error> {
    let cookie = Cookie::build((session_name.into(), user_id.0.to_string()))
        .domain(domain.into())
        .path("/")
        .secure(true)
        .http_only(true)
        .to_owned();
    Ok(cookie_jar.add(cookie))
}
