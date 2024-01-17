/*
:project: omni-notify
:author: L-ING
:copyright: (C) 2024 L-ING <hlf01@icloud.com>
:license: MIT, see LICENSE for more details.
*/

use actix_web::{web, HttpResponse};
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use pico_args::Arguments;
use std::error::Error;

use crate::structures::MailParams;

pub async fn email_handler(
    query: Option<web::Query<MailParams>>,
    form: Option<web::Form<MailParams>>,
    json: Option<web::Json<MailParams>>,
) -> Result<HttpResponse, Box<dyn Error>> {
    let mut args = Arguments::from_env();
    let email_address: String = args.value_from_str("--email")?;
    let smtp_server_address: String = args.value_from_str("--server")?;
    let username: String = args.value_from_str("--username")?;
    let password: String = args.value_from_str("--password")?;

    let params: MailParams;
    if let Some(query_data) = query {
        params = query_data.into_inner();
    } else if let Some(form_data) = form {
        params = form_data.into_inner();
    } else if let Some(json_data) = json {
        params = json_data.into_inner();
    } else {
        return Err("Invalid request.".into());
    }

    let email = Message::builder()
        .from(email_address.parse()?)
        .to(email_address.parse()?)
        .subject(&params.title)
        .header(ContentType::TEXT_PLAIN)
        .body(params.body.clone())?;

    let creds = Credentials::new(username, password);

    let mailer = SmtpTransport::starttls_relay(&smtp_server_address)?
        .credentials(creds)
        .build();

    let result: String = mailer.send(&email)?.message().collect();
    return Ok(HttpResponse::Ok().body(result));
}
