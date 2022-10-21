mod login;
mod logout;

use actix_web::web::{ServiceConfig, get, post, scope};
use actix_web::Responder;


pub async fn ping() -> impl Responder {
    "ping"
}


pub fn auth_views_factory(app: &mut ServiceConfig) {
    app.service(
            scope("v1/auth")
            .route("login", get().to(login::login))
            .route("login", post().to(login::login))
            .route("logout", get().to(logout::logout))
            .route("ping", get().to(ping))
    );
} 
