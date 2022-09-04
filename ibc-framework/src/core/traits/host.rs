use crate::core::traits::client::HasAnyClientTypes;
use crate::core::traits::ibc::HasIbcTypes;

pub trait HasHostMethods: HasIbcTypes + HasAnyClientTypes {
    fn host_height(&self) -> Self::Height;

    fn host_timestamp(&self) -> Self::Timestamp;

    fn host_consensus_state(&self) -> Self::AnyConsensusState;
}
