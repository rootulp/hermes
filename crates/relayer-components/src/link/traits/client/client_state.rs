use async_trait::async_trait;

use crate::core::traits::error::HasErrorType;
use crate::link::traits::client_state::HasCounterpartyClientStateType;
use crate::link::traits::ibc::HasIbcTypes;
use crate::std_prelude::*;

#[async_trait]
pub trait CanQueryClientState<Chain, Counterparty>:
    HasIbcTypes<Chain, Counterparty>
    + HasCounterpartyClientStateType<Chain, Counterparty>
    + HasErrorType
{
    async fn query_client_state(
        chain: &Chain,
        client_id: &Self::ClientId,
    ) -> Result<Self::CounterpartyClientState, Self::Error>;
}
