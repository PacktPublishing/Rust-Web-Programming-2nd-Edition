use tokio::net::TcpListener;
use tokio_util::codec::{BytesCodec, Decoder};

use futures::StreamExt;
use futures::sink::SinkExt;
use bytes::Bytes;

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

    let listener = TcpListener::bind(&addr).await.unwrap();
    println!("Listening on: {}", addr);

    loop {
        let (socket, _) = listener.accept().await.unwrap();

        tokio::spawn(async move {
            let mut framed = BytesCodec::new().framed(socket);
            let message = framed.next().await.unwrap();
                match message {
                    Ok(bytes) => {
                        let message = bincode::deserialize::<Message>(&bytes).unwrap();
                        println!("{:?}", message);
                        let message_bin = bincode::serialize(&message).unwrap();
                        let sending_message = Bytes::from(message_bin);
                        framed.send(sending_message).await.unwrap();
                    },
                    Err(err) => println!("Socket closed with error: {:?}", err),
                }
            println!("Socket received FIN packet and closed connection");
        });
    }
}
