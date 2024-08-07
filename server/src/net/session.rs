use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::sync::Mutex;
use axum::extract::ws::{Message, WebSocket};
use prost::Message as protoMessage;
use anyhow::Result;
use tokio::time::sleep;
use std::time::Duration;
use tokio::select;
use crate::net::packet::Packet;
use crate::net::handler::SessionCommandHandler;

#[derive(Clone)]
pub struct Session {
    socket: Arc<Mutex<WebSocket>>,
    tasks: Arc<Mutex<HashMap<String, Arc<AtomicBool>>>>,
}

impl Session {
    pub fn new(socket: WebSocket) -> Self {
        Self {
            socket: Arc::new(Mutex::new(socket)),
            tasks: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn run(&mut self) {
        loop {
            let msg = loop {
                let msg_opt = {
                    let mut socket = self.socket.lock().await;
                    select! {
                        msg = socket.recv() => msg,
                        _ = sleep(Duration::from_millis(100)) => None,
                    }
                };
                if let Some(msg) = msg_opt {
                    break msg;
                }
            };

            match msg {
                Ok(Message::Text(text_msg)) => {
                    let msg = Message::Text(text_msg);
                    if self.socket.lock().await.send(msg).await.is_err() {
                        // client disconnected
                        return;
                    }
                }
                Ok(Message::Binary(bin_msg)) => {
                    let packet = Packet::from(bin_msg);
                    Self::on_message(self, packet).await
                        .expect("Error in handling packet received");
                }
                Ok(Message::Ping(_ping_msg)) => {
                    continue;
                }
                Ok(Message::Pong(_pong_msg)) => {
                    continue;
                }
                Ok(_) => {
                    continue;
                }
                Err(_) => {
                    // client disconnected
                    return;
                }
            };
        }
    }

    pub async fn send(&mut self, cmd_id: u16, msg: impl protoMessage) -> Result<()> {
        let packet = Packet::new(cmd_id, msg.encode_to_vec());
        let msg = Message::Binary(Vec::<u8>::from(packet));
        let mut socket = self.socket.lock().await;
        socket.send(msg).await?;
        Ok(())
    }

    pub async fn include_task(&self, id: &String) -> bool {
        let tasks = self.tasks.lock().await;
        tasks.contains_key(id)
    }

    pub async fn add_task(&mut self, id: &String, task: Arc<AtomicBool>) {
        let mut tasks = self.tasks.lock().await;
        tasks.insert(id.clone(), task);
    }

    pub async fn remove_task(&mut self, id: &String) {
        let mut tasks = self.tasks.lock().await;
        tasks.remove(id);
    }

    pub async fn store_task(&mut self, id: &String, value: bool) {
        let tasks = self.tasks.lock().await;
        let task = tasks.get(id).unwrap();
        task.store(value, Ordering::Relaxed);
    }

    pub fn get_socket(&mut self) -> Arc<Mutex<WebSocket>> {
        self.socket.clone()
    }

    pub fn get_tasks(&mut self) -> Arc<Mutex<HashMap<String, Arc<AtomicBool>>>> {
        self.tasks.clone()
    }
}

impl SessionCommandHandler for Session {}
