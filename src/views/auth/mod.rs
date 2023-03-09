mod login;
mod logout;

use actix_web::web::{ServiceConfig, get, scope};
pub fn auth_views_factory(app: &mut ServiceConfig) {
    app.service(
        scope("api/v1/auth")
            .route("login", get().to(login::login))
            .route("logout", get().to(logout::logout))
    );
}