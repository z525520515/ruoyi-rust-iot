use serde::{Deserialize, Serialize};

/// 通道类型
#[repr(u8)]
#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
pub enum ChannelType {
    Unused = 0, // 这是公共接口，保证电路开路
    DI1 = 1,
    DI2 = 2,
    DI3 = 3,
    DI4 = 4,
    DI5 = 5,
    DI6 = 6,
    DI7 = 7,
    DI8 = 8,
    DI9 = 9,
    DI10 = 10,
    DI11 = 11,
    DI12 = 12,
    DI13 = 13,
    DI14 = 14,
    DI15 = 15,
    DI16 = 16,
    #[allow(non_camel_case_types)]
    DI1_DI16 = 17,
    AI1 = 18,
    AI2 = 19,
    AI3 = 20,
    AI4 = 21,
    AI5 = 22,
    AI6 = 23,
    AI7 = 24,
    AI8 = 25,
    DO1 = 26,
    DO2 = 27,
    DO3 = 28,
    DO4 = 29,
    RS485_1 = 30,
    RS485_2 = 31,
    RS232_1 = 32,
}

impl ChannelType {
    pub fn from_u8(value: u8) -> Self {
        assert!(value <= 32, "Invalid ChannelType value: {}", value);

        match value {
            0 => ChannelType::Unused,
            1 => ChannelType::DI1,
            2 => ChannelType::DI2,
            3 => ChannelType::DI3,
            4 => ChannelType::DI4,
            5 => ChannelType::DI5,
            6 => ChannelType::DI6,
            7 => ChannelType::DI7,
            8 => ChannelType::DI8,
            9 => ChannelType::DI9,
            10 => ChannelType::DI10,
            11 => ChannelType::DI11,
            12 => ChannelType::DI12,
            13 => ChannelType::DI13,
            14 => ChannelType::DI14,
            15 => ChannelType::DI15,
            16 => ChannelType::DI16,
            17 => ChannelType::DI1_DI16,
            18 => ChannelType::AI1,
            19 => ChannelType::AI2,
            20 => ChannelType::AI3,
            21 => ChannelType::AI4,
            22 => ChannelType::AI5,
            23 => ChannelType::AI6,
            24 => ChannelType::AI7,
            25 => ChannelType::AI8,
            26 => ChannelType::DO1,
            27 => ChannelType::DO2,
            28 => ChannelType::DO3,
            29 => ChannelType::DO4,
            30 => ChannelType::RS485_1,
            31 => ChannelType::RS485_2,
            32 => ChannelType::RS232_1,
            _ => unreachable!(),
        }
    }

    pub fn to_u8(&self) -> u8 {
        *self as u8
    }

    /// 是否是开关量端口
    pub fn is_distance_input(&self) -> bool {
        match self {
            ChannelType::DI1
            | ChannelType::DI2
            | ChannelType::DI3
            | ChannelType::DI4
            | ChannelType::DI5
            | ChannelType::DI6
            | ChannelType::DI7
            | ChannelType::DI8
            | ChannelType::DI9
            | ChannelType::DI10
            | ChannelType::DI11
            | ChannelType::DI12
            | ChannelType::DI13
            | ChannelType::DI14
            | ChannelType::DI15
            | ChannelType::DI16
            | ChannelType::DI1_DI16 => true,
            _ => false,
        }
    }

    /// 是否是模拟量端口
    pub fn is_analog_sensor(&self) -> bool {
        match self {
            ChannelType::AI1
            | ChannelType::AI2
            | ChannelType::AI3
            | ChannelType::AI4
            | ChannelType::AI5
            | ChannelType::AI6
            | ChannelType::AI7
            | ChannelType::AI8 => true,
            _ => false,
        }
    }

    /// 是否是rs485端口
    pub fn is_rs485_port(&self) -> bool {
        match self {
            ChannelType::RS485_1 | ChannelType::RS485_2 => true,
            _ => false,
        }
    }

    pub fn is_rs232_port(&self) -> bool {
        match self {
            ChannelType::RS232_1 => true,
            _ => false,
        }
    }

    pub fn is_digital_output(&self) -> bool {
        match self {
            ChannelType::DO1 | ChannelType::DO2 | ChannelType::DO3 | ChannelType::DO4 => true,
            _ => false,
        }
    }
}
