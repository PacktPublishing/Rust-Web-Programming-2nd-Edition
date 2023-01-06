use actix_web::{web, Responder};
use serde_json::value::Value;
use serde_json::Map;

use crate::state::read_file;


pub async fn get() -> impl Responder {
    let state: Map<String, Value> = read_file("./state.json");
    return web::Json(state);
}

