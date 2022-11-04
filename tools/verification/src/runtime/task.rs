use alloc::collections::LinkedList;
use core::future::Future;
use core::mem::MaybeUninit;
use core::pin::Pin;

use crate::runtime::future::poll_future;
use crate::runtime::globals::{
    clear_global_state_modified, is_global_state_modified, set_global_state_modified,
};
use crate::runtime::nondeterminism::{any_bool, any_usize, assume};
use crate::std_prelude::*;
use crate::types::cell::Cell;

const MAX_TASKS: usize = 8;

type TaskQueue = [Option<Pin<Box<dyn Future<Output = ()> + Send + Sync + 'static>>>; MAX_TASKS];

static mut TASK_QUEUE_INITIALIZED: bool = false;
static mut CURRENT_TASK_COUNT: usize = 0;
static mut TASK_QUEUE: MaybeUninit<TaskQueue> = MaybeUninit::uninit();

pub fn init_task_queue() {
    unsafe {
        assert!(!TASK_QUEUE_INITIALIZED);
        TASK_QUEUE_INITIALIZED = true;
        TASK_QUEUE.write(Default::default());
    }
}

pub fn assert_task_queue_initialized() {
    unsafe {
        assert!(TASK_QUEUE_INITIALIZED);
    }
}

fn increment_current_task_count() {
    let count = unsafe { &mut CURRENT_TASK_COUNT };
    assert!(*count < MAX_TASKS);
    *count += 1;
}

fn decrement_current_task_count() {
    let count = unsafe { &mut CURRENT_TASK_COUNT };
    assert!(*count > 0);
    *count -= 1;
}

fn current_task_count() -> usize {
    unsafe { CURRENT_TASK_COUNT }
}

fn borrow_queue() -> &'static TaskQueue {
    assert_task_queue_initialized();
    unsafe { TASK_QUEUE.assume_init_ref() }
}

fn borrow_mut_queue() -> &'static mut TaskQueue {
    assert_task_queue_initialized();
    set_global_state_modified();
    unsafe { TASK_QUEUE.assume_init_mut() }
}

pub fn spawn(future: Pin<Box<dyn Future<Output = ()> + Send + Sync + 'static>>) {
    increment_current_task_count();

    // let i = any_usize();
    // assume(i < MAX_TASKS);
    let i = 0;

    let m_task = &mut borrow_mut_queue()[i];
    // assume(m_task.is_none());
    *m_task = Some(future);
}

pub fn has_pending_tasks() -> bool {
    !borrow_queue().is_empty()
}

// pub fn remove<T>(list: &mut LinkedList<T>, at: usize) -> T {
//     list.remove(at)
// }

pub fn resume_any_task() {
    let queue = borrow_mut_queue();
    if current_task_count() == 0 {
        return;
    }

    clear_global_state_modified();

    // let i = any_usize();
    // assume(i < MAX_TASKS);
    let i = 0;

    let m_task = &mut queue[i];
    // assume(m_task.is_some());

    let task = m_task.as_mut().unwrap();
    poll_future(task).unwrap();

    // for _ in 0..queue.len() {
    // let i = any_usize();
    // assume(i < queue.len());

    // let mut task = queue.pop_front().unwrap();
    // // let should_run_task = any_bool();
    // if should_run_task {
    //     let res = poll_future(&mut task);
    // } else {
    //     queue.push_back(task);
    // }

    // let mut task = queue.remove(i);
    // assume(m_task.is_some());
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
