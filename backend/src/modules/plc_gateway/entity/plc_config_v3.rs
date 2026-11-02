// src/entity/plc_config.rs
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError, ValidationErrors};

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

fn validate_interface(interface: &str) -> Result<(), ValidationError> {
    let valid = ["eth0", "eth1", "rs485-1", "rs485-2"];
    if valid.contains(&interface) {
        Ok(())
    } else {
        Err(ValidationError::new("invalid interface"))
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlcConfigResponse {
    pub success: bool,
    pub message: String,
    pub uuid: String,
}

// ==================== 终极宏：plc_config! ====================

macro_rules! plc_config {
    // 空配置
    ($name:ident, $variant:ident, $protocol:literal) => {
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "camelCase")]
        pub struct $name {
            #[serde(flatten)]
            pub base: BasePlcConfig,
            #[serde(skip_deserializing)]
            pub protocol: String,
        }
        
        impl $name {
            pub fn new(base: BasePlcConfig) -> Self {
                Self { base, protocol: $protocol.to_string() }
            }
        }
        
        impl Validate for $name {
            fn validate(&self) -> Result<(), ValidationErrors> {
                self.base.validate()
            }
        }
    };

    // 带字段 + 验证
    ($name:ident, $variant:ident, $protocol:literal, $($field:ident: $ty:ty [$($rule:tt)*]),+) => {
        #[derive(Debug, Deserialize, Serialize)]
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
                Self { base, protocol: $protocol.to_string(), $($field),+ }
            }
        }
        
        impl Validate for $name {
        fn validate(&self) -> Result<(), ValidationErrors> {
            // 先验证 base
            self.base.validate()
        }
    }
    };
}

// 移除有问题的辅助验证函数，直接在手动实现中进行验证

// 手动实现具体的验证逻辑
impl Validate for SiemensS7Config {
    fn validate(&self) -> Result<(), ValidationErrors> {
        self.base.validate()?;
        
        let mut errors = ValidationErrors::new();
        
        if self.rack < 0 || self.rack > 255 {
            errors.add("rack", ValidationError::new("rack must be between 0 and 255"));
        }
        
        if self.slot < 0 || self.slot > 255 {
            errors.add("slot", ValidationError::new("slot must be between 0 and 255"));
        }
        
        if errors.is_empty() { Ok(()) } else { Err(errors) }
    }
}

impl Validate for MitsubishiMcConfig {
    fn validate(&self) -> Result<(), ValidationErrors> {
        self.base.validate()?;
        
        let mut errors = ValidationErrors::new();
        
        if self.frame != "1E" && self.frame != "3E" {
            errors.add("frame", ValidationError::new("frame must be 1E or 3E"));
        }
        
        if self.mode != "ascii" && self.mode != "binary" {
            errors.add("mode", ValidationError::new("mode must be ascii or binary"));
        }
        
        if errors.is_empty() { Ok(()) } else { Err(errors) }
    }
}

impl Validate for OpcUaConfig {
    fn validate(&self) -> Result<(), ValidationErrors> {
        self.base.validate()
    }
}

impl Validate for ModbusRtuConfig {
    fn validate(&self) -> Result<(), ValidationErrors> {
        self.base.validate()?;
        
        let mut errors = ValidationErrors::new();
        
        if self.baud_rate < 300 || self.baud_rate > 115200 {
            errors.add("baud_rate", ValidationError::new("baud_rate must be between 300 and 115200"));
        }
        
        if ![5, 6, 7, 8].contains(&self.data_bits) {
            errors.add("data_bits", ValidationError::new("data_bits must be 5, 6, 7, or 8"));
        }
        
        if self.parity != "none" && self.parity != "odd" && self.parity != "even" {
            errors.add("parity", ValidationError::new("parity must be none, odd, or even"));
        }
        
        if self.stop_bits != 1 && self.stop_bits != 2 {
            errors.add("stop_bits", ValidationError::new("stop_bits must be 1 or 2"));
        }
        
        if errors.is_empty() { Ok(()) } else { Err(errors) }
    }
}

// ==================== 手动声明所有配置 ====================

// 简单配置
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ModbusTcpConfig {
    #[serde(flatten)]
    pub base: BasePlcConfig,
    #[serde(skip_deserializing)]
    pub protocol: String,
}

impl ModbusTcpConfig {
    pub fn new(base: BasePlcConfig) -> Self {
        Self { base, protocol: "modbus_tcp".to_string() }
    }
}

impl Validate for ModbusTcpConfig {
    fn validate(&self) -> Result<(), ValidationErrors> {
        self.base.validate()
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ModbusRtuOverTcpConfig {
    #[serde(flatten)]
    pub base: BasePlcConfig,
    #[serde(skip_deserializing)]
    pub protocol: String,
}

impl ModbusRtuOverTcpConfig {
    pub fn new(base: BasePlcConfig) -> Self {
        Self { base, protocol: "modbus_rtu_over_tcp".to_string() }
    }
}

impl Validate for ModbusRtuOverTcpConfig {
    fn validate(&self) -> Result<(), ValidationErrors> {
        self.base.validate()
    }
}

// 带字段配置
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SiemensS7Config {
    #[serde(flatten)]
    pub base: BasePlcConfig,
    #[serde(skip_deserializing)]
    pub protocol: String,
    pub rack: u8,
    pub slot: u8,
}

impl SiemensS7Config {
    pub fn new(base: BasePlcConfig, rack: u8, slot: u8) -> Self {
        Self { base, protocol: "s7".to_string(), rack, slot }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MitsubishiMcConfig {
    #[serde(flatten)]
    pub base: BasePlcConfig,
    #[serde(skip_deserializing)]
    pub protocol: String,
    pub frame: String,
    pub mode: String,
}

impl MitsubishiMcConfig {
    pub fn new(base: BasePlcConfig, frame: String, mode: String) -> Self {
        Self { base, protocol: "mc".to_string(), frame, mode }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OpcUaConfig {
    #[serde(flatten)]
    pub base: BasePlcConfig,
    #[serde(skip_deserializing)]
    pub protocol: String,
    pub path: String,
}

impl OpcUaConfig {
    pub fn new(base: BasePlcConfig, path: String) -> Self {
        Self { base, protocol: "opcua".to_string(), path }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ModbusRtuConfig {
    #[serde(flatten)]
    pub base: BasePlcConfig,
    #[serde(skip_deserializing)]
    pub protocol: String,
    pub baud_rate: u32,
    pub data_bits: u8,
    pub parity: String,
    pub stop_bits: u8,
}

impl ModbusRtuConfig {
    pub fn new(base: BasePlcConfig, baud_rate: u32, data_bits: u8, parity: String, stop_bits: u8) -> Self {
        Self { base, protocol: "modbus_rtu".to_string(), baud_rate, data_bits, parity, stop_bits }
    }
}

// ==================== 枚举定义 ====================

#[derive(Debug, Deserialize)]
#[serde(tag = "protocol")]
pub enum PlcConfigRequest {
    #[serde(rename = "s7")] SiemensS7(SiemensS7Config),
    #[serde(rename = "mc")] MitsubishiMc(MitsubishiMcConfig),
    #[serde(rename = "opcua")] OpcUa(OpcUaConfig),
    #[serde(rename = "modbus_tcp")] ModbusTcp(ModbusTcpConfig),
    #[serde(rename = "modbus_rtu_over_tcp")] ModbusRtuOverTcp(ModbusRtuOverTcpConfig),
    #[serde(rename = "modbus_rtu")] ModbusRtu(ModbusRtuConfig),
}