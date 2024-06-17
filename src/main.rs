/*
:project: omni-notify
:author: L-ING
:copyright: (C) 2024 L-ING <hlf01@icloud.com>
:license: MIT, see LICENSE for more details.
*/

mod env;
mod error;
mod handlers;

use axum::routing::get;
use handlers::{email, pushdeer, telegram};

use axum::body::Body;
use axum::extract::Request;
use axum::middleware::{self, Next};
use axum::response::Response;
use axum::Router;
use env::Env;
use tokio::time::Instant;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;

async fn trace_middleware(req: Request<Body>, next: Next) -> Response {
    let method = req.method().clone();
    let uri = req.uri().clone();
    let version = req.version();

    let user_agent = req
        .headers()
        .get(axum::http::header::USER_AGENT)
        .and_then(|value| value.to_str().ok())
        .unwrap_or("-")
        .to_string();

    let start = Instant::now();
    let response = next.run(req).await;
    let latency = start.elapsed();

    tracing::info!(
        "method={}, uri={}, version={:?}, latency={:?}, status={}, user-agent={}",
        method,
        uri,
        version,
        latency,
        response.status(),
        user_agent
    );

    response
}

#[tokio::main]
async fn main() {
    let env = Env::new();

    tracing_subscriber::registry()
        .with(EnvFilter::new("info"))
        .with(tracing_subscriber::fmt::layer())
        .init();

    const HOST: &str = "0.0.0.0";

    tracing::info!("listening on http://{}:{}", &HOST, env.port);

    let env = Env::new();

    let router = Router::new()
        .route(email::PATH, get(email::handler).post(email::handler))
        .route(
            pushdeer::PATH,
            get(pushdeer::handler).post(pushdeer::handler),
        )
        .route(
            telegram::PATH,
            get(telegram::handler).post(telegram::handler),
        )
        .layer(middleware::from_fn(trace_middleware))
        .layer(env.clone().into_layer());

    let listener = tokio::net::TcpListener::bind(("0.0.0.0", env.port))
        .await
        .unwrap();

    axum::serve(listener, router).await.unwrap();
}
