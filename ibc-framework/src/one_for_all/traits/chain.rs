use crate::core::traits::client::HasAnyClientMethods;
use crate::core::traits::sync::Async;
use crate::one_for_all::traits::components::OfaComponents;
use crate::one_for_all::traits::error::OfaError;

pub trait OfaChainTypes: Async {
    type Error: OfaError;

    type Height: Async;

    type Timestamp: Ord + Async;

    type Duration: Ord + Async;

    type Message: Async;

    type MessageType: Eq + Async;

    type Signer: Async;

    type ClientId: Async;

    type ConnectionId: Async;

    type ChannelId: Async;

    type Port: Async;

    type MerkleProof: Async;

    type ClientType: Eq + Async;

    type AnyClientState: Async;

    type AnyConsensusState: Async;

    type AnyClientHeader: Async;

    type AnyMisbehavior: Async;
}

pub trait OfaChain: OfaChainTypes {
    type Components: OfaComponents<Self>;

    type AnyClientMethods: HasAnyClientMethods<
        Height = Self::Height,
        Timestamp = Self::Timestamp,
        Duration = Self::Duration,
        ClientType = Self::ClientType,
        AnyClientState = Self::AnyClientState,
        AnyConsensusState = Self::AnyConsensusState,
        AnyClientHeader = Self::AnyClientHeader,
        AnyMisbehavior = Self::AnyMisbehavior,
    >;

    fn host_height(&self) -> Self::Height;

    fn host_timestamp(&self) -> Self::Timestamp;

    fn add_duration(time: &Self::Timestamp, duration: &Self::Duration) -> Self::Timestamp;

    fn message_type(message: &Self::Message) -> &Self::MessageType;

    fn message_signer(message: &Self::Message) -> &Self::Signer;

    fn get_client_type(&self, client_id: &Self::ClientId) -> Result<Self::ClientType, Self::Error>;

    fn get_any_client_state(
        &self,
        client_id: &Self::ClientId,
    ) -> Result<Self::AnyClientState, Self::Error>;

    fn get_latest_any_consensus_state(
        &self,
        client_id: &Self::ClientId,
    ) -> Result<Self::AnyConsensusState, Self::Error>;

    fn get_any_consensus_state_at_height(
        &self,
        client_id: &Self::ClientId,
        height: &Self::Height,
    ) -> Result<Option<Self::AnyConsensusState>, Self::Error>;

    fn get_any_consensus_state_after_height(
        &self,
        client_id: &Self::ClientId,
        height: &Self::Height,
    ) -> Result<Option<Self::AnyConsensusState>, Self::Error>;

    fn get_any_consensus_state_before_height(
        &self,
        client_id: &Self::ClientId,
        height: &Self::Height,
    ) -> Result<Option<Self::AnyConsensusState>, Self::Error>;

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
