use async_trait::async_trait;

use crate::base::chain::traits::ibc_event::HasSendPacketEvent;
use crate::base::chain::traits::types::event::{HasEventSource, HasEventType};
use crate::base::core::traits::sync::Async;
use crate::base::relay::traits::packet_relayer::CanRelayPacket;
use crate::base::relay::traits::target::{ChainTarget, SourceTarget};
use crate::base::relay::traits::types::HasRelayTypes;
use crate::std_prelude::*;

#[async_trait]
pub trait EventRelayer<Relay, Target>: Async
where
    Relay: HasRelayTypes,
    Target: ChainTarget<Relay>,
    Target::TargetChain: HasEventSource,
{
    /**
       Should never return, unless some failure occured.
    */
    async fn relay_chain_events(
        relay: &Relay,
        event_source: &<Target::TargetChain as HasEventSource>::EventSource,
    ) -> Result<(), Relay::Error>;
}

#[async_trait]
pub trait CanRelayEvents<Target>: HasRelayTypes
where
    Target: ChainTarget<Self>,
    Target::TargetChain: HasEventSource,
{
    async fn relay_chain_events(
        &self,
        event_source: &<Target::TargetChain as HasEventSource>::EventSource,
    ) -> Result<(), Self::Error>;
}

struct SequentialSendPacketEventRelayer;

#[async_trait]
impl<Relay> EventRelayer<Relay, SourceTarget> for SequentialSendPacketEventRelayer
where
    Relay: CanRelayPacket,
    Relay::SrcChain: HasEventSource + HasSendPacketEvent<Relay::DstChain>,
{
    async fn relay_chain_events(
        relay: &Relay,
        event_source: &<Relay::SrcChain as HasEventSource>::EventSource,
    ) -> Result<(), Relay::Error> {
        loop {
            let event = Relay::SrcChain::receive_event(event_source)
                .await
                .map_err(Relay::src_chain_error)?;

            if let Some(send_packet_event) = Relay::SrcChain::try_extract_send_packet_event(event) {
                let packet =
                    Relay::SrcChain::extract_packet_from_send_packet_event(&send_packet_event);

                relay.relay_packet(packet).await?;

                // relay.runtime().spawn(async {
                // relay.relay_packet(packet).await?;
                // });
            }
        }
    }
}
