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

#[non_exhaustive]
pub enum Ordering {
    Relaxed,
    Release,
    Acquire,
    AcqRel,
    SeqCst,
}
