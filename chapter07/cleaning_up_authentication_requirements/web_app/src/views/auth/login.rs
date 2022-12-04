use crate::diesel;
use diesel::prelude::*;
use actix_web::{web, HttpResponse, Responder};

use crate::database::DB;
use crate::models::user::user::User;
use crate::json_serialization::{login::Login, login_response::LoginResponse};
use crate::schema::users;
use crate::jwt::JwToken;


pub async fn login(credentials: web::Json<Login>, db: DB) -> HttpResponse {

    let users = users::table
        .filter(users::columns::username.eq(credentials.username.clone()))
        .load::<User>(&db.connection).unwrap();

    if users.len() == 0 {
        return HttpResponse::NotFound().finish()
    } else if users.len() > 1 {
        return HttpResponse::Conflict().finish()
    }

    match users[0].verify(credentials.password.clone()) {
        true => {
            let user_id = users[0].clone().id;
            let token = JwToken::new(user_id);
            let raw_token = token.encode();
            let response = LoginResponse{token: raw_token.clone()};
            let body = serde_json::to_string(&response).unwrap();
            HttpResponse::Ok().append_header(("token", raw_token))
                              .json(&body)
        },
        false => HttpResponse::Unauthorized().finish()
    }
}

