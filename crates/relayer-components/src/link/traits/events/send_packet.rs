/*!
   Trait definitions for [`HasSendPacketEvent`].
*/

use crate::chain::traits::types::event::HasEventType;
use crate::core::traits::sync::Async;
use crate::link::traits::packet::HasOutgoingPacketType;

/**
   Indicates that a chain context's
   [`Event`](crate::chain::traits::types::event::HasEventType::Event)
   type contains a [`SendPacketEvent`](Self::SendPacketEvent) variant.
*/
pub trait HasSendPacketEvent<Chain, Counterparty>:
    HasOutgoingPacketType<Chain, Counterparty>
where
    Chain: HasEventType,
{
    type SendPacketEvent: Async;

    fn try_extract_send_packet_event(event: &Chain::Event) -> Option<Self::SendPacketEvent>;

    fn extract_packet_from_send_packet_event(event: &Self::SendPacketEvent)
        -> Self::OutgoingPacket;
}
