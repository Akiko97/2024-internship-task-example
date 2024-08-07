mod handler_func;

use paste::paste;
use anyhow::Result;
use tracing::Instrument;
use proto::*;
use super::session::Session;
use super::packet::Packet;
use handler_func::*;

macro_rules! trait_handler {
    ($($name:ident, $cmd_id:ident;)*) => {
        pub trait SessionCommandHandler {
            $(
                paste! {
                    async fn [<on_$name:snake>](session: &mut Session, msg: &$name) -> Result<()> {
                        [<on_$name:snake>](session, msg).await
                    }
                }
            )*

            async fn on_message(session: &mut Session, packet: Packet) -> Result<()> {
                use ::prost::Message;

                let cmd_id = packet.cmd_id;
                let msg = packet.msg;
                match cmd_id {
                    $(
                        cmd_id:: $cmd_id => {
                            let msg = $name::decode(&mut &msg[..])?;
                            paste! {
                                Self::[<on_$name:snake>](session, &msg)
                                    .instrument(tracing::info_span!(stringify!([<on_$name:snake>]), cmd_id = cmd_id))
                                    .await
                            }
                        }
                    )*
                    _ => {
                        tracing::warn!("Unknown command id: {cmd_id}");
                        Ok(())
                    }
                }
            }
        }
    };
}

trait_handler! {
    HeartbeatMsg, HEARTBEAT_MSG;
    RandomNumberRequest, RANDOM_NUMBER_REQUEST;
    StopRandomNumberRequest, STOP_RANDOM_NUMBER_REQUEST;
    IncrementalSequenceRequest, INCREMENTAL_SEQUENCE_REQUEST;
    StopIncrementalSequenceRequest, STOP_INCREMENTAL_SEQUENCE_REQUEST;
    EchoRequest, ECHO_REQUEST;
}
