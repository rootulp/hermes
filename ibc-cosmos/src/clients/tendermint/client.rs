use core::time::Duration;
use ibc::core::ics02_client::client_state::ClientState;
use ibc::core::ics02_client::client_type::ClientType;
use ibc::core::ics02_client::consensus_state::ConsensusState;
use ibc::timestamp::Timestamp;
use ibc::Height;
use ibc_framework::core::traits::client::{
    HasAnyClientMethods, HasAnyClientTypes, HasClientTypeFor, HasClientTypes,
};
use ibc_framework::core::traits::ibc::HasHostTypes;
use ibc_framework::core::traits::prism::Prism;

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

impl<T> Prism<T, T> for TendermintClient {
    fn into(subdata: T) -> T {
        subdata
    }

    fn try_from(data: T) -> Option<T> {
        Some(data)
    }

    fn try_from_ref(data: &T) -> Option<&T> {
        Some(data)
    }
}

impl HasClientTypeFor<TendermintClient> for TendermintClient {
    const CLIENT_TYPE: ClientType = ClientType::Tendermint;
}

impl HasHostTypes for TendermintClient {
    type Height = Height;

    type Timestamp = Timestamp;

    type Duration = Duration;
}

impl HasAnyClientMethods for TendermintClient {
    fn client_state_type(client_state: &TendermintClientState) -> Self::ClientType {
        client_state.client_type()
    }

    fn client_state_is_frozen(client_state: &TendermintClientState) -> bool {
        client_state.is_frozen()
    }

    fn client_state_trusting_period(client_state: &TendermintClientState) -> Self::Duration {
        client_state.trusting_period
    }

    fn client_state_latest_height(client_state: &TendermintClientState) -> Self::Height {
        client_state.latest_height()
    }

    fn consensus_state_timestamp(consensus_state: &TendermintConsensusState) -> Self::Timestamp {
        consensus_state.timestamp()
    }

    fn client_header_height(client_header: &TendermintClientHeader) -> Self::Height {
        client_header.height()
    }
}
