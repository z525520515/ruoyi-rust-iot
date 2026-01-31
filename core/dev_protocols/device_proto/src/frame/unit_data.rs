// use crate::frame::ChannelDataLength;

use super::ChannelType;
use super::error::ParseError;
use fixedbitset::FixedBitSet;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

/// 采集数据单位的长度
pub type UnitLength = u8; // 0 ~ 16
pub const UNIT_LENGTH_MAX: UnitLength = 16;

/// 单位置解析结果
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UnitParseResult {
    //字符串
    Msg(String),
    // 开关量
    CaseList(Vec<String>),
}

pub static UNIT_VALUE_MAP: Lazy<FixedBitSet> = Lazy::new(|| {
    let mut set = FixedBitSet::with_capacity(256);
    set.set_range(0..72, true);
    set.insert(255);
    set
});

/// 单位内容
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnitContent {
    // 原数据
    pub(crate) unit_value: UnitValue,
    // 解析内容
    #[allow(unused)]
    unit_parse_result: Option<UnitParseResult>,
}

impl UnitContent {
    // pub fn new(unit_value: UnitValue) -> Self {
    //     UnitContent {
    //         unit_value,
    //         unit_parse_result: None,
    //     }
    // }

    pub fn from_bytes(bytes: &[u8], channel_type: ChannelType) -> Result<Self, ParseError> {
        let unit_value = UnitValue::new()
            .set_channel_type(channel_type)
            .copy_from_bytes(bytes)
            .build();

        Ok(UnitContent {
            unit_value: unit_value,
            unit_parse_result: None,
        })
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.unit_value.to_vec()
    }

    pub fn get_res(&mut self) -> Result<UnitParseResult, ParseError> {
        if self.unit_parse_result.is_none() {
            self.unit_parse_result = Some(self.unit_value.parse_data()?);
        }
        Ok(self.unit_parse_result.as_ref().unwrap().clone())
    }
}

/// 单位值
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnitValue(Vec<u8>, Option<ChannelType>);

impl UnitValue {
    pub fn new() -> Self {
        UnitValue(vec![], Option::<ChannelType>::None)
    }

    pub fn set_channel_type(mut self, channel_type: ChannelType) -> Self {
        self.1 = Some(channel_type);
        self
    }

    pub fn copy_from_bytes(mut self, bytes: &[u8]) -> Self {
        self.0 = bytes.to_vec();
        self
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.0.clone()
    }

    pub fn to_bytes(&self) -> &[u8] {
        &self.0
    }

    pub fn build(self) -> Self {
        self
    }
    // 根据通道类型解析数据，返回解析结果
    pub fn parse_data(&self) -> Result<UnitParseResult, ParseError> {
        if self.1.is_none() {
            return Err(ParseError::Uninitialized {
                field: "UnitValue::ChannelType",
                reason: "ChannelType is not initialized".to_string(),
            });
        }

        if self.1.unwrap().is_digital_output() || self.1.unwrap().is_distance_input() {
            if self.0.is_empty() {
                return Ok(UnitParseResult::CaseList(vec![]));
            }

            let mut cases = Vec::with_capacity(self.0.len());

            self.0.iter().for_each(|&value| {
                if let Some(res) = Self::match_switch_type(value) {
                    cases.push(res.to_string());
                }
            });

            Ok(UnitParseResult::CaseList(cases))
        } else {
            // 将字节序列转换成 ASCII 字符串
            let ascii_str = String::from_utf8(self.0.clone())
                .map_err(|e| ParseError::StringParseError(e.to_string()))?;

            Ok(UnitParseResult::Msg(ascii_str))
        }
    }

    // 对比每个byte值和SwitchType，返回匹配的SwitchType
    fn match_switch_type(value: u8) -> Option<&'static str> {
        if !UNIT_VALUE_MAP.contains(value as usize) || value == 0 {
            return None;
        }

        Some(
            SwitchType::from_u8(value)
                .expect("前面的逻辑已经保证了SwitchType的合法性")
                .description(),
        )
    }
}

