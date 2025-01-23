use futures::{future, FutureExt};

impl<Context> super::WorldService<Context> for super::WorldServiceImpl
where
    Context: super::WorldSizeStore,
{
    type Error = super::Error;

    fn get_world_size<'a>(
        &'a self,
        ctx: &'a Context,
        params: super::GetWorldSizeParams,
    ) -> future::BoxFuture<'a, Result<super::Size, Self::Error>> {
        let super::GetWorldSizeParams {} = params;
        let size = ctx.world_size();
        let fut = future::ready(Ok(size));
        fut.boxed()
    }

    fn check_coordinate<'a>(
        &'a self,
        ctx: &'a Context,
        params: super::CheckCoordinateParams,
    ) -> future::BoxFuture<'a, Result<super::CheckCoordinateAnswer, Self::Error>> {
        let size = ctx.world_size();
        let super::CheckCoordinateParams { coordinate } = params;
        let res = if coordinate.x < size.width && coordinate.y < size.height {
            super::CheckCoordinateAnswer::Valid(coordinate)
        } else {
            super::CheckCoordinateAnswer::Invalid
        };
        future::ready(Ok(res)).boxed()
    }
}
