use actix_web::{web, App, HttpServer, Responder, HttpRequest};


async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", name)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
            .route("/say/hello", web::get().to(||
                async { "Hello Again!" }))

    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
