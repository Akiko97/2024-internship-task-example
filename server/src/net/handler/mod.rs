mod handler_func;

use paste::paste;
use anyhow::Result;
use tracing::Instrument;
use proto::*;
use super::session::Session;
use super::packet::Packet;

macro_rules! packet_handler {
    ($($category:ident::$name:ident;)*) => {
        pub trait SessionCommandHandler {
            async fn on_message(session: &mut Session, packet: Packet) -> Result<()> {
                use ::prost::Message;

                let cmd_id = packet.cmd_id;
                let msg = packet.msg;
                match cmd_id {
                    $(
                        ::proto::$name::CMD_ID => {
                            let msg = $name::decode(&mut &msg[..])?;
                            paste! {
                                crate::net::handler::[<$category:snake>]::[<on_$name:snake>](session, &msg)
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

packet_handler! {
    HandlerFunc::HeartbeatMsg;
    HandlerFunc::RandomNumberRequest;
    HandlerFunc::StopRandomNumberRequest;
    HandlerFunc::IncrementalSequenceRequest;
    HandlerFunc::StopIncrementalSequenceRequest;
    HandlerFunc::EchoRequest;
}
