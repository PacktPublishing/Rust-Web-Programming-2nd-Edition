use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};


#[derive(Debug, Clone)]
pub enum Order {
    BUY,
    SELL
}


#[derive(Debug, Clone)]
pub struct Message {
    pub order: Order,
    pub ticker: String,
    pub amount: f32
}



#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel::<Message>(1);

    let orders = [
        Message { order: Order::BUY, amount: 5.5, ticker: "BYND".to_owned()},
        Message { order: Order::BUY, amount: 5.5, ticker: "NET".to_owned()},
        Message { order: Order::BUY, amount: 5.5, ticker: "PLTR".to_owned()},
    ];

    tokio::spawn(async move {
        for order in orders {
            if let Err(e) = tx.send(order.clone()).await {
                println!("send error: {:?}", e);
                return;
            }
            println!("sent: {:?}", order);
        }
    });

    while let Some(i) = rx.recv().await {
        println!("got: {:?}", i);
    }
}
