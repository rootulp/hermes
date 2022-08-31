use core::marker::PhantomData;

use crate::core::aliases::client::{AnyClientHeader, AnyClientState, AnyConsensusState};
use crate::core::aliases::ibc::ClientId;
use crate::core::traits::client::{ClientTypes, HasAnyClient, HasClient};
use crate::core::traits::client_reader::ClientReader;
use crate::core::traits::handlers::update_client::{AnyUpdateClientHandler, UpdateClientHandler};

pub struct MismatchClientHeaderFormat<ClientType> {
    pub expected_client_type: ClientType,
}

pub struct CombineClientUpdateHandler<Handler, NextHandlers>(
    pub PhantomData<(Handler, NextHandlers)>,
);

impl<Context, Handler, NextHandlers, Client, AnyClient> AnyUpdateClientHandler<Context>
    for CombineClientUpdateHandler<Handler, NextHandlers>
where
    Context: ClientReader,
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
            let m_client_header = AnyClient::try_from_any_client_header(new_client_header);

            match m_client_header {
                Some(in_client_header) => {
                    let (new_client_state, new_consensus_state) =
                        Handler::check_header_and_update_state(
                            context,
                            client_id,
                            in_client_header,
                        )?;

                    Ok((
                        AnyClient::to_any_client_state(new_client_state),
                        AnyClient::to_any_consensus_state(new_consensus_state),
                    ))
                }
                None => Err(MismatchClientHeaderFormat {
                    expected_client_type: client_type,
                }
                .into()),
            }
        } else {
            NextHandlers::check_header_and_update_state(context, client_id, new_client_header)
        }
    }
}
