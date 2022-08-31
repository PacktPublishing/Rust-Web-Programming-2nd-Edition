use tokio::net::TcpListener;
use std::{thread, time};


#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:8080".to_string();

    let mut socket = TcpListener::bind(&addr).await.unwrap();
    println!("Listening on: {}", addr);

    while let Ok((mut stream, peer)) = socket.accept().await {
        println!("Incoming connection from: {}", peer.to_string());
        tokio::spawn(async move {
            println!("thread starting {} starting", peer.to_string());
            let five_seconds = time::Duration::from_secs(5);
            thread::sleep(five_seconds);
            println!("thread {} finishing", peer.to_string());
        });
    }
}
