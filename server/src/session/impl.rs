use axum_extra::extract::{
    cookie::{Cookie, Key},
    PrivateCookieJar,
};
use futures::FutureExt as _;
use sqlx::MySqlPool;

impl<Context> super::SessionService<Context> for super::SessionServiceImpl
where
    Context: AsRef<MySqlPool> + AsRef<Key>,
{
    type Error = super::Error;
    type Jar = PrivateCookieJar;

    fn extract<'a>(
        &'a self,
        ctx: &'a Context,
        params: super::ExtractParams<'a>,
    ) -> futures::future::BoxFuture<'a, Result<super::Session, Self::Error>> {
        let key: &Key = ctx.as_ref();
        let jar = PrivateCookieJar::from_headers(params.0, key.clone());

        extract(jar).boxed()
    }

    fn save<'a>(
        &'a self,
        ctx: &'a Context,
        params: super::SaveParams,
    ) -> futures::future::BoxFuture<'a, Result<Self::Jar, Self::Error>> {
        let key: &Key = ctx.as_ref();
        let jar = PrivateCookieJar::from_headers(params.header_map, key.clone());

        save(jar, params.user_id).boxed()
    }
}

async fn extract(cookie_jar: PrivateCookieJar) -> Result<super::Session, super::Error> {
    let user_id = cookie_jar
        .get("user_id")
        .and_then(|cookie| cookie.value().parse().ok())
        .map(crate::user::UserId)
        .ok_or(super::Error::Unauthorized)?;

    Ok(super::Session { user_id })
}

async fn save(
    cookie_jar: PrivateCookieJar,
    user_id: crate::user::UserId,
) -> Result<PrivateCookieJar, super::Error> {
    Ok(cookie_jar.add(Cookie::new("user_id", user_id.0.to_string())))
}
