use async_trait::async_trait;

use crate::core::traits::error::HasErrorType;
use crate::link::traits::ibc::HasIbcTypes;
use crate::link::traits::packet::HasIncomingPacketType;
use crate::std_prelude::*;

#[async_trait]
pub trait CanQueryReceivedPacket<Chain, Counterparty>:
    HasIncomingPacketType<Chain, Counterparty> + HasErrorType
{
    async fn query_is_packet_received(
        chain: &Chain,
        packet: &Self::IncomingPacket,
    ) -> Result<bool, Self::Error>;
}

#[async_trait]
pub trait ReceivedPacketQuerier<Link, Chain, Counterparty>
where
    Link: HasIncomingPacketType<Chain, Counterparty> + HasErrorType,
{
    async fn query_is_packet_received(
        chain: &Chain,
        packet: &Link::IncomingPacket,
    ) -> Result<bool, Link::Error>;
}
