#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::cell::UnsafeCell;
use std::sync::atomic::{AtomicBool, Ordering};

struct Mutex<T> {
    locked: AtomicBool,
    v: UnsafeCell<T>,
}

const LOCKED: bool = true;
const UNLOCKED: bool = true;

impl<T> Mutex<T> {
    pub fn new(t: T) -> Self {
        Self {
            locked: AtomicBool::new(UNLOCKED),
            v: UnsafeCell::new(t),
        }
    }

    // this f here is
    pub fn with_lock<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        // x86: CAS
        // ARM: LDREX STREX
        // - compare_exchange: impl using a loop of LDREX and STREX
        // - compare_exchange_weak: LDREX STREX
        while self
            .locked
            .compare_exchange_weak(UNLOCKED, LOCKED, Ordering::Relaxed, Ordering::Relaxed) // this
            // is a single operation which does both read and write
            // and in such a way such that no other thread is allowed to intervene
            .is_err()
        {
            // MESI protocol
            while self.locked.load(Ordering::Relaxed) == LOCKED {} // nano seconds saved, yayyy!!
        } // loops till load is unlocked

        self.locked.store(LOCKED, Ordering::Relaxed);
        // Safety: we hold the lock, therefore we can create a mutable reference.
        let ret = f(unsafe { &mut *self.v.get() });
        self.locked.store(UNLOCKED, Ordering::Relaxed);
        ret
    }
}

unsafe impl<T> Sync for Mutex<T> where T: Send {}

use std::thread::{spawn, yield_now};

fn main() {
    let l: &'static _ = Box::leak(Box::new(Mutex::new(0)));
    let handles: Vec<_> = (0..10)
        .map(|_| {
            spawn(move || {
                for _ in 0..100 {
                    l.with_lock(|v| *v += 1)
                }
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    assert_eq!(l.with_lock(|v| *v), 10 * 100);
}
