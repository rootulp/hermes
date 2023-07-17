use crate::core::traits::sync::Async;

pub trait HasInitConnectionOptionsType<Chain, Counterparty>: Async {
    type InitConnectionOptions: Async;
}

/**
    Payload that contains necessary counterparty information such as proofs and parameters
    in order for a self chain to build a connection handshake message.
*/
pub trait HasConnectionHandshakePayloads<Chain, Counterparty>: Async {
    type ConnectionOpenInitPayload: Async;

    type ConnectionOpenTryPayload: Async;

    type ConnectionOpenAckPayload: Async;

    type ConnectionOpenConfirmPayload: Async;
}

pub trait HasCounterpartyConnectionPayloads<Chain, Counterparty>:
    HasConnectionHandshakePayloads<
    Counterparty,
    Chain,
    ConnectionOpenInitPayload = Self::CountepartyConnectionOpenInitPayload,
    ConnectionOpenTryPayload = Self::CountepartyConnectionOpenTryPayload,
    ConnectionOpenAckPayload = Self::CountepartyConnectionOpenAckPayload,
    ConnectionOpenConfirmPayload = Self::CountepartyConnectionOpenConfirmPayload,
>
{
    type CounterpartyConnectionOpenInitPayload: Async;

    type CounterpartyConnectionOpenTryPayload: Async;

    type CounterpartyConnectionOpenAckPayload: Async;

    type CounterpartyConnectionOpenConfirmPayload: Async;
}

impl<Link, Chain, Counterparty> HasCounterpartyConnectionPayloads<Chain, Counterparty> for Link
where
    Link: HasConnectionHandshakePayloads<Counterparty, Chain>,
{
    type CounterpartyConnectionOpenInitPayload = Link::ConnectionOpenInitPayload;

    type CounterpartyConnectionOpenTryPayload = Link::ConnectionOpenTryPayload;

    type CounterpartyConnectionOpenAckPayload = Link::ConnectionOpenAckPayload;

    type CounterpartyConnectionOpenConfirmPayload = Link::ConnectionOpenConfirmPayload;
}
