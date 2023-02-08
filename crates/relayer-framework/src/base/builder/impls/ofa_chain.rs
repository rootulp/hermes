use async_trait::async_trait;

use crate::base::builder::traits::base_chain::{CanBuildBaseChain, HasBaseChain};
use crate::base::builder::traits::ofa_chain::{CanBuildOfaBaseChain, HasOfaBaseChain};
use crate::base::core::traits::error::HasErrorType;
use crate::base::one_for_all::types::chain::OfaChainWrapper;
use crate::std_prelude::*;

pub struct OfaChainToChainBuilder<InBuilder> {
    pub in_builder: InBuilder,
}

impl<InBuilder> HasBaseChain for OfaChainToChainBuilder<InBuilder>
where
    InBuilder: HasOfaBaseChain,
{
    type Chain = OfaChainWrapper<InBuilder::Chain>;
    type Counterparty = OfaChainWrapper<InBuilder::Counterparty>;
}

impl<InBuilder> HasErrorType for OfaChainToChainBuilder<InBuilder>
where
    InBuilder: HasOfaBaseChain + HasErrorType,
{
    type Error = InBuilder::Error;
}

#[async_trait]
impl<InBuilder> CanBuildBaseChain for OfaChainToChainBuilder<InBuilder>
where
    InBuilder: CanBuildOfaBaseChain,
{
    async fn build_base_chain(self) -> Result<Self::Chain, Self::Error> {
        let chain = self.in_builder.build_ofa_base_chain().await?;

        Ok(OfaChainWrapper::new(chain))
    }
}
