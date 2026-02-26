use crate::modules::plc_gateway::api::config_util::call_device;
use crate::modules::plc_gateway::unified::response::error_response;
use crate::modules::plc_gateway::util::config_util::{validate_frame, validate_mode};
use axum::{extract::Path, response::Json};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use validator::{Validate, ValidationErrors};

/// 默认的 uuid（空字符串），在序列化时也会输出 ""
fn default_uuid() -> String {
    "".to_string()
}

/// 所有分组共有的字段
#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct BaseGroup {
    #[serde(skip_deserializing, default = "default_uuid")]
    pub uuid: String,
    #[validate(length(min = 1, message = "gid 不能为空"))]
    pub gid: String,
    #[validate(length(min = 1, message = "name 不能为空"))]
    pub name: String,
}

/* ------------------- Modbus ------------------- */
#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PlcModbusGroupReq {
    #[serde(skip_deserializing)]
    pub protocol: String,
    #[serde(flatten)]
    pub base: BaseGroup,
    pub pid: String,
    pub slave_address: u8,
    #[validate(range(min = 0, max = 100, message = "param_r1 must be between 0 and 100"))]
    pub param_r1: u16,
    #[validate(range(min = 0, max = 100, message = "param_r2 must be between 0 and 100"))]
    pub param_r2: u16,
}
impl PlcModbusGroupReq {
    pub fn new(
        base: BaseGroup,
        pid: String,
        slave_address: u8,
        param_r1: u16,
        param_r2: u16,
    ) -> Self {
        Self {
            base,
            pid,
            slave_address,
            param_r1,
            param_r2,
            protocol: "modbus_tcp".to_string(),
        }
    }
}

