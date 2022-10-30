use hyper::{Body, Request, Response, Server};
use hyper::body;
use hyper::service::{make_service_fn, service_fn};
use std::net::SocketAddr;
use std::env;

use serde::{Serialize, Deserialize};
use serde_json;
use bytes::{BufMut, BytesMut};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncomingBody {
    pub one: String,
    pub two: i32
}
use std::{thread, time};


async fn handle(req: Request<Body>) -> Result<Response<Body>, &'static str> {
    let method = req.method().clone();
    println!("{}", method);
    let uri = req.uri();
    println!("{}", uri);

    let bytes = body::to_bytes(req.into_body()).await.unwrap();
    let response_body: IncomingBody = serde_json::from_slice(&bytes).unwrap();

    let mut buf = BytesMut::new().writer();
    serde_json::to_writer(&mut buf, &response_body)
    .expect("serialization of `serde_json::Value` into `BytesMut` cannot fail");
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

                let mut con = client.get_connection().unwrap();
                let _ : () = redis::cmd("LPUSH").arg("some_queue").arg(bytes.clone()).query(&mut con).unwrap();

                let outcome: Option<Vec<u8>> = redis::cmd("LPOP").arg("some_queue").query(&mut con).unwrap();
                std::mem::drop(con);

                match outcome {
                    Some(data) => {
                        let deserialized_struct: IncomingBody = bincode::deserialize(&data).unwrap();
                        println!("{:?}", deserialized_struct);
                    },
                    None => {
                        let five_seconds = time::Duration::from_secs(5);
                        thread::sleep(five_seconds);
                    }
                }
            }
            // let _ : () = redis::cmd("LPUSH").arg("some_queue").arg(bytes).query(&mut con).unwrap();
        }
        _ => {
            panic!("{} app type not supported", app_type);
        }

    }
}
