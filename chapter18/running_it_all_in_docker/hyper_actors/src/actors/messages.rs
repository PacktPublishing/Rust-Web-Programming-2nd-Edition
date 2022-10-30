use serde::Serialize;
use std::env;


/// The type of message being send across a channel.
/// 
/// # Fields
/// * INPUT: to denote messages that are inserting data into the state actor
/// * OUTPUT: to denote messages that are asking for data output from the state
/// * EMPTY: to denote messages that are returning nothing from the state because the state is empty
#[derive(Debug, Serialize)]
pub enum MessageType {
    INPUT,
    OUTPUT,
    EMPTY
}


/// JSON body that is going to be sent via HTTP to the librarian server.
/// 
/// # Fields
/// * chat_id: the ID of the chat the chat data belongs to
/// * block_data: the chat history of the patient
#[derive(Debug, Serialize)]
struct PostBody {
    pub chat_id: i32,
    pub block_data: String
}


/// This struct defines messages being sent to the state actor.
/// 
/// # Fields
/// * message_type: the type of message, if we are sending an input or output message
/// * chat_id: the ID of the chat in which we update the data for
/// * single_data: if input, this field will be populated to be inserted
/// * block_data: if output, the entire chat history of the patient to be sent to the librarian server
#[derive(Debug, Serialize)]
pub struct StateActorMessage {
    pub message_type: MessageType,
    pub chat_id: Option<i32>,
    pub single_data: Option<String>,
    pub block_data: Option<Vec<String>>
}


impl StateActorMessage {

    /// Sends ```self.block_data``` to another server.
    pub async fn send_to_server(&self) {
        let lib_url = env::var("SERVER_URL").unwrap();
        let joined = self.block_data.clone().unwrap().join("$");

        let body = PostBody {
            chat_id: self.chat_id.unwrap(),
            block_data: joined
        };

        let client = reqwest::Client::new();
        let res = client.post(lib_url)
                                  .json(&body)
                                  .send()
                                  .await.unwrap();
        println!("{:?}", res);
    }

}
