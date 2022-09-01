use crate::core::aliases::client::{AnyClientState, AnyConsensusState, ClientType};
use crate::core::traits::client::HasAnyClient;
use crate::core::traits::error::HasError;
use crate::core::traits::ibc::HasIbcTypes;

pub trait AnyClientReader: HasAnyClient + HasIbcTypes + HasError {
    fn get_client_type(
        &self,
        client_id: &Self::ClientId,
    ) -> Result<ClientType<Self::AnyClient>, Self::Error>;

    fn get_client_state(
        &self,
        client_id: &Self::ClientId,
    ) -> Result<AnyClientState<Self::AnyClient>, Self::Error>;

    fn get_consensus_state(
        &self,
        client_id: &Self::ClientId,
        height: &Self::Height,
    ) -> Result<AnyConsensusState<Self::AnyClient>, Self::Error>;
}
