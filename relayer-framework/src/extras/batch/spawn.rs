use core::marker::PhantomData;

use crate::std_prelude::*;
use crate::traits::contexts::chain::IbcChainContext;
use crate::traits::contexts::relay::RelayContext;
use crate::traits::runtime::log::{HasLogger, LevelDebug};
use crate::traits::runtime::sleep::CanSleep;
use crate::traits::runtime::spawn::HasSpawner;
use crate::traits::runtime::time::HasTime;
use crate::traits::target::ChainTarget;

use super::config::BatchConfig;
use super::context::{BatchContext, HasBatchContext};
use super::message_sender::HasIbcMessageSenderForBatchWorker;
use super::worker::BatchMessageWorker;

pub struct BatchMessageWorkerSpawner<Target>(PhantomData<Target>);

pub trait CanSpawnBatchMessageWorker<Relay, Target>
where
    Relay: RelayContext,
    Target: ChainTarget<Relay>,
    Relay: HasBatchContext<Target>,
{
    fn spawn_batch_message_worker(
        relay: Relay,
        config: BatchConfig,
        messages_receiver: <Relay::BatchContext as BatchContext>::MessagesReceiver,
    );
}

impl<Relay, Target, TargetChain, Runtime> CanSpawnBatchMessageWorker<Relay, Target>
    for BatchMessageWorkerSpawner<Target>
where
    Relay: RelayContext<Runtime = Runtime>,
    Relay: HasIbcMessageSenderForBatchWorker<Target>,
    Runtime: HasTime + CanSleep + HasSpawner + HasLogger<LevelDebug>,
    Relay: HasBatchContext<Target>,
    Target: ChainTarget<Relay, TargetChain = TargetChain>,
    TargetChain: IbcChainContext<Target::CounterpartyChain>,
    Relay::Error: Clone,
{
    fn spawn_batch_message_worker(
        relay: Relay,
        config: BatchConfig,
        messages_receiver: <Relay::BatchContext as BatchContext>::MessagesReceiver,
    ) {
        <BatchMessageWorker<Relay, Target, Relay::IbcMessageSenderForBatchWorker>>::spawn_batch_message_worker(
            relay,
            config,
            messages_receiver,
        )
    }
}