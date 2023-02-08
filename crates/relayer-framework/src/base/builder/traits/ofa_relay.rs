use async_trait::async_trait;

use crate::base::core::traits::error::HasErrorType;
use crate::base::core::traits::sync::Async;
use crate::base::one_for_all::traits::relay::{OfaBaseRelay, OfaRelayPreset};
use crate::std_prelude::*;

pub trait HasOfaBaseRelay: Async {
    type Relay: OfaBaseRelay<Preset = Self::Preset>;

    type Preset: OfaRelayPreset<Self::Relay>;
}

#[async_trait]
pub trait CanBuildOfaBaseRelay: HasOfaBaseRelay + HasErrorType {
    async fn build_ofa_base_relay(self) -> Result<Self::Relay, Self::Error>;
}
