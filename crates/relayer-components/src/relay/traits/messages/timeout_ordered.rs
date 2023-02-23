use async_trait::async_trait;

use crate::chain::types::aliases::{Height, Message};
use crate::relay::traits::types::HasRelayTypes;
use crate::std_prelude::*;

#[async_trait]
pub trait TimeoutOrderedPacketMessageBuilder<Relay: HasRelayTypes> {
    async fn build_timeout_ordered_packet_message(
        relay: &Relay,
        height: Height<Relay::DstChain>,
        packet: &Relay::Packet,
    ) -> Result<Message<Relay::SrcChain>, Relay::Error>;
}