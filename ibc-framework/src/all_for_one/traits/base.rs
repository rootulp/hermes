use crate::core::traits::client::HasClientHandler;
use crate::core::traits::client_reader::AnyClientReader;
use crate::core::traits::error::HasError;
use crate::core::traits::ibc::HasIbcTypes;

pub trait AfoContext: HasError + HasIbcTypes + HasClientHandler + AnyClientReader {}

impl<Context> AfoContext for Context where
    Context: HasError + HasIbcTypes + HasClientHandler + AnyClientReader
{
}
