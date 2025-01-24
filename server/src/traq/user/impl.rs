use sqlx::MySqlPool;

use crate::prelude::IntoStatus;
use crate::user::ProvideUserService;

impl<Context> super::TraqUserService<Context> for super::TraqUserServiceImpl
where
    Context: AsRef<MySqlPool> + ProvideUserService,
{
    type Error = super::Error;

    fn get_traq_user<'a>(
        &'a self,
        ctx: &'a Context,
        params: super::GetTraqUserParams,
    ) -> futures::future::BoxFuture<'a, Result<super::TraqUser, Self::Error>> {
        todo!()
    }

    fn find_traq_user<'a>(
        &'a self,
        ctx: &'a Context,
        params: super::FindTraqUserParams,
    ) -> futures::future::BoxFuture<'a, Result<Option<super::TraqUser>, Self::Error>> {
        todo!()
    }

    fn register_traq_user<'a>(
        &'a self,
        ctx: &'a Context,
        params: super::RegisterTraqUserParams,
    ) -> futures::future::BoxFuture<'a, Result<super::TraqUser, Self::Error>> {
        todo!()
    }
}
