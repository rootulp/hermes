use core::default::Default;

use crate::events::IbcEvent;
use crate::prelude::*;

pub trait Handler {
    /// Message types
    /// Conversion from raw-type to domain-type encodes stateless validation
    type RawMessage;
    type Message: TryFrom<Self::RawMessage, Error = Self::Error>;

    /// Context from host (includes light client related context)
    type ContextRead;
    type ContextWrite;

    /// Error and (intermediate) results
    type Error;
    type CheckResult;
    type ProcessResult;

    /// Subset of IbcEvents that this handler could emit
    type Event: Into<IbcEvent>;

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
        &mut self,
        ctx: &mut Self::ContextWrite,
        process_result: Self::ProcessResult,
    ) -> Result<(), Self::Error>;

    /// Log a message
    fn log_message(&mut self, _msg: impl ToString) {}

    /// Emit an event
    fn emit_event(&mut self, _event: Self::Event) {}
}

#[derive(Clone, Debug, Default)]
pub struct BaseHandler<Handler, Event> {
    log: Vec<String>,
    events: Vec<Event>,
    inner: Handler,
}

impl<T: Handler, Event> From<T> for BaseHandler<T, Event> {
    fn from(handler: T) -> Self {
        Self {
            log: vec![],
            events: vec![],
            inner: handler,
        }
    }
}

impl<T: Handler> Handler for BaseHandler<T, T::Event> {
    type RawMessage = T::RawMessage;
    type Message = T::Message;
    type ContextRead = T::ContextRead;
    type ContextWrite = T::ContextWrite;
    type Error = T::Error;
    type CheckResult = T::CheckResult;
    type ProcessResult = T::ProcessResult;
    type Event = T::Event;

    fn check(
        &mut self,
        ctx: &Self::ContextRead,
        msg: Self::Message,
    ) -> Result<Self::CheckResult, Self::Error> {
        self.inner.check(ctx, msg)
    }

    fn process(
        &mut self,
        ctx: &Self::ContextRead,
        check_result: Self::CheckResult,
    ) -> Result<Self::ProcessResult, Self::Error> {
        self.inner.process(ctx, check_result)
    }

    fn write(
        &mut self,
        ctx: &mut Self::ContextWrite,
        process_result: Self::ProcessResult,
    ) -> Result<(), Self::Error> {
        self.inner.write(ctx, process_result)
    }

    fn log_message(&mut self, msg: impl ToString) {
        self.log.push(msg.to_string());
    }

    fn emit_event(&mut self, event: Self::Event) {
        self.events.push(event);
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
    pub struct UpdateClientHandler<CtxRead, CtxWrite>(PhantomData<(CtxRead, CtxWrite)>);

    impl<CtxRead, CtxWrite> Default for UpdateClientHandler<CtxRead, CtxWrite> {
        fn default() -> Self {
            Self(Default::default())
        }
    }

    impl<CtxRead: ClientReader, CtxWrite: ClientKeeper> Handler
        for UpdateClientHandler<CtxRead, CtxWrite>
    {
        type RawMessage = RawMsgUpdateClient;
        type Message = MsgUpdateClient;
        type ContextRead = CtxRead;
        type ContextWrite = CtxWrite;
        type Error = Error;
        type CheckResult = UpdateClientCheckResult;
        type ProcessResult = UpdateClientResult;
        type Event = UpdateClientEvent;

        fn check(
            &mut self,
            ctx: &Self::ContextRead,
            msg: Self::Message,
        ) -> Result<Self::CheckResult, Self::Error> {
            update_client_check(self, |client_id| ctx.client_state(client_id), msg)
        }

        fn process(
            &mut self,
            ctx: &Self::ContextRead,
            check_result: Self::CheckResult,
        ) -> Result<Self::ProcessResult, Self::Error> {
            update_client_process(self, ctx, check_result)
        }

        fn write(
            &mut self,
            ctx: &mut Self::ContextWrite,
            process_result: Self::ProcessResult,
        ) -> Result<(), Self::Error> {
            update_client_write(self, ctx, process_result)
        }
    }

    /// Only instantiable by lib
    pub struct UpdateClientCheckResult {
        client_id: ClientId,
        client_state: Box<dyn ClientState>,
        header: Any,
    }

    pub fn update_client_check<T /* CtxRead: ClientReader */>(
        _handler: &T,
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

    pub fn update_client_process<T: Handler<Event = UpdateClientEvent>, CtxRead: ClientReader>(
        handler: &mut T,
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

        handler.emit_event(UpdateClientEvent::from(Attributes {
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

    pub fn update_client_write<T, CtxWrite: ClientKeeper>(
        _handler: &T,
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

    pub trait Ics26Keeper: ClientKeeper + ConnectionKeeper + ChannelKeeper {}

    impl<T: Ics26Context> Ics26Reader for T {}
    impl<T: Ics26Context> Ics26Keeper for T {}

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

    pub enum Ics26CheckResult {
        ClientUpdate(UpdateClientCheckResult),
    }

    pub enum Ics26ProcessResult {
        ClientUpdate(UpdateClientResult),
    }

    impl<CtxRead: Ics26Reader, CtxWrite: Ics26Keeper> Handler for Ics26Handler<CtxRead, CtxWrite> {
        type RawMessage = Any;
        type Message = Ics26Envelope;
        type ContextRead = CtxRead;
        type ContextWrite = CtxWrite;
        type Error = Error;
        type CheckResult = Ics26CheckResult;
        type ProcessResult = Ics26ProcessResult;
        type Event = IbcEvent;

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
            &mut self,
            _ctx: &mut Self::ContextWrite,
            _process_result: Self::ProcessResult,
        ) -> Result<(), Self::Error> {
            todo!()
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
    use crate::core::ics26_routing::experimental::{BaseHandler, Handler};
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

        let ctx = MockContext::new(
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
        let ics26_handler: Ics26Handler<_, MockContext> = Ics26Handler::new(&msg);
        let mut ics26_handler = BaseHandler::from(ics26_handler);
        let check_result = ics26_handler.check(&ctx, msg).unwrap();
        let _process_result = ics26_handler.process(&ctx, check_result).unwrap();

        assert_eq!(ics26_handler.events.len(), 1);
    }
}
