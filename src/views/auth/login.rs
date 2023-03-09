use std::collections::HashMap;
use actix_web::{web, HttpResponse};
use crate::database::DB;
use crate::json_serialization::login::Login;
use crate::jwt::JwtToken;
use crate::models::user::user::User;
use crate::schema::users as usr;
use crate::diesel;
use diesel::prelude::*;

pub async fn login(credentials: web::Json<Login>, db: DB) -> HttpResponse {
    let password = credentials.password.clone();
    let mut connection = db.connection;
    let users = usr::table
        .filter(usr::columns::username.eq(credentials.username.clone()))
        .load::<User>(&mut connection).unwrap();
    if users.len() == 0 {
        return HttpResponse::NotFound().await.unwrap();
    } else if users.len() > 1 {
        return HttpResponse::Conflict().await.unwrap();
    }
    match users[0].verify(password) {
        true => {
            let token = JwtToken::new(users[0].id);
            let raw_token = token.encode();
            let mut body = HashMap::new();
            body.insert("token", raw_token);
            HttpResponse::Ok().json(body)
        },
        false => HttpResponse::Unauthorized().await.unwrap()
    }
}