use async_trait::async_trait;

use crate::traits::chain_context::ChainContext;
use crate::traits::core::Async;
use crate::types::aliases::{Event, Message};

pub trait MessageSenderContext: ChainContext {
    type MessageSender: MessageSender<Self>;

    fn message_sender(&self) -> &Self::MessageSender;
}

#[async_trait]
pub trait MessageSender<Context>: Async
where
    Context: ChainContext,
{
    async fn send_messages(
        &self,
        context: &Context,
        messages: Vec<Message<Context>>,
    ) -> Result<Vec<Vec<Event<Context>>>, Context::Error>;
}