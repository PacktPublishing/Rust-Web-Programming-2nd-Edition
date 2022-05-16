use crate::schema::to_do;
use chrono::{NaiveDateTime, Utc};


#[derive(Insertable)]
#[table_name="to_do"]
pub struct NewItem {
    pub title: String,
    pub status: String,
    pub date: NaiveDateTime
}

impl NewItem {
    pub fn new(title: String) -> NewItem {
        let now = Utc::now().naive_local();
        return NewItem{title, status: String::from("pending"), date: now}
    }
}
