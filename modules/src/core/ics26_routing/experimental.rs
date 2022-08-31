use core::default::Default;
use core::fmt::Debug;

use crate::core::ics02_client::client_type::ClientType;
use crate::core::ics04_channel::commitment::PacketCommitment;
use crate::core::ics24_host::identifier::PortId;
use crate::core::ics24_host::path::{ClientTypePath, CommitmentsPath, Path as IbcPath};
use crate::core::ics26_routing::context::ModuleId;
use crate::events::IbcEvent;
use crate::prelude::*;

pub struct MsgReceipt<Event> {
    pub events: Vec<Event>,
    pub log: Vec<String>,
}

pub trait Handler {
    /// Error and (intermediate) results
    type Error;
    type CheckResult;
    type ProcessResult;

    /// Message types
    /// Conversion from raw-type to domain-type encodes stateless validation
    type RawMessage;
    type Message: TryFrom<Self::RawMessage, Error = Self::Error>;

    /// Subset of IbcEvents that this handler could emit
    type Event: Into<IbcEvent>;

    /// Context from host (includes light client related context)
    type ContextRead;
    type ContextWrite;

    /// Other facilities
    type Logger: Logger;
    type EventEmitter: EventEmitter<Event = Self::Event>;

    /// Stateless validation
    fn validate(&self, msg: Self::RawMessage) -> Result<Self::Message, Self::Error> {
        msg.try_into()
    }

    /// Stateful validation of the message (includes verification)
    fn check(
        &mut self,
        ctx: &Self::ContextRead,
        msg: Self::Message,
    ) -> Result<Self::CheckResult, Self::Error>;

    /// Process/Execute the message
    fn process(
        &mut self,
        ctx: &Self::ContextRead,
        check_result: Self::CheckResult,
    ) -> Result<Self::ProcessResult, Self::Error>;

    /// Write the result
    fn write(
        self,
        ctx: &mut Self::ContextWrite,
        process_result: Self::ProcessResult,
    ) -> Result<MsgReceipt<Self::Event>, Self::Error>;
}

pub trait Logger: Into<Vec<String>> {
    /// Return the logs generated so-far
    fn logs(&self) -> &[String];

    /// Log a message
    fn log_message(&mut self, _msg: impl ToString);
}

#[derive(Clone, Debug, Default)]
pub struct DefaultLogger {
    log: Vec<String>,
}

impl From<DefaultLogger> for Vec<String> {
    fn from(logger: DefaultLogger) -> Self {
        logger.log
    }
}

impl Logger for DefaultLogger {
    fn logs(&self) -> &[String] {
        self.log.as_ref()
    }

    fn log_message(&mut self, msg: impl ToString) {
        self.log.push(msg.to_string())
    }
}

pub trait EventEmitter: Into<Vec<Self::Event>> {
    /// Event type
    type Event;

    /// Return the events generated so-far
    fn events(&self) -> &[Self::Event];

    /// Emit an event
    fn emit_event(&mut self, _event: Self::Event);
}

#[derive(Clone, Debug)]
pub struct DefaultEventEmitter<Event> {
    events: Vec<Event>,
}

impl<Event> Default for DefaultEventEmitter<Event> {
    fn default() -> Self {
        Self { events: vec![] }
    }
}

impl<Event> From<DefaultEventEmitter<Event>> for Vec<Event> {
    fn from(event_emitter: DefaultEventEmitter<Event>) -> Self {
        event_emitter.events
    }
}

impl<Event> EventEmitter for DefaultEventEmitter<Event> {
    type Event = Event;

    fn events(&self) -> &[Self::Event] {
        self.events.as_ref()
    }

    fn emit_event(&mut self, event: Self::Event) {
        self.events.push(event)
    }
}

pub mod update_client {
    use std::marker::PhantomData;

    use ibc_proto::google::protobuf::Any;
    use ibc_proto::ibc::core::client::v1::MsgUpdateClient as RawMsgUpdateClient;

    use super::*;
    use crate::core::ics02_client::client_state::{ClientState, UpdatedState};
    use crate::core::ics02_client::context::{ClientKeeper, ClientReader};
    use crate::core::ics02_client::error::Error;
    use crate::core::ics02_client::events::{Attributes, UpdateClient as UpdateClientEvent};
    use crate::core::ics02_client::handler::update_client::Result as UpdateClientResult;
    use crate::core::ics02_client::msgs::update_client::MsgUpdateClient;
    use crate::core::ics24_host::identifier::ClientId;

