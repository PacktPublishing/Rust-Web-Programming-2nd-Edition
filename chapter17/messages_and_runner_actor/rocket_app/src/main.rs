#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;

use rocket::http::Header;
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};

mod schema;
mod database;
mod json_serialization;
mod models;
mod to_do;
mod config;
mod jwt;
mod views;

use views::auth::{login::{login, login_get}, logout::logout};
use views::to_do::{create::create, delete::delete, edit::edit, get::get};
use views::users::create::create_user;


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/hello/<name>/<age>")]
fn hello(name: String, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

#[get("/bye/<name>/<age>")]
fn bye(name: String, age: u8) -> String {
    format!("Goodbye, {} year old named {}!", age,
    name)
}

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {

    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, hello, bye])
                   .mount("/v1/item/", routes![create, delete, edit, get])
                   .mount("/v1/auth/", routes![login, logout, login_get])
                   .mount("/v1/user/", routes![create_user])
                   .attach(CORS)
                   .manage(CORS)
}
