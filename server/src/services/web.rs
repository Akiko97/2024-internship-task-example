use axum::{
    Router,
    routing::get,
    response::{Redirect, Response},
    http::{StatusCode, Uri},
    extract::State
};
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use std::boxed::Box;
use crate::ServerContext;
use crate::config::SERVER_CONFIG;

pub fn setup_routes(router: Router<ServerContext>) -> Router<ServerContext> {
    let base_path = SERVER_CONFIG.http.base_path.as_str();
    let base_path_continue = if base_path.ends_with("/") {
        format!("{base_path}")
    } else {
        format!("{base_path}/")
    };
    let index_path = format!("{base_path_continue}index.html");
    let get_file_path = format!("{base_path_continue}*path");
    let index_path: &'static str = Box::leak(index_path.into_boxed_str());
    let get_file_path = get_file_path.as_str();

    router
        .route(base_path, get(|| async { Redirect::permanent(index_path) }))
        .route(get_file_path, get(get_file))
}

async fn get_file(
    State(_): State<ServerContext>,
    uri: Uri,
) -> Result<Response, StatusCode> {
    let path = uri.path()
        .trim_start_matches(SERVER_CONFIG.http.base_path.as_str())
        .trim_start_matches("/")
        .to_string();
    serve_file_by_path(path).await
}

async fn serve_file_by_path(path: String) -> Result<Response, StatusCode> {
    let dist_path = SERVER_CONFIG.http.dist_path.as_str();
    let file_path = format!("{dist_path}/{path}");

    let mut file = match File::open(&file_path).await {
        Ok(file) => file,
        Err(_) => return Err(StatusCode::NOT_FOUND),
    };

    let mut contents = Vec::new();
    if let Err(_) = file.read_to_end(&mut contents).await {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    let mime_type = mime_guess::from_path(&file_path).first_or_octet_stream();

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", mime_type.as_ref())
        .body(contents.into())
        .unwrap())
}
