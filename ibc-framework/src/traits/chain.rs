use crate::aliases::client::{AnyClientState, AnyConsensusState, ClientType, ConsensusState};
use crate::traits::client::{AnyClientContext, ClientContext};

pub trait ChainContext {
    type Error;

    type Height;

    type Timestamp;

    type Message;

    type ClientId;

    type MerkleProof;
}

pub trait HasClient: ChainContext {
    type ClientTag;

    type ClientContext: ClientContext<Self::ClientTag>;

    fn error_from_client(err: <Self::ClientContext as AnyClientContext>::Error) -> Self::Error;

    fn host_consensus_state(&self) -> ConsensusState<Self::ClientContext, Self::ClientTag>;
}

pub trait HostContext: ChainContext {
    fn host_height(&self) -> Self::Height;

    fn host_timestamp(&self) -> Self::Timestamp;
}

pub trait ClientReaderContext: HasClient {
    fn get_client_type(
        &self,
        client_id: &Self::ClientId,
    ) -> Result<ClientType<Self::ClientContext>, Self::Error>;

    fn get_client_state(
        &self,
        client_id: &Self::ClientId,
    ) -> Result<AnyClientState<Self::ClientContext>, Self::Error>;

    fn get_consensus_state(
        &self,
        client_id: &Self::ClientId,
        height: &Self::Height,
    ) -> Result<AnyConsensusState<Self::ClientContext>, Self::Error>;
}
