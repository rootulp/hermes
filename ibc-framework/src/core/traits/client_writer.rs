use crate::core::traits::client::HasAnyClientTypes;
use crate::core::traits::error::HasError;
use crate::core::traits::ibc::HasIbcTypes;

pub trait AnyClientWriter: HasAnyClientTypes + HasIbcTypes + HasError {
    fn get_client_type(&self, client_id: &Self::ClientId) -> Result<Self::ClientType, Self::Error>;

    fn set_any_client_state(
        &self,
        client_id: &Self::ClientId,
        client_state: &Self::AnyClientState,
    ) -> Result<(), Self::Error>;

    fn set_any_consensus_state(
        &self,
        client_id: &Self::ClientId,
        consensus_state: &Self::AnyConsensusState,
    ) -> Result<(), Self::Error>;
}
