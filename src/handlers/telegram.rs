/*
:project: omni-notify
:author: L-ING
:copyright: (C) 2024 L-ING <hlf01@icloud.com>
:license: MIT, see LICENSE for more details.
*/

use actix_web::{web, HttpResponse};
use std::error::Error;
use pico_args::Arguments;

use crate::structures::{TelegramParams, RequestTelegramParams};

pub async fn telegram_handler(
    query: Option<web::Query<TelegramParams>>,
    form: Option<web::Form<TelegramParams>>,
    json: Option<web::Json<TelegramParams>>,
) -> Result<HttpResponse, Box<dyn Error>> {
    let mut args = Arguments::from_env();
    let tg_bot_token: String = args.value_from_str("--tg-bot-token")?;
    let chat_id: String = args.value_from_str("--tg-chat-id")?;

    let params: TelegramParams;
    if let Some(query_data) = query {
        params = query_data.into_inner();
    } else if let Some(form_data) = form {
        params = form_data.into_inner();
    } else if let Some(json_data) = json {
        params = json_data.into_inner();
    } else {
        return Err("Invalid request.".into());
    }

    let request_params = RequestTelegramParams {
        chat_id,
        text: params.text,
    };

    let url = format!("https://api.telegram.org/bot{tg_bot_token}/sendMessage");
    let client = reqwest::Client::new();
    let response = client.get(&url).query(&request_params).send().await?;
    let content = response.text().await?;

    return Ok(HttpResponse::Ok().body(content));
}