use crate::core::traits::sync::Async;

pub trait HasInitChannelOptionsType<Chain, Counterparty>: Async {
    type InitChannelOptions: Async;
}

/**
    Payload that contains necessary counterparty information such as proofs and parameters
    in order for a self chain to build a channel handshake message.
*/
pub trait HasChannelHandshakePayloads<Chain, Counterparty>: Async {
    type ChannelOpenTryPayload: Async;

    type ChannelOpenAckPayload: Async;

    type ChannelOpenConfirmPayload: Async;
}

pub trait HasCounterpartyChannelPayloads<Chain, Counterparty>:
    HasChannelHandshakePayloads<
    Counterparty,
    Chain,
    ChannelOpenTryPayload = Self::CountepartyChannelOpenTryPayload,
    ChannelOpenAckPayload = Self::CountepartyChannelOpenAckPayload,
    ChannelOpenConfirmPayload = Self::CountepartyChannelOpenConfirmPayload,
>
{
    type CounterpartyChannelOpenTryPayload: Async;

    type CounterpartyChannelOpenAckPayload: Async;

    type CounterpartyChannelOpenConfirmPayload: Async;
}

impl<Link, Chain, Counterparty> HasCounterpartyChannelPayloads<Chain, Counterparty> for Link
where
    Link: HasChannelHandshakePayloads<Counterparty, Chain>,
{
    type CounterpartyChannelOpenTryPayload = Link::ChannelOpenTryPayload;

    type CounterpartyChannelOpenAckPayload = Link::ChannelOpenAckPayload;

    type CounterpartyChannelOpenConfirmPayload = Link::ChannelOpenConfirmPayload;
}
