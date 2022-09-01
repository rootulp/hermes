use crate::core::aliases::client::AnyConsensusState;
use crate::core::traits::client::HasAnyClient;
use crate::core::traits::ibc::HasIbcTypes;

pub trait ChainHost: HasIbcTypes + HasAnyClient {
    fn host_height(&self) -> Self::Height;

    fn host_timestamp(&self) -> Self::Timestamp;

    fn host_consensus_state(&self) -> AnyConsensusState<Self::AnyClient>;
}
