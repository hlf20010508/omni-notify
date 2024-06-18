/*
:project: omni-notify
:author: L-ING
:copyright: (C) 2024 L-ING <hlf01@icloud.com>
:license: MIT, see LICENSE for more details.
*/

use axum::{debug_handler, Extension};
use std::sync::Arc;

use super::models::{PushdeerParams, RequestPushdeerParams};
use crate::env::Env;
use crate::error::{Error, Result};

pub static PATH: &str = "/pushdeer";

#[debug_handler]
pub async fn handler(
    Extension(env): Extension<Arc<Env>>,
    params: PushdeerParams,
) -> Result<String> {
    let env = env.pushdeer.clone()?;

    let url = "https://api2.pushdeer.com/message/push".to_string();

    let params = RequestPushdeerParams {
        pushkey: env.key,
        text: params.title,
        desp: params.body,
        type_field: "markdown".into(),
    };

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .query(&params)
        .send()
        .await
        .map_err(|e| Error::new(e, "failed to send request for pushdeer"))?;

    let result = response
        .text()
        .await
        .map_err(|e| Error::new(e, "failed to get response text for pushdeer"))?;

    Ok(result)
}
