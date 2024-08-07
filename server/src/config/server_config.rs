use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct HTTPConfig {
    pub base_path: String,
    pub dist_path: String,
}

#[derive(Deserialize, Serialize)]
pub struct WebsocketConfig {
    pub base_path: String,
}

#[derive(Deserialize, Serialize)]
pub struct ProxyConfig {
    pub base_path: String,
    pub forward_to: String,
}

#[derive(Deserialize, Serialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u32,
    pub http: HTTPConfig,
    pub websocket: WebsocketConfig,
    pub proxy: ProxyConfig,
}
