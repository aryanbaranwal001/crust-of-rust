#![allow(dead_code)]

// This section of code was made due to not able to understand the
// following concept

use std::cell::UnsafeCell;

pub struct Cell<T> {
    pub value: UnsafeCell<T>,
}

impl<T> Cell<T> {
    pub fn new(value: T) -> Self {
        Cell {
            value: UnsafeCell::new(value),
        }
    }

    pub fn set(&self, value: T) {
        unsafe {
            *self.value.get() = value;
        }
    }

    pub fn get(&self) -> &T {
        unsafe { &*self.value.get() }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    /// I just can't understand, why if the reference value is being deallocated
    /// why the reference gets updated with new value
    /// why this works?
    ///
    /// In video he says - "this works due to memory system"
    #[test]
    fn ref_stay1() {
        let x = Cell::new(String::from("hello"));
        let first = x.get();
        println!("[debug]: {:?}", first);

        x.set(String::new());
        println!("[debug]: {:?}", first);

        x.set(String::from("world"));
        println!("[debug]: {:?}", first);
    }
}
