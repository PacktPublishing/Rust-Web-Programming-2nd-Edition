use crate::diesel;
use diesel::prelude::*;

use crate::database::DBCONNECTION;
use crate::models::item::item::Item;
use crate::schema::to_do;

use serde::Serialize;
use std::vec::Vec;

use actix_web::{
    body::BoxBody, http::header::ContentType,
    HttpRequest, HttpResponse, Responder,
};

use crate::to_do::ItemTypes;
use crate::to_do::structs::base::Base;
use crate::to_do::{to_do_factory, enums::TaskStatus};


#[derive(Serialize)]
pub struct ToDoItems {
    pub pending_items: Vec<Base>,
    pub done_items: Vec<Base>,
    pub pending_item_count: i8,
    pub done_item_count: i8
}


impl ToDoItems {

    pub fn new(input_items: Vec<ItemTypes>) -> ToDoItems {
        let mut pending_array_buffer = Vec::new();
        let mut done_array_buffer = Vec::new();

        for item in input_items {
            match item {
                ItemTypes::Pending(packed) => pending_array_buffer.
                    push(packed.super_struct),
                ItemTypes::Done(packed) => done_array_buffer.push(
                    packed.super_struct)
            }
        }
        let done_count: i8 = done_array_buffer.len() as i8;
        let pending_count: i8 = pending_array_buffer.len() as i8;
        return ToDoItems{
            pending_items: pending_array_buffer,
            done_item_count: done_count,
            pending_item_count: pending_count,
            done_items: done_array_buffer
        }
    }

    pub fn get_state(user_id: i32) -> ToDoItems {
        let connection = DBCONNECTION.db_connection.get().unwrap();

        let items = to_do::table.filter(to_do::columns::user_id.eq(&user_id))
                                .order(to_do::columns::id.asc())
                                .load::<Item>(&connection)
                                .unwrap();

        let mut array_buffer = Vec::with_capacity(items.len());

        for item in items {
            let status = TaskStatus::from_string(item.status.as_str().to_string());
            let item = to_do_factory(&item.title, status);
            array_buffer.push(item);
        }
        return ToDoItems::new(array_buffer)
    }
}

impl Responder for ToDoItems {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest)
                  -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}


