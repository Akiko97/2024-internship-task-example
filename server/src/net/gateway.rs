use axum::{
    extract::ws::WebSocket
};
use crate::ServerContext;
use crate::net::session::Session;

pub async fn handle_socket(socket: WebSocket, _state: ServerContext) {
    let mut session = Session::new(socket);
    session.run().await;
}
