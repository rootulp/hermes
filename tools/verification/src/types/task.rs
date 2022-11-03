use core::future::Future;
use core::pin::Pin;

use crate::std_prelude::*;
use crate::types::cell::Cell;
use crate::types::state_change::StateChangeFlag;
use crate::utils::future::poll_future;
use crate::utils::nondeterminism::{any_bool, assume};

#[derive(Clone)]
pub struct TaskSpawner {
    queue: Cell<Vec<Pin<Box<dyn Future<Output = ()> + Send + Sync + 'static>>>>,
    flag: StateChangeFlag,
}

impl TaskSpawner {
    pub fn new(flag: &StateChangeFlag) -> Self {
        Self {
            queue: Cell::new(flag),
            flag: flag.clone(),
        }
    }

    pub fn spawn<F>(&self, future: F)
    where
        F: Future<Output = ()> + Send + Sync + 'static,
    {
        let mut queue = self.queue.borrow_mut();
        queue.push(Box::pin(future));
    }

    pub fn has_pending_tasks(&self) -> bool {
        !self.queue.borrow().is_empty()
    }

    pub fn resume_any_task(&self) {
        let mut queue = self.queue.borrow_mut();
        if queue.is_empty() {
            return;
        }

        let flag = &self.flag;
        flag.clear_state_modified();
        let mut ran_task = false;

        queue.retain_mut(|task| {
            if ran_task {
                return true;
            }

            let should_run_current_task = any_bool();

            if should_run_current_task {
                let res = poll_future(task);
                assume(res.is_some() || flag.is_state_modified());

                ran_task = true;
                return false;
            }

            true
        });

        assume(ran_task);
    }
}
