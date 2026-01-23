use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GatewayInvokeVO {
    /// 设备唯一标识的 Hash 值 (README 中的 deviceUniqueIDHash)
    pub device_hash: String,
    /// 协议栈类型 (例如: Control)
    pub protocol_type: String,
    /// 协议栈名称 (例如: ReservedControlProtocolStack)
    pub action: String,
    /// 方法名 (例如: deviceUniqueID)
    pub method: String,
    /// 具体的业务参数
    pub params: Value,
}