/// 开关量类型定义
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SwitchType {
    /// 无定义
    Undefined = 0,
    /// 主电电源
    MainPower = 1,
    /// 备用电源
    BackupPower = 2,
    /// 喷淋泵故障
    SprinklerPumpFault = 3,
    /// 喷淋泵1故障
    SprinklerPump1Fault = 4,
    /// 喷淋泵2故障
    SprinklerPump2Fault = 5,
    /// 喷淋泵3故障
    SprinklerPump3Fault = 6,
    /// 消防泵故障
    FirePumpFault = 7,
    /// 消防泵1故障
    FirePump1Fault = 8,
    /// 消防泵2故障
    FirePump2Fault = 9,
    /// 消防泵3故障
    FirePump3Fault = 10,
    /// 稳压泵故障
    PressurePumpFault = 11,
    /// 稳压泵1故障
    PressurePump1Fault = 12,
    /// 稳压泵2故障
    PressurePump2Fault = 13,
    /// 稳压泵3故障
    PressurePump3Fault = 14,
    /// 喷淋泵运行
    SprinklerPumpRunning = 15,
    /// 喷淋泵1运行
    SprinklerPump1Running = 16,
    /// 喷淋泵2运行
    SprinklerPump2Running = 17,
    /// 喷淋泵3运行
    SprinklerPump3Running = 18,
    /// 消防泵运行
    FirePumpRunning = 19,
    /// 消防泵1运行
    FirePump1Running = 20,
    /// 消防泵2运行
    FirePump2Running = 21,
    /// 消防泵3运行
    FirePump3Running = 22,
    /// 稳压泵运行
    PressurePumpRunning = 23,
    /// 稳压泵1运行
    PressurePump1Running = 24,
    /// 稳压泵2运行
    PressurePump2Running = 25,
    /// 稳压泵3运行
    PressurePump3Running = 26,
    /// 喷淋泵手动
    SprinklerPumpManual = 27,
    /// 喷淋泵自动
    SprinklerPumpAuto = 28,
    /// 喷淋泵1手动
    SprinklerPump1Manual = 29,
    /// 喷淋泵1自动
    SprinklerPump1Auto = 30,
    /// 喷淋泵2手动
    SprinklerPump2Manual = 31,
    /// 喷淋泵2自动
    SprinklerPump2Auto = 32,
    /// 喷淋泵3手动
    SprinklerPump3Manual = 33,
    /// 喷淋泵3自动
    SprinklerPump3Auto = 34,
    /// 消防泵手动
    FirePumpManual = 35,
    /// 消防泵自动
    FirePumpAuto = 36,
    /// 消防泵1手动
    FirePump1Manual = 37,
    /// 消防泵1自动
    FirePump1Auto = 38,
    /// 消防泵2手动
    FirePump2Manual = 39,
    /// 消防泵2自动
    FirePump2Auto = 40,
    /// 消防泵3手动
    FirePump3Manual = 41,
    /// 消防泵3自动
    FirePump3Auto = 42,
    /// 稳压泵手动
    PressurePumpManual = 43,
    /// 稳压泵自动
    PressurePumpAuto = 44,
    /// 稳压泵1手动
    PressurePump1Manual = 45,
    /// 稳压泵1自动
    PressurePump1Auto = 46,
    /// 稳压泵2手动
    PressurePump2Manual = 47,
    /// 稳压泵2自动
    PressurePump2Auto = 48,
    /// 稳压泵3手动
    PressurePump3Manual = 49,
    /// 稳压泵3自动
    PressurePump3Auto = 50,
    /// 喷淋泵停止
    SprinklerPumpStop = 51,
    /// 喷淋泵1停止
    SprinklerPump1Stop = 52,
    /// 喷淋泵2停止
    SprinklerPump2Stop = 53,
    /// 喷淋泵3停止
    SprinklerPump3Stop = 54,
    /// 消防泵停止
    FirePumpStop = 55,
    /// 消防泵1停止
    FirePump1Stop = 56,
    /// 消防泵2停止
    FirePump2Stop = 57,
    /// 消防泵3停止
    FirePump3Stop = 58,
    /// 稳压泵停止
    PressurePumpStop = 59,
    /// 稳压泵1停止
    PressurePump1Stop = 60,
    /// 稳压泵2停止
    PressurePump2Stop = 61,
    /// 稳压泵3停止
    PressurePump3Stop = 62,
    /// 喷淋泵1电源
    SprinklerPump1Power = 63,
    /// 喷淋泵2电源
    SprinklerPump2Power = 64,
    /// 喷淋泵3电源
    SprinklerPump3Power = 65,
    /// 消防泵1电源
    FirePump1Power = 66,
    /// 消防泵2电源
    FirePump2Power = 67,
    /// 消防泵3电源
    FirePump3Power = 68,
    /// 稳压泵1电源
    PressurePump1Power = 69,
    /// 稳压泵2电源
    PressurePump2Power = 70,
    /// 稳压泵3电源
    PressurePump3Power = 71,
    /// 其它
    Other = 255,
}

