use alloc::collections::LinkedList;
use core::future::Future;
use core::pin::Pin;

use crate::runtime::future::poll_future;
use crate::runtime::globals::{
    clear_global_state_modified, is_global_state_modified, set_global_state_modified,
};
use crate::runtime::nondeterminism::{any_bool, any_usize, assume};
use crate::std_prelude::*;
use crate::types::cell::Cell;

type TaskQueue = LinkedList<Pin<Box<dyn Future<Output = ()> + Send + Sync + 'static>>>;

static mut TASK_QUEUE: TaskQueue = LinkedList::new();

fn borrow_queue() -> &'static TaskQueue {
    unsafe { &TASK_QUEUE }
}

fn borrow_mut_queue() -> &'static mut TaskQueue {
    set_global_state_modified();
    unsafe { &mut TASK_QUEUE }
}

pub fn spawn(future: Pin<Box<dyn Future<Output = ()> + Send + Sync + 'static>>) {
    borrow_mut_queue().push_back(future);
}

pub fn has_pending_tasks() -> bool {
    !borrow_queue().is_empty()
}

// pub fn remove<T>(list: &mut LinkedList<T>, at: usize) -> T {
//     list.remove(at)
// }

pub fn resume_any_task() {
    let queue = borrow_mut_queue();
    if queue.is_empty() {
        return;
    }

    clear_global_state_modified();

    // for i in 0..queue.len() {
    // let i = any_usize();
    // assume(i < queue.len());

    let mut task = queue.pop_front().unwrap();
    // let mut task = remove(queue, i);
    // assume(m_task.is_some());
    let res = poll_future(&mut task).unwrap();
    // assume(res.is_some() || flag.is_state_modified());

    // if res.is_some() {
    //     queue.remove(i);
    // }

    // // queue.retain_mut(|task| {
    // for i in 0..queue.len() {
    //     let should_run_current_task = any_bool();

    //     if should_run_current_task {
    //         let task = queue.get_mut(i).unwrap();
    //         let res = poll_future(task);
    //         assume(res.is_some() || flag.is_state_modified());

    //         if res.is_some() {
    //             queue.remove(i);
    //         }
    //     }
    // }
    // });
    // }

    // assume(ran_task);
}