    #[derive(Clone, Debug)]
    pub struct UpdateClientHandler<
        CtxRead,
        CtxWrite,
        Logger = DefaultLogger,
        EventEmitter = DefaultEventEmitter<UpdateClientEvent>,
    > {
        logger: Logger,
        event_emitter: EventEmitter,
        _ctx: PhantomData<(CtxRead, CtxWrite)>,
    }

    impl<CtxRead, CtxWrite> Default for UpdateClientHandler<CtxRead, CtxWrite> {
        fn default() -> Self {
            Self {
                logger: DefaultLogger::default(),
                event_emitter: DefaultEventEmitter::default(),
                _ctx: Default::default(),
            }
        }
    }

    impl<
            CtxRead: ClientReader,
            CtxWrite: ClientKeeper,
            L: Logger,
            E: EventEmitter<Event = UpdateClientEvent>,
        > Handler for UpdateClientHandler<CtxRead, CtxWrite, L, E>
    {
        type Error = Error;
        type CheckResult = UpdateClientCheckResult;
        type ProcessResult = UpdateClientResult;
        type RawMessage = RawMsgUpdateClient;
        type Message = MsgUpdateClient;
        type Event = UpdateClientEvent;
        type ContextRead = CtxRead;
        type ContextWrite = CtxWrite;
        type Logger = L;
        type EventEmitter = E;

        fn check(
            &mut self,
            ctx: &Self::ContextRead,
            msg: Self::Message,
        ) -> Result<Self::CheckResult, Self::Error> {
            update_client_check(|client_id| ctx.client_state(client_id), msg)
        }

        fn process(
            &mut self,
            ctx: &Self::ContextRead,
            check_result: Self::CheckResult,
        ) -> Result<Self::ProcessResult, Self::Error> {
            update_client_process(&mut self.event_emitter, ctx, check_result)
        }

        fn write(
            self,
            ctx: &mut Self::ContextWrite,
            process_result: Self::ProcessResult,
        ) -> Result<MsgReceipt<Self::Event>, Self::Error> {
            update_client_write(ctx, process_result)?;
            Ok(MsgReceipt {
                events: self.event_emitter.into(),
                log: self.logger.into(),
            })
        }
    }

    /// Only instantiable by lib
    pub struct UpdateClientCheckResult {
        client_id: ClientId,
        client_state: Box<dyn ClientState>,
        header: Any,
    }

    pub fn update_client_check(
        // ctx: &CtxRead,
        client_state: impl FnOnce(&ClientId) -> Result<Box<dyn ClientState>, Error>,
        msg: MsgUpdateClient,
    ) -> Result<UpdateClientCheckResult, Error> {
        let MsgUpdateClient {
            client_id, header, ..
        } = msg;

        // let client_state = ctx.client_state(&client_id)?;
        let client_state = client_state(&client_id)?;

        if client_state.is_frozen() {
            Err(Error::client_frozen(client_id))
        } else {
            Ok(UpdateClientCheckResult {
                client_id,
                client_state,
                header,
            })
        }
    }

    pub fn update_client_process<
        E: EventEmitter<Event = UpdateClientEvent>,
        CtxRead: ClientReader,
    >(
        event_emitter: &mut E,
        ctx: &CtxRead,
        check_result: UpdateClientCheckResult,
    ) -> Result<UpdateClientResult, Error> {
        let UpdateClientCheckResult {
            client_id,
            client_state,
            header,
        } = check_result;

        let UpdatedState {
            client_state,
            consensus_state,
        } = client_state
            .check_header_and_update_state(ctx, client_id.clone(), header)
            .map_err(|e| Error::header_verification_failure(e.to_string()))?;

        event_emitter.emit_event(UpdateClientEvent::from(Attributes {
            client_id: client_id.clone(),
            client_type: client_state.client_type(),
            consensus_height: client_state.latest_height(),
        }));

        Ok(UpdateClientResult {
            client_id,
            client_state,
            consensus_state,
            processed_time: ClientReader::host_timestamp(ctx),
            processed_height: ctx.host_height(),
        })
    }

