use tokio::net::TcpListener;
use tokio::io::{BufReader, AsyncBufReadExt, AsyncWriteExt};

use serde::{Serialize, Deserialize};
use bincode;


#[derive(Serialize, Deserialize, Debug)]
struct Message {
    pub ticker: String,
    pub amount: f32
}


#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:8080".to_string();

    let socket = TcpListener::bind(&addr).await.unwrap();
    println!("Listening on: {}", addr);

    while let Ok((mut stream, peer)) = socket.accept().await {
        println!("Incoming connection from: {}", peer.to_string());

        tokio::spawn(async move {
            println!("thread starting {} starting", peer.to_string());
            let (reader, mut writer) = stream.split();

            let mut buf_reader = BufReader::new(reader);
            let mut buf = vec![];

            loop {
                match buf_reader.read_until(b'\n', &mut buf).await {
                    Ok(n) => {
                        if n == 0 {
                            println!("EOF received");
                            break;
                        }
                        let message = bincode::deserialize::<Message>(&buf).unwrap();
                        println!("{:?}", message);
                        let message_bin = bincode::serialize(&message).unwrap();
                        writer.write_all(&message_bin).await.unwrap();
                        writer.write_all(b"\n").await.unwrap();
                        buf.clear();
                    },
                    Err(e) => println!("Error receiving message: {}", e)
                }
            }
            println!("thread {} finishing", peer.to_string());
        });
    }
}
