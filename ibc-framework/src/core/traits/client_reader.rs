use crate::core::traits::client::HasAnyClientTypes;
use crate::core::traits::error::HasError;
use crate::core::traits::ibc::HasIbcTypes;

pub trait AnyClientReader: HasAnyClientTypes + HasIbcTypes + HasError {
    fn get_client_type(&self, client_id: &Self::ClientId) -> Result<Self::ClientType, Self::Error>;

    fn get_client_state(
        &self,
        client_id: &Self::ClientId,
    ) -> Result<Self::AnyClientState, Self::Error>;

    fn get_consensus_state(
        &self,
        client_id: &Self::ClientId,
        height: &Self::Height,
    ) -> Result<Self::AnyConsensusState, Self::Error>;
}
