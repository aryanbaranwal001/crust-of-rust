// a crate called "compile fail" or "try build" which is used to test code which doesn't compile
// but doc test can also do it

// cargo install cargo-expand

// patterns are patterns of rust syntax trees
// not arg list

// input only has to be syntaxtically (rust grammer) valid rust

// macro_rules! avec {
//     ($arg1:ty => $arg2: expr; $arg3: path) => {};
// }

// avec! {
//     u32 => x.foo(); std::path
// }

// this is syntactically valid, input has to be syntactically valid, output has
// to be valid rust, bcs that gets compiled

// whenever you try to push to a vec, if it extends its capacity, it clones the prev to
// and allocate a new vec with extended storage. this is done incrementatly and hence is costly
// which is why with capacity is cheap

/// another version
/// ```
/// () => {
///     Vec::new()
/// };
///
/// ( $($x:expr),* $(,)? ) => {{
///   let mut vs = Vec::new();
///   $(
///     vs.push($x);
///   )*
///   vs
/// }};
///
/// // this just do substitution
/// ( $elem:expr; $c:expr ) => {{
///     let count = $c;
///     // let mut vs = Vec::with_capacity(count);
///     let mut vs = Vec::new(); // I wonder why he did this 1:11:28
///     // vs.extend(std::iter::repeat($elem).take(count));
///     vs.resize(count, $elem);
///     vs
/// }};
/// ```
///
#[macro_export]
macro_rules! my_mac {

    ( $($x:expr),* ) => {{


        #[allow(unused_mut)]
        let mut vs = Vec::with_capacity($crate::my_mac![@COUNT; $( $x ),*]);

        $(vs.push($x);)*
        vs
    }};

    ( $($x:expr,)* $(,)? ) => {{
        $crate::my_mac![$( $x ),*]
    }};

    // this just do substitution
    ( $elem:expr; $c:expr ) => {{
        let count = $c;
        // let mut vs = Vec::with_capacity(count);
        let mut vs = Vec::new(); // I wonder why he did this 1:11:28
        // vs.extend(std::iter::repeat($elem).take(count));
        vs.resize(count, $elem);
        vs
    }};

    (@COUNT; $( $elem:expr ),* ) => {
        <[()]>::len(&[$( $crate::my_mac!(@SUBST; $elem) ),*])
    };

    (@SUBST; $elem:expr ) => {
        ()
    };

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_empty() {
        let vs: Vec<i32> = my_mac![];
        assert!(vs.is_empty());
    }

    #[test]
    fn two() {
        let vs = my_mac![1, 2];
        assert_eq!(vs[1], 2);
    }

    #[test]
    fn mac() {
        let vs = my_mac![1, 2, 3, 4];
        assert_eq!(vs[0], 1);
        assert_eq!(vs[3], 4);
    }

    #[test]
    fn with_rbracks() {
        let vs = my_mac!(1, 2, 3, 4);
        assert_eq!(vs[0], 1);
        assert_eq!(vs[3], 4);
    }

    #[test]
    fn with_comma() {
        let vs = my_mac![1, 2, 3, 4,];
        assert_eq!(vs[3], 4);
    }

    #[test]
    fn compressed() {
        let vs = my_mac![1; 30];
        assert_eq!(vs[29], 1);
    }

    #[test]
    fn edge() {
        let x = Some(1);
        let vs = my_mac![x.unwrap(); 30];
        assert_eq!(vs[29], 1);
    }

    // #[test]
    // fn mac_my() {
    //     let _ = my_try!(1 2 3 4);
    // }
    // if you don't specify a separator, then whitespace could be thought of as default
}

/// ```compile_fail
/// let x: Vec<i32> = vecmac::my_mac![32; "foo"];
/// ```
#[allow(dead_code)]
pub struct Placeholder;

// trait MaxValue {
//     fn max_value() -> Self;
// }
//
// macro_rules! impl_max {
//     ($t:ty) => {
//         impl $crate::MaxValue for $t {
//             fn max_value() -> Self {
//                 <$t>::MAX
//             }
//         }
//     };
// }
//
// impl_max!(u32);
// impl_max!(i32);
//
// $crate always refers to where macro was defined

// as per rules, the caller decides which delimiter to chose () [], not us

// macros aren't allowed to access variables outside of their scope

// Currently there is no hard n fast rule that a macro should only use a
// particular delimiter
//
// avec!();
// avec![];
// avec! {}
//

// if you are repeating $(...)* for two vars, you'll have to make sure
// they repeat for the same no. of times
