use actix_web::{web, HttpResponse};
use serde_json::value::Value;
use serde_json::Map;

use crate::state::read_file;

use crate::to_do::{to_do_factory, enums::TaskStatus};
use crate::json_serialization::{to_do_item::ToDoItem,
                                to_do_items::ToDoItems};
use crate::processes::process_input;
use crate::jwt::JwToken;


pub async fn edit(to_do_item: web::Json<ToDoItem>, token: JwToken) -> HttpResponse {
    println!("here is the message in the token: {}", token.message);
    let state: Map<String, Value> = read_file("./state.json");
    let status: TaskStatus;
    match &state.get(&to_do_item.title) {
        Some(result) => {
            status = TaskStatus::from_string(result.as_str()
                                                   .unwrap()
                                                   .to_string());
        }
        None=> {
            return HttpResponse::NotFound().json(
                format!("{} not in state", &to_do_item.title))
        }
    }
    let existing_item = to_do_factory(to_do_item.title.as_str(), status.clone());

    if &status.stringify() == &TaskStatus::from_string(
        to_do_item.status.as_str().to_string()).stringify() {
        return HttpResponse::Ok().json(ToDoItems::get_state())
    }
    process_input(existing_item, "edit".to_owned(), &state);
    return HttpResponse::Ok().json(ToDoItems::get_state())
}
