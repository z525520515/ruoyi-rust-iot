//! 设备类型定义模块
//!
//! 定义了所有70种设备类型及其元数据配置

use super::device_config::{DataLength, DataType, DeviceTypeInfo, SingleParameterConfig};
use super::error::{ParseError, ParseResult};
use serde::{Deserialize, Serialize};

/// 设备类型枚举
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DeviceType {
    /// 0 - 压力表 kPa
    PressureGaugeKpa = 0,
    /// 1 - 压力表 MPa
    PressureGaugeMpa = 1,
    /// 2 - 压力表 BAR
    PressureGaugeBar = 2,
    /// 3 - 压力表 PSI
    PressureGaugePsi = 3,
    /// 4 - 压力表 Pa
    PressureGaugePa = 4,
    /// 5 - 压力表 mBAR
    PressureGaugeMbar = 5,
    /// 6 - 压力表 Kgf/cm²
    PressureGaugeKgfcm2 = 6,
    /// 7 - 压力表 mmHg
    PressureGaugeMmHg = 7,
    /// 8 - 液位表 m
    LevelGaugeM = 8,
    /// 9 - 液位表 cm
    LevelGaugeCm = 9,
    /// 10 - 液位表 mm
    LevelGaugeMm = 10,
    /// 11 - 温度表 °C
    TemperatureGaugeC = 11,
    /// 12 - 温度表 °F
    TemperatureGaugeF = 12,
    /// 13 - 湿度表 %RH
    HumidityGauge = 13,
    /// 14 - 电流表
    CurrentMeter = 14,
    /// 15 - 电压表
    VoltageMeter = 15,
    /// 16 - 差压表（静压MPa、差压kPa）
    DifferentialPressureGauge1 = 16,
    /// 17 - 温压表（压力MPa、温度°C）
    TemperaturePressureGauge = 17,
    /// 18 - 温湿度表（湿度%RH、温度°C）
    TemperatureHumidityGauge = 18,
    /// 19 - 振动
    Vibration = 19,
    /// 20 - 水浸
    WaterImmersion = 20,
    /// 21 - 差压表（进压kPa、回压kPa）
    DifferentialPressureGauge2 = 21,
    /// 22 - 压力表 Pa (10Pa)
    PressureGaugePa10 = 22,
    /// 23 - 倾角 °
    InclinationAngle = 23,
    /// 24 - 消火栓（压力MPa、倾角°）
    FireHydrant = 24,
    /// 25 - 温度表（温度表°C、温度表°C）
    DualTemperatureGauge = 25,
    /// 26 - 力 KN
    ForceKN = 26,
    /// 27 - 流量计（瞬时流量m³/h、瞬时流速m/s、正累积流量m³、净累积流量m³）
    FlowMeter = 27,
    /// 28 - 16通道开关量采集终端
    SwitchChannel16 = 28,
    /// 29 - 液位表 m（超声波&液位探头）
    LevelGaugeUltrasonic = 29,
    /// 30 - 超声波液位表 m
    UltrasonicLevelGauge = 30,
    /// 31 - 可燃气体
    CombustibleGas = 31,
    /// 32 - 定位坐标（经度°、纬度°）
    GpsCoordinate = 32,
    /// 33 - 液位&压力表（液位m、压力MPa）
    LevelPressureGauge = 33,
    /// 34 - 消火栓压力
    FireHydrantPressure = 34,
    /// 35 - 差压表 kPa
    DifferentialPressureGaugeKpa = 35,
    /// 36 - 消防栓闷盖（水浸状态、闷盖状态）
    FireHydrantCover = 36,
    /// 37 - 井盖状态仪
    ManholeCoverStatus = 37,
    /// 38 - 液位&温度表（液位表、温度表）
    LevelTemperatureGauge = 38,
    /// 39 - 温度&异动（温度、异动）
    TemperatureAbnormal = 39,
    /// 40 - 井盖液位
    ManholeCoverLevel = 40,
    /// 41 - 噪声
    Noise = 41,
    /// 42 - 雷达测距仪
    RadarRangefinder = 42,
    /// 43 - 32通道开关量采集终端
    SwitchChannel32 = 43,
    /// 44 - 风速仪
    Anemometer = 44,
    /// 45 - MODBUS采集终端
    ModbusTerminal = 45,
    /// 46 - 磁致伸缩液位计
    MagnetostrictiveLevelGauge = 46,
    /// 47 - PH检测仪
    PhDetector = 47,
    /// 48 - 电导率检测仪
    ConductivityDetector = 48,
    /// 49 - 液位&倾角（液位m、倾角°）
    LevelInclinationGauge = 49,
    /// 50 - 4通道压力温度检测仪
    FourChannelPressureTemperature = 50,
    /// 51 - ORP检测仪
    OrpDetector = 51,
    /// 52 - 力 KN (32位)
    ForceKN32 = 52,
    /// 53 - 流量计（m³）
    FlowMeterM3 = 53,
    /// 54 - 氨氮检测仪
    AmmoniaNitrogenDetector = 54,
    /// 55 - COD检测仪
    CodDetector = 55,
    /// 56 - 溶解氧检测仪
    DissolvedOxygenDetector = 56,
    /// 57 - 浊度检测仪
    TurbidityDetector = 57,
    /// 58 - 消火栓流量（压力MPa、倾角、流量L/s、温度°C）
    FireHydrantFlow = 58,
    /// 59 - 门磁开关
    DoorMagneticSwitch = 59,
    /// 60 - 电表
    ElectricMeter = 60,
    /// 61 - 压力
    Pressure = 61,
    /// 62 - 振弦、温度
    VibrationWireTemperature = 62,
    /// 63 - 液位
    LevelGauge = 63,
    /// 64 - 蓝绿藻
    BlueGreenAlgae = 64,
    /// 65 - 三通道温度（温度、温度、温度）
    TripleChannelTemperature = 65,
    /// 66 - 双通道压力（压力、压力）
    DualChannelPressure = 66,
    /// 67 - 智能井盖监测终端
    SmartManholeCoverMonitor = 67,
    /// 68 - PH监测终端（PH、温度）
    PhMonitorTerminal = 68,
    /// 69 - 电导率监测终端（电导率、温度）
    ConductivityMonitorTerminal = 69,
}

