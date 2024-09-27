include!("../out/msg.rs");

pub trait CmdID {
    const CMD_ID: u16;

    fn get_cmd_id(&self) -> u16 {
        Self::CMD_ID
    }
}

pub trait Msg: prost::Message + CmdID {}
impl<T: prost::Message + CmdID> Msg for T {}
