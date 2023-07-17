use async_trait::async_trait;

use crate::chain::traits::types::height::HasHeightType;
use crate::core::traits::error::HasErrorType;
use crate::link::traits::connection::{
    HasConnectionHandshakePayloads, HasInitConnectionOptionsType,
};
use crate::link::traits::ibc::HasIbcTypes;
use crate::std_prelude::*;

#[async_trait]
pub trait CanBuildConnectionHandshakePayloads<Chain, Counterparty>:
    HasConnectionHandshakePayloads<Chain, Counterparty>
    + HasIbcTypes<Chain, Counterparty>
    + HasErrorType
where
    Chain: HasHeightType,
{
    async fn build_connection_open_init_payload(
        chain: &Chain,
    ) -> Result<Self::ConnectionOpenInitPayload, Self::Error>;

    async fn build_connection_open_try_payload(
        chain: &Chain,
        height: &Chain::Height,
        client_id: &Self::ClientId,
        connection_id: &Self::ConnectionId,
    ) -> Result<Self::ConnectionOpenTryPayload, Self::Error>;

    async fn build_connection_open_ack_payload(
        chain: &Chain,
        height: &Chain::Height,
        client_id: &Self::ClientId,
        connection_id: &Self::ConnectionId,
    ) -> Result<Self::ConnectionOpenAckPayload, Self::Error>;

    async fn build_connection_open_confirm_payload(
        chain: &Chain,
        height: &Chain::Height,
        client_id: &Self::ClientId,
        connection_id: &Self::ConnectionId,
    ) -> Result<Self::ConnectionOpenConfirmPayload, Self::Error>;
}

#[async_trait]
pub trait CanBuildConnectionHandshakeMessages<Chain, Counterparty>:
    HasInitConnectionOptionsType<Chain, Counterparty> + HasErrorType
where
    Counterparty: HasConnectionHandshakePayloads<Self>,
{
    async fn build_connection_open_init_message(
        &self,
        client_id: &Self::ClientId,
        counterparty_client_id: &Counterparty::ClientId,
        init_connection_options: &Self::InitConnectionOptions,
        counterparty_payload: Counterparty::ConnectionOpenInitPayload,
    ) -> Result<Self::Message, Self::Error>;

    async fn build_connection_open_try_message(
        &self,
        client_id: &Self::ClientId,
        counterparty_client_id: &Counterparty::ClientId,
        counterparty_connection_id: &Counterparty::ConnectionId,
        counterparty_payload: Counterparty::ConnectionOpenTryPayload,
    ) -> Result<Self::Message, Self::Error>;

    async fn build_connection_open_ack_message(
        &self,
        connection_id: &Self::ConnectionId,
        counterparty_connection_id: &Counterparty::ConnectionId,
        counterparty_payload: Counterparty::ConnectionOpenAckPayload,
    ) -> Result<Self::Message, Self::Error>;

    async fn build_connection_open_confirm_message(
        &self,
        connection_id: &Self::ConnectionId,
        counterparty_payload: Counterparty::ConnectionOpenConfirmPayload,
    ) -> Result<Self::Message, Self::Error>;
}