    pub fn update_client_write<CtxWrite: ClientKeeper>(
        ctx: &mut CtxWrite,
        process_result: UpdateClientResult,
    ) -> Result<(), Error> {
        let UpdateClientResult {
            client_id,
            client_state,
            consensus_state,
            processed_time,
            processed_height,
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
        ctx.store_update_height(
            client_id.clone(),
            client_state.latest_height(),
            processed_height,
        )?;

        Ok(())
    }
}

pub struct ModuleCallbackHandler<Handler> {
    inner: Handler,
}

pub struct ModuleCallbackCheckResult<CheckResult> {
    module_id: ModuleId,
    inner: CheckResult,
}

impl<H: Handler> Handler for ModuleCallbackHandler<H> {
    type Error = H::Error;
    type CheckResult = ModuleCallbackCheckResult<H::CheckResult>;
    type ProcessResult = H::ProcessResult;
    type RawMessage = H::RawMessage;
    type Message = H::Message;
    type Event = IbcEvent;
    type ContextRead = H::ContextRead;
    type ContextWrite = H::ContextWrite;
    type Logger = H::Logger;
    type EventEmitter = DefaultEventEmitter<IbcEvent>;

    fn check(
        &mut self,
        ctx: &Self::ContextRead,
        msg: Self::Message,
    ) -> Result<Self::CheckResult, Self::Error> {
        let module_id = ModuleId::new(format!("module{}", PortId::default()).into()).unwrap();
        self.inner
            .check(ctx, msg)
            .map(|check_result| ModuleCallbackCheckResult {
                module_id,
                inner: check_result,
            })
    }

    fn process(
        &mut self,
        ctx: &Self::ContextRead,
        check_result: Self::CheckResult,
    ) -> Result<Self::ProcessResult, Self::Error> {
        let _ = check_result.module_id;
        self.inner.process(ctx, check_result.inner)
    }

    fn write(
        self,
        ctx: &mut Self::ContextWrite,
        process_result: Self::ProcessResult,
    ) -> Result<MsgReceipt<Self::Event>, Self::Error> {
        self.inner
            .write(ctx, process_result)
            .map(|receipt| MsgReceipt {
                events: receipt.events.into_iter().map(Into::into).collect(),
                log: receipt.log,
            })
    }
}

pub mod ics26 {
    use ibc_proto::google::protobuf::Any;

    use super::*;
    use crate::core::ics02_client::context::{ClientKeeper, ClientReader};
    use crate::core::ics02_client::handler::update_client::Result as UpdateClientResult;
    use crate::core::ics02_client::msgs::ClientMsg;
    use crate::core::ics03_connection::context::{ConnectionKeeper, ConnectionReader};
    use crate::core::ics04_channel::context::{ChannelKeeper, ChannelReader};
    use crate::core::ics05_port::context::PortReader;
    use crate::core::ics26_routing::context::Ics26Context;
    use crate::core::ics26_routing::error::Error;
    use crate::core::ics26_routing::experimental::update_client::{
        UpdateClientCheckResult, UpdateClientHandler,
    };
    use crate::core::ics26_routing::msgs::Ics26Envelope;

    pub trait Ics26Reader: ClientReader + ConnectionReader + ChannelReader + PortReader {}

    impl<T: Ics26Context> Ics26Reader for T {}

    pub trait Ics26Keeper: ClientKeeper + ConnectionKeeper + ChannelKeeper {}

    impl<T: Ics26Context> Ics26Keeper for T {}

    pub enum Ics26CheckResult {
        ClientUpdate(UpdateClientCheckResult),
    }

    pub enum Ics26ProcessResult {
        ClientUpdate(UpdateClientResult),
    }

    pub enum Ics26Handler<CtxRead, CtxWrite> {
        ClientUpdate(UpdateClientHandler<CtxRead, CtxWrite>),
    }

    impl<CtxRead, CtxWrite> Ics26Handler<CtxRead, CtxWrite> {
        pub fn new(msg: &Ics26Envelope) -> Self {
            match msg {
                Ics26Envelope::Ics2Msg(ClientMsg::UpdateClient(_)) => {
                    Self::ClientUpdate(UpdateClientHandler::default())
                }
                _ => todo!(),
            }
        }
    }

