use async_trait::async_trait;

use crate::chain::traits::types::height::HasHeightType;
use crate::chain::traits::types::message::HasMessageType;
use crate::core::traits::error::HasErrorType;
use crate::link::traits::channel::{
    HasChannelHandshakePayloads, HasCounterpartyChannelPayloads, HasInitChannelOptionsType,
};
use crate::link::traits::ibc::HasIbcTypes;
use crate::std_prelude::*;

#[async_trait]
pub trait CanBuildChannelHandshakePayloads<Chain, Counterparty>:
    HasChannelHandshakePayloads<Chain, Counterparty> + HasIbcTypes<Chain, Counterparty> + HasErrorType
where
    Chain: HasHeightType,
{
    async fn build_channel_open_try_payload(
        chain: &Chain,
        height: &Chain::Height,
        port_id: &Self::PortId,
        channel_id: &Self::ChannelId,
    ) -> Result<Self::ChannelOpenTryPayload, Self::Error>;

    async fn build_channel_open_ack_payload(
        chain: &Chain,
        height: &Chain::Height,
        port_id: &Self::PortId,
        channel_id: &Self::ChannelId,
    ) -> Result<Self::ChannelOpenAckPayload, Self::Error>;

    async fn build_channel_open_confirm_payload(
        chain: &Chain,
        height: &Chain::Height,
        port_id: &Self::PortId,
        channel_id: &Self::ChannelId,
    ) -> Result<Self::ChannelOpenConfirmPayload, Self::Error>;
}

#[async_trait]
pub trait CanBuildChannelHandshakeMessages<Chain, Counterparty>:
    HasInitChannelOptionsType<Chain, Counterparty>
    + HasCounterpartyChannelPayloads
    + HasIbcTypes<Chain, Counterparty>
    + HasErrorType
where
    Chain: HasMessageType,
{
    async fn build_channel_open_init_message(
        chain: &Chain,
        port_id: &Self::PortId,
        counterparty_port_id: &Counterparty::PortId,
        init_channel_options: &Self::InitChannelOptions,
    ) -> Result<Chain::Message, Self::Error>;

    async fn build_channel_open_try_message(
        chain: &Chain,
        port_id: &Self::PortId,
        counterparty_port_id: &Counterparty::PortId,
        counterparty_channel_id: &Counterparty::ChannelId,
        counterparty_payload: Self::CounterpartyChannelOpenTryPayload,
    ) -> Result<Chain::Message, Self::Error>;

    async fn build_channel_open_ack_message(
        chain: &Chain,
        port_id: &Self::PortId,
        channel_id: &Self::ChannelId,
        counterparty_channel_id: &Counterparty::ChannelId,
        counterparty_payload: Self::CounterpartyChannelOpenAckPayload,
    ) -> Result<Chain::Message, Self::Error>;

    async fn build_channel_open_confirm_message(
        chain: &Chain,
        port_id: &Self::PortId,
        channel_id: &Self::ChannelId,
        counterparty_payload: Self::CounterpartyChannelOpenConfirmPayload,
    ) -> Result<Chain::Message, Self::Error>;
}
