/*
:project: omni-notify
:author: L-ING
:copyright: (C) 2024 L-ING <hlf01@icloud.com>
:license: MIT, see LICENSE for more details.
*/

use axum::body::Body;
use axum::http::{Method, Request, Response, StatusCode};
use axum::routing::get;
use axum::Router;
use http_body_util::BodyExt;
use tower::ServiceExt;

use super::email;
use crate::env::tests::get_env;
use crate::error::{Error, Result};
use crate::handlers::models::{EmailParams, PushdeerParams, TelegramParams};
use crate::handlers::{pushdeer, telegram};

pub trait ResponseExt {
    async fn to_string(self) -> Result<String>;
}

impl ResponseExt for Response<Body> {
    async fn to_string(self) -> Result<String> {
        let result = String::from_utf8(
            self.into_body()
                .collect()
                .await
                .map_err(|e| Error::new(e, "failed to collect response body"))?
                .to_bytes()
                .to_vec(),
        )
        .map_err(|e| Error::new(e, "failed to convert response body to string"))?;

        Ok(result)
    }
}

#[tokio::test]
async fn test_email() {
    let env = get_env();

    let router = Router::new()
        .route(email::PATH, get(email::handler).post(email::handler))
        .layer(env.into_layer());

    let data = EmailParams {
        title: "test title".to_string(),
        body: "test body".to_string(),
    };

    let body = serde_json::to_string(&data).unwrap();

    let req = Request::builder()
        .method(Method::POST)
        .uri(email::PATH)
        .header("Content-Type", "application/json")
        .body(body)
        .unwrap();

    let res = router.oneshot(req).await.unwrap();

    let status = res.status();
    let body = res.to_string().await.unwrap();

    println!("response body: {}", body);

    assert_eq!(status, StatusCode::OK);
}

#[tokio::test]
async fn test_pushdeer() {
    let env = get_env();

    let router = Router::new()
        .route(
            pushdeer::PATH,
            get(pushdeer::handler).post(pushdeer::handler),
        )
        .layer(env.into_layer());

    let data = PushdeerParams {
        title: "test title".to_string(),
        body: Some("test body".to_string()),
    };

    let body = serde_json::to_string(&data).unwrap();

    let req = Request::builder()
        .method(Method::POST)
        .uri(pushdeer::PATH)
        .header("Content-Type", "application/json")
        .body(body)
        .unwrap();

    let res = router.oneshot(req).await.unwrap();

    let status = res.status();
    let body = res.to_string().await.unwrap();

    println!("response body: {}", body);

    assert_eq!(status, StatusCode::OK);
}

#[tokio::test]
async fn test_telegram() {
    let env = get_env();

    let router = Router::new()
        .route(
            telegram::PATH,
            get(telegram::handler).post(telegram::handler),
        )
        .layer(env.into_layer());

    let data = TelegramParams {
        text: "test body".to_string(),
    };

    let body = serde_json::to_string(&data).unwrap();

    let req = Request::builder()
        .method(Method::POST)
        .uri(telegram::PATH)
        .header("Content-Type", "application/json")
        .body(body)
        .unwrap();

    let res = router.oneshot(req).await.unwrap();

    let status = res.status();
    let body = res.to_string().await.unwrap();

    println!("response body: {}", body);

    assert_eq!(status, StatusCode::OK);
}
