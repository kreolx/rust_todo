use serde_json::value::Value;
use serde_json::Map;
use actix_web::{HttpRequest, HttpResponse};
use crate::json_serialization::to_do_items::ToDoItems;
use crate::diesel;
use diesel::prelude::*;
use crate::database::{DB};
use crate::jwt::JwtToken;
use crate::models::items::item::Item;
use crate::models::items::new_item::NewItem;
use crate::schema::to_do as td;

use crate::state::read_file;
use crate::processes::process_input;

pub async fn create(req: HttpRequest, db: DB, token: JwtToken) -> HttpResponse {
    let title: String = req.match_info().get("title")
        .unwrap().to_string();
    let mut connection = db.connection;
    let items = td::table
        .filter(td::columns::title.eq(&title.as_str()))
        .filter(td::columns::user_id.eq(&token.user_id))
        .order(td::columns::id.asc())
        .load::<Item>(&mut connection)
        .unwrap();
    if items.len() == 0 {
        let new_post = NewItem::new(title, token.user_id);
        let _ = diesel::insert_into(td::table).values(&new_post)
            .execute(&mut connection);
    }
    return HttpResponse::Ok().json(ToDoItems::get_state(token.user_id))
}