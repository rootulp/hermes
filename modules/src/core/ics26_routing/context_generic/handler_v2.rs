use ibc_proto::google::protobuf::Any;
use ibc_proto::ibc::core::client::v1::MsgUpdateClient as RawMsgUpdateClient;

use crate::core::ics02_client::client_state::{ClientState, UpdatedState};
use crate::core::ics02_client::client_type::ClientType;
use crate::core::ics02_client::consensus_state::ConsensusState;
use crate::core::ics02_client::context::ClientReader;
use crate::core::ics02_client::error::Error;
use crate::core::ics02_client::events::{Attributes, UpdateClient as UpdateClientEvent};
use crate::core::ics02_client::height::Height;
use crate::core::ics02_client::msgs::update_client::MsgUpdateClient;
use crate::core::ics24_host::identifier::ClientId;
use crate::core::ics26_routing::context_generic::api::{DefaultIbcTypes, DynClientContext};
use crate::core::ics26_routing::context_generic::framework::{
    UpdateClientExecutionContext, UpdateClientValidationContext,
};
use crate::events::IbcEvent;
use crate::prelude::*;
use crate::timestamp::Timestamp;

pub struct Validator<T> {
    pub context: T,
}

impl<T> Validator<T>
where
    T: UpdateClientValidationContext<
        AnyClientContext = DynClientContext,
        IbcTypes = DefaultIbcTypes,
        Error = Error,
    >,
{
    pub fn validate(&self, msg: RawMsgUpdateClient) -> Result<(), Error> {
        let msg = msg.try_into()?;
        let check_result = check(self, msg)?;
        let process_result = process(self, check_result)?;
        verify(self, process_result)
    }
}

impl<T> Check for Validator<T>
where
    T: UpdateClientValidationContext<
        AnyClientContext = DynClientContext,
        IbcTypes = DefaultIbcTypes,
        Error = Error,
    >,
{
    fn client_state(&self, client_id: &ClientId) -> Result<Box<dyn ClientState>, Error> {
        self.context.client_state(client_id.clone())
    }
}

impl<T> Process for Validator<T>
where
    T: UpdateClientValidationContext<
        AnyClientContext = DynClientContext,
        IbcTypes = DefaultIbcTypes,
        Error = Error,
    >,
{
    fn host_height(&self) -> Height {
        self.context.host_height()
    }

    fn host_timestamp(&self) -> Timestamp {
        self.context.host_timestamp()
    }
}

impl<T> Verify for Validator<T>
where
    T: UpdateClientValidationContext<
        AnyClientContext = DynClientContext,
        IbcTypes = DefaultIbcTypes,
        Error = Error,
    >,
{
    fn consensus_state(
        &self,
        _client_id: &ClientId,
        _height: Height,
    ) -> Result<Box<dyn ConsensusState>, Error> {
        todo!()
    }

    fn next_consensus_state(
        &self,
        _client_id: &ClientId,
        _height: Height,
    ) -> Result<Option<Box<dyn ConsensusState>>, Error> {
        todo!()
    }

    fn prev_consensus_state(
        &self,
        _client_id: &ClientId,
        _height: Height,
    ) -> Result<Option<Box<dyn ConsensusState>>, Error> {
        todo!()
    }
}

impl<T> ClientReader for Validator<T>
where
    T: UpdateClientValidationContext<
        AnyClientContext = DynClientContext,
        IbcTypes = DefaultIbcTypes,
        Error = Error,
    >,
{
    fn client_type(&self, _client_id: &ClientId) -> Result<ClientType, Error> {
        todo!()
    }

    fn client_state(&self, _client_id: &ClientId) -> Result<Box<dyn ClientState>, Error> {
        todo!()
    }

    fn decode_client_state(&self, _client_state: Any) -> Result<Box<dyn ClientState>, Error> {
        todo!()
    }

    fn consensus_state(
        &self,
        _client_id: &ClientId,
        _height: crate::Height,
    ) -> Result<Box<dyn ConsensusState>, Error> {
        todo!()
    }

    fn next_consensus_state(
        &self,
        _client_id: &ClientId,
        _height: crate::Height,
    ) -> Result<Option<Box<dyn ConsensusState>>, Error> {
        todo!()
    }

    fn prev_consensus_state(
        &self,
        _client_id: &ClientId,
        _height: crate::Height,
    ) -> Result<Option<Box<dyn ConsensusState>>, Error> {
        todo!()
    }

    fn host_height(&self) -> crate::Height {
        todo!()
    }

    fn host_consensus_state(
        &self,
        _height: crate::Height,
    ) -> Result<Box<dyn ConsensusState>, Error> {
        todo!()
    }

    fn pending_host_consensus_state(&self) -> Result<Box<dyn ConsensusState>, Error> {
        todo!()
    }

    fn client_counter(&self) -> Result<u64, Error> {
        todo!()
    }
}

