use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use std::net::SocketAddr;
use std::env;

use serde_json;

use std::{thread, time};

mod tasks;
use tasks::{add::AddTask, subtract::SubtractTask, multiply::MultiplyTask, TaskType, TaskMessage};
use hyper::body;
use hyper::http::StatusCode;


async fn handle(req: Request<Body>) -> Result<Response<Body>, &'static str> {
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let task_type = req.uri().to_string().replace("/", "");
    let body_bytes = body::to_bytes(req.into_body()).await.unwrap();

    let bytes: Vec<u8>;
    let message_type: TaskType;

    match task_type.as_str() {
        "add" => {
            let body: AddTask = serde_json::from_slice(&body_bytes).unwrap();
            bytes = bincode::serialize(&body).unwrap();
            message_type = TaskType::ADD;
        },
        "multiply" => {
            let body: MultiplyTask = serde_json::from_slice(&body_bytes).unwrap();
            bytes = bincode::serialize(&body).unwrap();
            message_type = TaskType::MULTIPLY;
        },
        "subtract" => {
            let body: SubtractTask = serde_json::from_slice(&body_bytes).unwrap();
            bytes = bincode::serialize(&body).unwrap();
            message_type = TaskType::SUBTRACT;
        },
        _ => {
            let response = Response::builder().status(StatusCode::NOT_FOUND).body(Body::from("task not found"));
            return Ok(response.unwrap())
        }
    }

    let message = TaskMessage{task_type: message_type, task: bytes};
    let serialized_message = bincode::serialize(&message).unwrap();

    let mut con = client.get_connection().unwrap();
    let _ : () = redis::cmd("LPUSH").arg("some_queue").arg(serialized_message.clone()).query(&mut con).unwrap();
    std::mem::drop(con);

    Ok(Response::new(Body::from("task sent")))
}


#[tokio::main]
async fn main() {
    let app_type = env::var("APP_TYPE").unwrap();

    match app_type.as_str() {

        "server" => {
            let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
            let server = Server::bind(&addr).serve(make_service_fn( |_conn| {

                async {
                    Ok::<_, hyper::Error>(service_fn( move |req| {
                        async {handle(req).await}
                    }))
                }
            }));
        
            if let Err(e) = server.await {
                eprintln!("server error: {}", e);
            }
        },
        "worker" => {

            let client = redis::Client::open("redis://127.0.0.1/").unwrap();

            loop {
                let mut con = client.get_connection().unwrap();
                let outcome: Option<Vec<u8>> = redis::cmd("RPOP").arg("some_queue").query(&mut con).unwrap();
                std::mem::drop(con);

                match outcome {
                    Some(data) => {
                        let deserialized_message: TaskMessage = bincode::deserialize(&data).unwrap();

                        match deserialized_message.task_type {
                            TaskType::ADD => {
                                let deserialized_task: AddTask = bincode::deserialize(&deserialized_message.task).unwrap();
                                println!("add: {:?}", deserialized_task.run());
                            },
                            TaskType::MULTIPLY => {
                                let deserialized_task: MultiplyTask = bincode::deserialize(&deserialized_message.task).unwrap();
                                println!("multiply: {:?}", deserialized_task.run());
                            },
                            TaskType::SUBTRACT => {
                                let deserialized_task: SubtractTask = bincode::deserialize(&deserialized_message.task).unwrap();
                                println!("subtract: {:?}", deserialized_task.run());
                            }
                        }
                    },
                    None => {
                        println!("empty queue");
                        let five_seconds = time::Duration::from_secs(5);
                        thread::sleep(five_seconds);
                    }
                }
            }
        }
        _ => {
            panic!("{} app type not supported", app_type);
        }

    }
}
