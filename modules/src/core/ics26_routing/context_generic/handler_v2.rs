use ibc_proto::google::protobuf::Any;
use ibc_proto::ibc::core::client::v1::MsgUpdateClient as RawMsgUpdateClient;

use crate::core::ics02_client::client_state::{ClientState, UpdatedState};
use crate::core::ics02_client::consensus_state::ConsensusState;
use crate::core::ics02_client::context::ClientReader;
use crate::core::ics02_client::error::Error;
use crate::core::ics02_client::events::{Attributes, UpdateClient as UpdateClientEvent};
use crate::core::ics02_client::height::Height;
use crate::core::ics02_client::msgs::update_client::MsgUpdateClient;
use crate::core::ics24_host::identifier::ClientId;
use crate::core::ics26_routing::context_generic::api::EventEmitter;
use crate::events::IbcEvent;
use crate::prelude::*;
use crate::timestamp::Timestamp;

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
}

pub fn process<E, CtxRead>(
    event_emitter: &mut E,
    ctx: &CtxRead,
    check_result: CheckResult,
) -> Result<ProcessResult, Error>
where
    E: EventEmitter<Event = IbcEvent>,
    CtxRead: ClientReader,
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

    event_emitter.emit_event(
        UpdateClientEvent::from(Attributes {
            client_id: client_id.clone(),
            client_type: client_state.client_type(),
            consensus_height: client_state.latest_height(),
        })
        .into(),
    );

    Ok(ProcessResult {
        client_id,
        client_state,
        consensus_state,
        header,
        processed_time: ClientReader::host_timestamp(ctx),
        processed_height: ctx.host_height(),
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
        ..
    } = process_result;

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