pub struct Executor<T> {
    pub context: T,
}

impl<T> Executor<T>
where
    T: UpdateClientExecutionContext<
        AnyClientContext = DynClientContext,
        IbcTypes = DefaultIbcTypes,
        Error = Error,
    >,
{
    pub fn execute(&mut self, msg: RawMsgUpdateClient) -> Result<(), Error> {
        let msg = msg.try_into()?;
        let check_result = check(self, msg)?;
        let process_result = process(self, check_result)?;
        write(self, process_result)
    }
}

impl<T> Check for Executor<T>
where
    T: UpdateClientExecutionContext<
        AnyClientContext = DynClientContext,
        IbcTypes = DefaultIbcTypes,
        Error = Error,
    >,
{
    fn client_state(&self, client_id: &ClientId) -> Result<Box<dyn ClientState>, Error> {
        self.context.client_state(client_id.clone())
    }
}

impl<T> Process for Executor<T>
where
    T: UpdateClientExecutionContext<
        AnyClientContext = DynClientContext,
        IbcTypes = DefaultIbcTypes,
        Error = Error,
    >,
{
    fn host_height(&self) -> Height {
        self.context.host_height()
    }

    fn host_timestamp(&self) -> Timestamp {
        self.context.host_timestamp()
    }
}

impl<T> Write for Executor<T>
where
    T: UpdateClientExecutionContext<
        AnyClientContext = DynClientContext,
        IbcTypes = DefaultIbcTypes,
        Error = Error,
    >,
{
    fn emit_event(&mut self, event: IbcEvent) {
        self.context.emit_event(event)
    }

    fn store_client_state(
        &mut self,
        client_id: ClientId,
        client_state: Box<dyn ClientState>,
    ) -> Result<(), Error> {
        self.context.store_client_state(client_id, client_state)
    }

    fn store_consensus_state(
        &mut self,
        client_id: ClientId,
        height: Height,
        consensus_state: Box<dyn ConsensusState>,
    ) -> Result<(), Error> {
        self.context
            .store_consensus_state(client_id, height, consensus_state)
    }

    fn store_update_time(
        &mut self,
        client_id: ClientId,
        height: Height,
        timestamp: Timestamp,
    ) -> Result<(), Error> {
        self.context.store_update_time(client_id, height, timestamp)
    }

    fn store_update_height(
        &mut self,
        client_id: ClientId,
        height: Height,
        host_height: Height,
    ) -> Result<(), Error> {
        self.context
            .store_update_height(client_id, height, host_height)
    }
}

impl<T> ClientReader for Executor<T>
where
    T: UpdateClientExecutionContext<
        AnyClientContext = DynClientContext,
        IbcTypes = DefaultIbcTypes,
        Error = Error,
    >,
{
    fn client_type(&self, _client_id: &ClientId) -> Result<ClientType, Error> {
        todo!()
    }

    fn client_state(&self, _client_id: &ClientId) -> Result<Box<dyn ClientState>, Error> {
        todo!()
    }

    fn decode_client_state(&self, _client_state: Any) -> Result<Box<dyn ClientState>, Error> {
        todo!()
    }

    fn consensus_state(
        &self,
        _client_id: &ClientId,
        _height: crate::Height,
    ) -> Result<Box<dyn ConsensusState>, Error> {
        todo!()
    }

    fn next_consensus_state(
        &self,
        _client_id: &ClientId,
        _height: crate::Height,
    ) -> Result<Option<Box<dyn ConsensusState>>, Error> {
        todo!()
    }

    fn prev_consensus_state(
        &self,
        _client_id: &ClientId,
        _height: crate::Height,
    ) -> Result<Option<Box<dyn ConsensusState>>, Error> {
        todo!()
    }

    fn host_height(&self) -> crate::Height {
        todo!()
    }

    fn host_consensus_state(
        &self,
        _height: crate::Height,
    ) -> Result<Box<dyn ConsensusState>, Error> {
        todo!()
    }

    fn pending_host_consensus_state(&self) -> Result<Box<dyn ConsensusState>, Error> {
        todo!()
    }

    fn client_counter(&self) -> Result<u64, Error> {
        todo!()
    }
}

pub fn validate(msg: RawMsgUpdateClient) -> Result<MsgUpdateClient, Error> {
    msg.try_into()
}

