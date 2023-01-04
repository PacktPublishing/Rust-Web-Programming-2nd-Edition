use actix_web::{web, Responder};
use serde_json::value::Value;
use serde_json::Map;

use crate::state::read_file;
use crate::to_do::{ItemTypes, to_do_factory, enums::TaskStatus};
use crate::json_serialization::to_do_items::ToDoItems;


pub async fn get() -> impl Responder {
    let state: Map<String, Value> = read_file(
        "./state.json");
    let mut array_buffer = Vec::new();

    for (key, value) in state {
        let status = TaskStatus::from_string(
            value.as_str().unwrap().to_string());
        let item: ItemTypes = to_do_factory(
            &key, status);
        array_buffer.push(item);
    }
    let return_package: ToDoItems = ToDoItems::new(
        array_buffer);
    return web::Json(return_package);
}
