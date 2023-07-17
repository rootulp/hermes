use async_trait::async_trait;

use crate::chain::traits::types::height::HasHeightType;
use crate::chain::traits::types::message::HasMessageType;
use crate::core::traits::error::HasErrorType;
use crate::link::traits::packet::HasOutgoingPacketType;
use crate::std_prelude::*;

#[async_trait]
pub trait CanBuildReceivePacketMessage<Chain, Counterparty>:
    HasOutgoingPacketType<Chain, Counterparty> + HasErrorType
where
    Chain: HasHeightType,
    Counterparty: HasMessageType,
{
    async fn build_receive_packet_message(
        chain: &Chain,
        height: &Chain::Height,
        packet: &Self::OutgoingPacket,
    ) -> Result<Counterparty::Message, Self::Error>;
}
