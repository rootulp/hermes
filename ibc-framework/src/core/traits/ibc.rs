use crate::core::traits::sync::Async;

pub trait HasHostTypes: Async {
    type Height: Async;

    type Timestamp: Ord + Async;

    // Require non-negative duration
    type Duration: Ord + Async;
}

pub trait HasIbcTypes: HasHostTypes {
    type ClientId: Async;

    type MerkleProof: Async;

    type Message: Async;

    type MessageType: Eq + Async;

    type Signer: Async;
}

pub trait HasIbcMethods: HasIbcTypes {
    fn message_type(message: &Self::Message) -> Self::MessageType;

    fn add_duration(time: &Self::Timestamp, duration: &Self::Duration) -> Self::Timestamp;
}