impl SwitchType {
    /// 从u8值解析开关量类型
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Undefined),
            1 => Some(Self::MainPower),
            2 => Some(Self::BackupPower),
            3 => Some(Self::SprinklerPumpFault),
            4 => Some(Self::SprinklerPump1Fault),
            5 => Some(Self::SprinklerPump2Fault),
            6 => Some(Self::SprinklerPump3Fault),
            7 => Some(Self::FirePumpFault),
            8 => Some(Self::FirePump1Fault),
            9 => Some(Self::FirePump2Fault),
            10 => Some(Self::FirePump3Fault),
            11 => Some(Self::PressurePumpFault),
            12 => Some(Self::PressurePump1Fault),
            13 => Some(Self::PressurePump2Fault),
            14 => Some(Self::PressurePump3Fault),
            15 => Some(Self::SprinklerPumpRunning),
            16 => Some(Self::SprinklerPump1Running),
            17 => Some(Self::SprinklerPump2Running),
            18 => Some(Self::SprinklerPump3Running),
            19 => Some(Self::FirePumpRunning),
            20 => Some(Self::FirePump1Running),
            21 => Some(Self::FirePump2Running),
            22 => Some(Self::FirePump3Running),
            23 => Some(Self::PressurePumpRunning),
            24 => Some(Self::PressurePump1Running),
            25 => Some(Self::PressurePump2Running),
            26 => Some(Self::PressurePump3Running),
            27 => Some(Self::SprinklerPumpManual),
            28 => Some(Self::SprinklerPumpAuto),
            29 => Some(Self::SprinklerPump1Manual),
            30 => Some(Self::SprinklerPump1Auto),
            31 => Some(Self::SprinklerPump2Manual),
            32 => Some(Self::SprinklerPump2Auto),
            33 => Some(Self::SprinklerPump3Manual),
            34 => Some(Self::SprinklerPump3Auto),
            35 => Some(Self::FirePumpManual),
            36 => Some(Self::FirePumpAuto),
            37 => Some(Self::FirePump1Manual),
            38 => Some(Self::FirePump1Auto),
            39 => Some(Self::FirePump2Manual),
            40 => Some(Self::FirePump2Auto),
            41 => Some(Self::FirePump3Manual),
            42 => Some(Self::FirePump3Auto),
            43 => Some(Self::PressurePumpManual),
            44 => Some(Self::PressurePumpAuto),
            45 => Some(Self::PressurePump1Manual),
            46 => Some(Self::PressurePump1Auto),
            47 => Some(Self::PressurePump2Manual),
            48 => Some(Self::PressurePump2Auto),
            49 => Some(Self::PressurePump3Manual),
            50 => Some(Self::PressurePump3Auto),
            51 => Some(Self::SprinklerPumpStop),
            52 => Some(Self::SprinklerPump1Stop),
            53 => Some(Self::SprinklerPump2Stop),
            54 => Some(Self::SprinklerPump3Stop),
            55 => Some(Self::FirePumpStop),
            56 => Some(Self::FirePump1Stop),
            57 => Some(Self::FirePump2Stop),
            58 => Some(Self::FirePump3Stop),
            59 => Some(Self::PressurePumpStop),
            60 => Some(Self::PressurePump1Stop),
            61 => Some(Self::PressurePump2Stop),
            62 => Some(Self::PressurePump3Stop),
            63 => Some(Self::SprinklerPump1Power),
            64 => Some(Self::SprinklerPump2Power),
            65 => Some(Self::SprinklerPump3Power),
            66 => Some(Self::FirePump1Power),
            67 => Some(Self::FirePump2Power),
            68 => Some(Self::FirePump3Power),
            69 => Some(Self::PressurePump1Power),
            70 => Some(Self::PressurePump2Power),
            71 => Some(Self::PressurePump3Power),
            255 => Some(Self::Other),
            _ => None,
        }
    }

    /// 转换为u8值
    pub fn to_u8(self) -> u8 {
        self as u8
    }

    /// 获取开关量类型的描述
    pub fn description(&self) -> &'static str {
        match self {
            Self::Undefined => "无定义",
            Self::MainPower => "主电电源",
            Self::BackupPower => "备用电源",
            Self::SprinklerPumpFault => "喷淋泵故障",
            Self::SprinklerPump1Fault => "喷淋泵1故障",
            Self::SprinklerPump2Fault => "喷淋泵2故障",
            Self::SprinklerPump3Fault => "喷淋泵3故障",
            Self::FirePumpFault => "消防泵故障",
            Self::FirePump1Fault => "消防泵1故障",
            Self::FirePump2Fault => "消防泵2故障",
            Self::FirePump3Fault => "消防泵3故障",
            Self::PressurePumpFault => "稳压泵故障",
            Self::PressurePump1Fault => "稳压泵1故障",
            Self::PressurePump2Fault => "稳压泵2故障",
            Self::PressurePump3Fault => "稳压泵3故障",
            Self::SprinklerPumpRunning => "喷淋泵运行",
            Self::SprinklerPump1Running => "喷淋泵1运行",
            Self::SprinklerPump2Running => "喷淋泵2运行",
            Self::SprinklerPump3Running => "喷淋泵3运行",
            Self::FirePumpRunning => "消防泵运行",
            Self::FirePump1Running => "消防泵1运行",
            Self::FirePump2Running => "消防泵2运行",
            Self::FirePump3Running => "消防泵3运行",
            Self::PressurePumpRunning => "稳压泵运行",
            Self::PressurePump1Running => "稳压泵1运行",
            Self::PressurePump2Running => "稳压泵2运行",
            Self::PressurePump3Running => "稳压泵3运行",
            Self::SprinklerPumpManual => "喷淋泵手动",
            Self::SprinklerPumpAuto => "喷淋泵自动",
            Self::SprinklerPump1Manual => "喷淋泵1手动",
            Self::SprinklerPump1Auto => "喷淋泵1自动",
            Self::SprinklerPump2Manual => "喷淋泵2手动",
            Self::SprinklerPump2Auto => "喷淋泵2自动",
            Self::SprinklerPump3Manual => "喷淋泵3手动",
            Self::SprinklerPump3Auto => "喷淋泵3自动",
            Self::FirePumpManual => "消防泵手动",
            Self::FirePumpAuto => "消防泵自动",
            Self::FirePump1Manual => "消防泵1手动",
            Self::FirePump1Auto => "消防泵1自动",
            Self::FirePump2Manual => "消防泵2手动",
            Self::FirePump2Auto => "消防泵2自动",
            Self::FirePump3Manual => "消防泵3手动",
            Self::FirePump3Auto => "消防泵3自动",
            Self::PressurePumpManual => "稳压泵手动",
            Self::PressurePumpAuto => "稳压泵自动",
            Self::PressurePump1Manual => "稳压泵1手动",
            Self::PressurePump1Auto => "稳压泵1自动",
            Self::PressurePump2Manual => "稳压泵2手动",
            Self::PressurePump2Auto => "稳压泵2自动",
            Self::PressurePump3Manual => "稳压泵3手动",
            Self::PressurePump3Auto => "稳压泵3自动",
            Self::SprinklerPumpStop => "喷淋泵停止",
            Self::SprinklerPump1Stop => "喷淋泵1停止",
            Self::SprinklerPump2Stop => "喷淋泵2停止",
            Self::SprinklerPump3Stop => "喷淋泵3停止",
            Self::FirePumpStop => "消防泵停止",
            Self::FirePump1Stop => "消防泵1停止",
            Self::FirePump2Stop => "消防泵2停止",
            Self::FirePump3Stop => "消防泵3停止",
            Self::PressurePumpStop => "稳压泵停止",
            Self::PressurePump1Stop => "稳压泵1停止",
            Self::PressurePump2Stop => "稳压泵2停止",
            Self::PressurePump3Stop => "稳压泵3停止",
            Self::SprinklerPump1Power => "喷淋泵1电源",
            Self::SprinklerPump2Power => "喷淋泵2电源",
            Self::SprinklerPump3Power => "喷淋泵3电源",
            Self::FirePump1Power => "消防泵1电源",
            Self::FirePump2Power => "消防泵2电源",
            Self::FirePump3Power => "消防泵3电源",
            Self::PressurePump1Power => "稳压泵1电源",
            Self::PressurePump2Power => "稳压泵2电源",
            Self::PressurePump3Power => "稳压泵3电源",
            Self::Other => "其它",
        }
    }

    /// 判断是否为故障类型
    pub fn is_fault(&self) -> bool {
        matches!(
            self,
            Self::SprinklerPumpFault
                | Self::SprinklerPump1Fault
                | Self::SprinklerPump2Fault
                | Self::SprinklerPump3Fault
                | Self::FirePumpFault
                | Self::FirePump1Fault
                | Self::FirePump2Fault
                | Self::FirePump3Fault
                | Self::PressurePumpFault
                | Self::PressurePump1Fault
                | Self::PressurePump2Fault
                | Self::PressurePump3Fault
        )
    }

    /// 判断是否为运行状态
    pub fn is_running(&self) -> bool {
        matches!(
            self,
            Self::SprinklerPumpRunning
                | Self::SprinklerPump1Running
                | Self::SprinklerPump2Running
                | Self::SprinklerPump3Running
                | Self::FirePumpRunning
                | Self::FirePump1Running
                | Self::FirePump2Running
                | Self::FirePump3Running
                | Self::PressurePumpRunning
                | Self::PressurePump1Running
                | Self::PressurePump2Running
                | Self::PressurePump3Running
        )
    }

    /// 判断是否为停止状态
    pub fn is_stopped(&self) -> bool {
        matches!(
            self,
            Self::SprinklerPumpStop
                | Self::SprinklerPump1Stop
                | Self::SprinklerPump2Stop
                | Self::SprinklerPump3Stop
                | Self::FirePumpStop
                | Self::FirePump1Stop
                | Self::FirePump2Stop
                | Self::FirePump3Stop
                | Self::PressurePumpStop
                | Self::PressurePump1Stop
                | Self::PressurePump2Stop
                | Self::PressurePump3Stop
        )
    }

    /// 判断是否为手动模式
    pub fn is_manual(&self) -> bool {
        matches!(
            self,
            Self::SprinklerPumpManual
                | Self::SprinklerPump1Manual
                | Self::SprinklerPump2Manual
                | Self::SprinklerPump3Manual
                | Self::FirePumpManual
                | Self::FirePump1Manual
                | Self::FirePump2Manual
                | Self::FirePump3Manual
                | Self::PressurePumpManual
                | Self::PressurePump1Manual
                | Self::PressurePump2Manual
                | Self::PressurePump3Manual
        )
    }

    /// 判断是否为自动模式
    pub fn is_auto(&self) -> bool {
        matches!(
            self,
            Self::SprinklerPumpAuto
                | Self::SprinklerPump1Auto
                | Self::SprinklerPump2Auto
                | Self::SprinklerPump3Auto
                | Self::FirePumpAuto
                | Self::FirePump1Auto
                | Self::FirePump2Auto
                | Self::FirePump3Auto
                | Self::PressurePumpAuto
                | Self::PressurePump1Auto
                | Self::PressurePump2Auto
                | Self::PressurePump3Auto
        )
    }

    /// 判断是否为电源类型
    pub fn is_power(&self) -> bool {
        matches!(
            self,
            Self::MainPower
                | Self::BackupPower
                | Self::SprinklerPump1Power
                | Self::SprinklerPump2Power
                | Self::SprinklerPump3Power
                | Self::FirePump1Power
                | Self::FirePump2Power
                | Self::FirePump3Power
                | Self::PressurePump1Power
                | Self::PressurePump2Power
                | Self::PressurePump3Power
        )
    }

    /// 判断是否为喷淋泵相关
    pub fn is_sprinkler_pump(&self) -> bool {
        self.to_u8() >= 3 && self.to_u8() <= 6
            || self.to_u8() >= 15 && self.to_u8() <= 18
            || self.to_u8() >= 27 && self.to_u8() <= 34
            || self.to_u8() >= 51 && self.to_u8() <= 54
            || self.to_u8() >= 63 && self.to_u8() <= 65
    }

    /// 判断是否为消防泵相关
    pub fn is_fire_pump(&self) -> bool {
        self.to_u8() >= 7 && self.to_u8() <= 10
            || self.to_u8() >= 19 && self.to_u8() <= 22
            || self.to_u8() >= 35 && self.to_u8() <= 42
            || self.to_u8() >= 55 && self.to_u8() <= 58
            || self.to_u8() >= 66 && self.to_u8() <= 68
    }

    /// 判断是否为稳压泵相关
    pub fn is_pressure_pump(&self) -> bool {
        self.to_u8() >= 11 && self.to_u8() <= 14
            || self.to_u8() >= 23 && self.to_u8() <= 26
            || self.to_u8() >= 43 && self.to_u8() <= 50
            || self.to_u8() >= 59 && self.to_u8() <= 62
            || self.to_u8() >= 69 && self.to_u8() <= 71
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_switch_type_from_u8() {
        assert_eq!(SwitchType::from_u8(0), Some(SwitchType::Undefined));
        assert_eq!(SwitchType::from_u8(1), Some(SwitchType::MainPower));
        assert_eq!(SwitchType::from_u8(3), Some(SwitchType::SprinklerPumpFault));
        assert_eq!(
            SwitchType::from_u8(15),
            Some(SwitchType::SprinklerPumpRunning)
        );
        assert_eq!(
            SwitchType::from_u8(27),
            Some(SwitchType::SprinklerPumpManual)
        );
        assert_eq!(SwitchType::from_u8(51), Some(SwitchType::SprinklerPumpStop));
        assert_eq!(
            SwitchType::from_u8(63),
            Some(SwitchType::SprinklerPump1Power)
        );
        assert_eq!(SwitchType::from_u8(255), Some(SwitchType::Other));
        assert_eq!(SwitchType::from_u8(100), None);
    }

    #[test]
    fn test_switch_type_to_u8() {
        assert_eq!(SwitchType::Undefined.to_u8(), 0);
        assert_eq!(SwitchType::MainPower.to_u8(), 1);
        assert_eq!(SwitchType::SprinklerPumpFault.to_u8(), 3);
        assert_eq!(SwitchType::Other.to_u8(), 255);
    }

    #[test]
    fn test_switch_type_description() {
        assert_eq!(SwitchType::Undefined.description(), "无定义");
        assert_eq!(SwitchType::MainPower.description(), "主电电源");
        assert_eq!(SwitchType::SprinklerPumpFault.description(), "喷淋泵故障");
        assert_eq!(SwitchType::FirePump1Running.description(), "消防泵1运行");
    }

    #[test]
    fn test_switch_type_is_fault() {
        assert!(SwitchType::SprinklerPumpFault.is_fault());
        assert!(SwitchType::FirePump1Fault.is_fault());
        assert!(SwitchType::PressurePump2Fault.is_fault());
        assert!(!SwitchType::SprinklerPumpRunning.is_fault());
        assert!(!SwitchType::MainPower.is_fault());
    }

    #[test]
    fn test_switch_type_is_running() {
        assert!(SwitchType::SprinklerPumpRunning.is_running());
        assert!(SwitchType::FirePump1Running.is_running());
        assert!(SwitchType::PressurePump2Running.is_running());
        assert!(!SwitchType::SprinklerPumpStop.is_running());
        assert!(!SwitchType::SprinklerPumpFault.is_running());
    }

    #[test]
    fn test_switch_type_is_stopped() {
        assert!(SwitchType::SprinklerPumpStop.is_stopped());
        assert!(SwitchType::FirePump1Stop.is_stopped());
        assert!(SwitchType::PressurePump2Stop.is_stopped());
        assert!(!SwitchType::SprinklerPumpRunning.is_stopped());
        assert!(!SwitchType::MainPower.is_stopped());
    }

    #[test]
    fn test_switch_type_is_manual() {
        assert!(SwitchType::SprinklerPumpManual.is_manual());
        assert!(SwitchType::FirePump1Manual.is_manual());
        assert!(SwitchType::PressurePump2Manual.is_manual());
        assert!(!SwitchType::SprinklerPumpAuto.is_manual());
        assert!(!SwitchType::MainPower.is_manual());
    }

    #[test]
    fn test_switch_type_is_auto() {
        assert!(SwitchType::SprinklerPumpAuto.is_auto());
        assert!(SwitchType::FirePump1Auto.is_auto());
        assert!(SwitchType::PressurePump2Auto.is_auto());
        assert!(!SwitchType::SprinklerPumpManual.is_auto());
        assert!(!SwitchType::MainPower.is_auto());
    }

    #[test]
    fn test_switch_type_is_power() {
        assert!(SwitchType::MainPower.is_power());
        assert!(SwitchType::BackupPower.is_power());
        assert!(SwitchType::SprinklerPump1Power.is_power());
        assert!(SwitchType::FirePump2Power.is_power());
        assert!(SwitchType::PressurePump3Power.is_power());
        assert!(!SwitchType::SprinklerPumpRunning.is_power());
    }

    #[test]
    fn test_switch_type_is_sprinkler_pump() {
        assert!(SwitchType::SprinklerPumpFault.is_sprinkler_pump());
        assert!(SwitchType::SprinklerPump1Fault.is_sprinkler_pump());
        assert!(SwitchType::SprinklerPumpRunning.is_sprinkler_pump());
        assert!(SwitchType::SprinklerPump1Manual.is_sprinkler_pump());
        assert!(SwitchType::SprinklerPump2Stop.is_sprinkler_pump());
        assert!(SwitchType::SprinklerPump3Power.is_sprinkler_pump());
        assert!(!SwitchType::FirePumpFault.is_sprinkler_pump());
        assert!(!SwitchType::MainPower.is_sprinkler_pump());
    }

    #[test]
    fn test_switch_type_is_fire_pump() {
        assert!(SwitchType::FirePumpFault.is_fire_pump());
        assert!(SwitchType::FirePump1Fault.is_fire_pump());
        assert!(SwitchType::FirePumpRunning.is_fire_pump());
        assert!(SwitchType::FirePump1Manual.is_fire_pump());
        assert!(SwitchType::FirePump2Stop.is_fire_pump());
        assert!(SwitchType::FirePump3Power.is_fire_pump());
        assert!(!SwitchType::SprinklerPumpFault.is_fire_pump());
        assert!(!SwitchType::MainPower.is_fire_pump());
    }

    #[test]
    fn test_switch_type_is_pressure_pump() {
        assert!(SwitchType::PressurePumpFault.is_pressure_pump());
        assert!(SwitchType::PressurePump1Fault.is_pressure_pump());
        assert!(SwitchType::PressurePumpRunning.is_pressure_pump());
        assert!(SwitchType::PressurePump1Manual.is_pressure_pump());
        assert!(SwitchType::PressurePump2Stop.is_pressure_pump());
        assert!(SwitchType::PressurePump3Power.is_pressure_pump());
        assert!(!SwitchType::SprinklerPumpFault.is_pressure_pump());
        assert!(!SwitchType::MainPower.is_pressure_pump());
    }

    #[test]
    fn test_switch_type_roundtrip() {
        for value in [0u8, 1, 3, 15, 27, 51, 63, 71, 255] {
            if let Some(switch_type) = SwitchType::from_u8(value) {
                assert_eq!(switch_type.to_u8(), value);
            }
        }
    }
}
