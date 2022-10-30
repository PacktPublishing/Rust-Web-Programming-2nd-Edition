use super::messages::{MessageType, StateActorMessage};
use tokio::sync::mpsc;
use std::{thread, time};


type Receiver = mpsc::Receiver<StateActorMessage>;
type Sender = mpsc::Sender<StateActorMessage>;

/// This struct is an actor that periodically pings the state actor throughout the program to send batches of data to another server.
/// 
/// # Fields 
/// * interval: the number of seconds between each loop iteration
/// * receiver: the channel receiver in which the actor receives messages from
/// * sender: the channel sender in which the actor sends messages to
pub struct RunnerActor {
    pub interval: i32,
    pub receiver: Receiver,
    pub sender: Sender,
}

impl RunnerActor {

    /// The constructor for the LibRunnerActor struct.
    /// 
    /// # Arguments
    /// * receiver: the channel receiver in which the actor receives messages from
    /// * sender: the channel sender in which the actor sends messages to
    /// * interval: the number of seconds between each loop iteration
    pub fn new(receiver: Receiver, sender: Sender, interval: i32) -> RunnerActor {
        return RunnerActor { interval, receiver, sender }
    }        

    /// Runs the actor throughout the lifetime of the program. 
    pub async fn run(mut self) {
        println!("runner actor is running");
        let seconds = time::Duration::from_secs(self.interval as u64);

        loop {
            thread::sleep(seconds);
            let message = StateActorMessage {
                message_type: MessageType::OUTPUT,
                chat_id: None,
                single_data: None,
                block_data: None
            };
            match self.sender.send(message).await {
                Ok(_) => {
                    let message = self.receiver.recv().await.unwrap();
                    
                    match message.message_type {
                        MessageType::OUTPUT => {
                            message.send_to_server().await;
                        },
                        _ => {
                            println!("state is empty");
                        }
                    }
                },
                Err(_) => {
                    println!("runner is failed to send message");
                }
            };
        }
    }
}
