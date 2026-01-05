// removing form the front of a vec has a overhead cost, as it moves every data afterwards
// to the beginning. Hence we are using VecDeque

#![allow(dead_code)]
#![allow(unused_imports)]

use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;

// #[derive(Clone)]
// not doing this as it requires inner value to be clone as well
pub struct Sender<T> {
    inner: Arc<Inner<T>>,
}

// compiler understands on which type to .clone on, but its just not
// idiomatic rust

impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        Sender {
            inner: Arc::clone(&self.inner),
        }
    }
}

pub struct Receiver<T> {
    inner: Arc<Inner<T>>,
}

struct Inner<T> {
    queue: Mutex<VecDeque<T>>,
    available: Condvar,
}

impl<T> Sender<T> {
    fn send(&mut self, t: T) {
        let mut queue = self.inner.queue.lock().unwrap();
        queue.push_back(t);

        // you are not required this drop
        // but he did so for ig, improving the performance of it
        // drop(queue);

        self.inner.available.notify_one();
        // even sleeping this thread after notify_one,
        // the other thread received the data;
    }
}

impl<T> Receiver<T> {
    fn recv(&mut self) -> T {
        let mut queue = self.inner.queue.lock().unwrap();

        loop {
            match queue.pop_front() {
                Some(t) => return t,
                None => {
                    // os can wake this thread for some other reasons, that's why the
                    // for loop
                    queue = self.inner.available.wait(queue).unwrap();
                }
            }
        }
    }
}

impl<T> Inner<T> {
    fn new(vec: VecDeque<T>) -> Self {
        Inner {
            queue: Mutex::new(vec),
            available: Condvar::new(),
        }
    }
}

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let inner = Inner::new(VecDeque::new());
    let inner = Arc::new(inner);

    (
        Sender {
            inner: inner.clone(),
        },
        Receiver {
            inner: inner.clone(),
        },
    )
}

#[cfg(test)]

mod tests {
    use std::{thread, time::Duration};

    use super::*;

    // #[test]
    // fn deadlock_check() {
    //     let (mut tx, mut rx) = channel::<i32>();
    //     let jh1 = thread::spawn(move || {
    //         println!("[recv] waiting started");
    //         let t = rx.recv();
    //         println!("[recv] value from tx: {t}");
    //     });

    //     let jh2 = thread::spawn(move || {
    //         println!("[send] sending started");
    //         thread::sleep(Duration::from_millis(2000));
    //         tx.send(32);
    //         println!("[send] send the value");
    //     });
    //     let _ = jh1.join();
    //     let _ = jh2.join();
    // }

    // #[test]
    // fn notify_one_latest() {
    //     let data: Arc<(Mutex<&str>, Condvar)> = Arc::new((Mutex::new(""), Condvar::new()));

    //     let mut handle_vec = Vec::new();
    //     for i in 0..100 {
    //         let data = Arc::clone(&data);
    //         let handle = thread::spawn(move || {
    //             let (lock, cvar) = &*data;
    //             let guard = lock.lock().unwrap();
    //             println!("[recv] waiting started for: {}", i);
    //             let _unused = cvar.wait(guard);
    //             println!("[recv] waiting stopped for count: {}", i);
    //         });
    //         handle_vec.push(handle);
    //     }

    //     thread::sleep(Duration::from_secs(3));

    //     let data = Arc::clone(&data);
    //     let jh2 = thread::spawn(move || {
    //         let (_, cvar) = &*data;

    //         cvar.notify_one();

    //         println!("[send] notified one");
    //     });

    //     let _ = jh2.join();

    //     thread::sleep(Duration::from_secs(10));
    // }

    #[test]
    fn ping_pong() {
        let (mut tx, mut rx) = channel();
        tx.send(42);
        assert_eq!(rx.recv(), 42);
    }

    // #[test]
    // fn closed() {
    //     let (tx, mut rx) = channel::<()>();
    //     let _ = tx;
    //     let _ = rx.recv();
    // }
}