/* ------------------- Modbus ------------------- */
#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PlcModbusTcpGroupReq {
    #[serde(skip_deserializing)]
    pub protocol: String,
    #[serde(flatten)]
    pub base: BaseGroup,
    pub pid: String,
    pub slave_address: u8,
    #[validate(range(min = 0, max = 100, message = "param_r1 must be between 0 and 100"))]
    pub param_r1: u16,
    #[validate(range(min = 0, max = 100, message = "param_r2 must be between 0 and 100"))]
    pub param_r2: u16,
}
impl PlcModbusTcpGroupReq {
    pub fn new(
        base: BaseGroup,
        pid: String,
        slave_address: u8,
        param_r1: u16,
        param_r2: u16,
    ) -> Self {
        Self {
            base,
            pid,
            slave_address,
            param_r1,
            param_r2,
            protocol: "modbus_tcp".to_string(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PlcModbusRtuOverGroupReq {
    #[serde(skip_deserializing)]
    pub protocol: String,
    #[serde(flatten)]
    pub base: BaseGroup,
    pub pid: String,
    pub slave_address: u8,
    #[validate(range(min = 0, max = 100, message = "param_r1 must be between 0 and 100"))]
    pub param_r1: u16,
    #[validate(range(min = 0, max = 100, message = "param_r2 must be between 0 and 100"))]
    pub param_r2: u16,
}
impl PlcModbusRtuOverGroupReq {
    pub fn new(
        base: BaseGroup,
        pid: String,
        slave_address: u8,
        param_r1: u16,
        param_r2: u16,
    ) -> Self {
        Self {
            base,
            pid,
            slave_address,
            param_r1,
            param_r2,
            protocol: "modbus_rtu_over_tcp".to_string(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PlcModbusRtuGroupReq {
    #[serde(skip_deserializing)]
    pub protocol: String,
    #[serde(flatten)]
    pub base: BaseGroup,
    pub pid: String,
    pub slave_address: u8,
    #[validate(range(min = 0, max = 100, message = "param_r1 must be between 0 and 100"))]
    pub param_r1: u16,
    #[validate(range(min = 0, max = 100, message = "param_r2 must be between 0 and 100"))]
    pub param_r2: u16,
}
impl PlcModbusRtuGroupReq {
    pub fn new(
        base: BaseGroup,
        pid: String,
        slave_address: u8,
        param_r1: u16,
        param_r2: u16,
    ) -> Self {
        Self {
            base,
            pid,
            slave_address,
            param_r1,
            param_r2,
            protocol: "modbus_rtu".to_string(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PlcDLT645GroupReq {
    #[serde(skip_deserializing)]
    pub protocol: String,
    #[serde(flatten)]
    pub base: BaseGroup,
    pub pid: String,
    pub slave_address: u8,
    #[validate(range(min = 0, max = 100, message = "param_r1 must be between 0 and 100"))]
    pub param_r1: u16,
    #[validate(range(min = 0, max = 100, message = "param_r2 must be between 0 and 100"))]
    pub param_r2: u16,
}
impl PlcDLT645GroupReq {
    pub fn new(
        base: BaseGroup,
        pid: String,
        slave_address: u8,
        param_r1: u16,
        param_r2: u16,
    ) -> Self {
        Self {
            base,
            pid,
            slave_address,
            param_r1,
            param_r2,
            protocol: "dlt645_2007".to_string(),
        }
    }
}

/* ------------------- S7 ------------------- */
#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PlcS7GroupReq {
    #[serde(skip_deserializing)]
    pub protocol: String,
    #[serde(flatten)]
    pub base: BaseGroup,
    pub pid: String,
    #[validate(range(min = 0, max = 100, message = "rack must be between 0 and 100"))]
    pub rack: u16,
    #[validate(range(min = 0, max = 100, message = "slot must be between 0 and 100"))]
    pub slot: u16,
}
impl PlcS7GroupReq {
    pub fn new(base: BaseGroup, pid: String, rack: u16, slot: u16) -> Self {
        Self {
            base,
            pid,
            rack,
            slot,
            protocol: "s7".to_string(),
        }
    }
}

/* ------------------- MC ------------------- */
#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PlcMcGroupReq {
    #[serde(skip_deserializing)]
    pub protocol: String,
    #[serde(flatten)]
    pub base: BaseGroup,
    // 父级PLC测点配置的uuid
    pub pid: String,
    #[validate(custom(function = "validate_frame"))]
    pub frame: String,
    #[validate(custom(function = "validate_mode"))]
    pub mode: String,
}
impl PlcMcGroupReq {
    pub fn new(base: BaseGroup, pid: String, frame: String, mode: String) -> Self {
        Self {
            base,
            pid,
            frame,
            mode,
            protocol: "mc".to_string(),
        }
    }
}

/* ------------------- OPC UA ------------------- */
#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PlcOpcuaGroupReq {
    #[serde(skip_deserializing)]
    pub protocol: String,
    #[serde(flatten)]
    pub base: BaseGroup,
    pub pid: String,
    pub path: Option<String>,
}
impl PlcOpcuaGroupReq {
    pub fn new(base: BaseGroup, pid: String, path: Option<String>) -> Self {
        Self {
            base,
            pid,
            path,
            protocol: "opcua".to_string(),
        }
    }
}

/* ------------------- 统一的枚举 ------------------- */
#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "protocol")]
pub enum PlcGroupConfigRequest {
    #[serde(rename = "s7")]
    SiemensS7(PlcS7GroupReq),

    #[serde(rename = "mc")]
    MitsubishiMc(PlcMcGroupReq),

    #[serde(rename = "opcua")]
    OpcUa(PlcOpcuaGroupReq),

    #[serde(rename = "modbus_tcp")]
    ModbusTcp(PlcModbusGroupReq),

    #[serde(rename = "modbus_rtu_over_tcp")]
    ModbusRtuOverTcp(PlcModbusRtuOverGroupReq),

    #[serde(rename = "modbus_rtu")]
    ModbusRtu(PlcModbusRtuGroupReq),

    #[serde(rename = "dlt645_2007")]
    DLT(PlcDLT645GroupReq),
}

impl Validate for PlcGroupConfigRequest {
    fn validate(&self) -> Result<(), ValidationErrors> {
        match self {
            PlcGroupConfigRequest::SiemensS7(c) => c.validate(),
            PlcGroupConfigRequest::MitsubishiMc(c) => c.validate(),
            PlcGroupConfigRequest::OpcUa(c) => c.validate(),
            PlcGroupConfigRequest::ModbusTcp(c) => c.validate(),
            PlcGroupConfigRequest::ModbusRtuOverTcp(c) => c.validate(),
            PlcGroupConfigRequest::ModbusRtu(c) => c.validate(),
            PlcGroupConfigRequest::DLT(c) => c.validate(),
        }
    }
}

/* ------------------- 统一入口 ------------------- */
pub async fn add_plc_config_group(
    Path(ip): Path<String>,
    Json(params): Json<Value>,
) -> Json<Value> {
    log::info!("add plc config: ip={}, params={:#?}", ip, params);
    // ---------- 1. 若缺少 protocol， ----------
    if params.get("protocol").is_none() {
        return error_response("缺失【protocol】字段".to_string(), Some(400));
    }

    // ---------- 2. 反序列化 + 自动校验 ----------
    let request: PlcGroupConfigRequest = match serde_json::from_value(params.clone()) {
        Ok(r) => r,
        Err(e) => {
            log::error!("Failed to parse request: {}", e);
            return error_response(format!("Invalid request format: {}", e), Some(400));
        }
    };

    if let Err(errors) = request.validate() {
        log::warn!("Validation failed: {:?}", errors);
        return error_response(format!("Validation error: {:?}", errors), Some(400));
    }

    // ---------- 3. 序列化成要转发给设备的 JSON ----------
    // 此时 uuid 已经是 ""（来自 default_uuid），不需要再手动插入
    let forward = match serde_json::to_value(&request) {
        Ok(v) => v,
        Err(e) => {
            log::error!("Failed to serialize request: {}", e);
            return error_response(
                "Internal error while serializing request".to_string(),
                Some(500),
            );
        }
    };

    log::info!(
        "add plc config group: ip={}, forward_payload={:?}",
        ip,
        forward
    );

    let url = format!("http://{}:80/plc/api/v1/group", ip);
    call_device(url, forward, Some("POST")).await
}
