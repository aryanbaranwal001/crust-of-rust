use std::fmt::Debug;
use std::marker::PhantomData;

struct Oisann<T: Debug>(T);

impl<T: Debug> Drop for Oisann<T> {
    fn drop(&mut self) {
        println!("{:?}", self.0);
    }
}

struct EmptyIterator<T> {
    _t: PhantomData<fn() -> T>,
}

impl<T> Iterator for EmptyIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

use std::iter::Empty;

fn works() {
    let mut a = 42;
    let mut it = Empty::default();
    {
        let mut o = Some(Oisann(&mut a));
        o = it.next();
    }
    println!("{:?}", a);
}
