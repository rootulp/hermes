pub trait AnyClientContext {
    type Error;

    type ClientType: Eq;

    type AnyClientState;

    type AnyConsensusState;

    type AnyClientHeader;

    type AnyMisbehavior;
}

pub trait ClientContext<ClientTag>: AnyClientContext {
    const CLIENT_TYPE: Self::ClientType;

    type ClientState;

    type ConsensusState;

    type ClientHeader;

    type Misbehavior;

    fn to_any_client_state(client_state: Self::ClientState) -> Self::AnyClientState;

    fn try_from_any_client_state(client_state: &Self::AnyClientState)
        -> Option<&Self::ClientState>;

    fn to_any_consensus_state(consensus_state: Self::ConsensusState) -> Self::AnyConsensusState;

    fn try_from_any_consensus_state(
        consensus_state: &Self::AnyConsensusState,
    ) -> Option<&Self::ConsensusState>;

    fn to_any_client_header(header: Self::ClientHeader) -> Self::AnyClientHeader;

    fn try_from_any_client_header(header: &Self::AnyClientHeader) -> Option<&Self::ClientHeader>;

    fn to_any_misbehavior(misbehavior: Self::Misbehavior) -> Self::AnyMisbehavior;

    fn try_from_any_misbehavior(misbehavior: &Self::AnyMisbehavior) -> Option<&Self::Misbehavior>;
}