    impl<CtxRead: Ics26Reader, CtxWrite: Ics26Keeper> Handler for Ics26Handler<CtxRead, CtxWrite> {
        type Error = Error;
        type CheckResult = Ics26CheckResult;
        type ProcessResult = Ics26ProcessResult;
        type RawMessage = Any;
        type Message = Ics26Envelope;
        type Event = IbcEvent;
        type ContextRead = CtxRead;
        type ContextWrite = CtxWrite;
        type Logger = DefaultLogger;
        type EventEmitter = DefaultEventEmitter<Self::Event>;

        fn check(
            &mut self,
            ctx: &Self::ContextRead,
            msg: Self::Message,
        ) -> Result<Self::CheckResult, Self::Error> {
            match (msg, self) {
                (
                    Ics26Envelope::Ics2Msg(ClientMsg::UpdateClient(msg)),
                    Ics26Handler::ClientUpdate(handler),
                ) => handler
                    .check(ctx, msg)
                    .map(Ics26CheckResult::ClientUpdate)
                    .map_err(Error::ics02_client),
                _ => todo!(),
            }
        }

        fn process(
            &mut self,
            ctx: &Self::ContextRead,
            check_result: Self::CheckResult,
        ) -> Result<Self::ProcessResult, Self::Error> {
            match (check_result, self) {
                (
                    Ics26CheckResult::ClientUpdate(check_result),
                    Ics26Handler::ClientUpdate(handler),
                ) => handler
                    .process(ctx, check_result)
                    .map(Ics26ProcessResult::ClientUpdate)
                    .map_err(Error::ics02_client),
            }
        }

