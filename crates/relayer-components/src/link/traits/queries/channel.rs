use async_trait::async_trait;

use crate::chain::traits::types::chain_id::HasChainIdType;
use crate::core::traits::error::HasErrorType;
use crate::link::traits::ibc::HasIbcTypes;
use crate::std_prelude::*;

#[async_trait]
pub trait CanQueryCounterpartyChainIdFromChannel<Chain, Counterparty>:
    HasIbcTypes<Chain, Counterparty> + HasErrorType
where
    Counterparty: HasChainIdType,
{
    async fn query_chain_id_from_channel_id(
        chain: &Chain,
        channel_id: &Self::ChannelId,
        port_id: &Self::PortId,
    ) -> Result<Counterparty::ChainId, Self::Error>;
}
