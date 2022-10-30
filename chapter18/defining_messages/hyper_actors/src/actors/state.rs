use std::collections::{HashMap, VecDeque};
use std::mem;

use tokio::sync::mpsc;

use super::messages::{MessageType, StateActorMessage};

type Receiver = mpsc::Receiver<StateActorMessage>;
type Sender = mpsc::Sender<StateActorMessage>;


/// This actor is responsible for managing the state of all the chat logs for all chat IDs.
/// 
/// # Fields
/// * chat_queue: the order in which the chats were inserted into the state
/// * chats_logs: the chat logs in relation to the patient
/// * receiver: the receiver that accepts INPUT and OUTPUT messages through a channel
/// * sender: the sender that sends messages through a channel
#[derive(Debug)]
pub struct StateActor {
    pub chat_queue: VecDeque<i32>,
    pub chat_logs: HashMap<i32, Vec<String>>,
    pub receiver: Receiver,
    pub sender: Sender,
}

impl StateActor {
    
    /// The constructor for the StateActor struct.
    /// 
    /// # Arguments
    /// * receiver: the receiver that accepts INPUT and OUTPUT messages through a channel
    /// * sender: the sender that sends messages through a channel
    /// 
    /// # Returns
    /// The constructed state actor
    pub fn new(receiver: Receiver, sender: Sender) -> StateActor {
        let chat_queue: VecDeque<i32> = VecDeque::new();
        let chat_logs: HashMap<i32, Vec<String>> = HashMap::new();
        return StateActor {chat_queue, chat_logs, receiver, sender}
    }

    /// Gets message data from the ```self.chat_logs```.
    /// 
    /// # Arguments
    /// * chat_id: the ID of the patient's chat logs to be extracted
    /// 
    /// # Returns
    /// * chat log data that belongs to the patient
    pub fn get_message_data(&mut self, chat_id: i32) -> Vec<String> {
        let reference = self.chat_logs.get_mut(&chat_id).unwrap();
        let data = mem::take(reference);
        self.chat_logs.remove(&chat_id);
        return data
    }

    /// Inserts a message into the ```self.chat_logs```.
    /// 
    /// # Arguments
    /// * chat_id: the ID of the patient's chat logs to be inserted
    /// * message_data: the message data that is going to be inserted into the chat log
    pub fn insert_message(&mut self, chat_id: i32, message_data: String) {

        match self.chat_logs.get_mut(&chat_id) {
            Some(patient_log) => {
                patient_log.push(message_data);
            }, 
            None => {
                self.chat_queue.push_back(chat_id);
                let mut patient_log = Vec::new();
                patient_log.push(message_data);
                self.chat_logs.insert(chat_id, patient_log);
            }
        }

    }

    /// Processes an incoming message to either send chat log data out or insert new chat data. 
    /// 
    /// # Arguments
    /// message: the message to be processed
    async fn handle_message(&mut self, message: StateActorMessage) {
        println!("state actor is receiving a message");

        match message.message_type {
            MessageType::INPUT => {
                self.insert_message(message.chat_id.unwrap(), message.single_data.unwrap());
            },
            MessageType::OUTPUT => {
                match self.chat_queue.pop_front() {
                    Some(chat_id) => {
                        let data = self.get_message_data(chat_id);
                        let message = StateActorMessage {
                            message_type: MessageType::OUTPUT,
                            chat_id: Some(chat_id),
                            single_data: None,
                            block_data: Some(data)
                        };
                        let _ = self.sender.send(message).await.unwrap();
                    },
                    None => {
                        let message = StateActorMessage {
                            message_type: MessageType::EMPTY,
                            chat_id: None,
                            single_data: None,
                            block_data: None
                        };
                        let _ = self.sender.send(message).await.unwrap();
                    }
                }
            },
            MessageType::EMPTY => {
                panic!("empty messages should not be sent to the state actor");
            }
        }
        println!("{:?}", self.chat_logs);
        println!("{:?}", self.chat_queue);
    }

    /// Runs the actor throughout the lifetime of the program accepting messages. 
    pub async fn run(mut self) {
        println!("state actor is running");
        while let Some(msg) = self.receiver.recv().await {
            self.handle_message(msg).await;
        }
    }
}
