use crate::aliases::client::{AnyClientState, AnyConsensusState, ClientType};
use crate::aliases::ibc::{ClientId, Height};
use crate::traits::client::HasAnyClient;
use crate::traits::error::HasError;
use crate::traits::ibc::HasIbcTypes;

pub trait ClientReader: HasAnyClient + HasIbcTypes + HasError {
    fn get_client_type(
        &self,
        client_id: &ClientId<Self::IbcTypes>,
    ) -> Result<ClientType<Self::AnyClient>, Self::Error>;

    fn get_client_state(
        &self,
        client_id: &ClientId<Self::IbcTypes>,
    ) -> Result<AnyClientState<Self::AnyClient>, Self::Error>;

    fn get_consensus_state(
        &self,
        client_id: &ClientId<Self::IbcTypes>,
        height: &Height<Self::IbcTypes>,
    ) -> Result<AnyConsensusState<Self::AnyClient>, Self::Error>;
}
