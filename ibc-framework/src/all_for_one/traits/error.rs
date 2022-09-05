use crate::core::impls::message_handlers::dispatch::InjectUnknownMessageError;
use crate::core::impls::message_handlers::update_client::Error as UpdateClientError;
use crate::core::traits::client::InjectClientTypeMismatchError;
use crate::core::traits::error::{HasError, InjectError};

pub trait AfoErrorContext:
    HasError
    + InjectClientTypeMismatchError
    + InjectUnknownMessageError
    + InjectError<UpdateClientError>
{
}
