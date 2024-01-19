/*
:project: omni-notify
:author: L-ING
:copyright: (C) 2024 L-ING <hlf01@icloud.com>
:license: MIT, see LICENSE for more details.
*/

use actix_web::http::header;
use actix_web::middleware::{DefaultHeaders, Logger};
use actix_web::{web, App, HttpServer};
use env_logger::Env;

mod handlers;
mod structures;
mod env;

use env::PORT;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    const HOST: &str = "0.0.0.0";
    println!("Running on http://{}:{}", &HOST, *PORT);

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    return HttpServer::new(|| {
        App::new()
            .route("/email", web::route().to(handlers::email::email_handler))
            .route("/pushdeer", web::route().to(handlers::push::push_handler))
            .route("/telegram", web::route().to(handlers::telegram::telegram_handler))
            .wrap(DefaultHeaders::new().add((header::CONTENT_TYPE, "text/plain; charset=utf-8")))
            .wrap(Logger::default())
    })
    .bind((HOST, *PORT))?
    .run()
    .await;
}
