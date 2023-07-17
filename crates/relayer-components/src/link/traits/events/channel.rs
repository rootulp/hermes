use crate::chain::traits::types::event::HasEventType;
use crate::core::traits::sync::Async;
use crate::link::traits::ibc::HasIbcTypes;

pub trait HasChannelOpenInitEvent<Chain, Counterparty>: HasIbcTypes<Chain, Counterparty>
where
    Chain: HasEventType,
{
    type ChannelOpenInitEvent: Async;

    fn try_extract_channel_open_init_event(
        event: Self::Event,
    ) -> Option<Self::ChannelOpenInitEvent>;

    fn channel_open_init_event_channel_id(event: &Self::ChannelOpenInitEvent) -> &Self::ChannelId;
}

pub trait HasChannelOpenTryEvent<Chain, Counterparty>: HasIbcTypes<Counterparty>
where
    Chain: HasEventType,
{
    type ChannelOpenTryEvent: Async;

    fn try_extract_channel_open_try_event(event: Chain::Event)
        -> Option<Self::ChannelOpenTryEvent>;

    fn channel_open_try_event_channel_id(event: &Self::ChannelOpenTryEvent) -> &Self::ChannelId;
}
