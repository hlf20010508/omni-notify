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
        .map_err(|e| Error(format!("failed to parse arg {}: {}", arg_name, e)))?;
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
            address: get_arg_value("--email")?,
            smtp_server: get_arg_value("--smtp-server")?,
            smtp_username: get_arg_value("--smtp-username")?,
            smtp_password: get_arg_value("--smtp-password")?,
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
