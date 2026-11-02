use crate::modules::plc_gateway::api::config_util::call_device;
use crate::modules::plc_gateway::unified::response::error_response;
use axum::{extract::Path, response::Json};
use serde::Deserialize;
use serde_json::{Value, json};

// ====================== 协议枚举 ======================
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Protocol {
    Modbus,
    Dlt645,
    S7,
    Mc,
    Opcua,
}

// ====================== 共用请求结构体 ======================
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct CommonGroupReq {
    pub gid: String,
    pub name: String,
    pub pid: String,
    pub protocol: String,
    #[serde(default)]
    pub uuid: String, // update 时必填，add 时会覆盖
}

// ====================== 协议特有请求 ======================
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ModbusExtra {
    pub slave_address: u8,
    pub param_r1: u16,
    pub param_r2: u16,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Dlt645Extra {
    pub slave_address: String,
    pub param_r1: u16,
    pub param_r2: u16,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct S7Extra {
    pub rack: u16,
    pub slot: u16,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct McExtra {
    pub frame: String,
    pub mode: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct OpcuaExtra {
    pub path: Option<String>,
}

// ====================== Trait: 协议行为 ======================
trait GroupHandler {
    type Extra: for<'de> Deserialize<'de>;

    /// 支持的协议字符串（支持多个别名）
    fn protocols() -> &'static [&'static str];

    /// 解析额外字段
    fn parse_extra(payload: &Value) -> Result<Self::Extra, serde_json::Error>;

    /// 校验（可扩展）
    fn validate(common: &CommonGroupReq, extra: &Self::Extra) -> Result<(), String>;

    /// 构造转发 payload
    fn build_forward( common: &CommonGroupReq, extra: &Self::Extra) -> Value;
}

// ====================== 实现各协议 ======================
// --- Modbus ---
struct ModbusHandler;
impl GroupHandler for ModbusHandler {
    type Extra = ModbusExtra;

    fn protocols() -> &'static [&'static str] {
        &["modbus_tcp", "modbus_rtu_over_tcp", "modbus_rtu"]
    }

    fn parse_extra(payload: &Value) -> Result<Self::Extra, serde_json::Error> {
        serde_json::from_value(payload.clone())
    }

    fn validate( _common: &CommonGroupReq, extra: &Self::Extra) -> Result<(), String> {
        if extra.slave_address == 0 {
            Err("Slave address cannot be 0".to_string())
        } else {
            Ok(())
        }
    }

    fn build_forward( common: &CommonGroupReq, extra: &Self::Extra) -> Value {
        json!({
            "gid": common.gid,
            "uuid": common.uuid,
            "name": common.name,
            "pid": common.pid,
            "protocol": common.protocol,
            "slaveAddress": extra.slave_address,
            "paramR1": extra.param_r1,
            "paramR2": extra.param_r2
        })
    }
}

// --- DLT645 ---
struct Dlt645Handler;
impl GroupHandler for Dlt645Handler {
    type Extra = Dlt645Extra;

    fn protocols() -> &'static [&'static str] { &["dlt645_2007"] }

    fn parse_extra(payload: &Value) -> Result<Self::Extra, serde_json::Error> {
        serde_json::from_value(payload.clone())
    }

    fn validate(_common: &CommonGroupReq, _: &Self::Extra) -> Result<(), String> { Ok(()) }
    
    fn build_forward( common: &CommonGroupReq, extra: &Self::Extra) -> Value {
        json!({
            "gid": common.gid,
            "uuid": common.uuid,
            "name": common.name,
            "pid": common.pid,
            "protocol": common.protocol,
            "slaveAddress": extra.slave_address,
            "paramR1": extra.param_r1,
            "paramR2": extra.param_r2
        })
    }
}

// --- S7 ---
struct S7Handler;
impl GroupHandler for S7Handler {
    type Extra = S7Extra;

    fn protocols() -> &'static [&'static str] { &["s7"] }

    fn parse_extra(payload: &Value) -> Result<Self::Extra, serde_json::Error> {
        serde_json::from_value(payload.clone())
    }

    fn validate( common: &CommonGroupReq, _: &Self::Extra) -> Result<(), String> {
        if common.protocol != "s7" {
            Err("Protocol must be 's7' for S7 requests".to_string())
        } else {
            Ok(())
        }
    }

    fn build_forward( common: &CommonGroupReq, extra: &Self::Extra) -> Value {
        json!({
            "gid": common.gid,
            "uuid": common.uuid,
            "name": common.name,
            "pid": common.pid,
            "protocol": common.protocol,
            "rack": extra.rack,
            "slot": extra.slot
        })
    }
}

// --- MC ---
struct McHandler;
impl GroupHandler for McHandler {
    type Extra = McExtra;

