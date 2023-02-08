use async_trait::async_trait;

use crate::base::all_for_one::chain::{AfoBaseChain, AfoCounterpartyChain};
use crate::base::core::traits::error::HasErrorType;
use crate::base::core::traits::sync::Async;
use crate::std_prelude::*;

pub trait HasBaseChain: Async {
    type Chain: AfoBaseChain<Self::Counterparty>;
    type Counterparty: AfoCounterpartyChain<Self::Chain>;
}

#[async_trait]
pub trait CanBuildChain: HasBaseChain + HasErrorType {
    async fn build_chain(self) -> Result<Self::Chain, Self::Error>;
}
