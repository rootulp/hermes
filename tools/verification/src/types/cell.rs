use alloc::rc::Rc;
use core::cell::{Ref, RefCell, RefMut};

use crate::runtime::globals::set_global_state_modified;
use crate::std_prelude::*;

pub struct Cell<T> {
    cell: Rc<RefCell<T>>,
}

impl<T> Clone for Cell<T> {
    fn clone(&self) -> Self {
        Self {
            cell: self.cell.clone(),
        }
    }
}

unsafe impl<T: Send> Send for Cell<T> {}
unsafe impl<T: Sync> Sync for Cell<T> {}

impl<T> Cell<T> {
    pub fn new(val: T) -> Cell<T> {
        Cell {
            cell: Rc::new(RefCell::new(val)),
        }
    }

    pub fn borrow_mut(&self) -> RefMut<T> {
        set_global_state_modified();
        self.cell.borrow_mut()
    }

    pub fn borrow(&self) -> Ref<T> {
        self.cell.borrow()
    }
}
