impl<Context> super::MessageService<Context> for super::MessageServiceImpl {
    type Error = super::Error;

    fn get_message<'a>(
        &'a self,
        ctx: &'a Context,
        req: super::GetMessage,
    ) -> futures::future::BoxFuture<'a, Result<super::Message, Self::Error>> {
        todo!()
    }

    fn get_messages_in_area<'a>(
        &'a self,
        ctx: &'a Context,
        req: super::GetMessagesInArea,
    ) -> futures::future::BoxFuture<'a, Result<Vec<super::Message>, Self::Error>> {
        todo!()
    }

    fn create_message<'a>(
        &'a self,
        ctx: &'a Context,
        req: super::CreateMessage,
    ) -> futures::future::BoxFuture<'a, Result<super::Message, Self::Error>> {
        todo!()
    }
}
