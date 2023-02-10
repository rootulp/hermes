use async_trait::async_trait;

use crate::base::core::traits::error::HasErrorType;
use crate::base::core::traits::sync::Async;
use crate::base::one_for_all::traits::chain::{OfaIbcChain, OfaIbcChainPreset};
use crate::std_prelude::*;

pub trait HasOfaBaseChain: Async {
    type Chain: OfaIbcChain<Self::Counterparty, Preset = Self::Preset>;

    type Counterparty: OfaIbcChain<
        Self::Chain,
        IncomingPacket = <Self::Chain as OfaIbcChain<Self::Counterparty>>::OutgoingPacket,
        OutgoingPacket = <Self::Chain as OfaIbcChain<Self::Counterparty>>::IncomingPacket,
    >;

    type Preset: OfaIbcChainPreset<Self::Chain, Self::Counterparty>;
}

#[async_trait]
pub trait CanBuildOfaBaseChain: HasOfaBaseChain + HasErrorType {
    async fn build_ofa_base_chain(self) -> Result<Self::Chain, Self::Error>;
}
