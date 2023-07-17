use crate::chain::traits::types::height::HasHeightType;
use crate::core::traits::sync::Async;

pub trait HasClientStateType<Chain, Counterparty>: Async {
    /**
        The client state of the `Self` chain's client on the `Counterparty` chain
    */
    type ClientState: Async;
}

pub trait HasCounterpartyClientStateType<Chain, Counterparty>:
    HasClientStateType<Counterparty, Chain, ClientState = Self::CounterpartyClientState>
{
    type CounterpartyClientState: Async;
}

impl<Link, Chain, Counterparty> HasCounterpartyClientStateType<Chain, Counterparty> for Link
where
    Link: HasClientStateType<Counterparty, Chain>,
{
    type CounterpartyClientState = Link::ClientState;
}

pub trait HasClientStateFields<Chain, Counterparty>:
    HasClientStateType<Chain, Counterparty>
where
    Chain: HasHeightType,
{
    fn client_state_latest_height(client_state: &Self::ClientState) -> &Chain::Height;
}
