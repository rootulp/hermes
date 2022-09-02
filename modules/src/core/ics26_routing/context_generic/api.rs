use ibc_proto::google::protobuf::Any;

use crate::core::ics02_client::client_state::ClientState;
use crate::core::ics02_client::client_type::ClientType as IbcClientType;
use crate::core::ics02_client::consensus_state::ConsensusState;
use crate::core::ics02_client::header::Header;
use crate::core::ics02_client::misbehaviour::Misbehaviour;
use crate::core::ics24_host::identifier::ClientId as IbcClientId;
use crate::core::ics24_host::path::{ClientConsensusStatePath, ClientStatePath, ClientTypePath};
use crate::core::ics26_routing::context_generic::framework::{
    AnyClientContext, AnyClientState, AnyConsensusState, ClientId, ClientType, Height, IbcTypes,
    Timestamp, TypedStore, UpdateClientExecutionContext, UpdateClientValidationContext,
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

pub trait IbcStore<Context, Types, Error>:
    TypedStore<ClientTypePath, ClientType<Types>, Error = Error>
    + TypedStore<ClientStatePath, AnyClientState<Context>, Error = Error>
    + TypedStore<ClientConsensusStatePath, AnyConsensusState<Context>, Error = Error>
where
    Context: AnyClientContext,
    Types: IbcTypes,
{
}

impl<S, Context, Types, Error> IbcStore<Context, Types, Error> for S
where
    Context: AnyClientContext,
    Types: IbcTypes,
    S: TypedStore<ClientTypePath, ClientType<Types>, Error = Error>
        + TypedStore<ClientStatePath, AnyClientState<Context>, Error = Error>
        + TypedStore<ClientConsensusStatePath, AnyConsensusState<Context>, Error = Error>,
{
}

pub trait StoreError {
    fn path_not_found() -> Self;
}

pub trait Host {
    type Error: StoreError;
    type KvStore: IbcStore<DynClientContext, DefaultIbcTypes, Self::Error>;

    fn current_timestamp(&self) -> IbcTimestamp;

    fn current_height(&self) -> IbcHeight;

    fn store(&self) -> &Self::KvStore;

    fn store_mut(&mut self) -> &mut Self::KvStore;

    // events - commitment?
}

pub struct IbcHost<H> {
    host: H,
}

impl<H: Host> Host for IbcHost<H> {
    type Error = H::Error;
    type KvStore = H::KvStore;

    fn current_timestamp(&self) -> IbcTimestamp {
        self.host.current_timestamp()
    }

    fn current_height(&self) -> IbcHeight {
        self.host.current_height()
    }

    fn store(&self) -> &Self::KvStore {
        self.host.store()
    }

    fn store_mut(&mut self) -> &mut Self::KvStore {
        self.host.store_mut()
    }
}

impl<H: Host> UpdateClientValidationContext for IbcHost<H> {
    type AnyClientContext = DynClientContext;
    type IbcTypes = DefaultIbcTypes;
    type Error = H::Error;

    fn client_state(
        &self,
        client_id: ClientId<Self::IbcTypes>,
    ) -> Result<AnyClientState<Self::AnyClientContext>, Self::Error> {
        self.host
            .store()
            .get_pre(ClientStatePath(client_id))?
            .ok_or_else(Self::Error::path_not_found)
    }

    fn consensus_state(
        &self,
        client_id: ClientId<Self::IbcTypes>,
        height: Height<Self::IbcTypes>,
    ) -> Result<AnyConsensusState<Self::AnyClientContext>, Self::Error> {
        self.host
            .store()
            .get_pre(ClientConsensusStatePath {
                client_id,
                epoch: height.revision_number(),
                height: height.revision_height(),
            })?
            .ok_or_else(Self::Error::path_not_found)
    }

    fn host_timestamp(&self) -> Timestamp<Self::IbcTypes> {
        self.host.current_timestamp()
    }

    fn host_height(&self) -> Height<Self::IbcTypes> {
        self.host.current_height()
    }
}

impl<H: Host> UpdateClientExecutionContext for IbcHost<H> {
    type AnyClientContext = DynClientContext;
    type IbcTypes = DefaultIbcTypes;
    type Error = H::Error;

    fn client_state(
        &self,
        client_id: ClientId<Self::IbcTypes>,
    ) -> Result<AnyClientState<Self::AnyClientContext>, Self::Error> {
        self.host
            .store()
            .get(ClientStatePath(client_id))?
            .ok_or_else(Self::Error::path_not_found)
    }

    fn consensus_state(
        &self,
        client_id: ClientId<Self::IbcTypes>,
        height: Height<Self::IbcTypes>,
    ) -> Result<AnyConsensusState<Self::AnyClientContext>, Self::Error> {
        self.host
            .store()
            .get(ClientConsensusStatePath {
                client_id,
                epoch: height.revision_number(),
                height: height.revision_height(),
            })?
            .ok_or_else(Self::Error::path_not_found)
    }

    fn host_timestamp(&self) -> Timestamp<Self::IbcTypes> {
        self.host.current_timestamp()
    }

    fn host_height(&self) -> Height<Self::IbcTypes> {
        self.host.current_height()
    }

    fn store_client_type(
        &mut self,
        client_id: ClientId<Self::IbcTypes>,
        client_type: ClientType<Self::IbcTypes>,
    ) -> Result<(), Self::Error> {
        self.host
            .store_mut()
            .set(ClientTypePath(client_id), client_type)
    }

    fn store_client_state(
        &mut self,
        client_id: ClientId<Self::IbcTypes>,
        client_state: AnyClientState<Self::AnyClientContext>,
    ) -> Result<(), Self::Error> {
        self.host
            .store_mut()
            .set(ClientStatePath(client_id), client_state)
    }

    fn store_consensus_state(
        &mut self,
        client_id: ClientId<Self::IbcTypes>,
        height: Height<Self::IbcTypes>,
        consensus_state: AnyConsensusState<Self::AnyClientContext>,
    ) -> Result<(), Self::Error> {
        self.host.store_mut().set(
            ClientConsensusStatePath {
                client_id,
                epoch: height.revision_number(),
                height: height.revision_height(),
            },
            consensus_state,
        )
    }

    fn increase_client_counter(&mut self) {
        todo!()
    }

    fn store_update_time(
        &mut self,
        _client_id: ClientId<Self::IbcTypes>,
        _height: Height<Self::IbcTypes>,
        _timestamp: Timestamp<Self::IbcTypes>,
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn store_update_height(
        &mut self,
        _client_id: ClientId<Self::IbcTypes>,
        _height: Height<Self::IbcTypes>,
        _host_height: Height<Self::IbcTypes>,
    ) -> Result<(), Self::Error> {
        todo!()
    }
}
