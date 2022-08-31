use crate::core::traits::sync::Async;

pub trait HasError: Async {
    type Error: Async;
}
