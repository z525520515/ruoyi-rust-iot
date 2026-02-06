use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Linker 网关调用请求参数
/// 基于 README 协议规范设计
#[derive(Debug, Serialize, Deserialize)]
pub struct GatewayInvokeVO {
    /// 设备唯一标识的 Hash 值 (README 中的 deviceUniqueIDHash，Base64 编码)
    /// 例如: "ZGV2aWNlLWhhc2gtcGxhY2Vob2xkZXI="
    pub device_hash: String,

    /// 协议栈类型 (例如: "Control", "DTU", "JB3208", "NFS23030")
    /// README 中的 protocolStackType
    pub protocol_type: String,

    /// 协议栈名称 (例如: "ReservedControlProtocolStack")
    /// README 中的 protocolStackName
    pub stack_name: String,

    /// 指令动作模块 (例如: "system", "serial_port_manager")
    /// README 中的 action
    pub action: String,

    /// 指令方法名 (例如: "deviceUniqueID", "availableSerialPorts")
    /// README 中的 method
    pub method: String,

    /// 具体的业务参数 (根据方法不同而变化)
    /// README 中的 params
    #[serde(default)]
    pub params: Value,
}

/// ========== System 模块专用请求 ==========

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemInfoParams {}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChangePasswordParams {
    pub current_password: String,
    pub new_password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetTimeParams(pub i64);

#[derive(Debug, Serialize, Deserialize)]
pub struct SetTimeZoneParams(pub String);

#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceUniqueIDParams {}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceUniqueIDHashParams {}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationInfoParams {}

#[derive(Debug, Serialize, Deserialize)]
pub struct AvailableTimeZoneIDsParams {}

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemTimeZoneIDParams {}

#[derive(Debug, Serialize, Deserialize)]
pub struct ICCIDsParams {}

#[derive(Debug, Serialize, Deserialize)]
pub struct PerformRebootParams {}

/// ========== Serial Port Manager 模块专用请求 ==========

#[derive(Debug, Serialize, Deserialize)]
pub struct AvailableSerialPortsParams {}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenSerialPortParams {
    pub port_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CloseSerialPortParams {
    pub port_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CloseAllSerialPortsParams {
    pub port_names: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetBaudRateParams {
    pub port_name: String,
    pub baud_rate: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetDataBitsParams {
    pub port_name: String,
    pub data_bits: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetParityParams {
    pub port_name: String,
    pub parity: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetStopBitsParams {
    pub port_name: String,
    pub stop_bits: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetFlowControlParams {
    pub port_name: String,
    pub flow_control: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AvailableDataBitsParams {}

#[derive(Debug, Serialize, Deserialize)]
pub struct AvailableParitiesParams {}

#[derive(Debug, Serialize, Deserialize)]
pub struct AvailableStopBitsParams {}

#[derive(Debug, Serialize, Deserialize)]
pub struct AvailableFlowControlsParams {}