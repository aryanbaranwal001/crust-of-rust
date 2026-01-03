#![allow(dead_code)]

use std::cell::UnsafeCell;

pub struct Cell<T> {
    pub value: UnsafeCell<T>,
}

unsafe impl<T> Sync for Cell<T> {}

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

    pub fn get(&self) -> T
    where
        T: Copy,
    {
        unsafe { *self.value.get() }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn thread_races() {
        use std::sync::Arc;
        let x = Arc::new(Cell::new(0));
        let x1 = Arc::clone(&x);
        let jh1 = std::thread::spawn(move || {
            for _ in 0..1000_000 {
                let x = x1.get();
                x1.set(x + 1);
            }
        });

        let x2 = Arc::clone(&x);
        let jh2 = std::thread::spawn(move || {
            for _ in 0..1000_000 {
                let x = x2.get();
                x2.set(x + 1);
            }
        });

        jh1.join().unwrap();
        jh2.join().unwrap();

        assert_eq!(x.get(), 2000_000);
    }
}
