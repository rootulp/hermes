pub mod framework {
    pub type AnyClientState<Context> = <Context as AnyClientContext>::AnyClientState;
    pub type AnyConsensusState<Context> = <Context as AnyClientContext>::AnyConsensusState;
    pub type ClientType<Context> = <Context as IbcTypes>::ClientType;
    pub type ClientId<Context> = <Context as IbcTypes>::ClientId;
    pub type Height<Context> = <Context as IbcTypes>::Height;
    pub type Timestamp<Context> = <Context as IbcTypes>::Timestamp;

    pub trait AnyClientContext {
        type AnyClientState;
        type AnyConsensusState;
        type AnyClientHeader;
        type AnyMisbehavior;
    }

    pub trait IbcTypes {
        type ClientType;
        type ClientId;
        type Height;
        type Timestamp;
    }

    pub trait UpdateClientValidationContext {
        type AnyClientContext: AnyClientContext;
        type IbcTypes: IbcTypes;
        type Error;

        fn client_state(
            &self,
            client_id: &ClientId<Self::IbcTypes>,
        ) -> Result<AnyClientState<Self::AnyClientContext>, Self::Error>;

        fn consensus_state(
            &self,
            client_id: &ClientId<Self::IbcTypes>,
            height: Height<Self::IbcTypes>,
        ) -> Result<AnyConsensusState<Self::AnyClientContext>, Self::Error>;

        fn host_timestamp(&self) -> Timestamp<Self::IbcTypes>;

        fn host_height(&self) -> Height<Self::IbcTypes>;
    }

    pub trait UpdateClientExecutionContext {
        type AnyClientContext: AnyClientContext;
        type IbcTypes: IbcTypes;
        type Error;

        fn client_state(
            &self,
            client_id: &ClientId<Self::IbcTypes>,
        ) -> Result<AnyClientState<Self::AnyClientContext>, Self::Error>;

        fn consensus_state(
            &self,
            client_id: &ClientId<Self::IbcTypes>,
            height: Height<Self::IbcTypes>,
        ) -> Result<AnyConsensusState<Self::AnyClientContext>, Self::Error>;

        fn host_timestamp(&self) -> Timestamp<Self::IbcTypes>;

        fn host_height(&self) -> Height<Self::IbcTypes>;

        fn store_client_type(
            &mut self,
            client_id: ClientId<Self::IbcTypes>,
            client_type: ClientType<Self::IbcTypes>,
        ) -> Result<(), Self::Error>;

        fn store_client_state(
            &mut self,
            client_id: ClientId<Self::IbcTypes>,
            client_state: AnyClientState<Self::AnyClientContext>,
        ) -> Result<(), Self::Error>;

        fn store_consensus_state(
            &mut self,
            client_id: ClientId<Self::IbcTypes>,
            height: Height<Self::IbcTypes>,
            consensus_state: AnyConsensusState<Self::AnyClientContext>,
        ) -> Result<(), Self::Error>;

        fn increase_client_counter(&mut self);

        fn store_update_time(
            &mut self,
            client_id: ClientId<Self::IbcTypes>,
            height: Height<Self::IbcTypes>,
            timestamp: Timestamp<Self::IbcTypes>,
        ) -> Result<(), Self::Error>;

        fn store_update_height(
            &mut self,
            client_id: ClientId<Self::IbcTypes>,
            height: Height<Self::IbcTypes>,
            host_height: Height<Self::IbcTypes>,
        ) -> Result<(), Self::Error>;
    }
}

pub mod api {
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
}
