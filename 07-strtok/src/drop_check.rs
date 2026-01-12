#![allow(dead_code)]
#![allow(unused_variables)]

// suppose in ffi you have a raw pointer to data managed by c program, and then
// but when struct holding that raw pointer gets dropped, you need to drop the memeory
// allocated in c world, and hence to trigger that, we use phantom data in that struct
// so rust know that, that struct owns that data.

fn my_fn() {
    let x = String::new();
    let z = vec![&x];
    drop(x);
    // drop(z);
    // println!("{:?}", z);
}

struct TouchDrop<T: std::fmt::Debug>(T);

impl<T: std::fmt::Debug> Drop for TouchDrop<T> {
    fn drop(&mut self) {
        println!("{:?}", self.0);
    }
}

use std::marker::PhantomData;

struct Deserializer<T> {
    // some fields
    _t: PhantomData<T>,
}

struct Deserializer2<T> {
    // some fields
    _t: PhantomData<fn() -> T>,
    // this is covariant
}

struct Deserializer3<T> {
    // some fields
    _t: PhantomData<fn(T)>,
    // contravariant
}

struct Deserializer4<T> {
    // some fields
    _t1: PhantomData<fn() -> T>,
    _t2: PhantomData<fn(T)>,
    // this is invariant
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        my_fn();
    }
}
