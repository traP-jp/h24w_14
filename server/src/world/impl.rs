use futures::{future, FutureExt};

impl<Context> super::WorldService<Context> for super::WorldServiceImpl
where
    Context: super::WorldSizeStore,
{
    type Error = super::Error;

    fn get_world_size<'a>(
        &'a self,
        ctx: &'a Context,
        req: super::GetWorldSize,
    ) -> future::BoxFuture<'a, Result<super::Size, Self::Error>> {
        let super::GetWorldSize {} = req;
        let size = ctx.world_size();
        let fut = future::ready(Ok(size));
        fut.boxed()
    }

    fn check_coordinate<'a>(
        &'a self,
        ctx: &'a Context,
        req: super::CheckCoordinate,
    ) -> future::BoxFuture<'a, Result<super::CheckCoordinateAnswer, Self::Error>> {
        let size = ctx.world_size();
        let super::CheckCoordinate { coordinate } = req;
        let res = if coordinate.x < size.width && coordinate.y < size.height {
            super::CheckCoordinateAnswer::Valid(coordinate)
        } else {
            super::CheckCoordinateAnswer::Invalid
        };
        future::ready(Ok(res)).boxed()
    }
}
