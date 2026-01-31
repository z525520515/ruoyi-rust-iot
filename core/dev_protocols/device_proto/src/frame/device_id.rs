use serde::{Deserialize, Serialize};
use std::default::Default;

/// 设备编号
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeviceId([u8; 12]); // 12 字节字符串

impl DeviceId {
    pub fn from_slices(bytes: &[u8]) -> Self {
        assert!(bytes.len() <= 12, "Insufficient bytes to read DeviceId");
        let mut id = [0u8; 12];
        id[..bytes.len()].copy_from_slice(&bytes[0..bytes.len()]);

        Self(id)
    }

    pub fn to_bytes(&self) -> [u8; 12] {
        self.0
    }

    /// 不大于 12 字节的 ASCII 字符串
    pub fn new(id: &str) -> Self {
        if id.is_empty() {
            return Self::default();
        }

        assert!(id.len() <= 12, "DeviceId must be less than 12 bytes");
        let mut bytes = [0u8; 12];
        bytes[..id.len()].copy_from_slice(id.as_bytes());

        Self(bytes)
    }

    pub fn to_string(&self) -> String {
        String::from_utf8_lossy(&self.0).into_owned()
    }

    pub fn set_id(&mut self, id: &str) {
        assert!(id.len() <= 12, "DeviceId must be less than 12 bytes");
        self.0 = [0u8; 12];
        self.0[..id.len()].copy_from_slice(id.as_bytes());
    }
}

impl From<[u8; 12]> for DeviceId {
    fn from(value: [u8; 12]) -> Self {
        DeviceId::from_slices(&value)
    }
}
