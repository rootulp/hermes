use core::future::Future;
use core::pin::Pin;

use async_trait::async_trait;
use futures::stream::{self, StreamExt};

use crate::relay::traits::auto_relayer::{AutoRelayer, CanAutoRelay};
use crate::relay::traits::two_way::HasTwoWayRelay;
use crate::std_prelude::*;

pub struct ConcurrentTwoWayAutoRelay;

#[async_trait]
impl<BiRelay> AutoRelayer<BiRelay> for ConcurrentTwoWayAutoRelay
where
    BiRelay: HasTwoWayRelay,
    BiRelay::RelayAToB: CanAutoRelay,
    BiRelay::RelayBToA: CanAutoRelay,
{
    async fn auto_relay(birelay: &BiRelay) {
        let a_to_b_task: Pin<Box<dyn Future<Output = ()> + Send>> = Box::pin(async move {
            BiRelay::RelayAToB::auto_relay(birelay.relay_a_to_b()).await;
        });

        let b_to_a_task: Pin<Box<dyn Future<Output = ()> + Send>> = Box::pin(async move {
            BiRelay::RelayBToA::auto_relay(birelay.relay_b_to_a()).await;
        });

        stream::iter([a_to_b_task, b_to_a_task])
            .for_each_concurrent(None, |task| task)
            .await;
    }
}