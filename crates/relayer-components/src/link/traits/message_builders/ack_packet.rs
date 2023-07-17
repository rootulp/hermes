use async_trait::async_trait;

use crate::chain::traits::types::height::HasHeightType;
use crate::chain::traits::types::message::HasMessageType;
use crate::core::traits::error::HasErrorType;
use crate::link::traits::events::write_ack::HasWriteAcknowledgementEvent;
use crate::link::traits::packet::HasIncomingPacketType;
use crate::std_prelude::*;

#[async_trait]
pub trait CanBuildAckPacketMessage<Chain, Counterparty>:
    HasWriteAcknowledgementEvent<Chain, Counterparty>
    + HasIncomingPacketType<Chain, Counterparty>
    + HasErrorType
where
    Chain: HasHeightType,
    Counterparty: HasMessageType,
{
    async fn build_ack_packet_message(
        chain: &Chain,
        height: &Chain::Height,
        packet: &Self::IncomingPacket,
        ack: &Self::WriteAcknowledgementEvent,
    ) -> Result<Counterparty::Message, Self::Error>;
}
