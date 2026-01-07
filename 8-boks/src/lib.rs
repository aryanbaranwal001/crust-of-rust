#![allow(dead_code)]
#![allow(unused_variables)]

// when anything gets dropped, compiler needs to know whether to consider the drop
// a use of anything that's inside it

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

// impl<T> Drop for Boks<T> {
//     fn drop(&mut self) {
//         let _ = unsafe { Box::from_raw(self.p) };
//         // std::ptr::drop_in_place(self.p);
//         // this drops the T, but doesn't free the box
//     }
// }

impl<T> std::ops::Deref for Boks<T> {
    type Target = T;

    // SAFETY: this is valid since it was constructed from a valid T, and
    // turned into a pointer though Box which creates aligned poitners and hasn't been
    // freed, since self is alive.

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.p }
    }
}

impl<T> std::ops::DerefMut for Boks<T> {
    // SAFETY: this is valid since it was constructed from a valid T, and
    // turned into a pointer though Box which creates aligned poitners and hasn't been
    // freed, since self is alive.

    // Also, since we have  &mut self, no other mutalbe references has been given out to p.
    // this is required because you can have mutliple mut refs to a type from a raw ptr
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.p }
    }
}

fn main() {
    let x = 42; // x: i32
    let b = Boks::ny(x); // b: Boks<i32>

    println!("{:?}", *b);

    let mut y = 42;
    let mut b = Boks::ny(&mut y);

    println!("{:?}", y);

    **b = 45;
}
