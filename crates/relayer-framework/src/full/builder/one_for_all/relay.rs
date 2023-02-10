use async_trait::async_trait;

use crate::base::builder::traits::one_for_all::relay::{CanBuildOfaBaseRelay, HasOfaBaseRelay};
use crate::base::core::traits::error::HasErrorType;
use crate::full::one_for_all::traits::relay::OfaFullRelay;
use crate::std_prelude::*;

pub trait HasOfaFullRelay: HasOfaBaseRelay<Relay = Self::FullRelay> {
    type FullRelay: OfaFullRelay;
}

impl<Builder> HasOfaFullRelay for Builder
where
    Builder: HasOfaBaseRelay,
    Builder::Relay: OfaFullRelay,
{
    type FullRelay = Builder::Relay;
}

#[async_trait]
pub trait CanBuildOfaFullRelay: HasOfaFullRelay + HasErrorType {
    async fn build_ofa_full_relay(self) -> Result<Self::Relay, Self::Error>;
}

#[async_trait]
impl<Builder> CanBuildOfaFullRelay for Builder
where
    Builder: HasOfaFullRelay + CanBuildOfaBaseRelay,
{
    async fn build_ofa_full_relay(self) -> Result<Self::Relay, Self::Error> {
        self.build_ofa_base_relay().await
    }
}
