#![allow(unused)]
// note to self: i might have interchanged trait for type in notes, so be mindful of it

// iterator type is object safe because in methods which it takes generics or
// give away self, Self: Sized bound is implemented

// so for a trait to be object safe
// - not have generic methods
// - all methods need to have a receiver with self
// - trait can't have type that return self

/// ```
/// trait MyClone {
///    fn get(&self) -> Self;
/// }
///
/// struct MyStruct(String);
///
/// impl MyClone for MyStruct {
///    fn get(&self) -> Self {
///        MyStruct(self.0.clone())
///    }
/// }
///
/// fn main() {
///    pub fn clone(v: &dyn MyClone) {
///        let x = v.get();
///    }
/// }
/// ```
/// doesn't work because x is not sized basically, or rather the return type
/// self, is not sized
struct Cln;

pub trait Hei {
    fn hei(&self);

    fn weird(&self)
    where
        Self: Sized,
    {
    }

    // we are able to opt out of trait object from here because, dyn Hei, is a unsized
    // type and only through that we can call on methods, but fn weird has restriction of
    // self: sized, which doesn't allow to call from dyn Hei which is unsized
}
// you can also make a whole type to not allow for trait objects using following bound

/// pub trait Hei
/// where
/// Self: Sized, {}
/// this is done for backwards compatibility reasons
struct N;

impl Hei for &str {
    fn hei(&self) {
        println!("hei {}", self);
    }

    fn weird(&self) {}
}

impl Hei for String {
    fn hei(&self) {
        println!("hei {}", self);
    }

    fn weird(&self) {}
}

pub fn say_hei(s: &dyn Hei) {
    // type erasure happens
    s.hei();
    // s.weird();
    // can't do this as weird doesn't take any reference or pointer

    // (dyn Hei)::weird();
    // can't do this too as compiler doesn't know which weird to call
    // &str or &String implementation or default one.
    //
    // so what we basically want is that, we want to tell compiler we would only
    // be calling fn hei from the type Hei, when using as trait object. and not the
    // other functions, for which exactly we have a way
    //
    // even if we put &self in weird

    // s.weird();
}

pub fn works() {
    say_hei(&"s");
    say_hei(&String::from("value"));
}
