use core::marker::PhantomData;

use crate::core::aliases::client::{AnyClientHeader, AnyClientState, AnyConsensusState};
use crate::core::aliases::ibc::ClientId;
use crate::core::impls::handlers::update_client::lift::{
    LiftClientUpdateHandler, MismatchClientHeaderFormat,
};
use crate::core::traits::client::{ClientTypes, HasAnyClient, HasClient};
use crate::core::traits::client_reader::AnyClientReader;
use crate::core::traits::handlers::update_client::{AnyUpdateClientHandler, UpdateClientHandler};

pub struct CombineClientUpdateHandler<Handler, NextHandlers>(
    pub PhantomData<(Handler, NextHandlers)>,
);

impl<Context, Handler, NextHandlers, Client, AnyClient> AnyUpdateClientHandler<Context>
    for CombineClientUpdateHandler<Handler, NextHandlers>
where
    Context: AnyClientReader,
    Context: HasAnyClient<AnyClient = AnyClient>,
    AnyClient: HasClient<Client>,
    Client: ClientTypes,
    Handler: UpdateClientHandler<Context, Client = Client>,
    NextHandlers: AnyUpdateClientHandler<Context>,
    Context::Error: From<MismatchClientHeaderFormat<AnyClient::ClientType>>,
{
    fn check_header_and_update_state(
        context: &Context,
        client_id: &ClientId<Context::IbcTypes>,
        client_state: &AnyClientState<Context::AnyClient>,
        new_client_header: &AnyClientHeader<Context::AnyClient>,
    ) -> Result<
        (
            AnyClientState<Context::AnyClient>,
            AnyConsensusState<Context::AnyClient>,
        ),
        Context::Error,
    > {
        let client_type = context.get_client_type(client_id)?;

        if client_type == AnyClient::CLIENT_TYPE {
            <LiftClientUpdateHandler<Handler>>::check_header_and_update_state(
                context,
                client_id,
                client_state,
                new_client_header,
            )
        } else {
            NextHandlers::check_header_and_update_state(
                context,
                client_id,
                client_state,
                new_client_header,
            )
        }
    }
}
