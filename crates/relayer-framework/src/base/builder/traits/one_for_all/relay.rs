use alloc::sync::Arc;
use async_trait::async_trait;

use crate::base::builder::traits::one_for_all::chain::CanBuildOfaBaseChain;
use crate::base::core::traits::error::HasErrorType;
use crate::base::core::traits::sync::Async;
use crate::base::one_for_all::traits::relay::{OfaBaseRelay, OfaRelayPreset, OfaRelayTypes};
use crate::std_prelude::*;

pub trait HasOfaBaseRelay: Async {
    type Relay: OfaBaseRelay<Preset = Self::Preset>;

    type Preset: OfaRelayPreset<Self::Relay>;
}

#[async_trait]
pub trait CanBuildOfaBaseRelay: HasOfaBaseRelay + HasErrorType {
    async fn build_ofa_base_relay(&self) -> Result<Self::Relay, Self::Error>;
}

#[async_trait]
pub trait CanBuildOfaBaseRelayWithChains: HasOfaBaseRelay + HasErrorType {
    async fn build_ofa_base_relay_with_chains(
        &self,
        src_chain: Arc<<Self::Relay as OfaRelayTypes>::SrcChain>,
        dst_chain: Arc<<Self::Relay as OfaRelayTypes>::DstChain>,
    ) -> Result<Self::Relay, Self::Error>;
}

#[async_trait]
pub trait HasOfaBaseRelayChainBuilders: Async {
    type SrcChainBuilder: CanBuildOfaBaseChain;

    type DstChainBuilder: CanBuildOfaBaseChain;

    fn src_chain_builder(&self) -> &Self::SrcChainBuilder;

    fn dst_chain_builder(&self) -> &Self::DstChainBuilder;
}
