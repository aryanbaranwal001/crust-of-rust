fn main() {
    println!("Hello, world!");
    let mut x = bar::<i32>;
    println!("{}", std::mem::size_of_val(&x)); // x is a ZST because here a unique instance of fn
    // is created at compile time
    baz(bar::<u32>); // here function pointer is passed
    baz(bar::<i32>);
    bat(bar::<i32>);
    // defer coersion happens here
    // fn items are coersible into fn pointers
}

fn bar<T>(_: u32) -> u32 {
    0
}

fn baz(f: fn(u32) -> u32) {
    println!("{}", std::mem::size_of_val(&f));
}

// Now F is a generic type that implements the Fn trait
fn bat<F>(f: F)
where
    F: Fn(u32) -> u32,
{
    println!("{}", std::mem::size_of_val(&f));
}

// what is determined at runtime or compile time is determined by the type signature of the fn you
// are calling
//
// baz signature says f is a fn pointer
// bat signature says f is a fn
//
// so basically we can use fn ptrs to make a compile code a less bulky
