#![allow(dead_code)]

mod confusion;
mod race;

// you are never allowed ot cast a shared refernce to an exclusive referece,
// only through UnsafeCell, because if you do it by yourself, LLVM might optimize it
// in a way which doesn't produces the result you want.

// idiomatic rust basically says, put the constrains only where they are needed
// not all over the place

use std::cell::UnsafeCell;

pub struct Cell<T> {
    pub value: UnsafeCell<T>,
}

// implied by UnsafeCell
// impl<T> !Sync for Cell<T> {}
// unsafe impl<T> Sync for Cell<T> {}

impl<T> Cell<T> {
    pub fn new(value: T) -> Self {
        Cell {
            value: UnsafeCell::new(value),
        }
    }

    pub fn set(&self, value: T) {
        // SAFETY: we know no on e else is concurrently mutating self.value (because !Sync)
        // SAFETY: we know we are not invalidating any references, because we never give any out
        unsafe {
            *self.value.get() = value;
        }
    }

    pub fn get(&self) -> T
    where
        T: Copy,
    {
        // SAFETY: w eknow n o one else is modifying this value, since only this thread can mutate
        // becase !Sync of unsafecell, and it is executing this function instead.
        unsafe { *self.value.get() }
    }
}
