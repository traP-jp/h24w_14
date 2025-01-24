use reqwest::Client;

impl<Context> super::TraqAuthService<Context> for super::TraqAuthServiceImpl
where
    Context: AsRef<Client>,
{
    type Error = super::Error;

    // 最初に叩く
    fn oauth2_entrypoint_uri<'a>(
        &'a self,
        ctx: &'a Context,
        params: super::OAuth2EntrypointUriParams,
    ) -> futures::future::BoxFuture<'a, Result<String, Self::Error>> {
        todo!()
    }

    fn oauth2_handle_redirect<'a>(
        &'a self,
        ctx: &'a Context,
        req: http::Request<()>,
    ) -> futures::future::BoxFuture<'a, Result<super::AuthorizedUser, Self::Error>> {
        todo!()
    }

    fn check_authorized<'a>(
        &'a self,
        ctx: &'a Context,
        user: crate::traq::user::TraqUser,
    ) -> futures::future::BoxFuture<'a, Result<Option<super::AuthorizedUser>, Self::Error>> {
        todo!()
    }

    fn build_request_as_authorized_user<'a>(
        &'a self,
        ctx: &'a Context,
        params: super::BuildRequestAsAuthorizedUserParams<'a>,
    ) -> futures::future::BoxFuture<'a, Result<reqwest::RequestBuilder, Self::Error>> {
        todo!()
    }
}
