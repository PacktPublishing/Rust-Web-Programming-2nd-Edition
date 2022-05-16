use actix_web::Responder;
use crate::json_serialization::to_do_items::ToDoItems;


pub async fn get() -> impl Responder {
    return ToDoItems::get_state();
}
