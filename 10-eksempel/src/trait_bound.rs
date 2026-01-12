#![allow(unused)]

trait Hei {
    fn hei(&self);
}

impl Hei for String {
    fn hei(&self) {
        println!("Hei from String: {}", self);
    }
}

// 1. A generic function.
// We add `+ ?Sized` to allow `T` to be `dyn Hei`.
// Without `?Sized`, T defaults to Sized, and dyn Hei would be rejected.
fn generic_caller<T: Hei + ?Sized>(item: &T) {
    println!("Generic caller invoked...");
    item.hei();
}

fn main() {
    let concrete = String::from("Alice");

    // 2. Create a Boxed Trait Object
    let boxed_trait_object: Box<dyn Hei> = Box::new(concrete);

    // 3. Pass it to the generic function
    // We dereference the Box (*boxed_trait_object) to get `dyn Hei`.
    // So here, T = dyn Hei.
    generic_caller(&*boxed_trait_object);
}
