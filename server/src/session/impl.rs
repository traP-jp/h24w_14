use axum_extra::extract::{
    cookie::{Cookie, Key},
    PrivateCookieJar,
};
use futures::FutureExt as _;
use sqlx::MySqlPool;

use super::SessionName;

impl<Context> super::SessionService<Context> for super::SessionServiceImpl
where
    Context: AsRef<MySqlPool> + AsRef<Key> + AsRef<SessionName>,
{
    type Error = super::Error;
    type Jar = PrivateCookieJar;

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
    ) -> futures::future::BoxFuture<'a, Result<Self::Jar, Self::Error>> {
        let key: &Key = ctx.as_ref();
        let session_name: &SessionName = ctx.as_ref();
        let jar = PrivateCookieJar::from_headers(params.header_map, key.clone());

        save(jar, session_name.0.clone(), params.user_id).boxed()
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
    session_name: String,
    user_id: crate::user::UserId,
) -> Result<PrivateCookieJar, super::Error> {
    Ok(cookie_jar.add(Cookie::new(session_name, user_id.0.to_string())))
}
