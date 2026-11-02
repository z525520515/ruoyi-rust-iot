use serde::{Deserialize, Serialize};

/// 帧总长度
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FrameLength(u16);

impl FrameLength {
    pub fn from_slices(bytes: &[u8]) -> Self {
        assert!(bytes.len() >= 2, "Insufficient bytes to read FrameLength");
        Self(u16::from_be_bytes(bytes[0..2].try_into().unwrap()))
    }

    pub fn to_bytes(&self) -> [u8; 2] {
        self.0.to_be_bytes()
    }

    pub fn len(&self) -> u16 {
        self.0
    }

    pub fn set_len(&mut self, len: u16) {
        self.0 = len;
    }

    pub fn new(len: u16) -> Self {
        Self(len)
    }
}
