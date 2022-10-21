use tokio::net::TcpListener;
use tokio::io::{BufReader, AsyncBufReadExt, AsyncWriteExt};
use tokio::sync::mpsc;

mod actors;
mod order_tracker;
use actors::{OrderBookActor, BuyOrder, Message};
use order_tracker::{TrackerActor, GetTrackerActor, TrackerMessage};


#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:8080".to_string();

    let socket = TcpListener::bind(&addr).await.unwrap();
    println!("Listening on: {}", addr);

    let (tx, rx) = mpsc::channel::<Message>(1);
    let (tracker_tx, tracker_rx) = mpsc::channel::<TrackerMessage>(1);
    let tracker_tx_one = tracker_tx.clone();

    tokio::spawn(async move {
        let tracker_actor = TrackerActor::new(tracker_rx);
        tracker_actor.run().await;
    });
    tokio::spawn(async move {
        let order_book_actor = OrderBookActor::new(rx, tracker_tx_one.clone(), 20.0);
        order_book_actor.run().await;
    });
    println!("order book running now");

    while let Ok((mut stream, peer)) = socket.accept().await {
        println!("Incoming connection from: {}", peer.to_string());
        let tx_one = tx.clone();
        let tracker_tx_two = tracker_tx.clone();

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
                        println!("here is the data {:?}", data);
                        let command = data[0].clone();
                        match command.as_str() {
                            "BUY" => {
                                println!("buy order command processed");
                                let amount = data[1].parse::<f32>().unwrap();
                                let order_actor = BuyOrder::new(amount, data[2].clone(), tx_one.clone());
                                println!("{}: {}", order_actor.ticker, order_actor.amount);
                                order_actor.send().await;
                            },
                            "GET" => {
                                println!("get order command processed");
                                let get_actor = GetTrackerActor{sender: tracker_tx_two.clone()};
                                let state = get_actor.send().await;
                                println!("sending back: {:?}", state);
                                writer.write_all(state.as_bytes()).await.unwrap();
                            },
                            _ => {
                                panic!("{} command not supported", command);
                            }
                        }
                        buf.clear();
                    },
                    Err(e) => println!("Error receiving message: {}", e)
                }
            }
            println!("thread {} finishing", peer.to_string());
        });
    }
}
