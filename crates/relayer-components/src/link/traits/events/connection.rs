use crate::chain::traits::types::event::HasEventType;
use crate::core::traits::sync::Async;
use crate::link::traits::ibc::HasIbcTypes;

pub trait HasConnectionOpenTryEvent<Chain, Counterparty>: HasIbcTypes<Chain, Counterparty>
where
    Chain: HasEventType,
{
    type ConnectionOpenTryEvent: Async;

    fn try_extract_connection_open_try_event(
        event: Chain::Event,
    ) -> Option<Self::ConnectionOpenTryEvent>;

    fn connection_open_try_event_connection_id(
        event: &Self::ConnectionOpenTryEvent,
    ) -> &Self::ConnectionId;
}

pub trait HasConnectionOpenInitEvent<Chain, Counterparty>:
    HasIbcTypes<Chain, Counterparty>
where
    Chain: HasEventType,
{
    type ConnectionOpenInitEvent: Async;

    fn try_extract_connection_open_init_event(
        event: Chain::Event,
    ) -> Option<Self::ConnectionOpenInitEvent>;

    fn connection_open_init_event_connection_id(
        event: &Self::ConnectionOpenInitEvent,
    ) -> &Self::ConnectionId;
}
