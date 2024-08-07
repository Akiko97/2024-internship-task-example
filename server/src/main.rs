mod config;
mod util;
mod services;
mod net;

use anyhow::Result;
use axum::{Router, ServiceExt};
use axum::body::Body;
use axum::extract::Request;
use tokio::net::TcpListener;
use tracing::Level;
use hyper_util::{client::legacy::connect::HttpConnector, rt::TokioExecutor};

type HttpClient = hyper_util::client::legacy::Client<HttpConnector, Body>;

#[derive(Clone)]
pub struct ServerContext {
    pub http_client: HttpClient,
    // database here
}

#[tokio::main]
async fn main() -> Result<()> {
    util::init_tracing();
    config::init_config();

    let span = tracing::span!(Level::DEBUG, "main");
    let _ = span.enter();

    let app = create_router();

    let http_client: HttpClient =
        hyper_util::client::legacy::Client::<(), ()>::builder(TokioExecutor::new())
            .build(HttpConnector::new());

    let app = app.with_state(ServerContext {
        http_client,
    });

    let addr = format!("0.0.0.0:{}", config::SERVER_CONFIG.port);
    let server = TcpListener::bind(&addr).await?;

    tracing::info!("Server is listening at {addr}");
    axum::serve(server, ServiceExt::<Request>::into_make_service(app)).await?;

    Ok(())
}

fn create_router() -> Router<ServerContext> {
    let mut router = Router::new();
    router = services::websocket::setup_routes(router);
    router = services::reverse_proxy::setup_routes(router);
    router = services::web::setup_routes(router);
    router
}
