use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::modules::plc_gateway::util::config_util::{validate_frame, validate_mode};
// ==================== 公共结构和枚举 ====================

#[derive(Debug, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct BasePlcConfig {
    #[serde(default)]
    pub uuid: String,

    #[validate(length(min = 1, message = "name is required"))]
    pub name: String,

    #[serde(rename = "type", skip_deserializing)]
    pub plc_type: String,

    // #[validate(length(min = 1, message = "protocol is required"))]
    // pub protocol: String,
    #[validate(custom(function = "validate_interface"))]
    pub interface: String,

    #[serde(default)]
    pub disable: bool,

    pub ip: String,
    pub port: u16,
}

fn validate_interface(interface: &str) -> Result<(), validator::ValidationError> {
    let valid_interfaces = ["eth0", "eth1", "rs485-1", "rs485-2"];
    if valid_interfaces.contains(&interface) {
        Ok(())
    } else {
        Err(validator::ValidationError::new("invalid interface"))
    }
}

#[derive(Debug, Deserialize)]
#[serde(tag = "protocol")]
pub enum PlcConfigRequest {
    #[serde(rename = "s7")]
    SiemensS7(SiemensS7Config),
    #[serde(rename = "mc")]
    MitsubishiMc(MitsubishiMcConfig),
    #[serde(rename = "opcua")]
    OpcUa(OpcUaConfig),
    #[serde(rename = "modbus_tcp")]
    ModbusTcp(ModbusTcpConfig),
    #[serde(rename = "modbus_rtu_over_tcp")]
    ModbusRtuOverTcp(ModbusRtuOverTcpConfig),
    #[serde(rename = "modbus_rtu")]
    ModbusRtu(ModbusRtuConfig),
    #[serde(rename = "dlt645_2007")]
    DLT(DLT645_2007Config),
}
// #[derive(Debug, Deserialize)]
// #[serde(tag = "protocol", rename_all = "snake_case")]
// pub enum PlcConfigRequest {
//     s7(SiemensS7Config),
//     mc(MitsubishiMcConfig),
//     opcua(OpcUaConfig),
//     modbus_tcp(ModbusTcpConfig),
//     modbus_rtu_over_tcp(ModbusRtuOverTcpConfig),
//     modbus_rtu(ModbusRtuConfig),
// }

// ==================== 西门子S7配置 ====================

#[derive(Debug, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SiemensS7Config {
    #[serde(flatten)]
    pub base: BasePlcConfig,

    #[validate(range(min = 0, max = 100, message = "rack must be between 0 and 100"))]
    pub rack: u8,

    #[validate(range(min = 0, max = 100, message = "slot must be between 0 and 100"))]
    pub slot: u8,

    #[serde(skip_deserializing)]
    pub protocol: String, // 手动加回来（用于序列化输出）
}

impl SiemensS7Config {
    pub fn new(base: BasePlcConfig, rack: u8, slot: u8) -> Self {
        Self {
            base,
            rack,
            slot,
            protocol: "s7".to_string(),
        }
    }
}

// ==================== 三菱MC配置 ====================

#[derive(Debug, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MitsubishiMcConfig {
    #[serde(flatten)]
    pub base: BasePlcConfig,

    #[validate(custom(function = "validate_frame"))]
    #[serde(skip_deserializing, default = "default_frame")]
    pub frame: String,

    #[validate(custom(function = "validate_mode"))]
    #[serde(skip_deserializing, default = "default_mode")]
    pub mode: String,

    #[serde(skip_deserializing)]
    pub protocol: String, // 手动加回来（用于序列化输出）
}


fn default_frame() -> String {
    "1E".to_string()
}

fn default_mode() -> String {
    "binary".to_string() // 或 "ascii"，看你默认策略
}
impl MitsubishiMcConfig {
    pub fn new(base: BasePlcConfig, frame: String, mode: String) -> Self {
        Self {
            base,
            frame,
            mode,
            protocol: "mc".to_string(),
        }
    }
}

// ==================== OPC UA配置 ====================

#[derive(Debug, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct OpcUaConfig {
    #[serde(flatten)]
    pub base: BasePlcConfig,

    #[serde(default)]
    pub path: String,
    #[serde(skip_deserializing)]
    pub protocol: String, // 手动加回来（用于序列化输出）
}

impl OpcUaConfig {
    pub fn new(base: BasePlcConfig, path: String) -> Self {
        Self {
            base,
            path,
            protocol: "opcua".to_string(),
        }
    }
}

