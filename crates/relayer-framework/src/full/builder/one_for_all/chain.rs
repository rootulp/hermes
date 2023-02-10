use async_trait::async_trait;

use crate::base::builder::traits::one_for_all::chain::{CanBuildOfaBaseChain, HasOfaBaseChain};
use crate::base::core::traits::error::HasErrorType;
use crate::full::one_for_all::traits::chain::OfaFullChain;
use crate::std_prelude::*;

pub trait HasOfaFullChain: HasOfaBaseChain<Chain = Self::FullChain> {
    type FullChain: OfaFullChain;
}

impl<Builder> HasOfaFullChain for Builder
where
    Builder: HasOfaBaseChain,
    Builder::Chain: OfaFullChain,
{
    type FullChain = Builder::Chain;
}

#[async_trait]
pub trait CanBuildOfaFullChain: HasOfaFullChain + HasErrorType {
    async fn build_ofa_full_chain(self) -> Result<Self::Chain, Self::Error>;
}

#[async_trait]
impl<Builder> CanBuildOfaFullChain for Builder
where
    Builder: HasOfaFullChain + CanBuildOfaBaseChain,
{
    async fn build_ofa_full_chain(self) -> Result<Self::Chain, Self::Error> {
        self.build_ofa_base_chain().await
    }
}
