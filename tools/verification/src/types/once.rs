use core::cell::UnsafeCell;
use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};

use crate::runtime::future::pin_future;
use crate::runtime::task::spawn;
use crate::std_prelude::*;
use crate::types::cell::Cell;

pub struct ReceiverOnce<T: 'static> {
    cell: Cell<Option<T>>,
}

pub struct SenderOnce<T: 'static> {
    cell: Cell<Option<T>>,
}

pub fn new_channel_once<T>() -> (SenderOnce<T>, ReceiverOnce<T>) {
    let cell = Cell::new(None);
    let sender = SenderOnce { cell: cell.clone() };
    let receiver = ReceiverOnce { cell };
    (sender, receiver)
}

pub const fn sender_once_from_static<T>(cell: &'static UnsafeCell<Option<T>>) -> SenderOnce<T> {
    let cell = Cell::from_static(cell);
    let sender = SenderOnce { cell };
    sender
}

pub const fn receiver_once_from_static<T>(cell: &'static UnsafeCell<Option<T>>) -> ReceiverOnce<T> {
    let cell = Cell::from_static(cell);
    let receiver = ReceiverOnce { cell };
    receiver
}

impl<T> Future for ReceiverOnce<T> {
    type Output = T;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        let has_val = self.cell.borrow().is_some();

        if has_val {
            let val = self.cell.borrow_mut().take().unwrap();
            Poll::Ready(val)
        } else {
            Poll::Pending
        }
    }
}

impl<T: Send + Sync + 'static> SenderOnce<T> {
    pub fn send(self, val: T) {
        let cell = self.cell;
        *cell.borrow_mut() = Some(val);
        // spawn(pin_future(async move { *cell.borrow_mut() = Some(val) }));
    }
}