impl DeviceType {
    pub fn from_u8(value: u8) -> ParseResult<Self> {
        match value {
            0 => Ok(Self::PressureGaugeKpa),
            1 => Ok(Self::PressureGaugeMpa),
            2 => Ok(Self::PressureGaugeBar),
            3 => Ok(Self::PressureGaugePsi),
            4 => Ok(Self::PressureGaugePa),
            5 => Ok(Self::PressureGaugeMbar),
            6 => Ok(Self::PressureGaugeKgfcm2),
            7 => Ok(Self::PressureGaugeMmHg),
            8 => Ok(Self::LevelGaugeM),
            9 => Ok(Self::LevelGaugeCm),
            10 => Ok(Self::LevelGaugeMm),
            11 => Ok(Self::TemperatureGaugeC),
            12 => Ok(Self::TemperatureGaugeF),
            13 => Ok(Self::HumidityGauge),
            14 => Ok(Self::CurrentMeter),
            15 => Ok(Self::VoltageMeter),
            16 => Ok(Self::DifferentialPressureGauge1),
            17 => Ok(Self::TemperaturePressureGauge),
            18 => Ok(Self::TemperatureHumidityGauge),
            19 => Ok(Self::Vibration),
            20 => Ok(Self::WaterImmersion),
            21 => Ok(Self::DifferentialPressureGauge2),
            22 => Ok(Self::PressureGaugePa10),
            23 => Ok(Self::InclinationAngle),
            24 => Ok(Self::FireHydrant),
            25 => Ok(Self::DualTemperatureGauge),
            26 => Ok(Self::ForceKN),
            27 => Ok(Self::FlowMeter),
            28 => Ok(Self::SwitchChannel16),
            29 => Ok(Self::LevelGaugeUltrasonic),
            30 => Ok(Self::UltrasonicLevelGauge),
            31 => Ok(Self::CombustibleGas),
            32 => Ok(Self::GpsCoordinate),
            33 => Ok(Self::LevelPressureGauge),
            34 => Ok(Self::FireHydrantPressure),
            35 => Ok(Self::DifferentialPressureGaugeKpa),
            36 => Ok(Self::FireHydrantCover),
            37 => Ok(Self::ManholeCoverStatus),
            38 => Ok(Self::LevelTemperatureGauge),
            39 => Ok(Self::TemperatureAbnormal),
            40 => Ok(Self::ManholeCoverLevel),
            41 => Ok(Self::Noise),
            42 => Ok(Self::RadarRangefinder),
            43 => Ok(Self::SwitchChannel32),
            44 => Ok(Self::Anemometer),
            45 => Ok(Self::ModbusTerminal),
            46 => Ok(Self::MagnetostrictiveLevelGauge),
            47 => Ok(Self::PhDetector),
            48 => Ok(Self::ConductivityDetector),
            49 => Ok(Self::LevelInclinationGauge),
            50 => Ok(Self::FourChannelPressureTemperature),
            51 => Ok(Self::OrpDetector),
            52 => Ok(Self::ForceKN32),
            53 => Ok(Self::FlowMeterM3),
            54 => Ok(Self::AmmoniaNitrogenDetector),
            55 => Ok(Self::CodDetector),
            56 => Ok(Self::DissolvedOxygenDetector),
            57 => Ok(Self::TurbidityDetector),
            58 => Ok(Self::FireHydrantFlow),
            59 => Ok(Self::DoorMagneticSwitch),
            60 => Ok(Self::ElectricMeter),
            61 => Ok(Self::Pressure),
            62 => Ok(Self::VibrationWireTemperature),
            63 => Ok(Self::LevelGauge),
            64 => Ok(Self::BlueGreenAlgae),
            65 => Ok(Self::TripleChannelTemperature),
            66 => Ok(Self::DualChannelPressure),
            67 => Ok(Self::SmartManholeCoverMonitor),
            68 => Ok(Self::PhMonitorTerminal),
            69 => Ok(Self::ConductivityMonitorTerminal),
            _ => Err(ParseError::InvalidDataType {
                value,
                field: "device type".to_string(),
            }),
        }
    }

