use async_trait::async_trait;

use crate::base::all_for_one::relay::AfoBaseRelay;
use crate::base::builder::traits::chain::CanBuildBaseChain;
use crate::base::core::traits::error::HasErrorType;
use crate::base::core::traits::sync::Async;
use crate::base::relay::traits::types::HasRelayTypes;
use crate::base::runtime::traits::runtime::HasRuntime;
use crate::std_prelude::*;

use super::runtime::CanBuildBaseRuntime;

pub trait HasBaseRelay: Async {
    type Relay: AfoBaseRelay;
}

#[async_trait]
pub trait CanBuildBaseRelay: HasBaseRelay + HasErrorType {
    async fn build_base_relay(&self) -> Result<Self::Relay, Self::Error>;
}

#[async_trait]
pub trait CanBuildBaseRelayWithChainsAndRuntime: HasBaseRelay + HasErrorType {
    async fn build_base_relay_with_chains_and_runtime(
        &self,
        runtime: <Self::Relay as HasRuntime>::Runtime,
        src_chain: <Self::Relay as HasRelayTypes>::SrcChain,
        dst_chain: <Self::Relay as HasRelayTypes>::DstChain,
    ) -> Result<Self::Relay, Self::Error>;
}

pub trait HasChainAndRuntimeBuildersForBaseRelay: HasBaseRelay {
    type RuntimeBuilder: CanBuildBaseRuntime<Runtime = <Self::Relay as HasRuntime>::Runtime>;

    type SrcChainBuilder: CanBuildBaseChain<
        Chain = <Self::Relay as HasRelayTypes>::SrcChain,
        Counterparty = <Self::Relay as HasRelayTypes>::DstChain,
    >;

    type DstChainBuilder: CanBuildBaseChain<
        Chain = <Self::Relay as HasRelayTypes>::DstChain,
        Counterparty = <Self::Relay as HasRelayTypes>::SrcChain,
    >;

    fn src_chain_builder(&self) -> Self::SrcChainBuilder;

    fn dst_chain_builder(&self) -> Self::DstChainBuilder;
}
