use async_trait::async_trait;

use crate::base::builder::traits::relay::{CanBuildBaseRelay, HasBaseRelay};
use crate::base::core::traits::error::HasErrorType;
use crate::full::all_for_one::relay::AfoFullRelay;
use crate::std_prelude::*;

pub trait HasFullRelay: HasBaseRelay<Relay = Self::FullRelay> {
    type FullRelay: AfoFullRelay;
}

impl<Context> HasFullRelay for Context
where
    Context: HasBaseRelay,
    Context::Relay: AfoFullRelay,
{
    type FullRelay = Context::Relay;
}

#[async_trait]
pub trait CanBuildFullRelay: HasFullRelay + HasErrorType {
    async fn build_full_relay(self) -> Result<Self::Relay, Self::Error>;
}

#[async_trait]
impl<Context> CanBuildFullRelay for Context
where
    Context: HasFullRelay + CanBuildBaseRelay,
{
    async fn build_full_relay(self) -> Result<Self::Relay, Self::Error> {
        self.build_base_relay().await
    }
}
