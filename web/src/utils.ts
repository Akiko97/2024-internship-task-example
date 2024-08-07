import ByteBuffer from 'bytebuffer'

// Packet
const HEAD_MAGIC = 0x46415445; // FATE
const TAIL_MAGIC = 0x4C4F4F4D; // LOOM

export const encodePacket = (cmdID: number, msg: Uint8Array): ArrayBuffer => {
  const len = msg.length
  const buffer = new ByteBuffer(12 + len, true)
  buffer.order(ByteBuffer.LITTLE_ENDIAN)
  buffer.writeUint32(HEAD_MAGIC)
  buffer.writeUint16(cmdID)
  buffer.writeUint32(len)
  buffer.append(msg)
  buffer.writeUint32(TAIL_MAGIC)
  return buffer.buffer
}

export const decodePacket = (packet: ArrayBuffer): [number, Uint8Array] => {
  const buffer = ByteBuffer.wrap(packet, true)
  buffer.order(ByteBuffer.LITTLE_ENDIAN)
  const headMagic = buffer.readUint32()
  if (headMagic !== HEAD_MAGIC) {
    throw new Error('Invalid head magic')
  }
  const cmdID = buffer.readUint16()
  const len = buffer.readUint32()
  const msg = buffer.readBytes(len).toArrayBuffer()
  const tailMagic = buffer.readUint32()
  if (tailMagic !== TAIL_MAGIC) {
    throw new Error('Invalid tail magic')
  }
  return [cmdID, new Uint8Array(msg)]
}

// CMD ID
export const HEARTBEAT_MSG = 1;
export const RANDOM_NUMBER_REQUEST = 2;
export const STOP_RANDOM_NUMBER_REQUEST = 3;
export const INCREMENTAL_SEQUENCE_REQUEST = 4;
export const STOP_INCREMENTAL_SEQUENCE_REQUEST = 5;
export const ECHO_REQUEST = 6;
export const RANDOM_NUMBER_RESPONSE = 7;
export const STOP_RANDOM_NUMBER_RESPONSE = 8;
export const INCREMENTAL_SEQUENCE_RESPONSE = 9;
export const STOP_INCREMENTAL_SEQUENCE_RESPONSE = 10;
export const ECHO_RESPONSE = 11;
