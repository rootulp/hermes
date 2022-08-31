use crate::traits::sync::Async;

pub trait ClientTypes: Async {
    type ClientState: Async;

    type ConsensusState: Async;

    type ClientHeader: Async;

    type Misbehavior: Async;
}

pub trait AnyClientTypes: Async {
    type ClientType: Eq + Async;

    type AnyClientState: Async;

    type AnyConsensusState: Async;

    type AnyClientHeader: Async;

    type AnyMisbehavior: Async;
}

pub trait HasAnyClient: Async {
    type Client: ClientTypes;

    type AnyClient: HasClient<Self::Client>;
}

pub trait AnyClientMethods: AnyClientTypes {
    fn client_state_type(client_state: &Self::AnyClientState) -> Self::ClientType;
}

pub trait HasClient<Client>: AnyClientTypes
where
    Client: ClientTypes,
{
    const CLIENT_TYPE: Self::ClientType;

    fn to_any_client_state(client_state: Client::ClientState) -> Self::AnyClientState;

    fn try_from_any_client_state(
        client_state: &Self::AnyClientState,
    ) -> Option<&Client::ClientState>;

    fn to_any_consensus_state(consensus_state: Client::ConsensusState) -> Self::AnyConsensusState;

    fn try_from_any_consensus_state(
        consensus_state: &Self::AnyConsensusState,
    ) -> Option<&Client::ConsensusState>;

    fn to_any_client_header(header: Client::ClientHeader) -> Self::AnyClientHeader;

    fn try_from_any_client_header(header: &Self::AnyClientHeader) -> Option<&Client::ClientHeader>;

    fn to_any_misbehavior(misbehavior: Client::Misbehavior) -> Self::AnyMisbehavior;

    fn try_from_any_misbehavior(misbehavior: &Self::AnyMisbehavior)
        -> Option<&Client::Misbehavior>;
}
