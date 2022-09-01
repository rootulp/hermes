use ibc::clients::ics07_tendermint::client_state::ClientState as TendermintClientState;
use ibc::clients::ics07_tendermint::consensus_state::ConsensusState as TendermintConsensusState;
use ibc::clients::ics07_tendermint::header::Header as TendermintClientHeader;
use ibc::clients::ics07_tendermint::misbehaviour::Misbehaviour as TendermintMisbehavior;
use ibc::core::ics02_client::client_state::ClientState;
use ibc::core::ics02_client::client_type::ClientType;
use ibc::core::ics02_client::consensus_state::ConsensusState;
use ibc::core::ics02_client::header::Header as ClientHeader;
use ibc::core::ics02_client::misbehaviour::Misbehaviour;

use crate::core::impls::clients::tendermint::TendermintClient;
use crate::core::traits::client::{ContainsClient, HasAnyClientTypes};

pub struct DynamicClient;

pub struct DynClientState {
    pub client_state: Box<dyn ClientState>,
}

pub struct DynConsensusState {
    pub consensus_state: Box<dyn ConsensusState>,
}

pub struct DynClientHeader {
    pub client_header: Box<dyn ClientHeader>,
}

pub struct DynMisbehavior {
    pub misbehavior: Box<dyn Misbehaviour>,
}

impl HasAnyClientTypes for DynamicClient {
    type ClientType = ClientType;

    type AnyClientState = DynClientState;

    type AnyConsensusState = DynConsensusState;

    type AnyClientHeader = DynClientHeader;

    type AnyMisbehavior = DynMisbehavior;
}

impl ContainsClient<TendermintClient> for DynamicClient {
    const CLIENT_TYPE: ClientType = ClientType::Tendermint;

    fn to_any_client_state(client_state: TendermintClientState) -> DynClientState {
        DynClientState::new(client_state)
    }

    fn try_from_any_client_state(client_state: &DynClientState) -> Option<&TendermintClientState> {
        client_state.client_state.as_any().downcast_ref()
    }

    fn to_any_consensus_state(consensus_state: TendermintConsensusState) -> DynConsensusState {
        DynConsensusState::new(consensus_state)
    }

    fn try_from_any_consensus_state(
        consensus_state: &DynConsensusState,
    ) -> Option<&TendermintConsensusState> {
        consensus_state.consensus_state.as_any().downcast_ref()
    }

    fn to_any_client_header(header: TendermintClientHeader) -> DynClientHeader {
        DynClientHeader::new(header)
    }

    fn try_from_any_client_header(
        client_header: &DynClientHeader,
    ) -> Option<&TendermintClientHeader> {
        client_header.client_header.as_any().downcast_ref()
    }

    fn to_any_misbehavior(misbehavior: TendermintMisbehavior) -> DynMisbehavior {
        DynMisbehavior::new(misbehavior)
    }

    fn try_from_any_misbehavior(misbehavior: &DynMisbehavior) -> Option<&TendermintMisbehavior> {
        misbehavior.misbehavior.as_any().downcast_ref()
    }
}

impl DynClientState {
    fn new(client_state: impl ClientState) -> Self {
        Self {
            client_state: Box::new(client_state),
        }
    }
}

impl DynConsensusState {
    fn new(consensus_state: impl ConsensusState) -> Self {
        Self {
            consensus_state: Box::new(consensus_state),
        }
    }
}

impl DynClientHeader {
    fn new(client_header: impl ClientHeader) -> Self {
        Self {
            client_header: Box::new(client_header),
        }
    }
}

impl DynMisbehavior {
    fn new(misbehavior: impl Misbehaviour) -> Self {
        Self {
            misbehavior: Box::new(misbehavior),
        }
    }
}
