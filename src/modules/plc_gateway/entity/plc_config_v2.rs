// src/entity/plc_config.rs
use serde::{Deserialize, Serialize};
use validator::Validate;

// ==================== 公共结构 ====================

#[derive(Debug, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct BasePlcConfig {
    #[serde(default)]
    pub uuid: String,

    #[validate(length(min = 1, message = "name is required"))]
    pub name: String,

    #[serde(rename = "type")]
    pub plc_type: String,

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

// ==================== 响应结构 ====================

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlcConfigResponse {
    pub success: bool,
    pub message: String,
    pub uuid: String,
}

// ==================== 宏定义：自动生成 Config + new ====================

/// 简单配置：只有 base
macro_rules! declare_plc_config {
    ($name:ident, $variant:ident, $protocol:literal) => {
        #[derive(Debug, Deserialize, Serialize, Validate)]
        #[serde(rename_all = "camelCase")]
        pub struct $name {
            #[serde(flatten)]
            pub base: BasePlcConfig,

            #[serde(skip_deserializing)]
            pub protocol: String,
        }

        impl $name {
            pub fn new(base: BasePlcConfig) -> Self {
                Self {
                    base,
                    protocol: $protocol.to_string(),
                }
            }
        }
    };

    // 带额外字段的配置
    ($name:ident, $variant:ident, $protocol:literal, $($field:ident: $ty:ty),+) => {
        #[derive(Debug, Deserialize, Serialize, Validate)]
        #[serde(rename_all = "camelCase")]
        pub struct $name {
            #[serde(flatten)]
            pub base: BasePlcConfig,

            #[serde(skip_deserializing)]
            pub protocol: String,

            $(pub $field: $ty),+
        }

        impl $name {
            pub fn new(base: BasePlcConfig, $($field: $ty),+) -> Self {
                Self {
                    base,
                    protocol: $protocol.to_string(),
                    $($field),+
                }
            }
        }
    };
}

/// 专用宏：Modbus RTU（带复杂 validate）
macro_rules! declare_modbus_rtu_config {
    ($name:ident, $variant:ident, $protocol:literal) => {
        #[derive(Debug, Deserialize, Serialize, Validate)]
        #[serde(rename_all = "camelCase")]
        pub struct $name {
            #[serde(flatten)]
            pub base: BasePlcConfig,

            #[serde(skip_deserializing)]
            pub protocol: String,

            #[validate(range(min = 300, max = 115200, message = "baudRate must be between 300 and 115200"))]
            pub baud_rate: u32,

            #[validate(custom(function = "validate_data_bits"))]
            pub data_bits: u8,

            #[validate(custom(function = "validate_parity"))]
            pub parity: String,

            #[validate(custom(function = "validate_stop_bits"))]
            pub stop_bits: u8,
        }

        impl $name {
            pub fn new(
                base: BasePlcConfig,
                baud_rate: u32,
                data_bits: u8,
                parity: String,
                stop_bits: u8,
            ) -> Self {
                Self {
                    base,
                    protocol: $protocol.to_string(),
                    baud_rate,
                    data_bits,
                    parity,
                    stop_bits,
                }
            }
        }
    };
}

// ==================== 使用宏生成所有配置 ====================

// 西门子 S7
declare_plc_config!(
    SiemensS7Config,
    SiemensS7,
    "s7",
    rack: u8,
    slot: u8
);

// 三菱 MC
declare_plc_config!(
    MitsubishiMcConfig,
    MitsubishiMc,
    "mc",
    frame: String,
    mode: String
);

// OPC UA
declare_plc_config!(
    OpcUaConfig,
    OpcUa,
    "opcua",
    path: String
);

// Modbus TCP
declare_plc_config!(
    ModbusTcpConfig,
    ModbusTcp,
    "modbus_tcp"
);

// Modbus RTU over TCP
declare_plc_config!(
    ModbusRtuOverTcpConfig,
    ModbusRtuOverTcp,
    "modbus_rtu_over_tcp"
);

// Modbus RTU（专用宏）
declare_modbus_rtu_config!(
    ModbusRtuConfig,
    ModbusRtu,
    "modbus_rtu"
);

// ==================== 枚举定义 ====================

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
}

// ==================== 验证函数 ====================

fn validate_data_bits(data_bits: u8) -> Result<(), validator::ValidationError> {
    if [5, 6, 7, 8].contains(&data_bits) {
        Ok(())
    } else {
        Err(validator::ValidationError::new("dataBits must be 5, 6, 7, or 8"))
    }
}

fn validate_parity(parity: &str) -> Result<(), validator::ValidationError> {
    if ["none", "odd", "even"].contains(&parity) {
        Ok(())
    } else {
        Err(validator::ValidationError::new("parity must be none, odd, or even"))
    }
}

fn validate_stop_bits(stop_bits: u8) -> Result<(), validator::ValidationError> {
    if stop_bits == 1 || stop_bits == 2 {
        Ok(())
    } else {
        Err(validator::ValidationError::new("stopBits must be 1 or 2"))
    }
}

fn validate_frame(frame: &str) -> Result<(), validator::ValidationError> {
    if frame == "1E" || frame == "3E" {
        Ok(())
    } else {
        Err(validator::ValidationError::new("frame must be 1E or 3E"))
    }
}

fn validate_mode(mode: &str) -> Result<(), validator::ValidationError> {
    if mode == "ascii" || mode == "binary" {
        Ok(())
    } else {
        Err(validator::ValidationError::new("mode must be ascii or binary"))
    }
}