use async_trait::async_trait;

use crate::chain::traits::types::event::HasEventType;
use crate::chain::traits::types::message::HasMessageType;
use crate::core::traits::error::HasErrorType;
use crate::core::traits::sync::Async;
use crate::link::traits::ibc::HasIbcTypes;
use crate::std_prelude::*;

pub trait HasCreateClientOptions<Chain, Counterparty>: Async {
    type CreateClientPayloadOptions: Async;
}

pub trait HasCreateClientPayload<Chain, Counterparty>: Async {
    type CreateClientPayload: Async;
}

pub trait HasCounterpartyCreateClientPayload<Chain, Counterparty>:
    HasCreateClientPayload<
    Counterparty,
    Chain,
    CreateClientPayload = Self::CounterpartyCreateClientPayload,
>
{
    type CounterpartyCreateClientPayload: Async;
}

impl<Link, Chain, Counterparty> HasCounterpartyCreateClientPayload<Chain, Counterparty> for Link
where
    Link: HasCreateClientPayload<Counterparty, Chain>,
{
    type CounterpartyCreateClientPayload = Link::CreateClientPayload;
}

pub trait HasCreateClientEvent<Chain, Counterparty>: HasIbcTypes<Chain, Counterparty>
where
    Chain: HasEventType,
{
    type CreateClientEvent: Async;

    fn try_extract_create_client_event(event: Chain::Event) -> Option<Self::CreateClientEvent>;

    fn create_client_event_client_id(event: &Self::CreateClientEvent) -> &Self::ClientId;
}

#[async_trait]
pub trait CanBuildCreateClientPayload<Chain, Counterparty>:
    HasCreateClientOptions<Chain, Counterparty>
    + HasCreateClientPayload<Chain, Counterparty>
    + HasErrorType
{
    async fn build_create_client_payload(
        chain: &Chain,
        create_client_options: &Self::CreateClientPayloadOptions,
    ) -> Result<Self::CreateClientPayload, Self::Error>;
}

#[async_trait]
pub trait CanBuildCreateClientMessage<Chain, Counterparty>:
    HasCounterpartyCreateClientPayload<Chain, Counterparty> + HasErrorType
where
    Chain: HasMessageType,
{
    async fn build_create_client_message(
        chain: &Chain,
        counterparty_payload: Self::CounterpartyCreateClientPayload,
    ) -> Result<Chain::Message, Self::Error>;
}
