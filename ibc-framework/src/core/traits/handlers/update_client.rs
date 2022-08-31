use crate::core::aliases::client::{
    AnyClientHeader, AnyClientState, AnyConsensusState, ClientHeader, ClientState, ConsensusState,
};
use crate::core::aliases::ibc::ClientId;
use crate::core::traits::client::{ClientTypes, HasAnyClient};
use crate::core::traits::error::HasError;
use crate::core::traits::ibc::HasIbcTypes;
use crate::core::traits::sync::Async;

pub trait HasAnyUpdateClientHandler: HasIbcTypes + HasAnyClient + HasError {
    type AnyUpdateClientHandler: AnyUpdateClientHandler<Self>;
}

pub trait AnyUpdateClientHandler<Context>: Async
where
    Context: HasIbcTypes + HasAnyClient + HasError,
{
    fn check_header_and_update_state(
        context: &Context,
        client_id: &ClientId<Context::IbcTypes>,
        new_client_header: &AnyClientHeader<Context::AnyClient>,
    ) -> Result<
        (
            AnyClientState<Context::AnyClient>,
            AnyConsensusState<Context::AnyClient>,
        ),
        Context::Error,
    >;
}

pub trait UpdateClientHandler<Context>: Async
where
    Context: HasIbcTypes + HasError,
{
    type Client: ClientTypes;

    fn check_header_and_update_state(
        chain: &Context,
        client_id: &ClientId<Context::IbcTypes>,
        new_client_header: &ClientHeader<Self::Client>,
    ) -> Result<(ClientState<Self::Client>, ConsensusState<Self::Client>), Context::Error>;
}
