use actix_web::{Responder};
use crate::json_serialization::to_do_items::ToDoItems;
use crate::jwt::JwtToken;

pub async fn get(token: JwtToken) -> impl Responder {
    return ToDoItems::get_state(token.user_id);
}