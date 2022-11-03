// use alloc::collections::VecDeque;
// use core::cell::RefCell;

// use crate::std_prelude::*;

// pub struct Queue<T> {
//     queue: RefCell<Vec<T>>,
// }

// unsafe impl<T: Send> Send for Queue<T> {}
// unsafe impl<T: Sync> Sync for Queue<T> {}

// impl<T> Queue<T> {
//     pub const fn new() -> Self {
//         Self {
//             queue: RefCell::new(VecDeque::new())
//         }
//     }

//     pub fn push_back(&self, val: T) {
//         let mut queue_mut = self.queue.borrow_mut();
//         queue_mut.push_back(val);
//     }

//     pub fn pop_front(&self) -> Option<T> {
//         let mut queue_mut = self.queue.borrow_mut();
//         queue_mut.pop_front()
//     }
// }
