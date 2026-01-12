mod cell;
mod rc;
mod refcell;
// cell is used for smaller values, like numbers or flags

// CoW smart pointer

// cell => interior mutability
// refcell => borrow checker at runtime with interior mutability
// Rc => reference poitners at runtime, deallocation when count = 0,