    pub fn to_u8(self) -> u8 {
        self as u8
    }

    pub fn info(&self) -> &'static DeviceTypeInfo {
        &DEVICE_TYPE_TABLE[*self as usize]
    }

    pub fn name(&self) -> &'static str {
        self.info().name
    }

    pub fn alarm_groups(&self) -> u8 {
        self.info().alarm_groups
    }

    pub fn data_length(&self) -> DataLength {
        self.info().data_length
    }

    pub fn get_parameters_info(&self) -> &'static [SingleParameterConfig] {
        self.info().parameters
    }

    pub fn is_variable_length(&self) -> bool {
        self.info().is_variable_length()
    }
}

/// 设备类型配置表
static DEVICE_TYPE_TABLE: [DeviceTypeInfo; 70] = [
    // 0: 压力表 kPa
    DeviceTypeInfo::new(
        0,
        "压力表",
        1,
        DataLength::Fixed(2),
        &[SingleParameterConfig::new(
            "压力",
            DataType::I16,
            "kPa",
            0.1,
            0,
        )],
    ),
    // 1: 压力表 MPa
    DeviceTypeInfo::new(
        1,
        "压力表",
        1,
        DataLength::Fixed(2),
        &[SingleParameterConfig::new(
            "压力",
            DataType::I16,
            "MPa",
            0.001,
            0,
        )],
    ),
    // 2: 压力表 BAR
    DeviceTypeInfo::new(
        2,
        "压力表",
        1,
        DataLength::Fixed(2),
        &[SingleParameterConfig::new(
            "压力",
            DataType::I16,
            "BAR",
            0.01,
            0,
        )],
    ),
    // 3: 压力表 PSI
    DeviceTypeInfo::new(
        3,
        "压力表",
        1,
        DataLength::Fixed(2),
        &[SingleParameterConfig::new(
            "压力",
            DataType::I16,
            "PSI",
            0.1,
            0,
        )],
    ),
    // 4: 压力表 Pa
    DeviceTypeInfo::new(
        4,
        "压力表",
        1,
        DataLength::Fixed(2),
        &[SingleParameterConfig::new(
            "压力",
            DataType::I16,
            "Pa",
            1.0,
            0,
        )],
    ),
    // 5: 压力表 mBAR
    DeviceTypeInfo::new(
        5,
        "压力表",
        1,
        DataLength::Fixed(2),
        &[SingleParameterConfig::new(
            "压力",
            DataType::I16,
            "mBAR",
            1.0,
            0,
        )],
    ),
    // 6: 压力表 Kgf/cm²
    DeviceTypeInfo::new(
        6,
        "压力表",
        1,
        DataLength::Fixed(2),
        &[SingleParameterConfig::new(
            "压力",
            DataType::I16,
            "Kgf/cm²",
            0.01,
            0,
        )],
    ),
    // 7: 压力表 mmHg
    DeviceTypeInfo::new(
        7,
        "压力表",
        1,
        DataLength::Fixed(2),
        &[SingleParameterConfig::new(
            "压力",
            DataType::I16,
            "mmHg",
            0.01,
            0,
        )],
    ),
    // 8: 液位表 m
    DeviceTypeInfo::new(
        8,
        "液位表",
        1,
        DataLength::Fixed(2),
        &[SingleParameterConfig::new(
            "液位",
            DataType::I16,
            "m",
            0.01,
            0,
        )],
    ),
    // 9: 液位表 cm
    DeviceTypeInfo::new(
        9,
        "液位表",
        1,
        DataLength::Fixed(2),
        &[SingleParameterConfig::new(
            "液位",
            DataType::I16,
            "cm",
            0.1,
            0,
        )],
    ),
    // 10: 液位表 mm
    DeviceTypeInfo::new(
        10,
        "液位表",
        1,
        DataLength::Fixed(2),
        &[SingleParameterConfig::new(
            "液位",
            DataType::I16,
            "mm",
            1.0,
            0,
        )],
    ),
    // 11: 温度表 °C
    DeviceTypeInfo::new(
        11,
        "温度表",
        1,
        DataLength::Fixed(2),
        &[SingleParameterConfig::new(
            "温度",
            DataType::I16,
            "°C",
            0.1,
            0,
        )],
    ),
    // 12: 温度表 °F
    DeviceTypeInfo::new(
        12,
        "温度表",
        1,
        DataLength::Fixed(2),
        &[SingleParameterConfig::new(
            "温度",
            DataType::I16,
            "°F",
            0.1,
            0,
        )],
    ),
    // 13: 湿度表 %RH
    DeviceTypeInfo::new(
        13,
        "湿度表",
        1,
        DataLength::Fixed(2),
        &[SingleParameterConfig::new(
            "湿度",
            DataType::I16,
            "%RH",
            0.1,
            0,
        )],
    ),
    // 14: 电流表
    DeviceTypeInfo::new(
        14,
        "电流表",
        1,
        DataLength::Fixed(2),
        &[SingleParameterConfig::new(
            "电流",
            DataType::I16,
            "A",
            0.1,
            0,
        )],
    ),
    // 15: 电压表
    DeviceTypeInfo::new(
        15,
        "电压表",
        1,
        DataLength::Fixed(2),
        &[SingleParameterConfig::new(
            "电压",
            DataType::I16,
            "V",
            0.1,
            0,
        )],
    ),
    // 16: 差压表（静压MPa、差压kPa）
    DeviceTypeInfo::new(
        16,
        "差压表",
        2,
        DataLength::Fixed(4),
        &[
            SingleParameterConfig::new("静压", DataType::I16, "MPa", 0.001, 0),
            SingleParameterConfig::new("差压", DataType::I16, "kPa", 0.1, 2),
        ],
    ),
    // 17: 温压表（压力MPa、温度°C）
    DeviceTypeInfo::new(
        17,
        "温压表",
        2,
        DataLength::Fixed(4),
        &[
            SingleParameterConfig::new("压力", DataType::I16, "MPa", 0.001, 0),
            SingleParameterConfig::new("温度", DataType::I16, "°C", 0.1, 2),
        ],
    ),
    // 18: 温湿度表（湿度%RH、温度°C）
    DeviceTypeInfo::new(
        18,
        "温湿度表",
        2,
        DataLength::Fixed(4),
        &[
            SingleParameterConfig::new("湿度", DataType::I16, "%RH", 0.1, 0),
            SingleParameterConfig::new("温度", DataType::I16, "°C", 0.1, 2),
        ],
    ),
    // 19: 振动
    DeviceTypeInfo::new(
        19,
        "振动",
        1,
        DataLength::Fixed(2),
        &[SingleParameterConfig::new(
            "振动",
            DataType::I16,
            "mm/s",
            0.1,
            0,
        )],
    ),
    // 20: 水浸
    DeviceTypeInfo::new(
        20,
        "水浸",
        1,
        DataLength::Fixed(2),
        &[SingleParameterConfig::new(
            "水浸",
            DataType::I16,
            "",
            1.0,
            0,
        )],
    ),
    // 21: 差压表（进压kPa、回压kPa）
    DeviceTypeInfo::new(
        21,
        "差压表",
        2,
        DataLength::Fixed(4),
        &[
            SingleParameterConfig::new("进压", DataType::I16, "kPa", 1.0, 0),
            SingleParameterConfig::new("回压", DataType::I16, "kPa", 1.0, 2),
        ],
    ),
    // 22: 压力表 Pa (10Pa)
    DeviceTypeInfo::new(
        22,
        "压力表",
        1,
        DataLength::Fixed(2),
        &[SingleParameterConfig::new(
            "压力",
            DataType::I16,
            "Pa",
            10.0,
            0,
        )],
    ),
    // 23: 倾角 °
    DeviceTypeInfo::new(
        23,
        "倾角",
        1,
        DataLength::Fixed(2),
        &[SingleParameterConfig::new(
            "倾角",
            DataType::I16,
            "°",
            1.0,
            0,
        )],
    ),
    // 24: 消火栓（压力MPa、倾角°）
    DeviceTypeInfo::new(
        24,
        "消火栓",
        2,
        DataLength::Fixed(4),
        &[
            SingleParameterConfig::new("压力", DataType::I16, "MPa", 0.001, 0),
            SingleParameterConfig::new("倾角", DataType::I16, "°", 1.0, 2),
        ],
    ),
    // 25: 温度表（温度表°C、温度表°C）
    DeviceTypeInfo::new(
        25,
        "温度表",
        2,
        DataLength::Fixed(4),
        &[
            SingleParameterConfig::new("温度1", DataType::I16, "°C", 0.1, 0),
            SingleParameterConfig::new("温度2", DataType::I16, "°C", 0.1, 2),
        ],
    ),
    // 26: 力 KN
    DeviceTypeInfo::new(
        26,
        "力",
        1,
        DataLength::Fixed(2),
        &[SingleParameterConfig::new(
            "力",
            DataType::I16,
            "KN",
            1.0,
            0,
        )],
    ),
    // 27: 流量计（瞬时流量m³/h、瞬时流速m/s、正累积流量m³、净累积流量m³）
    DeviceTypeInfo::new(
        27,
        "流量计",
        1,
        DataLength::Fixed(16),
        &[
            SingleParameterConfig::new("瞬时流量", DataType::F32, "m³/h", 1.0, 0),
            SingleParameterConfig::new("瞬时流速", DataType::F32, "m/s", 1.0, 4),
            SingleParameterConfig::new("正累积流量", DataType::U32, "m³", 1.0, 8),
            SingleParameterConfig::new("净累积流量", DataType::U32, "m³", 1.0, 12),
        ],
    ),
    // 28: 16通道开关量采集终端
    DeviceTypeInfo::new(
        28,
        "16通道开关量采集终端",
        1,
        DataLength::Fixed(2),
        &[SingleParameterConfig::new(
            "开关状态",
            DataType::U16,
            "",
            1.0,
            0,
        )],
    ),
    // 29: 液位表 m（超声波&液位探头）
    DeviceTypeInfo::new(
        29,
        "液位表",
        1,
        DataLength::Fixed(6),
        &[
            SingleParameterConfig::new("液位探头", DataType::I16, "m", 0.01, 0),
            SingleParameterConfig::new("超声波", DataType::I16, "m", 0.01, 2),
            SingleParameterConfig::new("换算值", DataType::I16, "m", 0.01, 4),
        ],
    ),
    // 30: 超声波液位表 m
    DeviceTypeInfo::new(
        30,
        "超声波液位表",
        1,
        DataLength::Fixed(2),
        &[SingleParameterConfig::new(
            "液位",
            DataType::I16,
            "m",
            0.01,
            0,
        )],
    ),
    // 31: 可燃气体
    DeviceTypeInfo::new(
        31,
        "可燃气体",
        1,
        DataLength::Fixed(2),
        &[SingleParameterConfig::new(
            "可燃气体",
            DataType::U16,
            "%LEL",
            0.1,
            0,
        )],
    ),
    // 32: 定位坐标（经度°、纬度°）
    DeviceTypeInfo::new(
        32,
        "定位坐标",
        1,
        DataLength::Fixed(16),
        &[
            SingleParameterConfig::new("经度", DataType::F64, "°", 1.0, 0),
            SingleParameterConfig::new("纬度", DataType::F64, "°", 1.0, 8),
        ],
    ),
    // 33: 液位&压力表（液位m、压力MPa）
    DeviceTypeInfo::new(
        33,
        "液位&压力表",
        2,
        DataLength::Fixed(4),
        &[
            SingleParameterConfig::new("液位", DataType::I16, "m", 0.01, 0),
            SingleParameterConfig::new("压力", DataType::I16, "MPa", 0.001, 2),
        ],
    ),
    // 34: 消火栓压力
    DeviceTypeInfo::new(
        34,
        "消火栓压力",
        1,
        DataLength::Fixed(2),
        &[SingleParameterConfig::new(
            "压力",
            DataType::I16,
            "MPa",
            0.001,
            0,
        )],
    ),
    // 35: 差压表 kPa
    DeviceTypeInfo::new(
        35,
        "差压表",
        1,
        DataLength::Fixed(2),
        &[SingleParameterConfig::new(
            "差压",
            DataType::I16,
            "kPa",
            0.1,
            0,
        )],
    ),
    // 36: 消防栓闷盖（水浸状态、闷盖状态）
    DeviceTypeInfo::new(
        36,
        "消防栓闷盖",
        2,
        DataLength::Fixed(4),
        &[
            SingleParameterConfig::new("水浸状态", DataType::I16, "", 1.0, 0),
            SingleParameterConfig::new("闷盖状态", DataType::I16, "", 1.0, 2),
        ],
    ),
    // 37: 井盖状态仪
    DeviceTypeInfo::new(
        37,
        "井盖状态仪",
        1,
        DataLength::Fixed(2),
        &[SingleParameterConfig::new(
            "开关状态",
            DataType::I16,
            "",
            1.0,
            0,
        )],
    ),
    // 38: 液位&温度表（液位表、温度表）
    DeviceTypeInfo::new(
        38,
        "液位&温度表",
        2,
        DataLength::Fixed(4),
        &[
            SingleParameterConfig::new("液位", DataType::I16, "m", 0.01, 0),
            SingleParameterConfig::new("温度", DataType::I16, "°C", 0.1, 2),
        ],
    ),
    // 39: 温度&异动（温度、异动）
    DeviceTypeInfo::new(
        39,
        "温度&异动",
        2,
        DataLength::Fixed(4),
        &[
            SingleParameterConfig::new("温度", DataType::I16, "°C", 0.1, 0),
            SingleParameterConfig::new("异动", DataType::I16, "", 1.0, 2),
        ],
    ),
    // 40: 井盖液位
    DeviceTypeInfo::new(
        40,
        "井盖液位",
        1,
        DataLength::Fixed(2),
        &[SingleParameterConfig::new(
            "液位",
            DataType::I16,
            "m",
            0.01,
            0,
        )],
    ),
    // 41: 噪声
    DeviceTypeInfo::new(
        41,
        "噪声",
        1,
        DataLength::Fixed(2),
        &[SingleParameterConfig::new(
            "噪声",
            DataType::I16,
            "dB",
            1.0,
            0,
        )],
    ),
    // 42: 雷达测距仪
    DeviceTypeInfo::new(
        42,
        "雷达测距仪",
        1,
        DataLength::Fixed(2),
        &[SingleParameterConfig::new(
            "距离",
            DataType::I16,
            "m",
            0.01,
            0,
        )],
    ),
    // 43: 32通道开关量采集终端
    DeviceTypeInfo::new(
        43,
        "32通道开关量采集终端",
        2,
        DataLength::Fixed(4),
        &[SingleParameterConfig::new(
            "开关状态",
            DataType::U32,
            "",
            1.0,
            0,
        )],
    ),
    // 44: 风速仪
    DeviceTypeInfo::new(
        44,
        "风速仪",
        1,
        DataLength::Fixed(6),
        &[
            SingleParameterConfig::new("差压", DataType::I16, "Pa", 1.0, 0),
            SingleParameterConfig::new("风速", DataType::I16, "m/s", 0.01, 2),
            SingleParameterConfig::new("风量", DataType::I16, "m³/h", 1.0, 4),
        ],
    ),
    // 45: MODBUS采集终端（变长数据）
    DeviceTypeInfo::new(45, "MODBUS采集终端", 1, DataLength::Variable, &[]),
    // 46: 磁致伸缩液位计
    DeviceTypeInfo::new(
        46,
        "磁致伸缩液位计",
        1,
        DataLength::Fixed(2),
        &[SingleParameterConfig::new(
            "液位",
            DataType::I16,
            "mm",
            0.001,
            0,
        )],
    ),
    // 47: PH检测仪
    DeviceTypeInfo::new(
        47,
        "PH检测仪",
        1,
        DataLength::Fixed(2),
        &[SingleParameterConfig::new(
            "PH",
            DataType::I16,
            "pH",
            0.01,
            0,
        )],
    ),
    // 48: 电导率检测仪
    DeviceTypeInfo::new(
        48,
        "电导率检测仪",
        1,
        DataLength::Fixed(2),
        &[SingleParameterConfig::new(
            "电导率",
            DataType::I16,
            "uS/cm",
            0.1,
            0,
        )],
    ),
    // 49: 液位&倾角（液位m、倾角°）
    DeviceTypeInfo::new(
        49,
        "液位&倾角",
        2,
        DataLength::Fixed(4),
        &[
            SingleParameterConfig::new("液位", DataType::I16, "m", 0.01, 0),
            SingleParameterConfig::new("倾角", DataType::I16, "°", 1.0, 2),
        ],
    ),
    // 50: 4通道压力温度检测仪
    DeviceTypeInfo::new(
        50,
        "4通道压力温度检测仪",
        1,
        DataLength::Fixed(10),
        &[
            SingleParameterConfig::new("压力1", DataType::I16, "MPa", 0.001, 0),
            SingleParameterConfig::new("压力2", DataType::I16, "MPa", 0.001, 2),
            SingleParameterConfig::new("温度1", DataType::I16, "°C", 0.1, 4),
            SingleParameterConfig::new("温度2", DataType::I16, "°C", 0.1, 6),
            SingleParameterConfig::new("流量", DataType::I16, "m³/h", 0.01, 8),
        ],
    ),
    // 51: ORP检测仪
    DeviceTypeInfo::new(
        51,
        "ORP检测仪",
        1,
        DataLength::Fixed(2),
        &[SingleParameterConfig::new(
            "ORP",
            DataType::I16,
            "mV",
            1.0,
            0,
        )],
    ),
    // 52: 力 KN (32位)
    DeviceTypeInfo::new(
        52,
        "力",
        1,
        DataLength::Fixed(4),
        &[SingleParameterConfig::new(
            "力",
            DataType::I32,
            "KN",
            1.0,
            0,
        )],
    ),
    // 53: 流量计（m³）
    DeviceTypeInfo::new(
        53,
        "流量计",
        1,
        DataLength::Fixed(4),
        &[SingleParameterConfig::new(
            "累计流量",
            DataType::U32,
            "m³",
            0.01,
            0,
        )],
    ),
    // 54: 氨氮检测仪
    DeviceTypeInfo::new(
        54,
        "氨氮检测仪",
        1,
        DataLength::Fixed(6),
        &[
            SingleParameterConfig::new("氨氮", DataType::I16, "mg/L", 0.1, 0),
            SingleParameterConfig::new("pH", DataType::I16, "pH", 0.1, 2),
            SingleParameterConfig::new("温度", DataType::I16, "°C", 0.1, 4),
        ],
    ),
    // 55: COD检测仪
    DeviceTypeInfo::new(
        55,
        "COD检测仪",
        1,
        DataLength::Fixed(6),
        &[
            SingleParameterConfig::new("COD", DataType::I16, "mg/L", 0.1, 0),
            SingleParameterConfig::new("浊度", DataType::I16, "NTU", 0.1, 2),
            SingleParameterConfig::new("温度", DataType::I16, "°C", 0.1, 4),
        ],
    ),
    // 56: 溶解氧检测仪
    DeviceTypeInfo::new(
        56,
        "溶解氧检测仪",
        1,
        DataLength::Fixed(6),
        &[
            SingleParameterConfig::new("溶解氧", DataType::I16, "mg/L", 0.01, 0),
            SingleParameterConfig::new("温度", DataType::I16, "°C", 0.1, 2),
            SingleParameterConfig::new("溶解氧单位", DataType::I16, "", 1.0, 4),
        ],
    ),
    // 57: 浊度检测仪
    DeviceTypeInfo::new(
        57,
        "浊度检测仪",
        1,
        DataLength::Fixed(2),
        &[SingleParameterConfig::new(
            "浊度",
            DataType::I16,
            "NTU",
            0.1,
            0,
        )],
    ),
    // 58: 消火栓流量（压力MPa、倾角、流量L/s、温度°C）
    DeviceTypeInfo::new(
        58,
        "消火栓流量",
        1,
        DataLength::Fixed(8),
        &[
            SingleParameterConfig::new("压力", DataType::I16, "MPa", 0.001, 0),
            SingleParameterConfig::new("倾角", DataType::I16, "°", 1.0, 2),
            SingleParameterConfig::new("流量", DataType::I16, "L/s", 0.01, 4),
            SingleParameterConfig::new("温度", DataType::I16, "°C", 0.1, 6),
        ],
    ),
    // 59: 门磁开关
    DeviceTypeInfo::new(
        59,
        "门磁开关",
        1,
        DataLength::Fixed(16),
        &[SingleParameterConfig::new(
            "红外计数",
            DataType::U16,
            "次",
            1.0,
            0,
        )],
    ),
    // 60: 电表
    DeviceTypeInfo::new(
        60,
        "电表",
        1,
        DataLength::Fixed(20),
        &[
            SingleParameterConfig::new("有功总电量", DataType::U32, "kWh", 0.01, 0),
            SingleParameterConfig::new("A相电压", DataType::U16, "V", 0.1, 4),
            SingleParameterConfig::new("B相电压", DataType::U16, "V", 0.1, 6),
            SingleParameterConfig::new("C相电压", DataType::U16, "V", 0.1, 8),
            SingleParameterConfig::new("A相电流", DataType::U16, "A", 0.01, 10),
            SingleParameterConfig::new("B相电流", DataType::U16, "A", 0.01, 12),
            SingleParameterConfig::new("C相电流", DataType::U16, "A", 0.01, 14),
            SingleParameterConfig::new("总有功功率", DataType::U16, "kW", 0.001, 16),
            SingleParameterConfig::new("总功率因数", DataType::U16, "", 0.001, 18),
        ],
    ),
    // 61: 压力
    DeviceTypeInfo::new(
        61,
        "压力",
        1,
        DataLength::Fixed(2),
        &[SingleParameterConfig::new(
            "压力",
            DataType::U16,
            "MPa",
            0.01,
            0,
        )],
    ),
    // 62: 振弦、温度
    DeviceTypeInfo::new(
        62,
        "振弦、温度",
        2,
        DataLength::Fixed(8),
        &[
            SingleParameterConfig::new("振弦", DataType::I32, "模数", 0.01, 0),
            SingleParameterConfig::new("温度", DataType::I32, "°C", 0.1, 4),
        ],
    ),
    // 63: 液位
    DeviceTypeInfo::new(
        63,
        "液位",
        2,
        DataLength::Fixed(2),
        &[SingleParameterConfig::new(
            "液位",
            DataType::U16,
            "m",
            0.001,
            0,
        )],
    ),
    // 64: 蓝绿藻
    DeviceTypeInfo::new(
        64,
        "蓝绿藻",
        2,
        DataLength::Fixed(8),
        &[
            SingleParameterConfig::new("蓝绿藻", DataType::I32, "cell/mL", 1.0, 0),
            SingleParameterConfig::new("温度", DataType::I32, "°C", 0.1, 4),
        ],
    ),
    // 65: 三通道温度（温度、温度、温度）
    DeviceTypeInfo::new(
        65,
        "三通道温度",
        1,
        DataLength::Fixed(6),
        &[
            SingleParameterConfig::new("温度1", DataType::I16, "°C", 0.1, 0),
            SingleParameterConfig::new("温度2", DataType::I16, "°C", 0.1, 2),
            SingleParameterConfig::new("温度3", DataType::I16, "°C", 0.1, 4),
        ],
    ),
    // 66: 双通道压力（压力、压力）
    DeviceTypeInfo::new(
        66,
        "双通道压力",
        2,
        DataLength::Fixed(4),
        &[
            SingleParameterConfig::new("压力1", DataType::I16, "MPa", 0.001, 0),
            SingleParameterConfig::new("压力2", DataType::I16, "kPa", 0.01, 2),
        ],
    ),
    // 67: 智能井盖监测终端
    DeviceTypeInfo::new(
        67,
        "智能井盖监测终端",
        2,
        DataLength::Fixed(4),
        &[
            SingleParameterConfig::new("水浸", DataType::I16, "", 1.0, 0),
            SingleParameterConfig::new("倾角", DataType::I16, "°", 1.0, 2),
        ],
    ),
    // 68: PH监测终端（PH、温度）
    DeviceTypeInfo::new(
        68,
        "PH监测终端",
        2,
        DataLength::Fixed(4),
        &[
            SingleParameterConfig::new("pH", DataType::I16, "pH", 0.01, 0),
            SingleParameterConfig::new("温度", DataType::I16, "°C", 0.1, 2),
        ],
    ),
    // 69: 电导率监测终端（电导率、温度）
    DeviceTypeInfo::new(
        69,
        "电导率监测终端",
        2,
        DataLength::Fixed(8),
        &[
            SingleParameterConfig::new("电导率", DataType::I32, "uS/cm", 0.1, 0),
            SingleParameterConfig::new("温度", DataType::I32, "°C", 0.1, 4),
        ],
    ),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_device_type_from_u8() {
        assert_eq!(
            DeviceType::from_u8(0).unwrap(),
            DeviceType::PressureGaugeKpa
        );
        assert_eq!(
            DeviceType::from_u8(1).unwrap(),
            DeviceType::PressureGaugeMpa
        );
        assert_eq!(
            DeviceType::from_u8(69).unwrap(),
            DeviceType::ConductivityMonitorTerminal
        );
        assert!(DeviceType::from_u8(70).is_err());
    }

    #[test]
    fn test_device_type_to_u8() {
        assert_eq!(DeviceType::PressureGaugeKpa.to_u8(), 0);
        assert_eq!(DeviceType::PressureGaugeMpa.to_u8(), 1);
        assert_eq!(DeviceType::ConductivityMonitorTerminal.to_u8(), 69);
    }

    #[test]
    fn test_device_type_info() {
        let device_type = DeviceType::PressureGaugeMpa;
        let info = device_type.info();

        assert_eq!(info.type_id, 1);
        assert_eq!(info.name, "压力表");
        assert_eq!(info.alarm_groups, 1);
        assert_eq!(info.data_length, DataLength::Fixed(2));
        assert_eq!(info.parameters.len(), 1);
        assert_eq!(info.parameters[0].name, "压力");
        assert_eq!(info.parameters[0].unit, "MPa");
        assert_eq!(info.parameters[0].lsb, 0.001);
    }

    #[test]
    fn test_variable_length_device() {
        let device_type = DeviceType::ModbusTerminal;
        assert!(device_type.is_variable_length());
    }

    #[test]
    fn test_multi_parameter_device() {
        let device_type = DeviceType::TemperaturePressureGauge;
        let params = device_type.get_parameters_info();

        assert_eq!(params.len(), 2);
        assert_eq!(params[0].name, "压力");
        assert_eq!(params[1].name, "温度");
    }
}
