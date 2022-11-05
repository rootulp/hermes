use core::cell::UnsafeCell;
use core::mem::transmute;

use crate::runtime::globals::set_global_state_modified;
use crate::std_prelude::*;

pub struct Cell<T: 'static> {
    cell: &'static UnsafeCell<T>,
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
    pub fn new(val: T) -> Self {
        Cell {
            cell: Box::leak(Box::new(UnsafeCell::new(val))),
        }
    }

    pub fn borrow_mut(&self) -> &mut T {
        set_global_state_modified();
        unsafe { transmute(self.cell.get()) }
    }

    pub fn borrow(&self) -> &T {
        unsafe { transmute(self.cell.get()) }
    }
}
