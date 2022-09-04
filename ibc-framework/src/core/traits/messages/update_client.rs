use crate::core::traits::client::HasAnyClientTypes;
use crate::core::traits::error::HasError;
use crate::core::traits::ibc::HasIbcTypes;
use crate::core::traits::sync::Async;

pub trait HasUpdateClientMessage: HasAnyClientTypes + HasIbcTypes + HasError {
    const MESSAGE_TYPE: Self::MessageType;

    type UpdateClientMessage: Async;

    fn try_parse_message(message: Self::Message) -> Result<Self::UpdateClientMessage, Self::Error>;

    fn message_client_id(message: &Self::UpdateClientMessage) -> &Self::ClientId;

    fn message_client_header(message: &Self::UpdateClientMessage) -> &Self::AnyClientHeader;
}
