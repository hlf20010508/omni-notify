/*
:project: omni-notify
:author: L-ING
:copyright: (C) 2024 L-ING <hlf01@icloud.com>
:license: MIT, see LICENSE for more details.
*/

use axum::{
    async_trait,
    extract::{FromRequest, Query, Request},
    http::Method,
    Form, Json,
};
use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};

macro_rules! query_form_json_impl {
    ($params:ident) => {
        #[async_trait]
        impl<S> FromRequest<S> for $params
        where
            S: Send + Sync,
        {
            type Rejection = Error;

            async fn from_request(req: Request, state: &S) -> Result<Self> {
                let method = req.method().clone();

                let data = match method {
                    Method::GET => {
                        let Query(data) = Query::<$params>::from_request(req, state)
                            .await
                            .map_err(|e| {
                                Error(format!(
                                    "failed to parse query for {}: {}",
                                    stringify!($params),
                                    e
                                ))
                            })?;

                        data
                    }
                    Method::POST => {
                        let content_type = req
                            .headers()
                            .get("Content-Type")
                            .ok_or(Error(format!(
                                "Content-Type header not found for post for {}",
                                stringify!($params)
                            )))?
                            .to_str()
                            .map_err(|e| {
                                Error(format!(
                                    "failed to convert content type to str for {}: {}",
                                    stringify!($params),
                                    e
                                ))
                            })?;

                        match content_type {
                            "application/json" => {
                                let Json(data) = Json::<$params>::from_request(req, state)
                                    .await
                                    .map_err(|e| {
                                        Error(format!(
                                            "failed to parse json for {}: {}",
                                            stringify!($params),
                                            e
                                        ))
                                    })?;
                                data
                            }
                            "application/x-www-form-urlencoded" => {
                                let Form(data) = Form::<$params>::from_request(req, state)
                                    .await
                                    .map_err(|e| {
                                        Error(format!(
                                            "failed to parse form for {}: {}",
                                            stringify!($params),
                                            e
                                        ))
                                    })?;
                                data
                            }
                            _ => {
                                return Err(Error(format!(
                                    "Unsupported content type for {}",
                                    stringify!($params)
                                )))
                            }
                        }
                    }
                    _ => {
                        return Err(Error(format!(
                            "unsupported method for {}",
                            stringify!($params)
                        )))
                    }
                };

                Ok(data)
            }
        }
    };
}

#[derive(Deserialize)]
pub struct EmailParams {
    pub title: String,
    pub body: String,
}

query_form_json_impl!(EmailParams);

#[derive(Deserialize)]
pub struct PushdeerParams {
    pub title: String,
    pub body: Option<String>,
}

query_form_json_impl!(PushdeerParams);

#[derive(Serialize)]
pub struct RequestPushdeerParams {
    pub pushkey: String,
    pub text: String,
    pub desp: Option<String>,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Deserialize)]
pub struct TelegramParams {
    pub text: String,
}

query_form_json_impl!(TelegramParams);

#[derive(Serialize)]
pub struct RequestTelegramParams {
    pub chat_id: String,
    pub text: String,
}
