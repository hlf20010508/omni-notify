/*
:project: omni-notify
:author: L-ING
:copyright: (C) 2024 L-ING <hlf01@icloud.com>
:license: MIT, see LICENSE for more details.
*/

use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct MailParams {
    pub title: String,
    pub body: String,
}

#[derive(Deserialize)]
pub struct PushDeerParams {
    pub title: String,
    pub body: Option<String>,
}

#[derive(Deserialize)]
pub struct TelegramParams {
    pub text: String,
}

#[derive(Serialize)]
pub struct RequestTelegramParams {
    pub chat_id: String,
    pub text: String,
}

#[derive(Serialize)]
pub struct RequestPushDeerParams {
    pub pushkey: String,
    pub text: String,
    pub desp: Option<String>,
    #[serde(rename = "type")]
    pub type_field: String,
}
