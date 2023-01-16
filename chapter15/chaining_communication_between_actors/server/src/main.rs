use tokio::net::TcpListener;
use std::{thread, time};
use tokio::io::{BufReader, AsyncBufReadExt, AsyncWriteExt};
use tokio::sync::mpsc;

mod actors;
use actors::{OrderBookActor, BuyOrder, Message};


#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:8080".to_string();

    let mut socket = TcpListener::bind(&addr).await.unwrap();
    println!("Listening on: {}", addr);

    let (tx, rx) = mpsc::channel::<Message>(1);

    tokio::spawn(async move {
        let order_book_actor = OrderBookActor::new(rx, 20.0);
        order_book_actor.run().await;
    });
    println!("order book running now");

    while let Ok((mut stream, peer)) = socket.accept().await {
        println!("Incoming connection from: {}", peer.to_string());
        let tx_one = tx.clone();

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
                        let buf_string = String::from_utf8_lossy(&buf);
                        let data: Vec<String> = buf_string.split(";").map(|x| x.to_string()
                            .replace("\n", ""))
                            .collect();
                        let amount = data[0].parse::<f32>().unwrap();
                        let order_actor = BuyOrder::new(amount, data[1].clone(), tx_one.clone());
                        println!("{}: {}", order_actor.ticker, order_actor.amount);
                        order_actor.send().await;
                        buf.clear();
                    },
                    Err(e) => println!("Error receiving message: {}", e)
                }
            }
            println!("thread {} finishing", peer.to_string());
        });
    }
}
