use std::fmt::Display;

#[derive(Debug)]
pub enum ProtocolError {
    PayloadTooLarge(String),
    VersionMismatch(String),
    IncompletePacket(String),
    InvalidData(String),
}

impl Display for ProtocolError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for ProtocolError {}
