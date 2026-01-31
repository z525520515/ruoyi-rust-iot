use serde::{Deserialize, Serialize};

/// 电量状态
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PowerStatus {
    // 电池电量
    pub level: BatteryLevel,
    // 供电方式
    pub source: PowerSource,
    // 当前供电来源
    pub is_external_power: bool,
}

impl PowerStatus {
    pub fn from_u8(value: u8) -> Self {
        let level = BatteryLevel::from_byte(value);
        let source = PowerSource::from_byte(value);
        let is_external_power = if (value & 0b1000_0000) == 128 {
            true
        } else {
            false
        };

        PowerStatus {
            level,
            source,
            is_external_power,
        }
    }

    pub fn to_u8(&self) -> u8 {
        let mut value = 0u8;
        value |= self.level.to_byte();
        value |= self.source.to_byte();
        value |= if self.is_external_power { 0x80 } else { 0x00 };
        value
    }
}

// 供电方式
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PowerSource {
    // 电池
    Battery,
    // 外源
    External,
    /// 双供电
    Double,
    /// 保留
    Reserved,
}

impl PowerSource {
    const DOUBLE_MASK: u8 = 0b0000_0000;
    const BATTERY_MASK: u8 = 0b0010_0000;
    const EXTERNAL_MASK: u8 = 0b0100_0000;
    const RESERVED_MASK: u8 = 0b0110_0000;
    const POWER_SOURCE_MASK: u8 = 0b0110_0000;
    pub fn from_byte(byte: u8) -> Self {
        match byte & Self::POWER_SOURCE_MASK {
            Self::DOUBLE_MASK => PowerSource::Double,
            Self::BATTERY_MASK => PowerSource::Battery,
            Self::EXTERNAL_MASK => PowerSource::External,
            Self::RESERVED_MASK => PowerSource::Reserved,
            _ => unreachable!(),
        }
    }

    pub fn to_byte(&self) -> u8 {
        match self {
            PowerSource::Double => Self::DOUBLE_MASK,
            PowerSource::Battery => Self::BATTERY_MASK,
            PowerSource::External => Self::EXTERNAL_MASK,
            PowerSource::Reserved => Self::RESERVED_MASK,
        }
    }

    /// 创建不同状态的 PowerSource
    pub fn new_double() -> Self {
        PowerSource::Double
    }

    pub fn new_battery() -> Self {
        PowerSource::Battery
    }

    pub fn new_external() -> Self {
        PowerSource::External
    }

    pub fn new_reserved() -> Self {
        PowerSource::Reserved
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BatteryLevel(u8);

impl BatteryLevel {
    pub fn new(level: u8) -> Self {
        assert!(level <= 100, "Battery level must be between 0 and 100");
        Self(level)
    }

    pub fn get_level(&self) -> u8 {
        self.0
    }

    pub fn to_byte(&self) -> u8 {
        let rounded = ((self.0 + 2) / 5).min(20);
        rounded & 0b0001_1111
    }

    pub fn from_byte(byte: u8) -> Self {
        let level = (byte & 0b0001_1111) * 5;
        Self(level)
    }
}
