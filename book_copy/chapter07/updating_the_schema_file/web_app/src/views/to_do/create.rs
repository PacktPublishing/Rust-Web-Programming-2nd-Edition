use crate::diesel;
use diesel::prelude::*;

use actix_web::HttpResponse;
use actix_web::HttpRequest;

use crate::json_serialization::to_do_items::ToDoItems;

use crate::database::DB;
use crate::models::item::new_item::NewItem;
use crate::models::item::item::Item;
use crate::schema::to_do;


/// Creates a to-do item storing it in the state.json file. 
pub async fn create(req: HttpRequest, db: DB) -> HttpResponse {
    let title: String = req.match_info().get("title"
    ).unwrap().to_string();

    let items = to_do::table
        .filter(to_do::columns::title.eq(&title.as_str()))
        .order(to_do::columns::id.asc())
        .load::<Item>(&db.connection)
        .unwrap();

    if items.len() == 0 {
        let new_post = NewItem::new(title, 1);
        let _ = diesel::insert_into(to_do::table).values(&new_post)
            .execute(&db.connection);
    }
    return HttpResponse::Ok().json(ToDoItems::get_state())
}
