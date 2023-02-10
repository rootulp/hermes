use async_trait::async_trait;

use crate::base::all_for_one::birelay::AfoBaseBiRelay;
use crate::base::core::traits::error::HasErrorType;
use crate::base::core::traits::sync::Async;
use crate::base::relay::traits::two_way::HasTwoWayRelay;
use crate::std_prelude::*;

use super::relay::CanBuildBaseRelay;

pub trait HasBaseBiRelay: Async {
    type BiRelay: AfoBaseBiRelay;
}

#[async_trait]
pub trait CanBuildBaseBiRelay: HasBaseBiRelay + HasErrorType {
    async fn build_base_birelay(&self) -> Result<Self::BiRelay, Self::Error>;
}

#[async_trait]
pub trait CanBuildBaseBiRelayFromRelays: HasBaseBiRelay + HasErrorType {
    async fn build_base_birelay(
        &self,
        relay_a_to_b: <Self::BiRelay as HasTwoWayRelay>::RelayAToB,
        relay_b_to_a: <Self::BiRelay as HasTwoWayRelay>::RelayBToA,
    ) -> Result<Self::BiRelay, Self::Error>;
}

pub trait HasRelayBuildersForBaseBiRelay: HasBaseBiRelay {
    type RelayAToBBuilder: CanBuildBaseRelay<Relay = <Self::BiRelay as HasTwoWayRelay>::RelayAToB>;

    type RelayBToABuilder: CanBuildBaseRelay<Relay = <Self::BiRelay as HasTwoWayRelay>::RelayBToA>;

    fn relay_a_to_b_builder(&self) -> Self::RelayAToBBuilder;

    fn relay_b_to_a_builder(&self) -> Self::RelayAToBBuilder;
}
