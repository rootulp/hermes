use async_trait::async_trait;

use crate::base::all_for_one::relay::AfoBaseRelay;
use crate::base::core::traits::error::HasErrorType;
use crate::base::core::traits::sync::Async;
use crate::std_prelude::*;

pub trait HasBaseRelay: Async {
    type Relay: AfoBaseRelay;
}

#[async_trait]
pub trait CanBuildBaseRelay: HasBaseRelay + HasErrorType {
    async fn build_base_relay(&self) -> Result<Self::Relay, Self::Error>;
}
