use serde_json::value::Value;
use serde_json::Map;
use actix_web::HttpRequest;

use crate::to_do::{to_do_factory, enums::TaskStatus};
use crate::state::read_file;
use crate::processes::process_input;


/// Creates a to-do item storing it in the state.json file. 
pub async fn create(req: HttpRequest) -> String {
    let state: Map<String, Value> = read_file("./state.json"); // 1

    let title: String = req.match_info().get("title"
    ).unwrap().to_string(); // 2

    let item = to_do_factory(&title.as_str(), 
                                    TaskStatus::PENDING); // 3
    process_input(item, "create".to_string(), &state); // 4
    return format!("{} created", title) // 5
}
