use crate::core::traits::client::HasAnyClientMethods;
use crate::core::traits::sync::Async;

pub trait OfaChain: Async {
    type Components;

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

    fn client_state_type(client_state: &Self::AnyClientState) -> Self::ClientType;

    fn client_state_is_frozen(client_state: &Self::AnyClientState) -> bool;

    fn client_state_trusting_period(client_state: &Self::AnyClientState) -> Self::Duration;

    fn consensus_state_height(consensus_state: &Self::AnyConsensusState) -> Self::Height;

    fn consensus_state_timestamp(consensus_state: &Self::AnyConsensusState) -> Self::Timestamp;
}
