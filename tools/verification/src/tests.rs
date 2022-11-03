use alloc::rc::Rc;
use core::cell::{Ref, RefCell, RefMut, UnsafeCell};
use core::future::Future;
use core::pin::Pin;
use ibc_relayer_framework::base::chain::traits::queries::status::CanQueryChainStatus;

use crate::mock::{ChainStatus, MockChain};
use crate::std_prelude::*;
use crate::types::aliases::Natural;
use crate::types::cell::Cell;
use crate::types::runtime::TestRuntime;
use crate::types::state_change::StateChangeFlag;
use crate::types::task::TaskSpawner;
use crate::utils::future::poll_future;
use crate::utils::nondeterminism::any_natural;

/**
   A very basic test to test the model checking capabilities of Kani.
*/

fn foo() -> Pin<Box<dyn Future<Output = ()> + Send + Sync + 'static>> {
    Box::pin(async {
        let x = 3;
    })
}

fn bar() -> Pin<Box<dyn Future<Output = ()> + Send + Sync + 'static>> {
    let future = foo();
    let mut list = vec![future];
    list.pop().unwrap()
}

// #[kani::proof]
// #[kani::unwind(10)]
pub async fn test_kani() {
    // let arr: &'static mut RefCell<Vec<Pin<Box<dyn Future<Output=()> + Send + Sync + 'static>>>> = Box::leak(Box::new(RefCell::new(Vec::new())));
    // let flag = StateChangeFlag::new();
    // let arr: Rc<Vec<Box<u8>>> = Rc::new(Vec::new());
    // let arr: Vec<Cell<u8>> = Vec::new();
    // let queue: Vec<Pin<Box<dyn Future<Output=()> + Send + Sync + 'static>>> =
    //     Vec::new();
    // let cell = Cell::new(&flag);
    // let spawner = TaskSpawner::new(&flag);

    // let cell = Cell::new(&flag, 8u8);

    let runtime = TestRuntime::new();

    // let mut futures: Vec<Pin<Box<dyn Future<Output=()> + Send + Sync + 'static>>> = Vec::new();

    // futures.push(foo());
    // futures.push(bar());
    // let future = futures.get_mut(0).unwrap();

    // poll_future(future);

    let mut future = bar();
    poll_future(&mut future);

    // let (sender, receiver) = runtime.channel.new_channel::<u8>();

    // runtime.spawner.spawn(foo());

    // runtime.spawner.resume_any_task();

    // runtime.spawner.spawn(async move {
    //     sender.send(2);
    // });

    // runtime.spawner.spawn(async move {
    //     let val = receiver.recv().await;
    //     assert_ne!(val, 2);
    // });

    // while runtime.spawner.has_pending_tasks() {
    // runtime.spawner.resume_any_task();
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
