use async_trait::async_trait;

use crate::base::builder::traits::chain::{CanBuildBaseChain, HasBaseChain};
use crate::base::core::traits::error::HasErrorType;
use crate::full::all_for_one::chain::AfoFullChain;
use crate::std_prelude::*;

pub trait HasFullChain: HasBaseChain<Chain = Self::FullChain> {
    type FullChain: AfoFullChain<Self::Counterparty>;
}

impl<Context> HasFullChain for Context
where
    Context: HasBaseChain,
    Context::Chain: AfoFullChain<Context::Counterparty>,
{
    type FullChain = Context::Chain;
}

#[async_trait]
pub trait CanBuildFullChain: HasFullChain + HasErrorType {
    async fn build_full_chain(self) -> Result<Self::Chain, Self::Error>;
}

#[async_trait]
impl<Context> CanBuildFullChain for Context
where
    Context: HasFullChain + CanBuildBaseChain,
{
    async fn build_full_chain(self) -> Result<Self::Chain, Self::Error> {
        self.build_base_chain().await
    }
}
