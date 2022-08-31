use crate::aliases::client::{AnyClientState, AnyConsensusState, ClientType, ConsensusState};
use crate::traits::client::{ClientTypes, HasClient};

pub trait ChainTypes {
    type Error;

    type Height;

    type Timestamp;

    type Message;

    type ClientId;

    type MerkleProof;
}

pub trait HasAnyClient: ChainTypes {
    type Client: ClientTypes;

    type AnyClient: HasClient<Self::Client>;

    fn host_consensus_state(&self) -> ConsensusState<Self::Client>;
}

pub trait HostContext: ChainTypes {
    fn host_height(&self) -> Self::Height;

    fn host_timestamp(&self) -> Self::Timestamp;
}

pub trait ClientReaderContext: HasAnyClient {
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
