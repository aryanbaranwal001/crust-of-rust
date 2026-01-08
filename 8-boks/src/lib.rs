#![allow(dead_code)]
#![allow(unused_variables)]

// 51:00

#[cfg(any())]
mod drop;
mod empty;
mod phantom;
#[cfg(any())]
mod variance;
// to print a type use the following
// std::any::type_name::<T>()

// compiler assuems that suppose we have a type and it takes a generic T, the compiler
// assumes that the type will use T, if the type implements drop

// when anything gets dropped, compiler needs to know whether to consider the drop
// a use of anything that's inside it

use std::marker::PhantomData;
use std::ptr::NonNull;

pub struct Boks<T> {
    p: NonNull<T>,
    _t: PhantomData<T>,
    // _t: PhantomData<fn() -> T>,
    // not this because this mean we are no longer subjected to drop check
}

impl<T> Boks<T> {
    pub fn ny(t: T) -> Self {
        Boks {
            // Box never creates a null ptr
            p: unsafe { NonNull::new_unchecked(Box::into_raw(Box::new(t))) },
            _t: PhantomData,
        }
    }
}

// unsafe impl<#[may_dangle] T> Drop for Boks<T> {
// this says to compiler that we are promissing that we won't access the T,
// hence we won't do anything with T
impl<T> Drop for Boks<T> {
    fn drop(&mut self) {
        let _ = unsafe { Box::from_raw(self.p.as_mut()) };
        // std::ptr::drop_in_place(self.p);
        // this drops the T, but doesn't free the box
    }
}

impl<T> std::ops::Deref for Boks<T> {
    type Target = T;

    // SAFETY: this is valid since it was constructed from a valid T, and
    // turned into a pointer though Box which creates aligned poitners and hasn't been
    // freed, since self is alive.

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.p.as_ref() }
    }
}

impl<T> std::ops::DerefMut for Boks<T> {
    // SAFETY: this is valid since it was constructed from a valid T, and
    // turned into a pointer though Box which creates aligned poitners and hasn't been
    // freed, since self is alive.

    // Also, since we have  &mut self, no other mutalbe references has been given out to p.
    // this is required because you can have mutliple mut refs to a type from a raw ptr
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.p.as_mut() }
    }
}

use std::fmt::Debug;
struct Oisann<T: Debug>(T);

impl<T: Debug> Drop for Oisann<T> {
    fn drop(&mut self) {
        println!("{:?}", self.0);
    }
}

// fn first() {
//     let x = 42; // x: i32
//     let b = Boks::ny(x); // b: Boks<i32>

//     println!("{:?}", *b);

//     let mut y = 42;
//     let mut b = Boks::ny(&mut y);

//     let mut z = 42;
//     let b = Boks::ny(Oisann(&mut z));
//     println!("{:?}", z);
// }

fn second() {
    let s = String::from("hei");
    let mut box1 = Box::new(s.as_str());
    let box2: Box<&'static str> = Box::new("heisann");
    box1 = box2;
    // the above code compiles
    // below one doesn't, because *mut T is invariant in T

    let s = String::from("hei");
    let mut boks1 = Boks::ny(s.as_str());
    let boks2: Boks<&'static str> = Boks::ny("heisann");
    boks1 = boks2;
}

/// used nonnull becaue we want T to be covariant which is not possible if we used *mut T
///
/// we used phantomdata T because we want compiler to check the dorp impl of T,
/// when Boks is dropped
/// - here we are dropping T when boks is dropped and want to let the compiler know that
/// so it can perform borrow checker rules
///
struct Docs;
