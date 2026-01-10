#![allow(dead_code)]

/// - Memory model of rust is icnomplete but it mostly uses cpp memory model
/// - Atomic Types are not inherently shared
/// - Atomic means One Instruction, no thread can come in between
/// suppose for read and write a thread might come after read, and read that value
/// which results in losing an update.
///
/// - std::thread::spawn, the closure which captures everythign must have
/// a 'static lifetime, only in context where it might outlive the
/// data, like in threads
///
/// compareExchange is only and only allowed to fail, if the value it was expecting wasn't there
/// compareExchangeWeak can also fail if the correct value was passed in. Not usually but yeah
///
struct Notes;

/// the reason why our code still gave correct output with ordering::relaxed, is becauase x86_64 gives
/// the architecture basically guarantees acquire-release semantics for all operations
///
/// SeqCst interact with SeqCst, Acq can interact with this though
/// ThreadSanitizerAlgorithm
/// Loom of tokio-rs
///
/// if you want to learn more: look at some papers that implement concurrent data structures
///
/// read_volatile
///
/// best way to avoid the subtlies and problem atomics introduce is to never use the in the first place
/// because you absolutely want to make sure that your program is right, use loom, use threadsanitizer, just anything
///
struct N;

#[non_exhaustive]
pub enum Ordering {
    Relaxed,
    Release,
    Acquire,
    AcqRel,
    SeqCst,
}
