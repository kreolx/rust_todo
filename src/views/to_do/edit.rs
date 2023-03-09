use actix_web::{HttpResponse, web};
use serde_json::{Map, Value};
use web::Json;
use crate::database::DB;
use crate::json_serialization::to_do_item::ToDoItem;
use crate::json_serialization::to_do_items::ToDoItems;
use crate::processes::process_input;
use crate::state::read_file;
use crate::to_do::enums::TaskStatus;
use crate::jwt::JwtToken;
use crate::diesel;
use diesel::prelude::*;
use crate::schema::to_do as td;

pub async fn edit(to_do_item: Json<ToDoItem>, db: DB, token: JwtToken) ->HttpResponse {
    let results = td::table.filter(td::columns::title.eq(&to_do_item.title));
    let mut connection = db.connection;
    let _ = diesel::update(results)
        .set(td::columns::status.eq("DONE"))
        .execute(&mut connection);
    return HttpResponse::Ok().json(ToDoItems::get_state());
}