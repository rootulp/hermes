use crate::core::traits::sync::Async;

pub trait HasConsensusStateType<Chain, Counterparty>: Async {
    /**
        The consensus state of the `Self` chain's client on the `Counterparty` chain
    */
    type ConsensusState: Async;
}

pub trait HasCounterpartyConsensusStateType<Chain, Counterparty>:
    HasConsensusStateType<Counterparty, Chain, ConsensusState = Self::CounterpartyConsensusState>
{
    type CounterpartyConsensusState: Async;
}

impl<Link, Chain, Counterparty> HasCounterpartyConsensusStateType<Chain, Counterparty> for Link
where
    Link: HasConsensusStateType<Counterparty, Chain>,
{
    type CounterpartyConsensusState = Link::ConsensusState;
}
