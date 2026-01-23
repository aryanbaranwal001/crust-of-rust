#![allow(unused)]

/// when we do
/// fn bar() {}
/// let x = bar;
/// here x has a type, its not a function pointer,
/// its more a function item
///
pub struct Notes;

// you can pass a fn pointer to any of the fn fnmut fnonce
// because, folllowing

/// impl<F> FnOnce() for F
/// where
///     F: FnMut,
/// {
///     fn call(mut self) {
///         FnMut::call(&mut self)
///     }
/// }
/// // you can trivially make &mut from owned variable
///
/// impl<F> FnMut() for F
/// where
///     F: Fn(),
/// {
///     fn call(&mut self) {
/// Fn::call(&*self)
///     }
/// }
pub struct Notes2;
