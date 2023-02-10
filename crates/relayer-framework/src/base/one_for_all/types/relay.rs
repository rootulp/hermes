use alloc::sync::Arc;

pub struct OfaRelayWrapper<Relay> {
    pub relay: Arc<Relay>,
}

impl<Relay> OfaRelayWrapper<Relay> {
    pub fn new(relay: Relay) -> Self {
        Self {
            relay: Arc::new(relay),
        }
    }

    pub fn from_arc(relay: Arc<Relay>) -> Self {
        Self { relay }
    }
}

impl<Relay> Clone for OfaRelayWrapper<Relay> {
    fn clone(&self) -> Self {
        Self {
            relay: self.relay.clone(),
        }
    }
}
