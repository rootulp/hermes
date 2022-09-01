use crate::core::traits::prism::Prism;
use crate::core::traits::sync::Async;

pub trait HasClientTypes: Async {
    type ClientState: Async;

    type ConsensusState: Async;

    type ClientHeader: Async;

    type Misbehavior: Async;
}

pub trait HasAnyClientTypes: Async {
    type ClientType: Eq + Async;

    type AnyClientState: Async;

    type AnyConsensusState: Async;

    type AnyClientHeader: Async;

    type AnyMisbehavior: Async;
}

pub trait HasClientHandler: HasClientTypes + ContainsClient<Self::ClientHandler> {
    type ClientHandler: HasClientTypes<
        ClientState = Self::ClientState,
        ConsensusState = Self::ConsensusState,
        ClientHeader = Self::ClientHeader,
        Misbehavior = Self::Misbehavior,
    >;
}

pub trait HasClientPrisms<AnyClient, Client>:
    Prism<AnyClient::AnyClientState, Client::ClientState>
    + Prism<AnyClient::AnyConsensusState, Client::ConsensusState>
    + Prism<AnyClient::AnyClientHeader, Client::ClientHeader>
    + Prism<AnyClient::AnyMisbehavior, Client::Misbehavior>
where
    AnyClient: HasAnyClientTypes,
    Client: HasClientTypes,
{
    fn into_any_client_state(client_state: Client::ClientState) -> AnyClient::AnyClientState;

    fn try_from_any_client_state(
        client_state: &AnyClient::AnyClientState,
    ) -> Option<&Client::ClientState>;

    fn into_any_consensus_state(
        consensus_state: Client::ConsensusState,
    ) -> AnyClient::AnyConsensusState;

    fn try_from_any_consensus_state(
        consensus_state: &AnyClient::AnyConsensusState,
    ) -> Option<&Client::ConsensusState>;

    fn into_any_client_header(client_header: Client::ClientHeader) -> AnyClient::AnyClientHeader;

    fn try_from_any_client_header(
        client_header: &AnyClient::AnyClientHeader,
    ) -> Option<&Client::ClientHeader>;

    fn into_any_misbehavior(misbehavior: Client::Misbehavior) -> AnyClient::AnyMisbehavior;

    fn try_from_any_misbehavior(
        misbehavior: &AnyClient::AnyMisbehavior,
    ) -> Option<&Client::Misbehavior>;
}

impl<Context, AnyClient, Client> HasClientPrisms<AnyClient, Client> for Context
where
    AnyClient: HasAnyClientTypes,
    Client: HasClientTypes,
    Context: Prism<AnyClient::AnyClientState, Client::ClientState>
        + Prism<AnyClient::AnyConsensusState, Client::ConsensusState>
        + Prism<AnyClient::AnyClientHeader, Client::ClientHeader>
        + Prism<AnyClient::AnyMisbehavior, Client::Misbehavior>,
{
    fn into_any_client_state(client_state: Client::ClientState) -> AnyClient::AnyClientState {
        Context::into(client_state)
    }

    fn try_from_any_client_state(
        client_state: &AnyClient::AnyClientState,
    ) -> Option<&Client::ClientState> {
        Context::try_from_ref(client_state)
    }

    fn into_any_consensus_state(
        consensus_state: Client::ConsensusState,
    ) -> AnyClient::AnyConsensusState {
        Context::into(consensus_state)
    }

    fn try_from_any_consensus_state(
        consensus_state: &AnyClient::AnyConsensusState,
    ) -> Option<&Client::ConsensusState> {
        Context::try_from_ref(consensus_state)
    }

    fn into_any_client_header(client_header: Client::ClientHeader) -> AnyClient::AnyClientHeader {
        Context::into(client_header)
    }

    fn try_from_any_client_header(
        client_header: &AnyClient::AnyClientHeader,
    ) -> Option<&Client::ClientHeader> {
        Context::try_from_ref(client_header)
    }

    fn into_any_misbehavior(misbehavior: Client::Misbehavior) -> AnyClient::AnyMisbehavior {
        Context::into(misbehavior)
    }

    fn try_from_any_misbehavior(
        misbehavior: &AnyClient::AnyMisbehavior,
    ) -> Option<&Client::Misbehavior> {
        Context::try_from_ref(misbehavior)
    }
}

pub trait AnyClientMethods: HasAnyClientTypes {
    fn client_state_type(client_state: &Self::AnyClientState) -> Self::ClientType;
}

pub trait ContainsClient<Client>: HasAnyClientTypes + HasClientPrisms<Self, Client>
where
    Client: HasClientTypes,
{
    const CLIENT_TYPE: Self::ClientType;
}
