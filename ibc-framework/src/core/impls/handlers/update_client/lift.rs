use core::marker::PhantomData;

use crate::core::traits::client::{ContainsClient, HasClientTypes};
use crate::core::traits::error::HasError;
use crate::core::traits::handlers::update_client::{AnyUpdateClientHandler, UpdateClientHandler};
use crate::core::traits::ibc::HasIbcTypes;

pub struct MismatchClientHeaderFormat<ClientType> {
    pub expected_client_type: ClientType,
}

pub struct LiftClientUpdateHandler<Handler>(pub PhantomData<Handler>);

impl<Context, Handler, Client> AnyUpdateClientHandler<Context> for LiftClientUpdateHandler<Handler>
where
    Context: HasError + HasIbcTypes,
    Context: ContainsClient<Client>,
    Client: HasClientTypes,
    Handler: UpdateClientHandler<Context, Client = Client>,
    Context::Error: From<MismatchClientHeaderFormat<Context::ClientType>>,
{
    fn check_header_and_update_state(
        context: &Context,
        client_id: &Context::ClientId,
        client_state: &Context::AnyClientState,
        new_client_header: &Context::AnyClientHeader,
    ) -> Result<(Context::AnyClientState, Context::AnyConsensusState), Context::Error> {
        let client_state = Context::try_from_any_client_state(client_state).ok_or_else(|| {
            MismatchClientHeaderFormat {
                expected_client_type: Context::CLIENT_TYPE,
            }
        })?;

        let client_header =
            Context::try_from_any_client_header(new_client_header).ok_or_else(|| {
                MismatchClientHeaderFormat {
                    expected_client_type: Context::CLIENT_TYPE,
                }
            })?;

        let (new_client_state, new_consensus_state) = Handler::check_header_and_update_state(
            context,
            client_id,
            client_state,
            client_header,
        )?;

        Ok((
            Context::to_any_client_state(new_client_state),
            Context::to_any_consensus_state(new_consensus_state),
        ))
    }
}
