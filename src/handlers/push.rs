/*
:project: omni-notify
:author: L-ING
:copyright: (C) 2024 L-ING <hlf01@icloud.com>
:license: MIT, see LICENSE for more details.
*/

use actix_web::{web, HttpResponse};
use pico_args::Arguments;
use std::error::Error;

use crate::structures::{PushParams, RequestPushParams};

pub async fn push_handler(
    query: Option<web::Query<PushParams>>,
    form: Option<web::Form<PushParams>>,
    json: Option<web::Json<PushParams>>,
) -> Result<HttpResponse, Box<dyn Error>> {
    let mut args = Arguments::from_env();
    let pushkey: String = args.value_from_str("--pushdeer-key")?;
    let url: String = "https://api2.pushdeer.com/message/push".into();

    let push_params: PushParams;

    if let Some(query_data) = query {
        push_params = query_data.into_inner();
    } else if let Some(form_data) = form {
        push_params = form_data.into_inner();
    } else if let Some(json_data) = json {
        push_params = json_data.into_inner();
    } else {
        return Err("Invalid request.".into());
    }

    let params = RequestPushParams {
        pushkey,
        text: push_params.title,
        desp: push_params.body,
        type_field: "markdown".into(),
    };

    let client = reqwest::Client::new();
    let response = client.get(&url).query(&params).send().await?;
    return Ok(HttpResponse::Ok().body(response.text().await?));
}
