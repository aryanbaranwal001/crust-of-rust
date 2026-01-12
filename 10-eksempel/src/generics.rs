#![allow(unused)]

use std::iter::Extend;

/// ```
/// pub fn add_true(v: &mut dyn Extend<bool>) {
///     v.extend(std::iter::once(true));
/// }
/// struct MyVec<T>(Vec<T>);
/// impl<T> Extend<T> for MyVec<T> {
///     fn extend<I>(&mut self, iter: I)
///     where
///         I: IntoIterator<Item = T>,
///     {
///         // ...
///     }
/// }
/// ```
/// here, this doesn't work because because of monomorphisation, for each type of  
/// iter, there is a extend, but mono- doesn't happen because of trait object, and
/// we don't have any way to tell the compiler to look into this impl of extend for
/// std::iter::once type while calling extend in vtable, hence it doesn't work
struct Notes;
