use async_trait::async_trait;

use crate::base::all_for_one::runtime::HasAfoBaseRuntime;
use crate::base::core::traits::error::HasErrorType;
use crate::std_prelude::*;

#[async_trait]
pub trait CanBuildBaseRuntime: HasAfoBaseRuntime + HasErrorType {
    async fn build_base_runtime(&self) -> Self::Runtime;
}
