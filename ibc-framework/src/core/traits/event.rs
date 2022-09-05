use crate::core::traits::sync::Async;

pub trait HasEvent: Async {
    type Event: Async;
}

pub trait HasEventEmitter: HasEvent {
    fn emit_event(&self, event: &Self::Event);
}
