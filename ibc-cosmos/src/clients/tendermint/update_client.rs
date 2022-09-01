use ibc::Height;
use ibc_framework::core::traits::client::ContainsClient;
use ibc_framework::core::traits::client_reader::ClientReader;
use ibc_framework::core::traits::error::HasError;
use ibc_framework::core::traits::handlers::update_client::UpdateClientHandler;
use ibc_framework::core::traits::ibc::HasIbcTypes;
use tendermint::block::Height as BlockHeight;
use tendermint_light_client_verifier::types::{TrustedBlockState, UntrustedBlockState};

use crate::clients::tendermint::client::{
    TendermintClient, TendermintClientHeader, TendermintClientState, TendermintConsensusState,
};

pub enum UpdateTendermintClientError {
    MismatchRevision {
        current_revision: u64,
        update_revision: u64,
    },
    ConsensusStateNotFound {
        height: Height,
    },
    RevisionHeightOverflow {
        height: u64,
    },
}

impl<Context, Error> UpdateClientHandler<Context> for TendermintClient
where
    Context: HasError<Error = Error>,
    Context: HasIbcTypes<Height = Height>,
    Context: ClientReader<TendermintClient>,
    Context: ContainsClient<TendermintClient>,
    Error: From<UpdateTendermintClientError>,
{
    type Client = TendermintClient;

    fn check_header_and_update_state(
        context: &Context,
        client_id: &Context::ClientId,
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

        let new_consensus_state = TendermintConsensusState::from(new_client_header.clone());

        let m_current_client_consensus_state =
            context.get_consensus_state_at_height(client_id, &new_height)?;

        if m_current_client_consensus_state.as_ref() == Some(&new_consensus_state) {
            return Ok((client_state.clone(), new_consensus_state));
        }

        let trusted_height = new_client_header.trusted_height;

        let trusted_consensus_state = context
            .get_consensus_state_at_height(client_id, &trusted_height)?
            .ok_or_else(|| UpdateTendermintClientError::ConsensusStateNotFound {
                height: trusted_height,
            })?;

        let trusted_revision_height = trusted_height.revision_height();

        let trusted_block_height =
            BlockHeight::try_from(trusted_revision_height).map_err(|_| {
                UpdateTendermintClientError::RevisionHeightOverflow {
                    height: trusted_revision_height,
                }
            })?;

        let trusted_state = TrustedBlockState {
            header_time: trusted_consensus_state.timestamp,
            height: trusted_block_height,
            next_validators: &new_client_header.trusted_validator_set,
            next_validators_hash: trusted_consensus_state.next_validators_hash,
        };

        todo!()
    }
}
