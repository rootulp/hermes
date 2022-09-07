use crate::core::impls::message_handlers::dispatch::InjectUnknownMessageError;
use crate::core::impls::message_handlers::update_client::InjectUpdateClientError;
use crate::core::traits::client::InjectClientTypeMismatchError;
use crate::core::traits::error::HasError;

pub trait AfoErrorContext:
    HasError + InjectClientTypeMismatchError + InjectUnknownMessageError + InjectUpdateClientError
{
}

impl<Context> AfoErrorContext for Context where
    Context: HasError
        + InjectClientTypeMismatchError
        + InjectUnknownMessageError
        + InjectUpdateClientError
{
}
