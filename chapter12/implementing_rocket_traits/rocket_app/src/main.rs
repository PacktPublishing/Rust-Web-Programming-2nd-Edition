#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
use diesel::prelude::*;

use rocket::serde::json::Json;

mod schema;
mod database;
mod json_serialization;
mod models;
mod to_do;
mod config;
mod jwt;

use crate::models::item::item::Item;
use crate::json_serialization::to_do_items::ToDoItems;
use crate::models::item::new_item::NewItem;
use database::DB;


#[post("/create/<title>")]
fn item_create(title: String, db: DB) -> Json<ToDoItems> {
    let items = schema::to_do::table
        .filter(schema::to_do::columns::title.eq(&title.as_str()))
        .order(schema::to_do::columns::id.asc())
        .load::<Item>(&db.connection)
        .unwrap();

    if items.len() == 0 {
        let new_post = NewItem::new(title, 1);
        let _ = diesel::insert_into(schema::to_do::table).values(&new_post)
            .execute(&db.connection);
    }
    return Json(ToDoItems::get_state(1));
}


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

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, hello, bye])
                   .mount("/v1/item", routes![item_create])
}