        fn write(
            self,
            ctx: &mut Self::ContextWrite,
            process_result: Self::ProcessResult,
        ) -> Result<MsgReceipt<Self::Event>, Self::Error> {
            match (process_result, self) {
                (
                    Ics26ProcessResult::ClientUpdate(process_result),
                    Ics26Handler::ClientUpdate(handler),
                ) => handler
                    .write(ctx, process_result)
                    .map(|receipt| MsgReceipt {
                        events: receipt.events.into_iter().map(Into::into).collect(),
                        log: receipt.log,
                    })
                    .map_err(Error::ics02_client),
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::core::ics02_client::client_type::ClientType;
    use crate::core::ics02_client::height::Height;
    use crate::core::ics02_client::msgs::update_client::MsgUpdateClient;
    use crate::core::ics02_client::msgs::ClientMsg;
    use crate::core::ics24_host::identifier::{ChainId, ClientId};
    use crate::core::ics26_routing::experimental::ics26::Ics26Handler;
    use crate::core::ics26_routing::experimental::Handler;
    use crate::core::ics26_routing::msgs::Ics26Envelope;
    use crate::mock::context::MockContext;
    use crate::mock::host::HostType;
    use crate::prelude::*;
    use crate::test_utils::get_dummy_account_id;

    #[test]
    fn ics26() {
        let client_id = ClientId::new(ClientType::Tendermint, 0).unwrap();
        let client_height = Height::new(1, 20).unwrap();
        let update_height = Height::new(1, 21).unwrap();

        let mut ctx = MockContext::new(
            ChainId::new("mockgaiaA".to_string(), 1),
            HostType::Mock,
            5,
            Height::new(1, 1).unwrap(),
        )
        .with_client_parametrized(
            &client_id,
            client_height,
            Some(ClientType::Tendermint), // The target host chain (B) is synthetic TM.
            Some(client_height),
        );

        let ctx_b = MockContext::new(
            ChainId::new("mockgaiaB".to_string(), 1),
            HostType::SyntheticTendermint,
            5,
            update_height,
        );

        let signer = get_dummy_account_id();

        let mut block = ctx_b.host_block(update_height).unwrap().clone();
        block.set_trusted_height(client_height);

        let msg = Ics26Envelope::Ics2Msg(ClientMsg::UpdateClient(MsgUpdateClient {
            client_id: client_id.clone(),
            header: block.into(),
            signer,
        }));

        let mut ics26_handler = Ics26Handler::new(&msg);
        let check_result = ics26_handler.check(&ctx, msg).unwrap();
        let process_result = ics26_handler.process(&ctx, check_result).unwrap();
        let receipt = ics26_handler.write(&mut ctx, process_result).unwrap();

        assert_eq!(receipt.events.len(), 1);
    }
}

// pub trait Store {
//     type Error;
//     type Key;
//     type Value;
//
//     fn set(
//         &mut self,
//         key: Self::Key,
//         value: Self::Value,
//     ) -> Result<Option<Self::Value>, Self::Error>;
//
//     fn get(&self, key: &Self::Key) -> Result<Option<Self::Value>, Self::Error>;
//
//     fn delete(&mut self, key: &Self::Key) -> Result<(), Self::Error>;
// }
//
// pub struct IbcStore<S> {
//     inner: S,
// }
//
// impl<S> Store for IbcStore<S>
// where
//     S: Store,
//     S::Key: ValueForPath,
//     S::Value: From<<<S as Store>::Key as ValueForPath>::StoredValue>,
// {
//     type Error = S::Error;
//     type Key = S::Key;
//     type Value = S::Value;
//
//     fn set(
//         &mut self,
//         key: Self::Key,
//         value: Self::Value,
//     ) -> Result<Option<Self::Value>, Self::Error> {
//         self.inner.set(key, value)
//     }
//
//     fn get(&self, key: &Self::Key) -> Result<Option<Self::Value>, Self::Error> {
//         self.inner.get(key)
//     }
//
//     fn delete(&mut self, key: &Self::Key) -> Result<(), Self::Error> {
//         self.inner.delete(key)
//     }
// }

#[derive(Clone)]
pub enum IbcValue {
    ClientType(ClientType),
    CommitmentsPath(PacketCommitment),
}

impl From<ClientType> for IbcValue {
    fn from(client_type: ClientType) -> Self {
        Self::ClientType(client_type)
    }
}

impl TryFrom<IbcValue> for ClientType {
    type Error = ();

    fn try_from(_value: IbcValue) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl From<PacketCommitment> for IbcValue {
    fn from(commitment: PacketCommitment) -> Self {
        Self::CommitmentsPath(commitment)
    }
}

impl TryFrom<IbcValue> for PacketCommitment {
    type Error = ();

    fn try_from(_value: IbcValue) -> Result<Self, Self::Error> {
        todo!()
    }
}

pub trait ValueForPath {
    type StoredValue;
}

impl ValueForPath for ClientTypePath {
    type StoredValue = ClientType;
}

impl ValueForPath for CommitmentsPath {
    type StoredValue = PacketCommitment;
}

pub trait IbcStore<K, V>
where
    K: Into<IbcPath> + ValueForPath<StoredValue = V>,
{
    type Error;

    fn set(&mut self, key: K, value: V) -> Result<Option<V>, Self::Error>;

    fn get(&self, key: K) -> Result<Option<V>, Self::Error>;

    fn delete(&mut self, key: K) -> Result<(), Self::Error>;
}

#[cfg(test)]
mod tests {
    use alloc::collections::BTreeMap;

    use super::*;
    use crate::core::ics24_host::identifier::ClientId;
    use crate::core::ics24_host::Path as IbcPath;

    #[test]
    fn test_store() {
        #[derive(Default)]
        struct KvFoo(BTreeMap<IbcPath, IbcValue>);

        impl<K, V> IbcStore<K, V> for KvFoo
        where
            K: Into<IbcPath> + ValueForPath<StoredValue = V>,
            V: Into<IbcValue> + TryFrom<IbcValue, Error = ()>,
        {
            type Error = ();

            fn set(&mut self, key: K, value: V) -> Result<Option<V>, Self::Error> {
                if let Some(result) = self.0.insert(key.into(), value.into()) {
                    Ok(Some(result.try_into()?))
                } else {
                    Ok(None)
                }
            }

            fn get(&self, key: K) -> Result<Option<V>, Self::Error> {
                if let Some(result) = self.0.get(&key.into()) {
                    Ok(Some(result.clone().try_into()?))
                } else {
                    Ok(None)
                }
            }

            fn delete(&mut self, _key: K) -> Result<(), Self::Error> {
                todo!()
            }
        }

        let mut store = KvFoo::default();
        let _client_type = store.get(ClientTypePath(ClientId::default()));
        let _commitment = store.set(
            CommitmentsPath {
                port_id: Default::default(),
                channel_id: Default::default(),
                sequence: Default::default(),
            },
            PacketCommitment::from(vec![]),
        );
    }
}
