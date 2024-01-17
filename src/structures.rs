/*
:project: webhook
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
pub struct PushParams {
    pub title: String,
    pub body: Option<String>,
}

#[derive(Serialize)]
pub struct RequestPushParams {
    pub pushkey: String,
    pub text: String,
    pub desp: Option<String>,
    #[serde(rename = "type")]
    pub type_field: String,
}
