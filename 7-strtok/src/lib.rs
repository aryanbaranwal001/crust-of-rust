#![allow(dead_code)]

mod drop_check;
// variance can make a lifetime of longer var to shorter var, if it makes sense
// &'a mut &'a str
// and since &str from "" is 'static, its like
// &'static mut &'static str
// but this obv doesn't work as &mut x, is a stack var
// but code still compiles as compiler can simply reduce the lifetime of 'static to
// 'it_works

// I though that "this is always static"
// i guess its not

pub fn strtok<'a, 'b>(s: &'a mut &'b str, delimiter: char) -> &'b str {
    if let Some(i) = s.find(delimiter) {
        let prefix = &s[..i];
        let suffix = &s[(i + 1)..];
        *s = suffix;
        prefix
    } else {
        let prefix = *s;
        *s = "";
        prefix
    }
}

fn check_is_static(_: &'static str) {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut x: &'static str = "hello world";
        // check_is_static(x);
        let hello = strtok(&mut x, ' ');

        // <'a> &'a mut &'a str
        //
        // <'a> &'static mut &'static str
        // &'static mut &'static str
        // tries to make &mut static, as &mut are invariant in T,
        //
        assert_eq!(hello, "hello");
        assert_eq!(x, "world");
    }

    #[test]
    fn covariance() {
        let y: &'static str = "&str";

        let s = String::from("value");
        let mut x = s.as_str();
        let z = &mut x;

        *z = y
        // I am genius
    }
}

// T is subtype of U
// as T is atleast as long as useful as U
// 'static is subtype of 'a

// // Covariance
//
// fn foo()
//
// Covariance allows you to pass a longer-lived reference where a
// shorter-lived one is expected.
//
// // Contravariant
//
// Variance requirement is flipped, check it out at rustnomicon, 10.5 3.8ch table
//
// Its expected that a callback fn takes a fn that takes a short lifetime string
// and i have a fn that takes a long lifetime string, this is not okay
//
// fn execute_callback(f: fn(&'a str)) {
//     let s = "some_local_string"; // Assume this has lifetime 'a
//     f(s);
// }
//
// // Invariance

// fn main() {
//     let mut x: &'static str = "hello";
//     let y = String::new();
//     foo(&mut x, &y);
//     println!("{}", y);
// }

// fn foo<'a>(s: &mut &'a str, z: &'a str) {
//     *s = z;
// }

//  The following is &'a mut covariant in 'a
//
// pub fn bar() {
//     let mut y = true;
//     let mut z /* &'y mut bool */ = &mut y;
//
//     let x = Box::new(true);
//     let x: &'static mut bool = Box::leak(x);
//
//     // ignore htis line
//     let _ = z;
//
//     z = x; // &
//
//     // ignore this line
//     drop(z);
// }
