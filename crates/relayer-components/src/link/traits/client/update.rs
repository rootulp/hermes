use async_trait::async_trait;

use crate::core::traits::error::HasErrorType;
use crate::core::traits::sync::Async;
use crate::link::traits::client_state::HasClientStateType;
use crate::std_prelude::*;

pub trait HasUpdateClientPayload<Chain, Counterparty>: Async {
    type UpdateClientPayload: Async;
}

pub trait HasCounterpartyUpdateClientPayload<Chain, Counterparty>:
    HasUpdateClientPayload<
    Counterparty,
    Chain,
    UpdateClientPayload = Self::CounterpartyUpdateClientPayload,
>
{
    type CounterpartyUpdateClientPayload: Async;
}

#[async_trait]
pub trait CanBuildUpdateClientPayload<Chain, Counterparty>:
    HasUpdateClientPayload<Chain, Counterparty> + HasClientStateType<Chain, Counterparty> + HasErrorType
{
    async fn build_update_client_payload(
        &self,
        trusted_height: &Self::Height,
        target_height: &Self::Height,
        client_state: Self::ClientState,
    ) -> Result<Self::UpdateClientPayload, Self::Error>;
}

#[async_trait]
pub trait CanBuildUpdateClientMessage<Chain, Counterparty>:
    HasCounterpartyUpdateClientPayload<Chain, Counterparty> + HasErrorType
where
    Counterparty: HasUpdateClientPayload<Self>,
{
    async fn build_update_client_message(
        chain: &Chain,
        client_id: &Self::ClientId,
        payload: Self::CounterpartyUpdateClientPayload,
    ) -> Result<Vec<Self::Message>, Self::Error>;
}