// ==================== Modbus TCP配置 ====================

#[derive(Debug, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ModbusTcpConfig {
    #[serde(skip_deserializing)]
    pub protocol: String, // 手动加回来（用于序列化输出）

    #[serde(flatten)]
    pub base: BasePlcConfig,
}

impl ModbusTcpConfig {
    pub fn new(base: BasePlcConfig) -> Self {
        Self {
            base,
            protocol: "modbus_tcp".to_string(),
        }
    }
}

// ==================== Modbus RTU over TCP配置 ====================

#[derive(Debug, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ModbusRtuOverTcpConfig {
    #[serde(skip_deserializing)]
    pub protocol: String, // 手动加回来（用于序列化输出）

    #[serde(flatten)]
    pub base: BasePlcConfig,
}

impl ModbusRtuOverTcpConfig {
    pub fn new(base: BasePlcConfig) -> Self {
        Self {
            base,
            protocol: "modbus_rtu_over_tcp".to_string(),
        }
    }
}

// ==================== Modbus RTU配置 ====================

#[derive(Debug, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ModbusRtuConfig {
    #[serde(skip_deserializing)]
    pub protocol: String, // 手动加回来（用于序列化输出）

    #[serde(flatten)]
    pub base: BasePlcConfig,

    #[validate(custom(function = "validate_data_baud_rate"))]
    pub baud_rate: u32,

    #[validate(custom(function = "validate_data_bits"))]
    pub data_bits: u8,

    #[validate(custom(function = "validate_parity"))]
    pub parity: String,

    #[validate(custom(function = "validate_stop_bits"))]
    pub stop_bits: u8,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DLT645_2007Config {
    #[serde(skip_deserializing)]
    pub protocol: String, // 手动加回来（用于序列化输出）

    #[serde(flatten)]
    pub base: BasePlcConfig,

    #[validate(custom(function = "validate_data_baud_rate"))]
    pub baud_rate: u32,

    #[validate(custom(function = "validate_data_bits"))]
    pub data_bits: u8,

    #[validate(custom(function = "validate_parity"))]
    pub parity: String,

    #[validate(custom(function = "validate_stop_bits"))]
    pub stop_bits: u8,
}

fn validate_data_bits(data_bits: u8) -> Result<(), validator::ValidationError> {
    if [5, 6, 7, 8].contains(&data_bits) {
        Ok(())
    } else {
        Err(validator::ValidationError::new(
            "dataBits must be 5, 6, 7, or 8",
        ))
    }
}

fn validate_data_baud_rate(baud_rate: u32) -> Result<(), validator::ValidationError> {
    if baud_rate == 2400
        || baud_rate == 4800
        || baud_rate == 9600
        || baud_rate == 19200
        || baud_rate == 38400
        || baud_rate == 57600
        || baud_rate == 115200
    {
        Ok(())
    } else {
        Err(validator::ValidationError::new(
            "baudRate must be 2400, 4800, 9600, 19200, 38400, 57600, or 115200",
        ))
    }
}

fn validate_parity(parity: &str) -> Result<(), validator::ValidationError> {
    if parity == "none" || parity == "odd" || parity == "even" {
        Ok(())
    } else {
        Err(validator::ValidationError::new(
            "parity must be none, odd, or even",
        ))
    }
}

fn validate_stop_bits(stop_bits: u8) -> Result<(), validator::ValidationError> {
    if *&stop_bits == 1 || *&stop_bits == 2 {
        Ok(())
    } else {
        Err(validator::ValidationError::new("stopBits must be 1 or 2"))
    }
}

impl ModbusRtuConfig {
    pub fn new(
        base: BasePlcConfig,
        baud_rate: u32,
        data_bits: u8,
        parity: String,
        stop_bits: u8,
    ) -> Self {
        Self {
            base,
            baud_rate,
            data_bits,
            parity,
            stop_bits,
            protocol: "modbus_rtu".to_string(),
        }
    }
}

impl Validate for PlcConfigRequest {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        match self {
            PlcConfigRequest::SiemensS7(config) => config.validate(),
            PlcConfigRequest::MitsubishiMc(config) => config.validate(),
            PlcConfigRequest::OpcUa(config) => config.validate(),
            PlcConfigRequest::ModbusTcp(config) => config.validate(),
            PlcConfigRequest::ModbusRtuOverTcp(config) => config.validate(),
            PlcConfigRequest::ModbusRtu(config) => config.validate(),
            PlcConfigRequest::DLT(config) => config.validate(),
        }
    }
}
