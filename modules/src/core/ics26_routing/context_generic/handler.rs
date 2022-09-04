use alloc::boxed::Box;
use alloc::string::ToString;
use alloc::vec::Vec;
use core::marker::PhantomData;

use ibc_proto::google::protobuf::Any;
use ibc_proto::ibc::core::client::v1::MsgUpdateClient as RawMsgUpdateClient;

use crate::core::ics02_client::client_state::{ClientState, UpdatedState};
use crate::core::ics02_client::consensus_state::ConsensusState;
use crate::core::ics02_client::context::ClientReader;
use crate::core::ics02_client::error::Error as Ics02Error;
use crate::core::ics02_client::msgs::update_client::MsgUpdateClient;
use crate::core::ics24_host::identifier::ClientId;
use crate::core::ics26_routing::context_generic::api::{
    DefaultIbcTypes, DynClientContext, EventEmitter, Host, IbcHost, StoreError,
};
use crate::core::ics26_routing::context_generic::framework::{
    TypedStore, UpdateClientValidationContext,
};
use crate::events::IbcEvent;
use crate::timestamp::Timestamp;
use crate::Height;

pub trait Phase {
    type Error;
    type Input;
    type Output;
    type Context;

    fn run(&self, context: Self::Context, input: Self::Input) -> Result<Self::Output, Self::Error>;
}

pub struct Decode<Raw, Domain, P> {
    inner: P,
    _types: PhantomData<(Raw, Domain)>,
}

impl<Raw, Domain, P> Phase for Decode<Raw, Domain, P>
where
    P: Phase<Input = Domain>,
    Raw: TryInto<Domain, Error = P::Error>,
{
    type Error = P::Error;
    type Input = Raw;
    type Output = P::Output;
    type Context = P::Context;

    fn run(&self, context: Self::Context, input: Self::Input) -> Result<Self::Output, Self::Error> {
        let message = input.try_into()?;
        self.inner.run(context, message)
    }
}

pub struct CheckResult {
    client_id: ClientId,
    client_state: Box<dyn ClientState>,
    header: Any,
}

pub struct Check<T>(PhantomData<T>);

impl<T> Phase for Check<T>
where
    T: Phase<Input = CheckResult, Error = Ics02Error>,
    T::Context: UpdateClientValidationContext<
        AnyClientContext = DynClientContext,
        IbcTypes = DefaultIbcTypes,
        Error = T::Error,
    >,
{
    type Error = T::Error;
    type Input = MsgUpdateClient;
    type Output = CheckResult;
    type Context = T::Context;

    fn run(&self, context: Self::Context, input: Self::Input) -> Result<Self::Output, Self::Error> {
        let MsgUpdateClient {
            client_id, header, ..
        } = input;

        let client_state = context.client_state(client_id.clone())?;

        if client_state.is_frozen() {
            Err(Ics02Error::client_frozen(client_id))
        } else {
            Ok(CheckResult {
                client_id,
                client_state,
                header,
            })
        }
    }
}

struct Process<H>(PhantomData<H>);

pub struct ProcessResult {
    pub client_id: ClientId,
    pub client_state: Box<dyn ClientState>,
    pub consensus_state: Box<dyn ConsensusState>,
    pub processed_time: Timestamp,
    pub processed_height: Height,
}

impl<T> Phase for Process<T>
where
    T: Phase<Input = ProcessResult, Error = Ics02Error>,
    T::Context: UpdateClientValidationContext<
            AnyClientContext = DynClientContext,
            IbcTypes = DefaultIbcTypes,
            Error = T::Error,
        > + ClientReader,
{
    type Error = Ics02Error;
    type Input = CheckResult;
    type Output = ProcessResult;
    type Context = T::Context;

    fn run(&self, context: Self::Context, input: Self::Input) -> Result<Self::Output, Self::Error> {
        let CheckResult {
            client_id,
            client_state,
            header,
        } = input;

        let UpdatedState {
            client_state,
            consensus_state,
        } = client_state
            .check_header_and_update_state(&context, client_id.clone(), header)
            .map_err(|e| Ics02Error::header_verification_failure(e.to_string()))?;

        // TODO: Add support for events
        // event_emitter.emit_event(UpdateClientEvent::from(Attributes {
        //     client_id: client_id.clone(),
        //     client_type: client_state.client_type(),
        //     consensus_height: client_state.latest_height(),
        // }));

        Ok(ProcessResult {
            client_id,
            client_state,
            consensus_state,
            processed_time: ClientReader::host_timestamp(&context),
            processed_height: ClientReader::host_height(&context),
        })
    }
}

struct Nul<H>(PhantomData<H>);

impl<H> Phase for Nul<H> {
    type Error = Ics02Error;
    type Input = CheckResult;
    type Output = ();
    type Context = IbcHost<H>;

    fn run(
        &self,
        _context: Self::Context,
        _input: Self::Input,
    ) -> Result<Self::Output, Self::Error> {
        todo!()
    }
}

pub fn update_client_handler() -> PhantomData<impl Phase> {
    struct DummyStore;

    impl<K, V> TypedStore<K, V> for DummyStore {
        type Error = Ics02Error;

        fn set(&mut self, _key: K, _value: V) -> Result<(), Self::Error> {
            todo!()
        }

        fn get(&self, _key: K) -> Result<Option<V>, Self::Error> {
            todo!()
        }

        fn delete(&mut self, _key: K) -> Result<(), Self::Error> {
            todo!()
        }
    }

    #[derive(Clone, Debug)]
    pub struct DefaultEventEmitter {
        events: Vec<IbcEvent>,
    }

    impl EventEmitter for DefaultEventEmitter {
        type Event = IbcEvent;

        fn events(&self) -> &[Self::Event] {
            self.events.as_ref()
        }

        fn emit_event(&mut self, event: Self::Event) {
            self.events.push(event)
        }
    }

    struct DummyHost;

    impl Host for DummyHost {
        type Error = Ics02Error;
        type KvStore = DummyStore;
        type EventEmitter = DefaultEventEmitter;

        fn current_timestamp(&self) -> Timestamp {
            todo!()
        }

        fn current_height(&self) -> Height {
            todo!()
        }

        fn store(&self) -> &Self::KvStore {
            todo!()
        }

        fn store_mut(&mut self) -> &mut Self::KvStore {
            todo!()
        }

        fn event_emitter(&self) -> &Self::EventEmitter {
            todo!()
        }

        fn event_emitter_mut(&mut self) -> &mut Self::EventEmitter {
            todo!()
        }
    }

    impl StoreError for Ics02Error {
        fn path_not_found() -> Self {
            Self::client_not_found(ClientId::default())
        }
    }

    PhantomData::<Decode<RawMsgUpdateClient, MsgUpdateClient, Check<Nul<IbcHost<DummyHost>>>>>
}
