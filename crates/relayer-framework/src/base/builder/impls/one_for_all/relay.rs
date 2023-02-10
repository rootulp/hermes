use async_trait::async_trait;

use crate::base::builder::traits::one_for_all::relay::{CanBuildOfaBaseRelay, HasOfaBaseRelay};
use crate::base::builder::traits::relay::{CanBuildBaseRelay, HasBaseRelay};
use crate::base::core::traits::error::HasErrorType;
use crate::base::one_for_all::types::relay::OfaRelayWrapper;
use crate::std_prelude::*;

pub struct OfaRelayToRelayBuilder<InBuilder> {
    pub in_builder: InBuilder,
}

impl<InBuilder> HasBaseRelay for OfaRelayToRelayBuilder<InBuilder>
where
    InBuilder: HasOfaBaseRelay,
{
    type Relay = OfaRelayWrapper<InBuilder::Relay>;
}

impl<InBuilder> HasErrorType for OfaRelayToRelayBuilder<InBuilder>
where
    InBuilder: HasErrorType,
{
    type Error = InBuilder::Error;
}

#[async_trait]
impl<InBuilder> CanBuildBaseRelay for OfaRelayToRelayBuilder<InBuilder>
where
    InBuilder: CanBuildOfaBaseRelay,
{
    async fn build_base_relay(&self) -> Result<Self::Relay, Self::Error> {
        let relay = self.in_builder.build_ofa_base_relay().await?;
        Ok(OfaRelayWrapper::new(relay))
    }
}
