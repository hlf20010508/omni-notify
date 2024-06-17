/*
:project: omni-notify
:author: L-ING
:copyright: (C) 2024 L-ING <hlf01@icloud.com>
:license: MIT, see LICENSE for more details.
*/

use std::sync::Arc;

use axum::{debug_handler, Extension};
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

use super::models::EmailParams;
use crate::env::Env;
use crate::error::{Error, Result};

pub static PATH: &str = "/email";

#[debug_handler]
pub async fn handler(Extension(env): Extension<Arc<Env>>, params: EmailParams) -> Result<String> {
    let env = env.email.clone()?;

    let email = Message::builder()
        .from(
            env.address
                .parse()
                .map_err(|e| Error(format!("failed to parse email address for from: {}", e)))?,
        )
        .to(env
            .address
            .parse()
            .map_err(|e| Error(format!("failed to parse email address for to: {}", e)))?)
        .subject(params.title)
        .header(ContentType::TEXT_PLAIN)
        .body(params.body)
        .map_err(|e| Error(format!("failed to build email: {}", e)))?;

    let creds = Credentials::new(env.smtp_username.into(), env.smtp_password.into());

    let mailer = SmtpTransport::starttls_relay(&env.smtp_server)
        .map_err(|e| {
            Error(format!(
                "failed to upgrade connection to tls for email: {}",
                e
            ))
        })?
        .credentials(creds)
        .build();

    let result: String = mailer
        .send(&email)
        .map_err(|e| Error(format!("failed to send email: {}", e)))?
        .message()
        .collect();

    Ok(result)
}
