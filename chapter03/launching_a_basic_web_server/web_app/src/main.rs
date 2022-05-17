use actix_web::{web, App, HttpServer, Responder, HttpRequest};


async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", name)
}

/// Main function for running the server. This is the entry point for the entire compiled program and as a 
/// result, the server is defined, views are added, and then the server is run.
/// 
/// # Running in dev mode 
/// Running the server in dev can be done with the following command:
/// ```bash
/// cargo run config.yml
/// ``` 
/// In the dev repo the config file is defined in the root path.
/// 
/// # Running in release mode
/// We first need to compile the server in release mode with the following command: 
/// ```bash 
/// cargo build --release
/// ```
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new( || {
        println!("http server factory is firing");
        App::new()
        .route("/", web::get().to(greet))
        .route("/{name}", web::get().to(greet))
        .route("/say/hello", web::get().to(|| async { "Hello Again!" }))
    })
    .bind("127.0.0.1:8080")?
    .workers(3)
    .run()
    .await
}