// removing form the front of a vec has a overhead cost, as it moves every data afterwards
// to the beginning. Hence we are using VecDeque

// Flavors:
// - Synchronous channels: Channel where send() can block. Limited capacity.
//   - Mutex + Condvar + VecDeque
//   - Atomic VecDeque (atomic queue) + thread::park + thread::Thread::notify
//
// - Asynchronous channels: Channel where send() cannot block. Unbounded.
//   - Mutex + Condvar + VecDeque
//   - Mutex + Condvar + LinkedList
//   - Atomic linked list, linked list of T
//   - Atomic block linked list, linked list of atomic VecDeque<T>
//
// - Rendezvous channels: Synchronous with capacity = 0. Used for thread synchronization.
//
// - Oneshot channels: Any capacity. In practice, only one call to send().
//
// these are different implementations choosen at runtime, for optimizations

// flume, crossbeam

#![allow(dead_code)]
#![allow(unused_imports)]

use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;

// #[derive(Clone)]
// not doing this as it requires inner value to be clone as well
pub struct Sender<T> {
    shared: Arc<Shared<T>>,
}

// compiler understands on which type to .clone on, but its just not
// idiomatic rust

impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        let mut inner = self.shared.inner.lock().unwrap();
        inner.senders += 1;

        Sender {
            shared: Arc::clone(&self.shared),
        }
    }
}

pub struct Receiver<T> {
    shared: Arc<Shared<T>>,
    buffer: VecDeque<T>,
}

struct Shared<T> {
    inner: Mutex<Inner<T>>,
    available: Condvar,
}

struct Inner<T> {
    queue: VecDeque<T>,
    senders: usize,
}

impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        let mut inner = self.shared.inner.lock().unwrap();
        inner.senders -= 1;

        if inner.senders == 0 {
            self.shared.available.notify_one();
        }

        // explicit drops are better
        drop(inner);
    }
}

impl<T> Sender<T> {
    fn send(&mut self, t: T) {
        let mut inner = self.shared.inner.lock().unwrap();
        inner.queue.push_back(t);

        // you are not required this drop
        // but he did so for ig, improving the performance of it
        // drop(queue);

        self.shared.available.notify_one();
        // even sleeping this thread after notify_one,
        // the other thread received the data;
    }
}

impl<T> Receiver<T> {
    fn recv(&mut self) -> Option<T> {
        if let Some(t) = self.buffer.pop_front() {
            return Some(t);
        }

        let mut guard = self.shared.inner.lock().unwrap();

        loop {
            match guard.queue.pop_front() {
                Some(t) => {
                    if !guard.queue.is_empty() {
                        std::mem::swap(&mut self.buffer, &mut guard.queue);
                    }
                    return Some(t);
                }
                None if guard.senders == 0 => {
                    return None;
                }

                None => {
                    // os can wake this thread for some other reasons, that's why the
                    // for loop
                    guard = self.shared.available.wait(guard).unwrap();
                }
            };
        }
    }
}

impl<T> Shared<T> {
    fn new(vec: VecDeque<T>) -> Self {
        Shared {
            inner: Mutex::new(Inner::new(vec)),
            available: Condvar::new(),
        }
    }
}

impl<T> Inner<T> {
    fn new(vec: VecDeque<T>) -> Self {
        Inner {
            queue: vec,
            senders: 1,
        }
    }
}

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let shared = Shared::new(VecDeque::new());
    let shared = Arc::new(shared);

    (
        Sender {
            shared: shared.clone(),
        },
        Receiver {
            shared: shared.clone(),
            buffer: VecDeque::new(),
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
        assert_eq!(rx.recv().unwrap(), 42);
    }

    #[test]
    fn closed() {
        let (tx, mut rx) = channel::<()>();
        drop(tx);
        let _ = rx.recv();
    }
}
