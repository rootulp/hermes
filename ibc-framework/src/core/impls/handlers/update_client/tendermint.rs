use core::marker::PhantomData;
use ibc::clients::ics07_tendermint::client_state::ClientState as TendermintClientState;
use ibc::clients::ics07_tendermint::consensus_state::ConsensusState as TendermintConsensusState;
use ibc::clients::ics07_tendermint::header::Header as TendermintClientHeader;
use ibc::Height;

use crate::core::aliases::client::{AnyClientHeader, AnyClientState, AnyConsensusState};
use crate::core::aliases::ibc::ClientId;
use crate::core::impls::clients::tendermint::TendermintClient;
use crate::core::traits::client::{AnyClientTypes, ClientTypes, HasAnyClient, HasClient};
use crate::core::traits::client_reader::AnyClientReader;
use crate::core::traits::error::HasError;
use crate::core::traits::handlers::update_client::{AnyUpdateClientHandler, UpdateClientHandler};
use crate::core::traits::ibc::{HasIbcTypes, IbcTypes};

pub enum UpdateTendermintClientError {
    MismatchRevision {
        current_revision: u64,
        update_revision: u64,
    },
}

pub struct UpdateTendermintClient;

impl<Context, Ibc, AnyClient, Error, AnyConsensusState> UpdateClientHandler<Context>
    for UpdateTendermintClient
where
    Context: HasError<Error = Error>,
    Context: HasIbcTypes<IbcTypes = Ibc>,
    Context: AnyClientReader<Client = AnyClient>,
    Ibc: IbcTypes<Height = Height>,
    AnyClient: AnyClientTypes<AnyConsensusState = AnyConsensusState>,
    AnyClient: HasClient<TendermintClient, AnyConsensusState = AnyConsensusState>,
    Error: From<UpdateTendermintClientError>,
{
    type Client = TendermintClient;

    fn check_header_and_update_state(
        context: &Context,
        client_id: &Ibc::ClientId,
        client_state: &TendermintClientState,
        new_client_header: &TendermintClientHeader,
    ) -> Result<(TendermintClientState, TendermintConsensusState), Context::Error> {
        let new_height = new_client_header.height();

        let current_revision = client_state.chain_id.version();
        let update_revision = new_client_header.height().revision_number();

        if current_revision != update_revision {
            return Err(UpdateTendermintClientError::MismatchRevision {
                current_revision,
                update_revision,
            }
            .into());
        }

        // let current_any_client_consensus_state: AnyConsensusState = Context::get_consensus_state(context, client_id, &new_height)?;

        // let current_client_consensus_state = <AnyClient as HasClient<TendermintClient>>::try_from_any_consensus_state(&current_any_client_consensus_state);

        // let new_consensus_state = TendermintConsensusState::from(new_client_header.clone());

        // let any_consensus_state = context.get_consensus_state(client_id, height)?;

        todo!()
    }
}
