use crate::{
    constants::{HEADER_LENGTH_OFFSET, HEADER_SIZE, PACKET_MAX_SIZE, TYPE_ENC_INDEX, VERSION},
    error::ProtocolError,
};

#[derive(Debug, Clone, Copy)]
pub enum Encoding {
    JSON,
    String,
    Bytes,
}

#[derive(Debug, Clone, Copy)]
pub enum PacketType {
    Error,
    GameRequest,
    MovePiece,
    Resign,
    DrawOffer,
    DrawOfferAccept,
    DrawOfferReject,
    CloseConnection,
}

#[derive(Debug)]
pub struct Packet {
    data: Vec<u8>,
    len: usize,
}

impl Packet {
    pub fn encode(
        packet_type: PacketType,
        encoding: Encoding,
        payload: &[u8],
    ) -> Result<Self, ProtocolError> {
        if payload.len() > PACKET_MAX_SIZE - HEADER_SIZE {
            return Err(ProtocolError::PayloadTooLarge(format!(
                "Payload exceeds max size of {}",
                PACKET_MAX_SIZE - HEADER_SIZE
            )));
        }

        let mut data = Vec::with_capacity(HEADER_SIZE + payload.len());

        // Version
        data.push(VERSION);

        // Type and Encoding
        let type_and_encoding = ((encoding as u8) << 6) | (packet_type as u8);

        data.push(type_and_encoding);

        // Payload length
        data.extend(&(payload.len() as u16).to_be_bytes());

        // Payload
        data.extend_from_slice(payload);

        Ok(Packet {
            len: data.len(),
            data,
        })
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, ProtocolError> {
        if bytes.len() < HEADER_SIZE {
            return Err(ProtocolError::InvalidData("Data too short".to_string()));
        }

        if bytes[0] != VERSION {
            return Err(ProtocolError::VersionMismatch(format!(
                "Version mismatch: expected {}, got {}",
                VERSION, bytes[0]
            )));
        }

        let length =
            u16::from_be_bytes([bytes[HEADER_LENGTH_OFFSET], bytes[HEADER_LENGTH_OFFSET + 1]])
                as usize;

        if bytes.len() < HEADER_SIZE + length {
            return Err(ProtocolError::IncompletePacket(
                "Incomplete packet".to_string(),
            ));
        }

        Ok(Packet {
            data: bytes.to_vec(),
            len: HEADER_SIZE + length,
        })
    }

    pub fn payload(&self) -> &[u8] {
        &self.data[HEADER_SIZE..self.len]
    }

    pub fn packet_type(&self) -> PacketType {
        PacketType::from(self.data[TYPE_ENC_INDEX] & 0x3F)
    }

    pub fn encoding(&self) -> Encoding {
        Encoding::from(self.data[TYPE_ENC_INDEX] >> 6)
    }
}

pub struct PacketFramer {
    buffer: Vec<u8>,
}

impl PacketFramer {
    pub fn new() -> Self {
        Self {
            buffer: Vec::with_capacity(PACKET_MAX_SIZE),
        }
    }

    pub fn push(&mut self, data: &[u8]) -> Result<Option<Packet>, ProtocolError> {
        self.buffer.extend_from_slice(data);

        if self.buffer.len() >= HEADER_SIZE {
            let payload_length = u16::from_be_bytes([
                self.buffer[HEADER_LENGTH_OFFSET],
                self.buffer[HEADER_LENGTH_OFFSET + 1],
            ]) as usize;

            if self.buffer.len() >= HEADER_SIZE + payload_length {
                let packet = Packet::from_bytes(&self.buffer[..HEADER_SIZE + payload_length])?;
                self.buffer.drain(..HEADER_SIZE + payload_length);
                return Ok(Some(packet));
            }
        }

        Ok(None)
    }
}

impl From<u8> for PacketType {
    fn from(value: u8) -> Self {
        match value {
            0 => PacketType::Error,
            1 => PacketType::GameRequest,
            2 => PacketType::MovePiece,
            3 => PacketType::Resign,
            4 => PacketType::DrawOffer,
            5 => PacketType::DrawOfferAccept,
            6 => PacketType::DrawOfferReject,
            7 => PacketType::CloseConnection,
            _ => PacketType::Error,
        }
    }
}

impl From<u8> for Encoding {
    fn from(value: u8) -> Self {
        match value {
            0 => Encoding::JSON,
            1 => Encoding::String,
            _ => Encoding::Bytes,
        }
    }
}
