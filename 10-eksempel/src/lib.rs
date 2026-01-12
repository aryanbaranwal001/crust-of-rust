#![allow(unused)]

mod generics;
mod notes;
mod notes2;
mod notes3;
mod trait_bound;

pub fn say_hei(s: Box<dyn AsRef<str>>) {
    // what happens when s goes out of scope?

    // how exactly drop is called on this trait object type?
    // Ans: every v table stores drop (and some other data) with othre methods,
    // because ofc its necessary
}

// And parameters to a fn and return, must always be sized, because
// compiler can't generate code for the arbitrary

// dyn Trait -> * -> (*mut data, *mut vtable)
// [u8]      -> * -> (*mut data, usize length)
// str       -> * -> (*mut data, usize length)

fn ba_2() -> Box<[u8]> {
    Box::new([]) as Box<[u8]>
}

// seach context in std
// reach RawWakerVTable

struct Foo {
    f1: bool,
    f2: bool,
    data: [u8],
}

// last field can be a dst as we statically know the pointer to the dst data,
// if it was in middle, then pointer values have to be changed at runtime,
// which doesn't happen.

//-------------------------------------------
// NOTE: THE FOLLOWING IS OUTDATED, AS LAST EXAMPLE COMPILES

fn foo(f: &dyn Fn()) {} // this is a trait object pointer (wide pointer)
// has data to calling (because of which the closure can capture the environment)
// the fn, and also to the vtable (the actual closure/fn addr)

// 1. data
// 2. closure addr

fn bar<T: Fn()>(f: T) {} // same as below
fn baz(f: impl Fn()) {}

fn bar2<H>(f: H)
where
    H: Fn(), // this has to be a fn, can't be a closure as
             // Fn() is really a fn pointer
{
    // unsafe {
    //     let x: &[u64] = &std::mem::transmute(f);
    //     println!("this is bar fn: {:?}", x);
    // }
}

// but it seems, they have updated it of some sort
fn main() {
    let x = "hello";
    foo(&|| {
        let _ = &x;
    });
    bar2(|| {
        let _ = &x;
    });

    baz(|| {
        // this impl fn is sort of generic, so for every copy of closure
        // you pass into, a new baz is created (monomorphiszed), which
        // might end up making binary big
        let _ = &x;
    });
}

trait X {
    fn foo(&self, f: &dyn Fn()) {}
}

fn quox(x: &dyn X) {}
