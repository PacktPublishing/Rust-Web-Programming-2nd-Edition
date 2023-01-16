use hyper::{Body, Request, Response, Server};
use hyper::body;
use hyper::service::{make_service_fn, service_fn};
use std::net::SocketAddr;
use std::env;

use serde::{Serialize, Deserialize};
use serde_json;
use bytes::{BufMut, BytesMut};
use std::{thread, time};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncomingBody {
    pub one: String,
    pub two: i32
}


async fn handle(req: Request<Body>) -> Result<Response<Body>, &'static str> {

    let bytes = body::to_bytes(req.into_body()).await.unwrap();
    let response_body: IncomingBody = serde_json::from_slice(&bytes).unwrap();

    let mut buf = BytesMut::new().writer();
    serde_json::to_writer(&mut buf, &response_body).unwrap();
    Ok(Response::new(Body::from(buf.into_inner().freeze())))
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
                let body = IncomingBody{one: "one".to_owned(), two: 2};
                let bytes = bincode::serialize(&body).unwrap();

                let outcome: Option<Vec<u8>>;
                {
                    let mut con = client.get_connection().unwrap();
                    let _ : () = redis::cmd("LPUSH").arg("some_queue")
                                                    .arg(bytes.clone())
                                                    .query(&mut con)
                                                    .unwrap();
                    // pop our task from the queue
                    outcome = redis::cmd("LPOP").arg("some_queue")
                                                .query(&mut con)
                                                .unwrap();
                }
                match outcome {
                    Some(data) => {
                        let deserialized_struct: IncomingBody = bincode::deserialize(&data).unwrap();
                        println!("{:?}", deserialized_struct);
                        let five_seconds = time::Duration::from_secs(5);
                        tokio::time::sleep(five_seconds).await;
                    },
                    None => {
                        let duration = tokio::time::Duration::from_secs(5);
                        tokio::time::sleep(duration).await;
                    }
                }
            }
        }
        _ => {
            panic!("{} app type not supported", app_type);
        }
    }
}

