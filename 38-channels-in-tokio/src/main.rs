use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(32); // buffer size 32

    tokio::spawn(async move {
        for i in 0..5 {
            tx.send(i).await.unwrap();
            println!("Sent {}", i);
            sleep(Duration::from_millis(500)).await;
        }
    });

    while let Some(msg) = rx.recv().await {
        println!("Received {}", msg);
    }
}