use crate::core::traits::client::HasAnyClientTypes;
use crate::core::traits::error::HasError;
use crate::core::traits::ibc::HasIbcTypes;
use crate::core::traits::message::HasMessageTypes;
use crate::core::traits::sync::Async;

pub trait HasUpdateClientMessage:
    HasAnyClientTypes + HasIbcTypes + HasMessageTypes + HasError
{
    const MESSAGE_TYPE: Self::MessageType;

    type UpdateClientMessage: Async;

    fn try_extract_update_client_message(
        message: &Self::Message,
    ) -> Result<&Self::UpdateClientMessage, Self::Error>;

    fn message_client_id(message: &Self::UpdateClientMessage) -> &Self::ClientId;

    fn message_client_header(message: &Self::UpdateClientMessage) -> &Self::AnyClientHeader;
}

pub trait UpdateClientMessageHandler<Context>: Async
where
    Context: HasUpdateClientMessage,
{
    fn handle_update_client_message(
        context: &Context,
        message: &Context::UpdateClientMessage,
    ) -> Result<(), Context::Error>;
}

pub trait HasUpdateClientMessageHandler: HasUpdateClientMessage {
    type UpdateClientMessageHandler: UpdateClientMessageHandler<Self>;

    fn handle_update_client_message(
        &self,
        message: &Self::UpdateClientMessage,
    ) -> Result<(), Self::Error> {
        Self::UpdateClientMessageHandler::handle_update_client_message(self, message)
    }
}
