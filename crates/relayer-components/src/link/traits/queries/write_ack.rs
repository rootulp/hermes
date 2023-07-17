use async_trait::async_trait;

use crate::core::traits::error::HasErrorType;
use crate::link::traits::events::write_ack::HasWriteAcknowledgementEvent;
use crate::link::traits::packet::HasIncomingPacketType;
use crate::std_prelude::*;

#[async_trait]
pub trait CanQueryWriteAcknowledgement<Chain, Counterparty>:
    HasWriteAcknowledgementEvent<Chain, Counterparty>
    + HasIncomingPacketType<Chain, Counterparty>
    + HasErrorType
{
    async fn query_write_acknowledgement_event(
        chain: &Chain,
        packet: &Self::IncomingPacket,
    ) -> Result<Option<Self::WriteAcknowledgementEvent>, Self::Error>;
}
