use async_trait::async_trait;

use crate::chain::traits::types::message::HasMessageType;
use crate::core::traits::error::HasErrorType;
use crate::link::traits::client_state::HasCounterpartyClientStateType;
use crate::link::traits::consensus_state::HasCounterpartyConsensusStateType;
use crate::std_prelude::*;

#[async_trait]
pub trait CanBuildCreateClientMessage<Chain, Counterparty>:
    HasCounterpartyClientStateType<Chain, Counterparty>
    + HasCounterpartyConsensusStateType<Chain, Counterparty>
    + HasErrorType
where
    Chain: HasMessageType,
{
    async fn build_create_client_message(
        client_state: &Self::CounterpartyClientState,
        consensus_state: &Self::CounterpartyConsensusState,
    ) -> Result<Chain::Message, Self::Error>;
}
