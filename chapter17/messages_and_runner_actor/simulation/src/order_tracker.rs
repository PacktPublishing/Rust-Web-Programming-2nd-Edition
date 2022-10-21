use tokio::sync::{mpsc, oneshot};
use std::collections::HashMap;


pub struct TrackerMessage {
    pub command: String,
    pub ticker: Option<String>,
    pub amount: Option<f32>,
    pub respond_to: oneshot::Sender<String>
}


pub struct GetTrackerActor {
    pub sender: mpsc::Sender<TrackerMessage>
}

impl GetTrackerActor {

    pub async fn send(self) -> String {
        println!("GET function firing");
        let (send, recv) = oneshot::channel();
        let message = TrackerMessage { 
            command: "GET".to_string(), ticker: None, amount: None, respond_to: send 
        };
        let _ = self.sender.send(message).await;
        match recv.await {
            Err(e) => panic!("{}", e),
            Ok(outcome) =>  return outcome
        }
    }
}


pub struct TrackerActor {
    pub receiver: mpsc::Receiver<TrackerMessage>,
    pub db: HashMap<String, f32>
}

impl TrackerActor {
    pub fn new(receiver: mpsc::Receiver<TrackerMessage>) -> Self {
        let db: HashMap<String, f32> = HashMap::new();
        return TrackerActor { receiver, db}
    }

    fn send_state(&self, respond_to: oneshot::Sender<String>) {
        let mut buffer = Vec::new();

        for key in self.db.keys() {
            let amount = self.db.get(key).unwrap();
            buffer.push(key.clone());
            buffer.push(":".to_string());
            buffer.push(format!("{};", amount));
        }
        buffer.push("\n".to_string());
        println!("sending state: {}", buffer.join(""));
        let _ = respond_to.send(buffer.join(""));
    }

    fn handle_message(&mut self, message: TrackerMessage) {
        match message.command.as_str() {
            "GET" => {
                println!("getting state");
                self.send_state(message.respond_to);
            },
            "BUY" => {
                let ticker = message.ticker.unwrap();
                let amount = message.amount.unwrap();
                match self.db.get(&ticker) {
                    Some(ticker_amount) => {
                        self.db.insert(ticker, ticker_amount + amount);
                    },
                    None => {
                        self.db.insert(ticker, amount);
                    }
                }
                println!("db: {:?}", self.db);
            },
            _ => {
                panic!("{} command is not supported", message.command);
            }
        }
    }

    pub async fn run(mut self) {
        println!("tracker actor is running");
        while let Some(msg) = self.receiver.recv().await {
            self.handle_message(msg);
        }
    }
}
