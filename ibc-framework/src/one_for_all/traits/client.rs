use crate::core::traits::sync::Async;

pub trait OfaAnyClient: Async {
    type Height: Async;

    type Duration: Async;

    type Timestamp: Async;

    type ClientType: Eq + Async;

    type AnyClientState: Async;

    type AnyConsensusState: Async;

    type AnyClientHeader: Async;

    type AnyMisbehavior: Async;

    fn client_state_type(client_state: &Self::AnyClientState) -> Self::ClientType;

    fn client_state_is_frozen(client_state: &Self::AnyClientState) -> bool;

    fn client_state_trusting_period(client_state: &Self::AnyClientState) -> Self::Duration;

    fn consensus_state_height(consensus_state: &Self::AnyConsensusState) -> Self::Height;

    fn consensus_state_timestamp(consensus_state: &Self::AnyConsensusState) -> Self::Timestamp;
}

pub trait OfaClient<Context: OfaAnyClient>: Async {
    type Error;

    type ClientState: Async;

    type ConsensusState: Async;

    type ClientHeader: Async;

    type Misbehavior: Async;

    const CLIENT_TYPE: Context::ClientType;

    fn into_any_client_state(client_state: Self::ClientState) -> Context::AnyClientState;

    fn try_from_any_client_state(
        client_state: Context::AnyClientState,
    ) -> Result<Self::ClientState, Self::Error>;

    fn try_from_any_client_state_ref(
        client_state: &Context::AnyClientState,
    ) -> Result<&Self::ClientState, Self::Error>;

    fn into_any_consensus_state(
        consensus_state: Self::ConsensusState,
    ) -> Context::AnyConsensusState;

    fn try_from_any_consensus_state(
        consensus_state: Context::AnyConsensusState,
    ) -> Result<Self::ConsensusState, Self::Error>;

    fn try_from_any_consensus_state_ref(
        consensus_state: &Context::AnyConsensusState,
    ) -> Result<&Self::ConsensusState, Self::Error>;

    fn into_any_client_header(client_header: Self::ClientHeader) -> Context::AnyClientHeader;

    fn try_from_any_client_header(
        client_header: Context::AnyClientHeader,
    ) -> Result<Self::ClientHeader, Self::Error>;

    fn try_from_any_client_header_ref(
        client_header: &Context::AnyClientHeader,
    ) -> Result<&Self::ClientHeader, Self::Error>;

    fn into_any_misbehavior(misbehavior: Self::Misbehavior) -> Context::AnyMisbehavior;

    fn try_from_any_misbehavior(
        misbehavior: Context::AnyMisbehavior,
    ) -> Result<Self::Misbehavior, Self::Error>;

    fn try_from_any_misbehavior_ref(
        misbehavior: &Context::AnyMisbehavior,
    ) -> Result<&Self::Misbehavior, Self::Error>;
}
