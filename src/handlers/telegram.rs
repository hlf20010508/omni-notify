/*
:project: omni-notify
:author: L-ING
:copyright: (C) 2024 L-ING <hlf01@icloud.com>
:license: MIT, see LICENSE for more details.
*/

use axum::Extension;
use std::sync::Arc;

use super::models::{RequestTelegramParams, TelegramParams};
use crate::env::Env;
use crate::error::{Error, Result};

pub static PATH: &str = "/telegram";

pub async fn handler(
    Extension(env): Extension<Arc<Env>>,
    params: TelegramParams,
) -> Result<String> {
    let env = env.telegram.clone()?;

    let request_params = RequestTelegramParams {
        chat_id: env.chat_id,
        text: params.text,
    };

    let url = format!("https://api.telegram.org/bot{}/sendMessage", env.bot_token);

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .query(&request_params)
        .send()
        .await
        .map_err(|e| Error::new(e, "failed to send request for telegram"))?;

    let result = response
        .text()
        .await
        .map_err(|e| Error::new(e, "failed to get response text for pushdeer"))?;

    Ok(result)
}
