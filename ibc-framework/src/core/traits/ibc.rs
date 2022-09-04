use crate::core::traits::sync::Async;

pub trait HasIbcTypes: Async {
    type Height: Async;

    type Timestamp: Async;

    type ClientId: Async;

    type MerkleProof: Async;

    type Message: Async;

    type MessageType: Eq + Async;

    type Signer: Async;

    fn message_type(message: &Self::Message) -> Self::MessageType;
}
