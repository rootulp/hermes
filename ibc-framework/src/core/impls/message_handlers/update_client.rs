use crate::core::traits::client::HasAnyClientMethods;
use crate::core::traits::client_reader::AnyClientReader;
use crate::core::traits::error::InjectError;
use crate::core::traits::handlers::update_client::HasAnyUpdateClientHandler;
use crate::core::traits::host::HasHostMethods;
use crate::core::traits::messages::update_client::{
    HasUpdateClientMessage, UpdateClientMessageHandler,
};

pub enum Error {
    ClientIsFrozen,
}

pub struct BaseHandleUpdateClientMessage;

impl<Context> UpdateClientMessageHandler<Context> for BaseHandleUpdateClientMessage
where
    Context: HasUpdateClientMessage,
    Context: AnyClientReader,
    Context: HasAnyUpdateClientHandler,
    Context: HasAnyClientMethods,
    Context: InjectError<Error>,
    Context: HasHostMethods,
{
    #[allow(unused_variables)]
    fn handle_update_client_message(
        context: &Context,
        message: &Context::UpdateClientMessage,
    ) -> Result<(), Context::Error> {
        let client_id = Context::message_client_id(message);
        let new_any_client_header = Context::message_client_header(message);

        let current_any_client_state = context.get_any_client_state(client_id)?;

        if Context::client_state_is_frozen(&current_any_client_state) {
            return Err(Context::inject_error(Error::ClientIsFrozen));
        }

        let now = context.host_timestamp();

        let latest_consensus_state = context.get_latest_any_consensus_state(client_id)?;

        // let last_updated_time = Context::consensus_state_timestamp(&latest_consensus_state);

        let (new_any_client_state, new_any_consensus_state) = context
            .check_client_header_and_update_state(
                client_id,
                &current_any_client_state,
                new_any_client_header,
            )?;

        Ok(())
    }
}
