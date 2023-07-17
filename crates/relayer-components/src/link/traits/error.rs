use crate::core::traits::error::HasErrorType;

pub trait HasLinkError<Chain, Counterparty>: HasErrorType
where
    Chain: HasErrorType,
{
    fn chain_error(e: Chain::Error) -> Self::Error;
}
