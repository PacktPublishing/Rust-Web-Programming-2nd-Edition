use crate::schema::to_do;
use chrono::NaiveDateTime;
use super::super::user::user::User;


#[derive(Queryable, Identifiable, Associations)]
#[belongs_to(User)]
#[table_name="to_do"]
pub struct Item {
    pub id: i32,
    pub title: String,
    pub status: String,
    pub date: NaiveDateTime,
    pub user_id: i32,
}

