use crate::core::aliases::client::AnyConsensusState;
use crate::core::aliases::ibc::{Height, Timestamp};
use crate::core::traits::client::HasAnyClient;
use crate::core::traits::ibc::HasIbcTypes;

pub trait ChainHost: HasIbcTypes + HasAnyClient {
    fn host_height(&self) -> Height<Self::IbcTypes>;

    fn host_timestamp(&self) -> Timestamp<Self::IbcTypes>;

    fn host_consensus_state(&self) -> AnyConsensusState<Self::AnyClient>;
}
