use actix_web::{App, HttpServer};
mod views;
mod to_do;
mod state;
mod processes;
mod json_serialization;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let app = App::new().configure(views::views_factory);
        return app
    })
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
