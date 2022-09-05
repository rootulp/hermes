use crate::core::traits::client::HasAnyClientTypes;
use crate::core::traits::event::HasEvent;
use crate::core::traits::ibc::HasIbcTypes;

pub trait InjectUpdateClientEvent: HasAnyClientTypes + HasEvent + HasIbcTypes {
    fn inject_update_client_event(
        client_id: &Self::ClientId,
        client_type: &Self::ClientType,
        consensus_height: &Self::Height,
        header: &Self::AnyClientHeader,
    ) -> Self::Event;
}
