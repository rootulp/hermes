use crate::core::traits::error::{HasError, InjectError, MismatchClientType};
use crate::core::traits::prism::Prism;
use crate::core::traits::sync::Async;

pub trait HasAnyClientTypes: Async {
    type ClientType: Eq + Async;

    type AnyClientState: Async;

    type AnyConsensusState: Async;

    type AnyClientHeader: Async;

    type AnyMisbehavior: Async;
}

pub trait HasAnyClientMethods: HasAnyClientTypes {
    fn client_state_type(client_state: &Self::AnyClientState) -> Self::ClientType;

    fn client_state_is_frozen(client_state: &Self::AnyClientState) -> bool;

    // fn client_state_trusting_period(client_state: &Self::AnyClientState) -> Duration;

    // fn consensus_state_timestamp(consensus_state: &Self::AnyConsensusState) -> Self::Timestamp;
}

pub trait HasOwnClient: HasClientTypes + HasClientTypeFor<Self::OwnClient> {
    type OwnClient: HasClientTypes<
        ClientState = Self::ClientState,
        ConsensusState = Self::ConsensusState,
        ClientHeader = Self::ClientHeader,
        Misbehavior = Self::Misbehavior,
    >;
}

pub trait HasClientTypes: Async {
    type ClientState: Async;

    type ConsensusState: Async;

    type ClientHeader: Async;

    type Misbehavior: Async;
}

pub trait HasClientTypeFor<Client>: HasAnyClientTypes {
    const CLIENT_TYPE: Self::ClientType;
}

pub trait HasClientPrisms<AnyClient, Client>:
    Prism<AnyClient::AnyClientState, Client::ClientState>
    + Prism<AnyClient::AnyConsensusState, Client::ConsensusState>
    + Prism<AnyClient::AnyClientHeader, Client::ClientHeader>
    + Prism<AnyClient::AnyMisbehavior, Client::Misbehavior>
    + HasError
where
    AnyClient: HasAnyClientTypes,
    Client: HasClientTypes,
{
    fn into_any_client_state(client_state: Client::ClientState) -> AnyClient::AnyClientState;

    fn try_from_any_client_state(
        client_state: AnyClient::AnyClientState,
    ) -> Result<Client::ClientState, Self::Error>;

    fn try_from_any_client_state_ref(
        client_state: &AnyClient::AnyClientState,
    ) -> Result<&Client::ClientState, Self::Error>;

    fn into_any_consensus_state(
        consensus_state: Client::ConsensusState,
    ) -> AnyClient::AnyConsensusState;

    fn try_from_any_consensus_state(
        consensus_state: AnyClient::AnyConsensusState,
    ) -> Result<Client::ConsensusState, Self::Error>;

    fn try_from_any_consensus_state_ref(
        consensus_state: &AnyClient::AnyConsensusState,
    ) -> Result<&Client::ConsensusState, Self::Error>;

    fn into_any_client_header(client_header: Client::ClientHeader) -> AnyClient::AnyClientHeader;

    fn try_from_any_client_header(
        client_header: AnyClient::AnyClientHeader,
    ) -> Result<Client::ClientHeader, Self::Error>;

    fn try_from_any_client_header_ref(
        client_header: &AnyClient::AnyClientHeader,
    ) -> Result<&Client::ClientHeader, Self::Error>;

    fn into_any_misbehavior(misbehavior: Client::Misbehavior) -> AnyClient::AnyMisbehavior;

    fn try_from_any_misbehavior(
        misbehavior: AnyClient::AnyMisbehavior,
    ) -> Result<Client::Misbehavior, Self::Error>;

    fn try_from_any_misbehavior_ref(
        misbehavior: &AnyClient::AnyMisbehavior,
    ) -> Result<&Client::Misbehavior, Self::Error>;
}

