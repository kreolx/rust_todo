mod create;
mod get;
mod edit;
mod delete;

use actix_web::web::{ServiceConfig, get, scope, post};
pub fn to_do_views_factory(app: &mut ServiceConfig) {
    app.service(scope("api/v1/item")
        .route("delete", post().to(delete::delete))
        .route("edit", post().to(edit::edit))
        .route("get", get().to(get::get))
        .route("create/{title}", post().to(create::create))
        .route("create/{title}", get().to(create::create)));
}