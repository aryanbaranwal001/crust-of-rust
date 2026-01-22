#![allow(unused)]

use std::future::Future;
use tokio;
fn main() {
    let x = foo1();
    let y = foo2();

    // assert_eq!(x, y);
}

// Basically, async is used mainly like IO, networking, and timers. It allows thread
// to do something else while we are waiting on disk
//
// where there are computation and stuffs, async doesn't really add much.

// the output vars => usize & Future<Output = usize> gets changed in same way
// following two are equivalent
// 1.
async fn foo1() -> usize {
    0
}

// 2.
fn foo2() -> impl Future<Output = usize> {
    async { 0 }
}

// mio basically asks os to do a task, and
// wake me up after its done, and till then it sleeps

fn tokio_a() {
    let mut f1 = tokio::fs::File::open("foo");
    let mut f2 = tokio::fs::File::create("bar");
    /// ```
    ///    let copy = tokio::io::copy(&mut f1, &mut f2);
    /// ```
    struct PlaceHolder;
    // we use tokio lib fs and not std::fs as std::fs is not async
    // it doesn't know how to communicate with the executor

    // if select branch, you need to be concerned that, since you race two asyncs,
    // when one completes and exists, other branches might be in middle of smt like
    // copying and stuff, because those branches just end, so you need to handle them
}
