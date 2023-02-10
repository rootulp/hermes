use async_trait::async_trait;

use crate::base::builder::traits::chain::{CanBuildBaseChain, HasBaseChain};
use crate::base::core::traits::error::HasErrorType;
use crate::full::all_for_one::chain::AfoFullChain;
use crate::std_prelude::*;

pub trait HasFullChain: HasBaseChain<Chain = Self::FullChain> {
    type FullChain: AfoFullChain<Self::Counterparty>;
}

impl<Builder> HasFullChain for Builder
where
    Builder: HasBaseChain,
    Builder::Chain: AfoFullChain<Builder::Counterparty>,
{
    type FullChain = Builder::Chain;
}

#[async_trait]
pub trait CanBuildFullChain: HasFullChain + HasErrorType {
    async fn build_full_chain(&self) -> Result<Self::Chain, Self::Error>;
}

#[async_trait]
impl<Builder> CanBuildFullChain for Builder
where
    Builder: HasFullChain + CanBuildBaseChain,
{
    async fn build_full_chain(&self) -> Result<Self::Chain, Self::Error> {
        self.build_base_chain().await
    }
}
