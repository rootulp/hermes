use crate::core::traits::error::InjectError;
use crate::core::traits::ibc::HasIbcTypes;
use crate::core::traits::message_handler::MessageHandler;
use crate::core::traits::messages::update_client::{
    HasUpdateClientMessage, HasUpdateClientMessageHandler,
};

pub struct DispatchIbcMessages;

pub struct UnknownMessage<MessageType> {
    pub message_type: MessageType,
}

impl<Context> MessageHandler<Context> for DispatchIbcMessages
where
    Context: HasIbcTypes,
    Context: HasUpdateClientMessageHandler,
    Context: InjectError<UnknownMessage<Context::MessageType>>,
{
    fn handle_message(context: &Context, message: &Context::Message) -> Result<(), Context::Error> {
        let message_type = Context::message_type(message);

        // TODO: Handle all IBC messages here

        if message_type == <Context as HasUpdateClientMessage>::MESSAGE_TYPE {
            let update_client_message = Context::try_extract_update_client_message(message)?;

            context.handle_update_client_message(update_client_message)?;

            Ok(())
        } else {
            Err(Context::inject_error(UnknownMessage { message_type }))
        }
    }
}
