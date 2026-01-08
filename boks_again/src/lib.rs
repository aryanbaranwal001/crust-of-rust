#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(unused_imports)]

use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut, Drop};

#[derive(Debug)]
pub struct Boks<T> {
    p: *mut T,
}

impl<T> Boks<T> {
    pub fn ny(t: T) -> Self {
        Boks {
            p: Box::into_raw(Box::new(t)),
        }
    }
}

impl<T> Drop for Boks<T> {
    fn drop(&mut self) {
        unsafe {
            Box::from_raw(self.p);
        }
    }
}

impl<T> Deref for Boks<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.p }
    }
}

impl<T> DerefMut for Boks<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.p }
    }
}

fn main() {
    let mut x = 42;
    let mut b = Boks::ny(&mut x);

    // **b = 45;
    println!("{}", x);

    // println!("{:?}", b);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        main();
    }
}

// inside a data structure such as b, it you use it in any of its form, b, *b, **b
// it will be considered as if trying to use the inner refernce to mut T
// but if its not used anywhere, lifetime of b OR &mut T (idk) gets reduced to its last use

// but when have a drop implementation of Boks, which is the type of b, then
// compiler assumes that you will use the value inside it in the drop
// hence lifetime of b OR &mut T (idk) expands to last of scope. I think its better to say &mut T
// in here -----------------^

// also when out Boks goes out of scope, it doesn't bother to call the destructor of inner type
// and it just leaks the memory
