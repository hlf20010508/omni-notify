use axum::Extension;
use pico_args::Arguments;
use std::str::FromStr;
use std::{fmt::Display, sync::Arc};

use crate::error::{Error, Result};

fn get_arg_value<T>(arg_name: &'static str) -> Result<T>
where
    T: FromStr,
    T::Err: Display,
{
    let mut args = Arguments::from_env();
    let value: T = args
        .value_from_str(arg_name)
        .map_err(|e| Error::new(e, "failed to parse arg"))?;
    // .context("failed to parse arg");
    return Ok(value);
}

#[derive(Debug, Clone)]
pub struct EmailEnv {
    pub address: String,
    pub smtp_server: String,
    pub smtp_username: String,
    pub smtp_password: String,
}

impl EmailEnv {
    fn new() -> Result<Self> {
        Ok(EmailEnv {
            address: get_arg_value("--email-address")?,
            smtp_server: get_arg_value("--email-smtp-server")?,
            smtp_username: get_arg_value("--email-smtp-username")?,
            smtp_password: get_arg_value("--email-smtp-password")?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct PushdeerEnv {
    pub key: String,
}

impl PushdeerEnv {
    fn new() -> Result<Self> {
        Ok(PushdeerEnv {
            key: get_arg_value("--pushdeer-key")?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct TelegramEnv {
    pub bot_token: String,
    pub chat_id: String,
}

impl TelegramEnv {
    fn new() -> Result<Self> {
        Ok(TelegramEnv {
            bot_token: get_arg_value("--tg-bot-token")?,
            chat_id: get_arg_value("--tg-chat-id")?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Env {
    pub port: u16,
    pub email: Result<EmailEnv>,
    pub pushdeer: Result<PushdeerEnv>,
    pub telegram: Result<TelegramEnv>,
}

impl Env {
    pub fn new() -> Self {
        let port = get_arg_value("--port").unwrap_or(8080);
        let email = EmailEnv::new();
        let pushdeer = PushdeerEnv::new();
        let telegram = TelegramEnv::new();

        Env {
            port,
            email,
            pushdeer,
            telegram,
        }
    }

    pub fn into_layer(self) -> Extension<Arc<Self>> {
        Extension(Arc::new(self))
    }
}

#[cfg(test)]
pub mod tests {
    use dotenv::dotenv;
    use std::env;

    use super::*;

    fn get_email_env() -> Result<EmailEnv> {
        dotenv().ok();

        let address =
            env::var("EMAIL_ADDRESS").map_err(|e| Error::new(e, "failed to parse arg"))?;
        let smtp_server =
            env::var("EMAIL_SMTP_SERVER").map_err(|e| Error::new(e, "failed to parse arg"))?;
        let smtp_username =
            env::var("EMAIL_SMTP_USERNAME").map_err(|e| Error::new(e, "failed to parse arg"))?;
        let smtp_password =
            env::var("EMAIL_SMTP_PASSWORD").map_err(|e| Error::new(e, "failed to parse arg"))?;

        Ok(EmailEnv {
            address,
            smtp_server,
            smtp_username,
            smtp_password,
        })
    }

    fn get_pushdeer_env() -> Result<PushdeerEnv> {
        dotenv().ok();

        let key = env::var("PUSHDEER_KEY").map_err(|e| Error::new(e, "failed to parse arg"))?;

        Ok(PushdeerEnv { key })
    }

    fn get_telegram_env() -> Result<TelegramEnv> {
        dotenv().ok();

        let bot_token =
            env::var("TG_BOT_TOKEN").map_err(|e| Error::new(e, "failed to parse arg"))?;
        let chat_id = env::var("TG_CHAT_ID").map_err(|e| Error::new(e, "failed to parse arg"))?;

        Ok(TelegramEnv { bot_token, chat_id })
    }

    pub fn get_env() -> Env {
        dotenv().ok();

        let port = env::var("PORT")
            .unwrap_or("8080".to_string())
            .parse()
            .unwrap();

        // let

        Env {
            port,
            email: get_email_env(),
            pushdeer: get_pushdeer_env(),
            telegram: get_telegram_env(),
        }
    }
}
