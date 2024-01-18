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
use std::error::Error;

use crate::env::{EMAIL, SMTP_PASSWORD, SMTP_SERVER, SMTP_USERNAME};
use crate::structures::MailParams;

pub async fn email_handler(
    query: Option<web::Query<MailParams>>,
    form: Option<web::Form<MailParams>>,
    json: Option<web::Json<MailParams>>,
) -> Result<HttpResponse, Box<dyn Error>> {
    let email_address = EMAIL.as_ref()?;
    let smtp_server = SMTP_SERVER.as_ref()?;
    let smtp_username = SMTP_USERNAME.as_ref()?;
    let smtp_password = SMTP_PASSWORD.as_ref()?;

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
        .body(params.body)?;

    let creds = Credentials::new(smtp_username.into(), smtp_password.into());

    let mailer = SmtpTransport::starttls_relay(smtp_server)?
        .credentials(creds)
        .build();

    let result: String = mailer.send(&email)?.message().collect();
    return Ok(HttpResponse::Ok().body(result));
}
