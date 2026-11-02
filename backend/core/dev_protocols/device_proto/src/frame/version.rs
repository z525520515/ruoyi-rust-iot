use serde::{Deserialize, Serialize};

/// 协议版本
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProtocolVersion(u8);

impl ProtocolVersion {
    pub fn from_u8(version: u8) -> Self {
        Self(version)
    }

    pub fn to_u8(&self) -> u8 {
        self.0
    }
}
