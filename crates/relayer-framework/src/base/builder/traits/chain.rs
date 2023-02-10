use async_trait::async_trait;

use crate::base::all_for_one::chain::{AfoBaseChain, AfoCounterpartyChain};
use crate::base::core::traits::error::HasErrorType;
use crate::base::core::traits::sync::Async;
use crate::base::runtime::traits::runtime::HasRuntime;
use crate::std_prelude::*;

use super::runtime::CanBuildBaseRuntime;

pub trait HasBaseChain: Async {
    type Chain: AfoBaseChain<Self::Counterparty>;
    type Counterparty: AfoCounterpartyChain<Self::Chain>;
}

#[async_trait]
pub trait CanBuildBaseChain: HasBaseChain + HasErrorType {
    async fn build_base_chain(&self) -> Result<Self::Chain, Self::Error>;
}

#[async_trait]
pub trait CanBuildBaseChainWithRuntime: HasBaseChain + HasErrorType {
    async fn build_base_chain_with_runtime(
        &self,
        runtime: <Self::Chain as HasRuntime>::Runtime,
    ) -> Result<Self::Chain, Self::Error>;
}

pub trait HasRuntimeBuilderForChain: HasBaseChain {
    type RuntimeBuilder: CanBuildBaseRuntime<Runtime = <Self::Chain as HasRuntime>::Runtime>;
}
