use tokio::sync::{mpsc, mpsc::Sender};
use hyper::{Body, Request, Response, Server};
use hyper::body;
use hyper::service::{make_service_fn, service_fn};
use serde_json;
use serde::Deserialize;
use std::net::SocketAddr;

mod actors;
use actors::state::StateActor;
use actors::runner::RunnerActor;
use actors::messages::StateActorMessage;
use actors::messages::MessageType;


/// This struct is responsible for deserializing JSON data from incoming HTTP requests. 
/// 
/// # Fields
/// * chat_id: the ID of the patient the chat data belongs to
/// * turn: the turn that the chat data belongs to
/// * input: the question asked to the chatbot
/// * output: the output that the chatbot gave to the question asked
#[derive(Deserialize, Debug)]
struct IncomingBody {
      pub chat_id: i32,
      pub turn: i32,
      pub input: String,
      pub output: String
}


/// Handles the incoming requests for the server. 
/// 
/// # Arguments
/// * req: the HTTP request that the server has accepted
/// * channel_sender: the sender to enable messages to be sent over a channel to the state actor
async fn handle(req: Request<Body>, channel_sender: Sender<StateActorMessage>) -> Result<Response<Body>, &'static str> {
    println!("incoming message from the outside");

    let method = req.method().clone();
    println!("{}", method);
    let uri = req.uri();
    println!("{}", uri);

    let bytes = body::to_bytes(req.into_body()).await.unwrap();
    let string_body = String::from_utf8(bytes.to_vec()).expect("response was not valid utf-8");
    let value: IncomingBody = serde_json::from_str(&string_body.as_str()).unwrap();

    let message = StateActorMessage {
        message_type: MessageType::INPUT,
        chat_id: Some(value.chat_id),
        single_data: Some(format!("{}>>{}>>{}>>", value.input, value.output, value.turn)),
        block_data: None
    };
    channel_sender.send(message).await.unwrap();
    Ok(Response::new(format!("{:?}", value).into()))
}


#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    // define the two channels to enable communication throughout 
    let (state_tx, state_rx) = mpsc::channel::<StateActorMessage>(1);
    let (runner_tx, runner_rx) = mpsc::channel::<StateActorMessage>(1);
    let channel_sender = state_tx.clone();

    // spin off a thread for our state actor
    tokio::spawn(async move {
        let state_actor = StateActor::new(state_rx, runner_tx);
        state_actor.run().await;
    });

    // spin off a thread for our lib runner actor
    tokio::spawn(async move {
        let lib_runner_actor = RunnerActor::new(runner_rx, state_tx, 30);
        lib_runner_actor.run().await;
    });

    // run the server 
    let server = Server::bind(&addr).serve(make_service_fn( |_conn| {
        
        let channel = channel_sender.clone();

        async {
            // async block is only executed once, so just pass it on to the closure
            Ok::<_, hyper::Error>(service_fn( move |req| {
                // but this closure may also be called multiple times, so make
                // a clone for each call, and move the clone into the async block
                let channel = channel.clone();
                // async move { handle(req, channel).await }
                async {handle(req, channel).await}
            }))
        }
    }));

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
