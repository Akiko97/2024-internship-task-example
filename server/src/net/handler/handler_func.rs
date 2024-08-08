use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use anyhow::Result;
use chrono::prelude::*;
use rand::Rng;
use crate::net::session::Session;
use proto::*;

pub async fn on_heartbeat_msg(
    session: &mut Session,
    msg: &HeartbeatMsg
) -> Result<()> {
    let now = Utc::now();
    let timestamp = now.timestamp_millis();
    let rsp = HeartbeatMsg {
        id: msg.id.clone(),
        timestamp,
    };
    session.send(cmd_id::HEARTBEAT_MSG, rsp).await
}

pub async fn on_random_number_request(
    session: &mut Session,
    msg: &RandomNumberRequest
) -> Result<()> {
    let id = msg.id.clone();
    if session.include_task(&id).await {
        let rsp = RandomNumberResponse {
            id: msg.id.clone(),
            status: Some(Status {
                code: StatusCode::InvalidRequest as i32,
                message: format!("Already running task with id {id}"),
            }),
            ..Default::default()
        };
        session.send(cmd_id::RANDOM_NUMBER_RESPONSE, rsp).await
    } else {
        let task = Arc::new(AtomicBool::new(true));
        session.add_task(&id, task.clone()).await;
        let interval = msg.interval as u64;
        let min = msg.min;
        let max = msg.max;
        let mut session = session.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(interval));
            while task.load(Ordering::Relaxed) {
                interval.tick().await;
                let random_number = rand::thread_rng().gen_range(min..=max);
                let rsp = RandomNumberResponse {
                    id: id.clone(),
                    number: random_number,
                    ..Default::default()
                };
                session.send(cmd_id::RANDOM_NUMBER_RESPONSE, rsp).await
                    .expect("Error in sending response");
            }
            session.remove_task(&id).await;
        });
        Ok(())
    }
}

pub async fn on_stop_random_number_request(
    session: &mut Session,
    msg: &StopRandomNumberRequest
) -> Result<()> {
    let id = msg.id.clone();
    if session.include_task(&id).await {
        session.store_task(&id, false).await;
        let rsp = StopRandomNumberResponse {
            id: msg.id.clone(),
            status: Some(Status {
                code: StatusCode::Success as i32,
                message: "Success".to_string(),
            }),
            ..Default::default()
        };
        session.send(cmd_id::STOP_RANDOM_NUMBER_RESPONSE, rsp).await
    } else {
        let rsp = StopRandomNumberResponse {
            id: msg.id.clone(),
            status: Some(Status {
                code: StatusCode::InvalidRequest as i32,
                message: format!("Get invalid id {id}"),
            }),
            ..Default::default()
        };
        session.send(cmd_id::STOP_RANDOM_NUMBER_RESPONSE, rsp).await
    }
}

pub async fn on_incremental_sequence_request(
    session: &mut Session,
    msg: &IncrementalSequenceRequest
) -> Result<()> {
    let id = msg.id.clone();
    if session.include_task(&id).await {
        let rsp = IncrementalSequenceResponse {
            id: msg.id.clone(),
            status: Some(Status {
                code: StatusCode::InvalidRequest as i32,
                message: format!("Already running task with id {id}"),
            }),
            ..Default::default()
        };
        session.send(cmd_id::INCREMENTAL_SEQUENCE_RESPONSE, rsp).await
    } else {
        let task = Arc::new(AtomicBool::new(true));
        session.add_task(&id, task.clone()).await;
        let interval = msg.interval as u64;
        let start = msg.start;
        let end = msg.end;
        let mut session = session.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(interval));
            let mut num = start;
            while task.load(Ordering::Relaxed) && num <= end {
                interval.tick().await;
                let rsp = IncrementalSequenceResponse {
                    id: id.clone(),
                    number: num,
                    ..Default::default()
                };
                session.send(cmd_id::INCREMENTAL_SEQUENCE_RESPONSE, rsp).await
                    .expect("Error in sending response");
                num += 1;
            }
            session.remove_task(&id).await;
        });
        Ok(())
    }
}

pub async fn on_stop_incremental_sequence_request(
    session: &mut Session,
    msg: &StopIncrementalSequenceRequest
) -> Result<()> {
    let id = msg.id.clone();
    if session.include_task(&id).await {
        session.store_task(&id, false).await;
        let rsp = StopIncrementalSequenceResponse {
            id: msg.id.clone(),
            status: Some(Status {
                code: StatusCode::Success as i32,
                message: "Success".to_string(),
            }),
            ..Default::default()
        };
        session.send(cmd_id::STOP_INCREMENTAL_SEQUENCE_RESPONSE, rsp).await
    } else {
        let rsp = StopIncrementalSequenceResponse {
            id: msg.id.clone(),
            status: Some(Status {
                code: StatusCode::InvalidRequest as i32,
                message: format!("Get invalid id {id}"),
            }),
            ..Default::default()
        };
        session.send(cmd_id::STOP_INCREMENTAL_SEQUENCE_RESPONSE, rsp).await
    }
}

pub async fn on_echo_request(
    session: &mut Session,
    msg: &EchoRequest
) -> Result<()> {
    let rsp = EchoResponse {
        message: msg.message.clone(),
        ..Default::default()
    };
    session.send(cmd_id::ECHO_RESPONSE, rsp).await
}
