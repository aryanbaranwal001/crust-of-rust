use tokio::sync::mpsc;
use tokio::time::{Duration, sleep};

async fn select() {
    let (tx, mut rx) = mpsc::channel::<String>(32);

    // Simulate some background worker sending messages
    tokio::spawn(async move {
        for i in 1..=5 {
            tx.send(format!("Message #{}", i)).await.unwrap();
            sleep(Duration::from_secs(1)).await;
        }
    });

    println!("Waiting for messages or timeout...");

    loop {
        tokio::select! {
            // Case 1: Got a message from channel
            Some(msg) = rx.recv() => {
                println!("Received: {}", msg);
            }

            // Case 2: 10 second timeout (overall operation timeout)
            _ = sleep(Duration::from_secs(10)) => {
                println!("Timeout reached - exiting");
                break;
            }

            // Case 3: Press Ctrl+C (or external shutdown signal)
            _ = tokio::signal::ctrl_c() => {
                println!("\nReceived Ctrl+C - shutting down gracefully");
                break;
            }
        }
    }

    println!("Done.");
}
