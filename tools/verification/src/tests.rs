use alloc::rc::Rc;
use core::any;
use core::cell::{Ref, RefCell, RefMut, UnsafeCell};
use core::future::{Future, IntoFuture};
use core::pin::Pin;
use futures::future::FutureExt;
use ibc_relayer_framework::base::chain::traits::queries::status::CanQueryChainStatus;
use ibc_relayer_framework::base::transaction::impls::poll;

use crate::mock::{ChainStatus, MockChain};
use crate::runtime::future::{pin_future, poll_future, poll_future_generic};
use crate::runtime::nondeterminism::{any_bool, any_natural, any_usize, assume};
// use crate::runtime::task::{resume_any_task, spawn};
use crate::std_prelude::*;
use crate::types::aliases::Natural;
use crate::types::cell::Cell;
use crate::types::once::{new_channel_once, ReceiverOnce, SenderOnce};

/**
   A very basic test to test the model checking capabilities of Kani.
*/

fn run_any_task<F1, F2>(
    task1: &mut Option<Pin<Box<F1>>>,
    task2: &mut Option<Pin<Box<F2>>>,
    done: bool,
) -> bool
where
    F1: Future<Output = ()>,
    F2: Future<Output = ()>,
{
    if done {
        true
    } else if task1.is_none() && task2.is_none() {
        true
    } else {
        if any_bool() {
            assume(task1.is_some());
            let task = task1.as_mut().unwrap();
            let res = poll_future_generic(task);
            if res.is_some() {
                *task1 = None;
            }
        } else {
            assume(task2.is_some());
            let task = task2.as_mut().unwrap();
            let res = poll_future_generic(task);
            if res.is_some() {
                *task2 = None;
            }
        }
        false
    }
}

pub async fn test_kani() {
    let (sender, receiver) = new_channel_once::<u8>();

    let mut task1 = Some(Box::pin(async move {
        sender.send(2);
    }));

    let mut task2 = Some(Box::pin(async move {
        let val = receiver.await;
        assert!(val == 2);
    }));

    let done = run_any_task(&mut task1, &mut task2, false);
    let done = run_any_task(&mut task1, &mut task2, done);
    // let done = run_any_task(&mut task1, &mut task2, done);

    // panic!("done");

    assert!(!(task1.is_none() && task2.is_none()));

    // if task1.is_none() && task2.is_none() {
    //     // panic!("execution completed");
    //     return
    // }

    // poll_future_generic(&mut task1).unwrap();
    // poll_future_generic(&mut task2).unwrap();

    // let mut future = pin_future(async {
    //     receiver.await;
    // });
    // spawn(future);
    // poll_future(&mut future);

    // spawn(foo());

    // spawn(pin_future(async move {
    // }));

    // spawn(pin_future(async move {
    //     sender.send(2);
    // }));

    // spawn(pin_future(async move {
    //     let val = receiver.await;
    //     assert!(val != 2);
    // }));

    // resume_any_task();
    // resume_any_task();
    // resume_any_task();
    // resume_any_task(2);
    // resume_any_task(1);

    // while runtime.spawner.has_pending_tasks() {
    // resume_any_task();

    // resume_any_task();
    // resume_any_task();

    // receiver.await;
    // resume_any_task();
    // }

    // let init_height = any_natural();
    // let init_timestamp = any_natural();

    // let mut chain = MockChain {
    //     current_status: ChainStatus {
    //         height: init_height,
    //         timestamp: init_timestamp,
    //     },
    // };

    // // If we do not check the chain's current height before
    // // incrementing, this would result in an error from Kani
    // if chain.current_status.height < Natural::MAX {
    //     chain.current_status.height += 1;

    //     let mut future = chain.query_chain_status();

    //     // MVP that we can poll future result manually inside Kani
    //     let status = poll_future(&mut future).unwrap().unwrap();

    //     assert_eq!(status.height, chain.current_status.height);
    // }
}

#[cfg(kani)]
#[kani::proof]
#[kani::unwind(10)]
async fn run_test_kani() {
    // format!("{}", 0);
    // let arr: Rc<RefCell<Vec<Pin<Box<dyn Future<Output=()> + Send + Sync + 'static>>>>> = Rc::new(RefCell::new(Vec::new()));

    test_kani().await;
}