impl<Context, AnyClient, Client> HasClientPrisms<AnyClient, Client> for Context
where
    AnyClient: HasClientTypeFor<Client>,
    Client: HasClientTypes,
    Context: InjectError<MismatchClientType<AnyClient::ClientType>>,
    Context: Prism<AnyClient::AnyClientState, Client::ClientState>
        + Prism<AnyClient::AnyConsensusState, Client::ConsensusState>
        + Prism<AnyClient::AnyClientHeader, Client::ClientHeader>
        + Prism<AnyClient::AnyMisbehavior, Client::Misbehavior>,
{
    fn into_any_client_state(client_state: Client::ClientState) -> AnyClient::AnyClientState {
        Context::into(client_state)
    }

    fn try_from_any_client_state(
        client_state: AnyClient::AnyClientState,
    ) -> Result<Client::ClientState, Context::Error> {
        Context::try_from(client_state).ok_or_else(|| {
            Context::inject_error(MismatchClientType {
                expected_client_type: AnyClient::CLIENT_TYPE,
            })
        })
    }

    fn try_from_any_client_state_ref(
        client_state: &AnyClient::AnyClientState,
    ) -> Result<&Client::ClientState, Context::Error> {
        Context::try_from_ref(client_state).ok_or_else(|| {
            Context::inject_error(MismatchClientType {
                expected_client_type: AnyClient::CLIENT_TYPE,
            })
        })
    }

    fn into_any_consensus_state(
        consensus_state: Client::ConsensusState,
    ) -> AnyClient::AnyConsensusState {
        Context::into(consensus_state)
    }

    fn try_from_any_consensus_state(
        consensus_state: AnyClient::AnyConsensusState,
    ) -> Result<Client::ConsensusState, Context::Error> {
        Context::try_from(consensus_state).ok_or_else(|| {
            Context::inject_error(MismatchClientType {
                expected_client_type: AnyClient::CLIENT_TYPE,
            })
        })
    }

    fn try_from_any_consensus_state_ref(
        consensus_state: &AnyClient::AnyConsensusState,
    ) -> Result<&Client::ConsensusState, Context::Error> {
        Context::try_from_ref(consensus_state).ok_or_else(|| {
            Context::inject_error(MismatchClientType {
                expected_client_type: AnyClient::CLIENT_TYPE,
            })
        })
    }

    fn into_any_client_header(client_header: Client::ClientHeader) -> AnyClient::AnyClientHeader {
        Context::into(client_header)
    }

    fn try_from_any_client_header(
        client_header: AnyClient::AnyClientHeader,
    ) -> Result<Client::ClientHeader, Context::Error> {
        Context::try_from(client_header).ok_or_else(|| {
            Context::inject_error(MismatchClientType {
                expected_client_type: AnyClient::CLIENT_TYPE,
            })
        })
    }

    fn try_from_any_client_header_ref(
        client_header: &AnyClient::AnyClientHeader,
    ) -> Result<&Client::ClientHeader, Context::Error> {
        Context::try_from_ref(client_header).ok_or_else(|| {
            Context::inject_error(MismatchClientType {
                expected_client_type: AnyClient::CLIENT_TYPE,
            })
        })
    }

    fn into_any_misbehavior(misbehavior: Client::Misbehavior) -> AnyClient::AnyMisbehavior {
        Context::into(misbehavior)
    }

    fn try_from_any_misbehavior(
        misbehavior: AnyClient::AnyMisbehavior,
    ) -> Result<Client::Misbehavior, Context::Error> {
        Context::try_from(misbehavior).ok_or_else(|| {
            Context::inject_error(MismatchClientType {
                expected_client_type: AnyClient::CLIENT_TYPE,
            })
        })
    }

    fn try_from_any_misbehavior_ref(
        misbehavior: &AnyClient::AnyMisbehavior,
    ) -> Result<&Client::Misbehavior, Context::Error> {
        Context::try_from_ref(misbehavior).ok_or_else(|| {
            Context::inject_error(MismatchClientType {
                expected_client_type: AnyClient::CLIENT_TYPE,
            })
        })
    }
}

pub trait ContainsClient<Client>:
    HasAnyClientTypes + HasClientPrisms<Self, Client> + HasClientTypeFor<Client>
where
    Client: HasClientTypes,
{
}

impl<Context, Client> ContainsClient<Client> for Context
where
    Client: HasClientTypes,
    Context: HasAnyClientTypes + HasClientPrisms<Context, Client> + HasClientTypeFor<Client>,
{
}
