use actix_web::HttpResponse;
use serde_json::Value;
use serde_json::Map;
use actix_web::HttpRequest;

use crate::to_do::{to_do_factory, enums::TaskStatus};
use crate::json_serialization::to_do_items::ToDoItems;
use crate::state::read_file;
use crate::processes::process_input;


pub async fn create(req: HttpRequest) -> HttpResponse {
    let state: Map<String, Value> = read_file("./state.json");

    let title: String = req.match_info().get("title"
    ).unwrap().to_string();

    let item = to_do_factory(&title.as_str(),
                             TaskStatus::PENDING);
    process_input(item, "create".to_string(), &state);
    return HttpResponse::Ok().json(ToDoItems::get_state())
}
