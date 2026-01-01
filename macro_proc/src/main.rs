use macro_proc::SayHi;
use macro_proc_derive::SayHi;

fn main() {
    Hello::say_hi();
}

#[derive(SayHi)]
struct Hello {}
