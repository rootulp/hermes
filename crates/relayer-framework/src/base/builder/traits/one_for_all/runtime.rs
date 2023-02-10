use crate::base::core::traits::error::HasErrorType;
use crate::base::core::traits::sync::Async;
use crate::base::one_for_all::traits::runtime::OfaBaseRuntime;
use crate::std_prelude::*;
use async_trait::async_trait;

pub trait HasOfaBaseRuntime: Async {
    type Runtime: OfaBaseRuntime;
}

#[async_trait]
pub trait CanBuildOfaBaseRuntime: HasOfaBaseRuntime + HasErrorType {
    async fn build_ofa_base_runtime(&self) -> Self::Runtime;
}
