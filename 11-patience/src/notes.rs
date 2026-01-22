#![allow(unused)]

use tokio;
// Concurrency          →  One cook juggling many dishes
//                          (keeps switching, many things in progress)
//
// Parallelism          →  Many cooks each making one dish at the same time
//                          (true simultaneous work)
//
// Asynchronous         →  Cook puts pizza in oven → goes to make salad
//                          (doesn't stand & stare at oven)
//
// Multithreading       →  One cook with 8 arms (or 8 cooks in same kitchen sharing tools)
//
// Multiprocessing      →  8 completely separate kitchens (no shared tools/memory)
//
// Distributed          →  1000 kitchens in different cities

// let (file1, file2, file3) = join!(future1, future2,fuutre3);
//
// try_join_all => gives in order like above
// FuturesUnordered

// suppose you are making async on every connections, for 100,000 connections also
// you are going to do it in one thread
// so in order to communicate to the runtime that this async can be run in another thread
// you are do tokio::spawn, which has to be static.
// doing this makes our code concurrent and parallel

///
///
/// ```
///     loop {
///         tokio::select! {   // cannot find macro `select` in this scope
///             stream <- (&mut network).await => {
///                 // do something on stream
///             }
///             line <- (&mut terminal).await => {
///                 // do something with line
///                 break;
///             }
///             foo <- (&mut foo).await => {
///             }
///             _ <- copy.await => {
///             }
///         };
///     }
/// ```
/// if both stream and line are ready it polls the first, adn then loops
/// runs seelct again, so so that stream is safe to poll again, fusedfutures are used
///
///
/// so that select! kind of remembers if it .await on, is done through
/// using shared references or exclusive references to futures.
struct FusedFutures;

/// StateMachine {
///     Chunk1 {}
///     Chunk2 {}
/// }
///
/// you can't use async in trait objects as the fun returns a Future which is a
/// StateMachine basically holds all the inside futures and other related data.
/// So each fn gives out a different Future size and compiler doesn't know that
/// hence it gives an error,
///
/// how async trait solves this problem is basically wrapping the Future in Pin<Box>>
struct AwaitPoints;

use tokio::sync::Mutex as Tmutex;
