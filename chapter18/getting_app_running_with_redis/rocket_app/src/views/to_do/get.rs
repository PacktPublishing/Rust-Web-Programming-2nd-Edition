use rocket::serde::json::Json;

use crate::json_serialization::to_do_items::ToDoItems;
use crate::jwt::JwToken;


#[get("/get")]
pub async fn get(token: JwToken) -> Json<ToDoItems> {
    return Json(ToDoItems::get_state(token.user_id))
}
