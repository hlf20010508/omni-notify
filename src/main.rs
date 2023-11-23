use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use env_logger::Env;

mod handlers;
mod responses;
mod structures;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    const HOST: &str = "0.0.0.0";
    const PORT: u16 = 8080;
    println!("Running on http://{}:{}", &HOST, &PORT);

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    return HttpServer::new(|| {
        App::new()
            .route(
                "/get/notify/email",
                web::get().to(handlers::get::notify::email),
            )
            .route(
                "/post/json/notify/email",
                web::post().to(handlers::post::json::notify::email),
            )
            .route(
                "/post/form/notify/email",
                web::post().to(handlers::post::form::notify::email),
            )
            .wrap(Logger::default())
    })
    .bind((HOST, PORT))?
    .run()
    .await;
}
