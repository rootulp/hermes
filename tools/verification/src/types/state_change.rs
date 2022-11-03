use alloc::rc::Rc;
use core::cell::RefCell;

use crate::std_prelude::*;

#[derive(Clone)]
pub struct StateChangeFlag {
    flag: Rc<RefCell<bool>>,
}

unsafe impl Send for StateChangeFlag {}
unsafe impl Sync for StateChangeFlag {}

impl StateChangeFlag {
    pub fn new() -> Self {
        Self {
            flag: Rc::new(RefCell::new(false)),
        }
    }

    pub fn is_state_modified(&self) -> bool {
        *self.flag.borrow()
    }

    pub fn set_state_modified(&self) {
        *self.flag.borrow_mut() = true;
    }

    pub fn clear_state_modified(&self) {
        *self.flag.borrow_mut() = false;
    }
}
