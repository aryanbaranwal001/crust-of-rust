#![allow(dead_code)]

// when anything gets dropped, compiler needs to know whether to consider the drop
// a use of anything that's inside it
//
// when a type is dropped, its considered a test of use of all the fields of that type
// So basically this PhantomData type is used to tell compiler that the raw ptr it
// points to must not outlive the T, this allows compiler to check the drop of Rc,
// with the drop of the inside value, even though, its not reallly required in our implementation
// here, as We aren't giving user any api so he could drop the inside value before
// the Rc. IDK why that last line wasn't said by JonHoo, he says that our implementation is
// incomplete

// this provides with shared ownership

use crate::cell::Cell;
use std::marker::PhantomData;
use std::ops::Deref;
use std::ptr::NonNull;

struct RcInner<T> {
    value: T,
    refcount: Cell<usize>,
}

pub struct Rc<T> {
    inner: NonNull<RcInner<T>>,
    _marker: PhantomData<T>,
}

impl<T> Rc<T> {
    pub fn new(v: T) -> Self {
        let inner = Box::new(RcInner {
            value: v,
            refcount: Cell::new(1),
        });

        // we didn't do dereferencing the box, as when it goes out of scope
        // data inside will get drop which doesn't happen when we call into_raw on it

        Rc {
            // SAFETY: Box doesn't give us a null ptr
            inner: unsafe { NonNull::new_unchecked(Box::into_raw(inner)) },
            _marker: PhantomData,
        }
    }
}

impl<T> Clone for Rc<T> {
    fn clone(&self) -> Self {
        let inner = unsafe { self.inner.as_ref() };

        inner.refcount.set(inner.refcount.get() + 1);
        Rc {
            inner: self.inner,
            _marker: PhantomData,
        }
    }
}

impl<T> Deref for Rc<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        // SAFETY: self.inner is a Box that is only deallocated when the last Rc goes away
        // we have an Rc, therefore the Box has not been deallocated, so deref is fine.
        unsafe { &self.inner.as_ref().value }
    }
}

impl<T> Drop for Rc<T> {
    fn drop(&mut self) {
        let inner = unsafe { self.inner.as_ref() };
        let c = inner.refcount.get();

        if c == 1 {
            // SAFETY: only one Rc is left hence only one is being dropped
            // there, after this there will be no rcs and hence no references to T
            let _ = unsafe { Box::from_raw(self.inner.as_ptr()) };
        } else {
            inner.refcount.set(c - 1);
        }
    }
}
