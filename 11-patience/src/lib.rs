#![allow(unused)]
mod notes;
mod select;

// What you write:
#[tokio::main]
async fn main_p() { // this is fn main only
    // your async code...
}

// What it roughly expands to (simplified):

fn main_r() {
    // this is fn main only
    // Creates a multi-threaded Tokio runtime (default)
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all() // io, time, process signals, etc.
        .build()
        .expect("Failed to create Tokio runtime");

    // Blocks current thread until the future completes
    rt.block_on(async {
        // your original async main body goes here
        // plus some glue for panic handling / shutdown
    })
}
