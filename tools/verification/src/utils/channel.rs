// use core::cell::RefCell;
// use alloc::rc::Rc;

// use crate::types::queue::Queue;
// use crate::std_prelude::*;

// pub struct Channel<T> {
//     send_queue: Queue<T>,
//     recv_queue: Vec<Rc<RefCell<Option<T>>>>,
// }

// unsafe impl<T: Send> Send for Channel<T> {}
// unsafe impl<T: Sync> Sync for Channel<T> {}

// impl<T> Channel<T> {
//     fn new() -> Channel<T> {
//         Channel {
//             send_queue: Queue::new(),
//             recv_queue: Vec::new(),
//         }
//     }

//     fn send(&self, val: &T) {

//     }
// }
