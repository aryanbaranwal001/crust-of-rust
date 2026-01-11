#![allow(unused)]

/// Monomorphization happens for types which appear in code
///
/// for eg, hashmap, for a instance of it (K, V) it generates a copy of that and  methods
/// which is used and since it knows the concrete type, compiler can make it more optimized
///
/// pub fn bool_then<T>(b: bool, f: impl FnOnce() -> T) -> Option<T> {
///    if b {
///        Some(f())
/// } else {
///        None
/// }
///}
/// Rust can (not compulsorily) optimize this as putting the f in the some(f)
/// only (inlining) which makes it
/// slightly more efficient
///
/// All elements in the array must exactly the same size
///
/// All type parameters have an implicit bound of Sized.

struct Notes;
