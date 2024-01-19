/*
:project: omni-notify
:author: L-ING
:copyright: (C) 2024 L-ING <hlf01@icloud.com>
:license: MIT, see LICENSE for more details.
*/

use actix_web::{web, HttpResponse};
use std::error::Error;

use crate::env::PUSHDEER_KEY;
use crate::structures::{PushDeerParams, RequestPushDeerParams};

pub async fn push_handler(
    query: Option<web::Query<PushDeerParams>>,
    form: Option<web::Form<PushDeerParams>>,
    json: Option<web::Json<PushDeerParams>>,
) -> Result<HttpResponse, Box<dyn Error>> {
    let pushdeer_key = PUSHDEER_KEY.as_ref()?;
    let url: String = "https://api2.pushdeer.com/message/push".into();

    let pushdeer_params: PushDeerParams;

    if let Some(query_data) = query {
        pushdeer_params = query_data.into_inner();
    } else if let Some(form_data) = form {
        pushdeer_params = form_data.into_inner();
    } else if let Some(json_data) = json {
        pushdeer_params = json_data.into_inner();
    } else {
        return Err("Invalid request.".into());
    }

    let params = RequestPushDeerParams {
        pushkey: pushdeer_key.into(),
        text: pushdeer_params.title,
        desp: pushdeer_params.body,
        type_field: "markdown".into(),
    };

    let client = reqwest::Client::new();
    let response = client.get(&url).query(&params).send().await?;
    return Ok(HttpResponse::Ok().body(response.text().await?));
}
