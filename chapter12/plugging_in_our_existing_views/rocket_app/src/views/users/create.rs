use crate::diesel;
use diesel::prelude::*;

use rocket::serde::json::Json;
use rocket::http::Status;

use crate::database::DB;
use crate::json_serialization::new_user::NewUserSchema;
use crate::models::user::new_user::NewUser;
use crate::schema::users;


#[post("/create", data = "<new_user>", format = "json")]
pub async fn create_user(new_user: Json<NewUserSchema>, db: DB) -> Status {
    let name: String = new_user.name.clone();
    let email: String = new_user.email.clone();
    let password: String = new_user.password.clone();
    let new_user = NewUser::new(name, email, password);
    let insert_result = diesel::insert_into(users::table).values(&new_user).execute(&db.connection);
    match insert_result {
        Ok(_) => Status::Created,
        Err(_) => Status::Conflict
    }
}
