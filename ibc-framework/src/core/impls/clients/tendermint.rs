use ibc::core::ics02_client::client_type::ClientType;

use crate::core::traits::client::{ContainsClient, HasAnyClientTypes, HasClientTypes};

pub use ibc::clients::ics07_tendermint::client_state::ClientState as TendermintClientState;
pub use ibc::clients::ics07_tendermint::consensus_state::ConsensusState as TendermintConsensusState;
pub use ibc::clients::ics07_tendermint::header::Header as TendermintClientHeader;
pub use ibc::clients::ics07_tendermint::misbehaviour::Misbehaviour as TendermintMisbehavior;

pub struct TendermintClient;

impl HasClientTypes for TendermintClient {
    type ClientState = TendermintClientState;

    type ConsensusState = TendermintConsensusState;

    type ClientHeader = TendermintClientHeader;

    type Misbehavior = TendermintMisbehavior;
}

impl HasAnyClientTypes for TendermintClient {
    type ClientType = ClientType;

    type AnyClientState = TendermintClientState;

    type AnyConsensusState = TendermintConsensusState;

    type AnyClientHeader = TendermintClientHeader;

    type AnyMisbehavior = TendermintMisbehavior;
}

impl ContainsClient<TendermintClient> for TendermintClient {
    const CLIENT_TYPE: ClientType = ClientType::Tendermint;

    fn to_any_client_state(client_state: TendermintClientState) -> TendermintClientState {
        client_state
    }

    fn try_from_any_client_state(
        client_state: &TendermintClientState,
    ) -> Option<&TendermintClientState> {
        Some(client_state)
    }

    fn to_any_consensus_state(
        consensus_state: TendermintConsensusState,
    ) -> TendermintConsensusState {
        consensus_state
    }

    fn try_from_any_consensus_state(
        consensus_state: &TendermintConsensusState,
    ) -> Option<&TendermintConsensusState> {
        Some(consensus_state)
    }

    fn to_any_client_header(client_header: TendermintClientHeader) -> TendermintClientHeader {
        client_header
    }

    fn try_from_any_client_header(
        client_header: &TendermintClientHeader,
    ) -> Option<&TendermintClientHeader> {
        Some(client_header)
    }

    fn to_any_misbehavior(misbehavior: TendermintMisbehavior) -> TendermintMisbehavior {
        misbehavior
    }

    fn try_from_any_misbehavior(
        misbehavior: &TendermintMisbehavior,
    ) -> Option<&TendermintMisbehavior> {
        Some(misbehavior)
    }
}
