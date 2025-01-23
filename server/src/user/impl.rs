use futures::{future, FutureExt};

impl<Context> super::UserService<Context> for super::UserServiceImpl
where
    Context: super::UserStore,
{
    type Error = super::Error;

    fn get_user<'a>(
        &'a self,
        ctx: &'a Context,
        req: super::GetUser,
    ) -> future::BoxFuture<'a, Result<super::User, Self::Error>> {
        let user = ctx.find(req.id);
        let fut = future::ready(user.ok_or(super::Error::NotFound));
        fut.boxed()
    }

    fn create_user<'a>(
        &'a self,
        ctx: &'a Context,
        req: super::CreateUser,
    ) -> future::BoxFuture<'a, Result<super::User, Self::Error>> {
        let user = ctx.create(req.name, req.display_name);
        let fut = future::ready(Ok(user));
        fut.boxed()
    }
}
