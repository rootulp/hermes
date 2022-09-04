use crate::core::traits::sync::Async;

pub trait HasError: Async {
    type Error: Async;
}

pub trait InjectError<E>: HasError {
    fn inject_error(error: E) -> Self::Error;
}

pub struct MismatchClientType<ClientType> {
    pub expected_client_type: ClientType,
}

pub struct MismatchMessageType<MessageType> {
    pub expected_message_type: MessageType,
}
