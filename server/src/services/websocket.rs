use axum::{
    extract::{ws::WebSocketUpgrade, State},
    response::Response,
    routing::get,
    Router,
};
use crate::config::SERVER_CONFIG;
use crate::ServerContext;
use crate::net::gateway::handle_socket;

pub fn setup_routes(router: Router<ServerContext>) -> Router<ServerContext> {
    router
        .route(SERVER_CONFIG.websocket.base_path.as_str(), get(websocket_handler))
}

async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<ServerContext>
) -> Response {
    ws.on_upgrade(| socket | handle_socket(socket, state))
}
