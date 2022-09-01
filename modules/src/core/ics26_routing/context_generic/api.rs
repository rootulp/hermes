use ibc_proto::google::protobuf::Any;

use crate::core::ics02_client::client_state::ClientState;
use crate::core::ics02_client::client_type::ClientType as IbcClientType;
use crate::core::ics02_client::consensus_state::ConsensusState;
use crate::core::ics02_client::header::Header;
use crate::core::ics02_client::misbehaviour::Misbehaviour;
use crate::core::ics24_host::identifier::ClientId as IbcClientId;
use crate::core::ics24_host::path::{ClientStatePath, ClientTypePath};
use crate::core::ics26_routing::context_generic::framework::{
    AnyClientContext, AnyClientState, AnyConsensusState, ClientId, ClientType, HasIbcTypes, HasStore, Height, IbcTypes, Timestamp, TypedStore, UpdateClientExecutionContext,
    UpdateClientValidationContext,
};
use crate::prelude::Box;
use crate::{timestamp::Timestamp as IbcTimestamp, Height as IbcHeight};

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

pub struct DynClientContext;

impl AnyClientContext for DynClientContext {
    type AnyClientState = Box<dyn ClientState>;
    type AnyConsensusState = Box<dyn ConsensusState>;
    type AnyClientHeader = Box<dyn Header>;
    type AnyMisbehaviour = Box<dyn Misbehaviour>;
}

pub struct DefaultIbcTypes;

impl IbcTypes for DefaultIbcTypes {
    type ClientType = IbcClientType;
    type ClientId = IbcClientId;
    type Height = IbcHeight;
    type Timestamp = IbcTimestamp;
}

pub trait IbcStore<Context, Types>:
    TypedStore<ClientTypePath, ClientType<Types>>
    + TypedStore<ClientStatePath, AnyClientState<Context>>
    + TypedStore<ClientStatePath, AnyConsensusState<Context>>
where
    Context: AnyClientContext,
    Types: IbcTypes,
{
}

impl<S, Context, Types> IbcStore<Context, Types> for S
where
    Context: AnyClientContext,
    Types: IbcTypes,
    S: TypedStore<ClientTypePath, ClientType<Types>>
        + TypedStore<ClientStatePath, AnyClientState<Context>>
        + TypedStore<ClientStatePath, AnyConsensusState<Context>>,
{
}

pub trait Host {
    type Error;
    type KvStore: IbcStore<DynClientContext, DefaultIbcTypes>;

    fn current_timestamp(&self) -> IbcTimestamp;

    fn current_height(&self) -> IbcHeight;

    fn store(&self) -> &Self::KvStore;

    // events - commitment?
}
