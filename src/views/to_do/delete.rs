use actix_web::{HttpResponse, web};
use serde_json::{Map, Value};
use crate::database::{DB};
use crate::json_serialization::to_do_item::ToDoItem;
use crate::json_serialization::to_do_items::ToDoItems;
use crate::jwt::JwtToken;
use crate::processes::process_input;
use crate::state::read_file;
use crate::to_do::enums::TaskStatus;
use crate::diesel;
use diesel::prelude::*;
use crate::models::items::item::Item;
use crate::schema::to_do as td;

pub async fn delete(to_do_item: web::Json<ToDoItem>, db: DB, token: JwtToken) -> HttpResponse {
    let mut connection = db.connection;
    let items = td::table
        .filter(td::columns::title.eq(&to_do_item.title.as_str()))
        .filter(td::columns::user_id.eq(&token.user_id))
        .order(td::columns::id.asc())
        .load::<Item>(&mut connection)
        .unwrap();
    let _ = diesel::delete(&items[0]).execute(&mut connection);
    return HttpResponse::Ok().json(ToDoItems::get_state(token.user_id));
}