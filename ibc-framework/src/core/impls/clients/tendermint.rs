use ibc::core::ics02_client::client_type::ClientType;

use crate::core::traits::client::{ContainsClient, HasAnyClientTypes, HasClientTypes};
use crate::core::traits::prism::Prism;

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

    fn try_from_ref(data: &T) -> Option<&T> {
        Some(data)
    }
}

impl ContainsClient<TendermintClient> for TendermintClient {
    const CLIENT_TYPE: ClientType = ClientType::Tendermint;
}
