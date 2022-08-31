use crate::diesel;
use diesel::prelude::*;
use rocket::serde::json::Json;

use crate::database::DB;
use crate::models::user::user::User;
use crate::json_serialization::{login::Login, login_response::LoginResponse};
use crate::schema::users;
use crate::jwt::JwToken;



#[post("/login", data = "<credentials>", format = "json")]
pub async fn login<'a>(credentials: Json<Login>, db: DB) -> Json<LoginResponse> {
    let username: String = credentials.username.clone();
    let password: String = credentials.password.clone();

    let users = users::table
        .filter(users::columns::username.eq(username.as_str()))
        .load::<User>(&db.connection).unwrap();

    match users[0].clone().verify(password) {
        true => {
            let user_id = users[0].clone().id;
            let token = JwToken::new(user_id);
            let raw_token = token.encode();
            let body = LoginResponse{token: raw_token.clone()};
            return Json(body)
        },
        false => panic!("unauthorised")
    }
}


#[get("/login", data = "<credentials>", format = "json")]
pub async fn login_get<'a>(credentials: Json<Login>, db: DB) -> Json<LoginResponse> {
    let username: String = credentials.username.clone();
    let password: String = credentials.password.clone();

    let users = users::table
        .filter(users::columns::username.eq(username.as_str()))
        .load::<User>(&db.connection).unwrap();

    match users[0].clone().verify(password) {
        true => {
            let user_id = users[0].clone().id;
            let token = JwToken::new(user_id);
            let raw_token = token.encode();
            let body = LoginResponse{token: raw_token.clone()};
            return Json(body)
        },
        false => panic!("unauthorised")
    }
}