pub trait Check {
    fn client_state(&self, client_id: &ClientId) -> Result<Box<dyn ClientState>, Error>;
}

pub struct CheckResult {
    client_id: ClientId,
    client_state: Box<dyn ClientState>,
    header: Any,
}

pub fn check<CtxRead>(ctx: &CtxRead, msg: MsgUpdateClient) -> Result<CheckResult, Error>
where
    CtxRead: Check,
{
    let MsgUpdateClient {
        client_id, header, ..
    } = msg;

    let client_state = ctx.client_state(&client_id)?;

    if client_state.is_frozen() {
        Err(Error::client_frozen(client_id))
    } else {
        Ok(CheckResult {
            client_id,
            client_state,
            header,
        })
    }
}

pub trait Process {
    fn host_height(&self) -> Height;

    fn host_timestamp(&self) -> Timestamp;
}

pub struct ProcessResult {
    pub client_id: ClientId,
    pub client_state: Box<dyn ClientState>,
    pub consensus_state: Box<dyn ConsensusState>,
    pub header: Any,
    pub processed_time: Timestamp,
    pub processed_height: Height,
    pub event: UpdateClientEvent,
}

pub fn process<CtxRead>(ctx: &CtxRead, check_result: CheckResult) -> Result<ProcessResult, Error>
where
    CtxRead: Process + ClientReader,
{
    let CheckResult {
        client_id,
        client_state,
        header,
    } = check_result;

    let UpdatedState {
        client_state,
        consensus_state,
    } = client_state
        // TODO: Replace with client_state.updated_state()
        .check_header_and_update_state(ctx, client_id.clone(), header.clone())
        .map_err(|e| Error::header_verification_failure(e.to_string()))?;

    let event = UpdateClientEvent::from(Attributes {
        client_id: client_id.clone(),
        client_type: client_state.client_type(),
        consensus_height: client_state.latest_height(),
    });

    Ok(ProcessResult {
        client_id,
        client_state,
        consensus_state,
        header,
        processed_time: ClientReader::host_timestamp(ctx),
        processed_height: ClientReader::host_height(ctx),
        event,
    })
}

pub trait Verify {
    fn consensus_state(
        &self,
        client_id: &ClientId,
        height: Height,
    ) -> Result<Box<dyn ConsensusState>, Error>;

    fn next_consensus_state(
        &self,
        client_id: &ClientId,
        height: Height,
    ) -> Result<Option<Box<dyn ConsensusState>>, Error>;

    fn prev_consensus_state(
        &self,
        client_id: &ClientId,
        height: Height,
    ) -> Result<Option<Box<dyn ConsensusState>>, Error>;
}

pub fn verify<CtxRead>(ctx: &CtxRead, process_result: ProcessResult) -> Result<(), Error>
where
    CtxRead: Verify + ClientReader, // TODO: Remove `ClientReader` bound
{
    let ProcessResult {
        client_id,
        client_state,
        header,
        ..
    } = process_result;

    let UpdatedState { .. } = client_state
        .check_header_and_update_state(ctx, client_id, header)
        .map_err(|e| Error::header_verification_failure(e.to_string()))?;

    /* Verification */

    Ok(())
}

pub trait Write {
    fn emit_event(&mut self, event: IbcEvent);

    fn store_client_state(
        &mut self,
        client_id: ClientId,
        client_state: Box<dyn ClientState>,
    ) -> Result<(), Error>;

    fn store_consensus_state(
        &mut self,
        client_id: ClientId,
        height: Height,
        consensus_state: Box<dyn ConsensusState>,
    ) -> Result<(), Error>;

    fn store_update_time(
        &mut self,
        client_id: ClientId,
        height: Height,
        timestamp: Timestamp,
    ) -> Result<(), Error>;

    fn store_update_height(
        &mut self,
        client_id: ClientId,
        height: Height,
        host_height: Height,
    ) -> Result<(), Error>;
}

pub fn write<CtxWrite>(ctx: &mut CtxWrite, process_result: ProcessResult) -> Result<(), Error>
where
    CtxWrite: Write,
{
    let ProcessResult {
        client_id,
        client_state,
        consensus_state,
        processed_time,
        processed_height,
        event,
        ..
    } = process_result;

    ctx.emit_event(event.into());

    ctx.store_client_state(client_id.clone(), client_state.clone())?;
    ctx.store_consensus_state(
        client_id.clone(),
        client_state.latest_height(),
        consensus_state,
    )?;
    ctx.store_update_time(
        client_id.clone(),
        client_state.latest_height(),
        processed_time,
    )?;
    ctx.store_update_height(client_id, client_state.latest_height(), processed_height)?;

    Ok(())
}
