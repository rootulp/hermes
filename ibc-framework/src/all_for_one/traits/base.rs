use crate::core::traits::client::{HasAnyClientMethods, HasOwnClient};
use crate::core::traits::client_reader::AnyClientReader;
use crate::core::traits::error::{HasError, InjectError, MismatchClientType};
use crate::core::traits::handlers::update_client::HasAnyUpdateClientHandler;
use crate::core::traits::host::HasHostMethods;
use crate::core::traits::ibc::HasIbcTypes;

pub trait AfoChainContext:
    HasError
    + HasIbcTypes
    + HasOwnClient
    + HasAnyClientMethods
    + AnyClientReader
    + HasAnyUpdateClientHandler
    + HasHostMethods
    + InjectError<MismatchClientType<Self::ClientType>>
{
}

impl<Context> AfoChainContext for Context where
    Context: HasError
        + HasIbcTypes
        + HasOwnClient
        + HasAnyClientMethods
        + AnyClientReader
        + HasAnyUpdateClientHandler
        + HasHostMethods
        + InjectError<MismatchClientType<Self::ClientType>>
{
}
