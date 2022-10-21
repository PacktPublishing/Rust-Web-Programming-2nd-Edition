use crate::diesel;
use diesel::prelude::*;

use rocket::serde::json::Json;

use crate::database::DB;
use crate::schema::to_do;

use crate::json_serialization::{to_do_item::ToDoItem, 
                                to_do_items::ToDoItems};
use crate::jwt::JwToken;
use crate::models::item::item::Item;


#[post("/delete", data = "<to_do_item>", format = "json")]
pub async fn delete(to_do_item: Json<ToDoItem>, token: JwToken, db: DB) -> Json<ToDoItems> {
    let items = to_do::table
        .filter(to_do::columns::title.eq(&to_do_item.title.as_str()))
        .filter(to_do::columns::user_id.eq(&token.user_id))
        .order(to_do::columns::id.asc())
        .load::<Item>(&db.connection)
        .unwrap();
    let _ = diesel::delete(&items[0]).execute(&db.connection);

    return Json(ToDoItems::get_state(token.user_id))
}
