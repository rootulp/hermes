use ibc_proto::google::protobuf::Any;

use super::framework::{UpdateClientExecutionContext, UpdateClientValidationContext};

pub fn execute<Context: UpdateClientExecutionContext>(
    _ctx: &mut Context,
    _message: Any,
) -> Result<(), Context::Error> {
    todo!()
}

pub fn validate<Context: UpdateClientValidationContext>(
    _ctx: &Context,
    _message: Any,
) -> Result<(), Context::Error> {
    todo!()
}
