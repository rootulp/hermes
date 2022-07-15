use crate::prelude::*;

use ibc_proto::google::protobuf::Any;
use tendermint_proto::Protobuf;

use crate::core::ics02_client::error::Error;

use crate::core::ics24_host::identifier::ClientId;
use crate::Height;

use super::header::AnyHeader;

pub trait Misbehaviour:
    Clone + core::fmt::Debug + Send + Sync + Protobuf<Any, Error = Error>
{
    /// The type of client (eg. Tendermint)
    fn client_id(&self) -> &ClientId;

    /// The height of the consensus state
    fn height(&self) -> Height;
}

#[derive(Clone, Debug, PartialEq)]
pub struct MisbehaviourEvidence<M> {
    pub misbehaviour: M,
    pub supporting_headers: Vec<AnyHeader>,
}
