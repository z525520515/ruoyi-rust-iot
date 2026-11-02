use serde::{Deserialize, Serialize};

/// CRC16 校验，起始值为 0xFFFF，多项式为 8005，MODBUS 标准
const CRC16_POLY: u16 = 0xA001;
const CRC16_INIT: u16 = 0xFFFF;

pub fn crc16(data: &[u8]) -> u16 {
    let mut crc = CRC16_INIT;
    for &byte in data {
        crc ^= byte as u16;
        for _ in 0..8 {
            if (crc & 0x0001) != 0 {
                crc = (crc >> 1) ^ CRC16_POLY;
            } else {
                crc >>= 1;
            }
        }
    }
    crc & 0xFFFF
}

/// 16 通道开关量采集终端
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChannelList16([bool; 16]);

impl ChannelList16 {
    pub fn new() -> Self {
        ChannelList16([false; 16])
    }

    pub fn set(&mut self, index: usize, is_open: bool) {
        assert!(index < 16, "Index out of bounds for DIChannelList16");
        self.0[index] = is_open;
    }

    pub fn get(&self, index: usize) -> bool {
        assert!(index < 16, "Index out of bounds for DIChannelList16");
        self.0[index]
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        assert!(
            bytes.len() >= 2,
            "Bytes length must be at least 2 for ChannelList16"
        );

        ChannelList16::from_u16(u16::from_be_bytes(bytes[..2].try_into().unwrap()))
    }

    pub fn from_u16(value: u16) -> Self {
        let channels = std::array::from_fn(|i| (value & (1 << i)) != 0);

        ChannelList16(channels)
    }

    pub fn to_u16(&self) -> u16 {
        let mut value = 0;
        self.0.iter().enumerate().for_each(|(i, &is_open)| {
            if is_open {
                value |= 1 << i;
            }
        });

        value
    }

    pub fn to_bytes(&self) -> [u8; 2] {
        self.to_u16().to_be_bytes()
    }
}

impl std::ops::Index<usize> for ChannelList16 {
    type Output = bool;
    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < 16, "Index out of bounds for ChannelList16");
        &self.0[index]
    }
}

impl std::ops::IndexMut<usize> for ChannelList16 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index < 16, "Index out of bounds for ChannelList16");
        &mut self.0[index]
    }
}

/// 32 通道开关量采集终端
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChannelList32((ChannelList16, ChannelList16));

impl ChannelList32 {
    pub fn new() -> Self {
        ChannelList32((ChannelList16::new(), ChannelList16::new()))
    }

    pub fn set(&mut self, index: usize, is_open: bool) {
        assert!(index < 32, "Index out of bounds for ChannelList32");
        if index < 16 {
            self.0.0[index] = is_open;
        } else {
            self.0.1[index - 16] = is_open;
        }
    }

    pub fn get(&self, index: usize) -> bool {
        assert!(index < 32, "Index out of bounds for ChannelList32");
        if index < 16 {
            self.0.0[index]
        } else {
            self.0.1[index - 16]
        }
    }

    pub fn from_u32(value: u32) -> Self {
        let channels: [bool; 32] = std::array::from_fn(|i| (value & (1 << i)) != 0);

        ChannelList32((
            ChannelList16(channels[0..16].to_vec().try_into().unwrap()),
            ChannelList16(channels[16..32].to_vec().try_into().unwrap()),
        ))
    }

    pub fn to_u32(&self) -> u32 {
        let mut value1: u16 = 0;

        self.0.0.0.iter().enumerate().for_each(|(i, &is_open)| {
            if is_open {
                value1 |= 1 << i;
            }
        });

        let mut value2: u16 = 0;
        self.0.1.0.iter().enumerate().for_each(|(i, &is_open)| {
            if is_open {
                value2 |= 1 << i;
            }
        });

        ((value2 as u32) << 16) | (value1 as u32)
    }

    pub fn to_bytes(&self) -> [u8; 4] {
        self.to_u32().to_be_bytes()
    }
}

impl std::ops::Index<usize> for ChannelList32 {
    type Output = bool;
    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < 32, "Index out of bounds for ChannelList32");
        if index < 16 {
            &self.0.0[index]
        } else {
            &self.0.1[index - 16]
        }
    }
}

impl std::ops::IndexMut<usize> for ChannelList32 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index < 32, "Index out of bounds for ChannelList32");
        if index < 16 {
            &mut self.0.0[index]
        } else {
            &mut self.0.1[index - 16]
        }
    }
}
