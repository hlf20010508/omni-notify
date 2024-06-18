/*
:project: omni-notify
:author: L-ING
:copyright: (C) 2024 L-ING <hlf01@icloud.com>
:license: MIT, see LICENSE for more details.
*/

use std::fmt::Display;

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

#[derive(Debug, Clone)]
pub struct Error(pub String);

impl Error {
    pub fn new<T, U>(e: T, message: U) -> Self
    where
        T: Display,
        U: Display,
    {
        Self(format!("{}: {}", message.to_string(), e))
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.0).into_response()
    }
}

pub type Result<T> = std::result::Result<T, Error>;
