use crate::core::traits::client::HasAnyClient;
use crate::core::traits::client_reader::ClientReader;
use crate::core::traits::error::HasError;
use crate::core::traits::ibc::HasIbcTypes;

pub trait AfoContext: HasError + HasIbcTypes + HasAnyClient + ClientReader {}

impl<Context> AfoContext for Context where
    Context: HasError + HasIbcTypes + HasAnyClient + ClientReader
{
}
