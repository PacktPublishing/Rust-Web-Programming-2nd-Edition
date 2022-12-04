use crate::diesel;
use diesel::prelude::*;
use actix_web::{web, HttpResponse, Responder};

use crate::database::DB;
use crate::models::user::user::User;
use crate::json_serialization::login::Login;
use crate::schema::users;
use crate::jwt::JwToken;


pub async fn login(credentials: web::Json<Login>, db: DB) -> impl Responder {
    let users = users::table
        .filter(users::columns::username.eq(
            credentials.username.clone())
        ).load::<User>(&db.connection).unwrap();

    if users.len() == 0 {
        return HttpResponse::NotFound()
    } else if users.len() > 1 {
        return HttpResponse::Conflict()
    }
    match users[0].verify(credentials.password.clone()) {
        true => {
            let token = JwToken::new(users[0].id);
            let raw_token = token.encode();
            HttpResponse::Ok().append_header(
                ("token", raw_token)).take()
        },
        false => HttpResponse::Unauthorized()
    }

}

