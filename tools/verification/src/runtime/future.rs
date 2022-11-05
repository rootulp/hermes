use core::future::{poll_fn, Future};
use core::pin::Pin;
use core::task::{Context, Poll};
use futures::task::noop_waker;

use crate::std_prelude::*;

pub fn pin_future<T>(
    future: impl Future<Output = T> + Send + Sync + 'static,
) -> Pin<Box<dyn Future<Output = T> + Send + Sync + 'static>> {
    Box::pin(future)
}

pub fn poll_future<T>(
    future: &mut Pin<Box<dyn Future<Output = T> + Send + Sync + '_>>,
) -> Option<T> {
    let waker = noop_waker();
    let mut context = Context::from_waker(&waker);

    let poll = future.as_mut().poll(&mut context);
    match poll {
        Poll::Ready(res) => Some(res),
        Poll::Pending => None,
    }
}

pub fn poll_future_generic<F, T>(future: &mut Pin<Box<F>>) -> Option<T>
where
    F: Future<Output = T>,
{
    let waker = noop_waker();
    let mut context = Context::from_waker(&waker);

    let poll = future.as_mut().poll(&mut context);
    match poll {
        Poll::Ready(res) => Some(res),
        Poll::Pending => None,
    }
}

pub async fn new_future<T, F>(poller: F) -> T
where
    F: Fn() -> Option<T>,
{
    poll_fn(move |_| {
        let res = poller();
        match res {
            Some(val) => Poll::Ready(val),
            None => Poll::Pending,
        }
    })
    .await
}
