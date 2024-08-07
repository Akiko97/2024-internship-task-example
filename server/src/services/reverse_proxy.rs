use axum::{
    Router,
    routing::get,
    extract::{Request, State},
    http::uri::{PathAndQuery, Uri},
    response::{IntoResponse, Response},
};
use hyper::StatusCode;

use crate::ServerContext;
use crate::config::SERVER_CONFIG;

pub fn setup_routes(router: Router<ServerContext>) -> Router<ServerContext> {
    let base_path = SERVER_CONFIG.proxy.base_path.as_str();
    let path = if base_path.ends_with("/") {
        format!("{base_path}*path")
    } else {
        format!("{base_path}/*path")
    };
    router
        .route(path.as_str(), get(forward_to))
}

async fn forward_to(
    State(context): State<ServerContext>,
    mut req: Request,
) -> Result<Response, StatusCode> {
    let path = req.uri().path();
    let path_query = req
        .uri()
        .path_and_query()
        .map_or(path, PathAndQuery::as_str);
    let path_query = path_query
        .trim_start_matches(SERVER_CONFIG.proxy.base_path.as_str());

    let uri = format!("{}{}", SERVER_CONFIG.proxy.forward_to, path_query);

    *req.uri_mut() = Uri::try_from(uri).unwrap();

    Ok(context
        .http_client
        .request(req)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?
        .into_response())
}
