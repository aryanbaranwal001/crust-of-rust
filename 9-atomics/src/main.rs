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
            // and in such a way such that no other throead is allowed to intervene
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

// #[test]
fn too_relaxed() -> (usize, usize) {
    use std::sync::atomic::AtomicUsize;
    let x: &'static _ = Box::leak(Box::new(AtomicUsize::new(0)));
    let y: &'static _ = Box::leak(Box::new(AtomicUsize::new(0)));
    let t1 = spawn(move || {
        // t1: JoinHandle<usize>
        let r1 = y.load(Ordering::Relaxed); // r1: usize
        x.store(r1, Ordering::Relaxed);
        r1
    });
    let t2 = spawn(move || {
        // t2: JoinHandle<usize>
        let r2 = x.load(Ordering::Relaxed); // r2: usize
        y.store(42, Ordering::Relaxed);
        r2
    });
    let r1 = t1.join().unwrap(); // r1: usize
    let r2 = t2.join().unwrap(); // r2: usize
    (r1, r2)
}

// #[test]
// fn stress() {
//     for i in 0..1000_000 {
//         println!("[test] run: {}", i);
//         let (r1, r2) = too_relaxed();
//         assert_ne!(r1, r2);
//     }
// }

fn not_main() {
    use std::sync::atomic::AtomicUsize;
    let x: &'static _ = Box::leak(Box::new(AtomicBool::new(false)));
    let y: &'static _ = Box::leak(Box::new(AtomicBool::new(false)));
    let z: &'static _ = Box::leak(Box::new(AtomicUsize::new(0)));

    let _tx = spawn(move || {
        x.store(true, Ordering::SeqCst);
    });

    let _ty = spawn(move || {
        y.store(true, Ordering::SeqCst);
    });

    let t1 = spawn(move || {
        while !x.load(Ordering::SeqCst) {}
        if y.load(Ordering::SeqCst) {
            z.fetch_add(1, Ordering::Relaxed);
        }
    });

    let t2 = spawn(move || {
        while !y.load(Ordering::SeqCst) {}
        if x.load(Ordering::SeqCst) {
            z.fetch_add(1, Ordering::Relaxed);
        }
    });

    t1.join().unwrap();
    t2.join().unwrap();
    let z = z.load(Ordering::SeqCst);

    // What are the possible values for z?
    //  - Is 0 possible?
    //      Restrictions:
    //          we know that t1 must run "after" tx
    //          we know that t2 must run "after" ty
    //      Given that..
    //          .. tx .. t1 ..
    //          ty t2 tx t1 -> t1 will incr z
    //          ty tx ty t2 t1 -> t1 & t2 will incr z
    //          .. tx .. t1 ty t2 -> t2 will incr z
    //      Seems impossible to have a thread schedule where z == 0
    //
    //         t2    t1,t2
    //
    //  MO(x): false true
    //
    //         t1    t1,t2
    //
    //  MO(y): false true
    //
    //  - Is 1 possible?
    //      Yes: tx, t1, ty, t2
    //  - Is 2 possible?
    //      Yes: tx, ty, t1, t2
}
