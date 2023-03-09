use actix_web::{App, HttpServer};
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
            .configure(views::views_factory);
        return app;
    })
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
