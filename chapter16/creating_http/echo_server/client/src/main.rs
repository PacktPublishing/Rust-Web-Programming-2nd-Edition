use tokio::net::TcpStream;
use tokio_util::codec::{BytesCodec, Decoder};

use futures::sink::SinkExt;
use futures::StreamExt;
use bytes::Bytes;

use bincode;

use std::error::Error;

mod http_frame;
use http_frame::{HttpFrame, Header, Body};


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let stream = TcpStream::connect("127.0.0.1:8080").await?;
    let mut framed = BytesCodec::new().framed(stream);

    let message = HttpFrame{
        header: Header{
            method: "POST".to_string(),
            uri: "www.freshcutswags.com/stock/purchase".to_string()
        },
        body: Body{
            ticker: "BYND".to_string(),
            amount: 3.2,
        }
    };
    let message_bin = bincode::serialize(&message).unwrap();
    let sending_message = Bytes::from(message_bin);

    framed.send(sending_message).await.unwrap();
    let message = framed.next().await.unwrap().unwrap();
    let message = bincode::deserialize::<HttpFrame>(&message).unwrap();
    println!("{:?}", message);
    Ok(())
}
