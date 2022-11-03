use alloc::rc::Rc;
use core::cell::{Ref, RefCell, RefMut};

use crate::std_prelude::*;
use crate::types::state_change::StateChangeFlag;

pub struct Cell<T: 'static> {
    cell: &'static RefCell<T>,
    flag: StateChangeFlag,
}

impl<T> Clone for Cell<T> {
    fn clone(&self) -> Self {
        Self {
            cell: self.cell.clone(),
            flag: self.flag.clone(),
        }
    }
}

unsafe impl<T: Send> Send for Cell<T> {}
unsafe impl<T: Sync> Sync for Cell<T> {}

impl<T: Default + 'static> Cell<T> {
    pub fn new(flag: &StateChangeFlag) -> Cell<T> {
        Cell {
            cell: Box::leak(Box::new(RefCell::new(T::default()))),
            flag: flag.clone(),
        }
    }

    pub fn borrow_mut(&self) -> RefMut<T> {
        self.flag.set_state_modified();
        self.cell.borrow_mut()
    }

    pub fn borrow(&self) -> Ref<T> {
        self.cell.borrow()
    }
}
