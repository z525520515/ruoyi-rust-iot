pub mod alarm_parameters;
pub mod alarm_status;
pub mod channel_data;
pub mod channel_type;
pub mod device_config;
pub mod device_data;
pub mod device_id;
pub mod device_type;
pub mod error;
pub mod frame_length;
pub mod parameter_setting_unit;
pub mod parser;
pub mod power_status;
pub mod timestamp;
pub mod unit_data;
pub mod version;

pub use alarm_parameters::AlarmParameters;
pub use alarm_status::AlarmStatus;
pub use channel_data::{AIInputData, ChannelDataType, DIInputData, DOOutputData, RSPortData};
pub use channel_type::ChannelType;
pub use device_config::{
    DataLength, DataType, DeviceTypeInfo, ParameterValue, SingleParameterConfig,
};
pub use device_data::{DeviceData, ModbusData, Parameter};
pub use device_id::DeviceId;
pub use device_type::DeviceType;
pub use error::{ParseError, ParseResult};
pub use frame_length::FrameLength;
pub use parameter_setting_unit::{ContactPoint, ParamSetUnitSize, ParameterSet, SettingParameter};
pub use parser::DeviceDataParser;
pub use power_status::{BatteryLevel, PowerSource, PowerStatus};
pub use timestamp::Timestamp;
pub use unit_data::{
    SwitchType, UNIT_LENGTH_MAX, UNIT_VALUE_MAP, UnitContent, UnitLength, UnitParseResult,
};
pub use version::ProtocolVersion;

use serde::{Deserialize, Serialize};

/// 信号强度
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SignalStrength(i8);

impl SignalStrength {
    pub fn from_dbm(dbm: i8) -> Self {
        SignalStrength(dbm)
    }

    pub fn from_u8(value: u8) -> Self {
        SignalStrength(value as i8)
    }

    pub fn to_byte(&self) -> u8 {
        self.0 as u8
    }

    pub fn get_dbm(&self) -> i8 {
        self.0
    }

    pub fn get_unit(&self) -> &'static str {
        "dBm"
    }
}

/// 通道号
/// 对应的网关的数据通道号ID，支持设置 32 个通道
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChannelId(u8);

impl ChannelId {
    pub fn from_u8(id: u8) -> Self {
        assert!(id <= 31, "ChannelId must be between 0 and 31");

        ChannelId(id)
    }

    pub fn to_u8(&self) -> u8 {
        self.0
    }
}

/// 通道数据长度
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChannelDataLength(u16);

impl ChannelDataLength {
    pub fn from_u16(length: u16) -> Self {
        assert!(length > 0, "ChannelDataLength must be greater than 0");
        ChannelDataLength(length)
    }

    pub fn to_u16(&self) -> u16 {
        self.0
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        assert!(
            bytes.len() >= 2,
            "ChannelDataLength must be at least 2 bytes"
        );
        let length = u16::from_be_bytes([bytes[0], bytes[1]]);
        ChannelDataLength(length)
    }

    pub fn to_bytes(&self) -> [u8; 2] {
        self.0.to_be_bytes()
    }
}
