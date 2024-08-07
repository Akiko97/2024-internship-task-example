use std::io::{Cursor, Read};
use byteorder::{ReadBytesExt, LittleEndian};

const HEAD_MAGIC: u32 = 0x46415445; // FATE
const TAIL_MAGIC: u32 = 0x4C4F4F4D; // LOOM

pub struct Packet {
    pub cmd_id: u16,
    size: u32,
    pub msg: Vec<u8>,
}

impl From<Vec<u8>> for Packet {
    fn from(value: Vec<u8>) -> Self {
        let mut cursor = Cursor::new(value);

        let head_magic = cursor.read_u32::<LittleEndian>()
            .expect("Fail to read head_magic");
        assert_eq!(head_magic, HEAD_MAGIC);

        let cmd_id = cursor.read_u16::<LittleEndian>()
            .expect("Fail to read cmd_id");

        let size = cursor.read_u32::<LittleEndian>()
            .expect("Fail to read size");

        let mut msg = vec![0; size as usize];
        cursor.read_exact(&mut msg).expect("Fail to read msg");

        let tail_magic = cursor.read_u32::<LittleEndian>()
            .expect("Fail to read tail_magic");
        assert_eq!(tail_magic, TAIL_MAGIC);

        Self {
            cmd_id,
            size,
            msg,
        }
    }
}

impl From<Packet> for Vec<u8> {
    fn from(value: Packet) -> Self {
        let mut out = Vec::new();

        out.extend(HEAD_MAGIC.to_le_bytes());
        out.extend(value.cmd_id.to_le_bytes());
        out.extend(value.size.to_le_bytes());
        out.extend(value.msg);
        out.extend(TAIL_MAGIC.to_le_bytes());

        out
    }
}

impl Packet {
    pub fn new(cmd_id: u16, msg: Vec<u8>) -> Self {
        Self {
            cmd_id,
            size: msg.len() as u32,
            msg,
        }
    }
}
