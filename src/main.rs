use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use env_logger::Env;

mod handlers;
mod responses;
mod requests;
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
                "/get/email",
                web::get().to(handlers::get::email),
            )
            .route(
                "/post/json/email",
                web::post().to(handlers::post::json::email),
            )
            .route(
                "/post/form/email",
                web::post().to(handlers::post::form::email),
            )
            .route(
                "/get/push",
                web::get().to(handlers::get::push),
            )
            .route(
                "/post/json/push",
                web::post().to(handlers::post::json::push),
            )
            .route(
                "/post/form/push",
                web::post().to(handlers::post::form::push),
            )
            .wrap(Logger::default())
    })
    .bind((HOST, PORT))?
    .run()
    .await;
}
