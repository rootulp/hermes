use crate::core::traits::sync::Async;

pub trait HasIbcTypes: Async {
    type Height: Async;

    type Timestamp: Async;

    type Message: Async;

    type ClientId: Async;

    type MerkleProof: Async;
}
