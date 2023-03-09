use actix_web::{App, HttpServer, middleware::Logger};
use actix_web::dev::Service;
#[macro_use] extern crate diesel;

mod views;
mod processes;
mod state;
mod to_do;
mod json_serialization;
mod jwt;
mod database;
mod schema;
mod models;
mod config;

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    env_logger::init_from_env(env_logger::Env::new()
        .default_filter_or("info"));
    HttpServer::new(|| {
        let app = App::new()
            .wrap_fn(|req, srv| {
                println!("{:?}", req);
                let future = srv.call(req);
                async {
                    let result = future.await?;
                    Ok(result)
                }
            })
            .configure(views::views_factory)
            .wrap(Logger::new("%a %{User-Agent}i %r %s %D"));
        return app;
    })
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