    fn protocols() -> &'static [&'static str] { &["mc"] }

    fn parse_extra(payload: &Value) -> Result<Self::Extra, serde_json::Error> {
        serde_json::from_value(payload.clone())
    }

    fn validate( common: &CommonGroupReq, _: &Self::Extra) -> Result<(), String> {
        if common.protocol != "mc" {
            Err("Protocol must be 'mc' for MC requests".to_string())
        } else {
            Ok(())
        }
    }

    fn build_forward(common: &CommonGroupReq, extra: &Self::Extra) -> Value {
        json!({
            "gid": common.gid,
            "uuid": common.uuid,
            "name": common.name,
            "pid": common.pid,
            "protocol": common.protocol,
            "frame": extra.frame,
            "mode": extra.mode
        })
    }
}

// --- OPCUA ---
struct OpcuaHandler;
impl GroupHandler for OpcuaHandler {
    type Extra = OpcuaExtra;

    fn protocols() -> &'static [&'static str] { &["opcua"] }

    fn parse_extra(payload: &Value) -> Result<Self::Extra, serde_json::Error> {
        serde_json::from_value(payload.clone())
    }

    fn validate( common: &CommonGroupReq, _: &Self::Extra) -> Result<(), String> {
        if common.protocol != "opcua" {
            Err("Protocol must be 'opcua' for OPCUA requests".to_string())
        } else {
            Ok(())
        }
    }

    fn build_forward( common: &CommonGroupReq, extra: &Self::Extra) -> Value {
        json!({
            "gid": common.gid,
            "uuid": common.uuid,
            "name": common.name,
            "pid": common.pid,
            "protocol": common.protocol,
            "path": extra.path
        })
    }
}



// ====================== 统一处理 ======================
async fn handle_plc_group(
    ip: String,
    mut payload: Value,
    is_add: bool, // true=add, false=update
) -> Json<Value> {
    // --- 1. uuid 处理 ---
    if is_add {
        payload["uuid"] = json!("");
    } else {
        let uuid = payload.get("uuid").and_then(|v| v.as_str()).unwrap_or("");
        if uuid.is_empty() {
            return error_response("uuid is required".to_string(), Some(400));
        }
    }

    // --- 2. 解析 protocol ---
    let protocol_str = payload
        .get("protocol")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    if protocol_str.is_empty() {
        return error_response("protocol is required".to_string(), Some(400));
    }

    // --- 3. 解析 common ---
    let common: CommonGroupReq = match serde_json::from_value(payload.clone()) {
        Ok(c) => c,
        Err(e) => return error_response(format!("Invalid common fields: {}", e), Some(400)),
    };

    // --- 4. 分发到处理器 ---
    macro_rules! dispatch_handler {
        ($($handler:ty),*) => {
            $(
                if <$handler>::protocols().contains(&protocol_str.as_str()) {
                    return process_with_handler::<$handler>(&ip, &common, &payload, is_add).await;
                }
            )*
        };
    }

    dispatch_handler!(
        ModbusHandler,
        Dlt645Handler,
        S7Handler,
        McHandler,
        OpcuaHandler
    );

    error_response(format!("Unsupported protocol: {}", protocol_str), Some(400))
}

// --- 使用具体 handler ---
async fn process_with_handler<H: GroupHandler>(
    ip: &str,
    common: &CommonGroupReq,
    payload: &Value,
    is_add: bool,
) -> Json<Value> {
    let extra = match H::parse_extra(payload) {
        Ok(e) => e,
        Err(e) => return error_response(format!("Invalid payload: {}", e), Some(400)),
    };

    
   if let Err(msg) = H::validate(common, &extra) {
        return error_response(msg, Some(400));
    }

    let forward = H::build_forward(common, &extra);

    log::info!(
        "{} plc config group: ip={}, payload={:?}",
        if is_add { "add" } else { "update" },
        ip,
        forward
    );

    let url = if is_add {
        format!("http://{}:80/plc/api/v1/group", ip)
    } else {
        format!("http://{}:80/plc/api/v1/group/{}", ip, common.uuid)
    };

    call_device(url, forward, Some(if is_add { "POST" } else { "PUT" })).await
}

pub async fn add_plc_config_group(
    Path(ip): Path<String>,
    Json(payload): Json<Value>,
) -> Json<Value> {
    handle_plc_group(ip, payload, true).await
}

pub async fn update_plc_config_group(
    Path(ip): Path<String>,
    Json(payload): Json<Value>,
) -> Json<Value> {
    handle_plc_group(ip, payload, false).await
}




pub async fn del_plc_config_group(Path((ip, uuid)): Path<(String, String)>) -> Json<Value> {
    let url = format!("http://{}/plc/api/v1/group/{}", &ip, uuid);
    call_device(url, json!([]), Some("DELETE")).await
}