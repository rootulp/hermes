use async_trait::async_trait;

use crate::chain::traits::types::height::HasHeightType;
use crate::core::traits::error::HasErrorType;
use crate::link::traits::consensus_state::{
    HasConsensusStateType, HasCounterpartyConsensusStateType,
};
use crate::std_prelude::*;

#[async_trait]
pub trait ConsensusStateQuerier<Link, Chain, Counterparty>
where
    Link: HasCounterpartyConsensusStateType<Chain, Counterparty> + HasErrorType,
    Counterparty: HasHeightType,
{
    async fn query_consensus_state(
        chain: &Chain,
        client_id: &Link::ClientId,
        height: &Counterparty::Height,
    ) -> Result<Link::CounterpartyConsensusState, Link::Error>;
}

#[async_trait]
pub trait CanQueryConsensusState<Chain, Counterparty>:
    HasCounterpartyConsensusStateType<Chain, Counterparty> + HasErrorType
where
    Counterparty: HasHeightType,
{
    async fn query_consensus_state(
        chain: &Chain,
        client_id: &Self::ClientId,
        height: &Counterparty::Height,
    ) -> Result<Self::CounterpartyConsensusState, Self::Error>;
}
