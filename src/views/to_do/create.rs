use serde_json::value::Value;
use serde_json::Map;
use actix_web::{HttpRequest, HttpResponse};
use crate::json_serialization::to_do_items::ToDoItems;
use crate::diesel;
use diesel::prelude::*;
use crate::database::{DB};
use crate::models::items::item::Item;
use crate::models::items::new_item::NewItem;
use crate::schema::to_do as td;

use crate::state::read_file;
use crate::processes::process_input;

pub async fn create(req: HttpRequest, db: DB) -> HttpResponse {
    let title: String = req.match_info().get("title")
        .unwrap().to_string();
    let mut connection = db.connection;
    let items = td::table
        .filter(td::columns::title.eq(&title.as_str()))
        .order(td::columns::id.asc())
        .load::<Item>(&mut connection)
        .unwrap();
    if items.len() == 0 {
        let new_post = NewItem::new(title, 1);
        let _ = diesel::insert_into(td::table).values(&new_post)
            .execute(&mut connection);
    }
    return HttpResponse::Ok().json(ToDoItems::get_state())
}