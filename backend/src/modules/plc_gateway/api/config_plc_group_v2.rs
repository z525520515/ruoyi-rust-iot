use crate::modules::plc_gateway::api::config_util::call_device;
use crate::modules::plc_gateway::unified::response::error_response;
use axum::{extract::Path, response::Json};
use serde::Deserialize;
use serde_json::{Value, json};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlcModbusGroupReq {
    pub gid: String,
    pub name: String,
    // 父级PLC测点配置的uuid
    pub pid: String,
    pub slave_address: u8,
    pub param_r1: u16,
    pub param_r2: u16,
    pub uuid: String,
    pub protocol: String,
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlcDLGroupReq {
    pub gid: String,
    pub name: String,
    // 父级PLC测点配置的uuid
    pub pid: String,
    pub slave_address: String,
    pub param_r1: u16,
    pub param_r2: u16,
    pub uuid: String,
    pub protocol: String,
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlcS7GroupReq {
    pub gid: String,
    pub name: String,
    // 父级PLC测点配置的uuid
    pub pid: String,
    pub protocol: String,
    pub rack: u16,
    pub slot: u16,
    pub uuid: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlcMcGroupReq {
    pub gid: String,
    pub name: String,
    // 父级PLC测点配置的uuid
    pub pid: String,
    pub protocol: String,
    pub frame: String,
    pub mode: String,
    pub uuid: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlcOpcuaGroupReq {
    pub gid: String,
    pub name: String,
    // 父级PLC测点配置的uuid
    pub pid: String,
    pub protocol: String,
    pub path: Option<String>,
    pub uuid: String,
}

/// Unified handler for adding PLC config groups across protocols.
///
/// The endpoint is /plc/group/{ip}, and the body is JSON with fields depending on the protocol.
/// If "protocol" is not present in the body, it defaults to "modbus_tcp" (for Modbus compatibility).
///
/// For Modbus (no "protocol" in body, or explicitly "modbus_tcp"):
/// - Expected fields: gid, name, pid, slaveAddress, paramR1, paramR2
///
/// For S7 (include "protocol": "s7" or similar):
/// - Expected fields: gid, name, pid, protocol, rack, slot
///
/// For MC (include "protocol": "mc" or similar):
/// - Expected fields: gid, name, pid, protocol, frame, mode
///
/// For OPCUA (include "protocol": "opcua" or similar):
/// - Expected fields: gid, name, pid, protocol, path (optional)
///
/// This handler deserializes to the appropriate struct for validation, adds "uuid": "" to the payload,
/// sets "protocol" if missing, then forwards to the device.
pub async fn add_plc_config_group(
    Path(ip): Path<String>,
    Json(mut payload): Json<Value>,
) -> Json<Value> {
    payload["uuid"] = json!("");
    // Extract or set default protocol
    let protocol = payload
        .get("protocol")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    if protocol.is_empty() {
        return error_response(
            "Invalid JSON payload: expected object".to_string(),
            Some(400),
        );
    }

    // Deserialize to the appropriate struct based on protocol for validation
    let forward = match protocol.as_str() {
        "modbus_tcp" | "modbus_rtu_over_tcp" | "modbus_rtu" => {
            match serde_json::from_value::<PlcModbusGroupReq>(payload.clone()) {
                Ok(req) => {
                    // Perform any additional validation if needed
                    if req.slave_address == 0 {
                        return error_response("Slave address cannot be 0".to_string(), Some(400));
                    }
                    // Build forward payload
                    json!({
                        "gid": req.gid,
                        "uuid": req.uuid,
                        "name": req.name,
                        "pid": req.pid,
                        "protocol": req.protocol,
                        "slaveAddress": req.slave_address,
                        "paramR1": req.param_r1,
                        "paramR2": req.param_r2
                    })
                }
                Err(e) => {
                    return error_response(format!("Invalid Modbus payload: {}", e), Some(400));
                }
            }
        }
        "dlt645_2007" => {
            match serde_json::from_value::<PlcDLGroupReq>(payload.clone()) {
                Ok(req) => {
                    // Build forward payload
                    json!({
                        "gid": req.gid,
                         "uuid": req.uuid,
                        "name": req.name,
                        "pid": req.pid,
                        "protocol": req.protocol,
                        "slaveAddress": req.slave_address,
                        "paramR1": req.param_r1,
                        "paramR2": req.param_r2
                    })
                }
                Err(e) => {
                    return error_response(
                        format!("Invalid dlt645_2007 payload: {}", e),
                        Some(400),
                    );
                }
            }
        }
        "s7" => {
            match serde_json::from_value::<PlcS7GroupReq>(payload.clone()) {
                Ok(req) => {
                    // Perform any additional validation if needed
                    // Example: req.protocol must be "s7"
                    if req.protocol != "s7" {
                        return error_response(
                            "Protocol must be 's7' for S7 requests".to_string(),
                            Some(400),
                        );
                    }
                    // Build forward payload
                    json!({
                        "gid": req.gid,
                        "uuid": req.uuid,
                        "name": req.name,
                        "pid": req.pid,
                        "protocol": req.protocol,
                        "rack": req.rack,
                        "slot": req.slot
                    })
                }
                Err(e) => return error_response(format!("Invalid S7 payload: {}", e), Some(400)),
            }
        }
        "mc" => {
            match serde_json::from_value::<PlcMcGroupReq>(payload.clone()) {
                Ok(req) => {
                    // Perform any additional validation if needed
                    // Example: req.protocol must be "mc"
                    if req.protocol != "mc" {
                        return error_response(
                            "Protocol must be 'mc' for MC requests".to_string(),
                            Some(400),
                        );
                    }
                    // Build forward payload
                    json!({
                        "gid": req.gid,
                        "uuid": req.uuid,
                        "name": req.name,
                        "pid": req.pid,
                        "protocol": req.protocol,
                        "frame": req.frame,
                        "mode": req.mode
                    })
                }
                Err(e) => return error_response(format!("Invalid MC payload: {}", e), Some(400)),
            }
        }
        "opcua" => {
            match serde_json::from_value::<PlcOpcuaGroupReq>(payload.clone()) {
                Ok(req) => {
                    // Perform any additional validation if needed
                    // Example: req.protocol must be "opcua"
                    if req.protocol != "opcua" {
                        return error_response(
                            "Protocol must be 'opcua' for OPCUA requests".to_string(),
                            Some(400),
                        );
                    }
                    // Build forward payload
                    json!({
                        "gid": req.gid,
                         "uuid": req.uuid,
                        "name": req.name,
                        "pid": req.pid,
                        "protocol": req.protocol,
                        "path": req.path,
                    })
                }
                Err(e) => {
                    return error_response(format!("Invalid OPCUA payload: {}", e), Some(400));
                }
            }
        }
        _ => return error_response("Unsupported protocol: {}".to_string(), Some(400)),
    };

    log::info!("add plc config group: ip={}, payload={:?}", ip, forward);

    let url = format!("http://{}:80/plc/api/v1/group", &ip);
    call_device(url, forward, Some("POST")).await
}

pub async fn update_plc_config_group(
    Path(ip): Path<String>,
    Json(payload): Json<Value>,
) -> Json<Value> {
    let uuid = payload
        .get("uuid")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    if uuid.is_empty() {
        return error_response("uuid is required".to_string(), Some(400));
    }

    let protocol = payload
        .get("protocol")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    if protocol.is_empty() {
        return error_response(
            "Invalid JSON payload: expected object".to_string(),
            Some(400),
        );
    }
    // Deserialize to the appropriate struct based on protocol for validation
    let forward = match protocol.as_str() {
        "modbus_tcp" | "modbus_rtu_over_tcp" | "modbus_rtu" => {
            match serde_json::from_value::<PlcModbusGroupReq>(payload.clone()) {
                Ok(req) => {
                    // Perform any additional validation if needed
                    if req.slave_address == 0 {
                        return error_response("Slave address cannot be 0".to_string(), Some(400));
                    }
                    // Build forward payload
                    json!({
                        "gid": req.gid,
                        "uuid": req.uuid,
                        "name": req.name,
                        "pid": req.pid,
                        "protocol": req.protocol,
                        "slaveAddress": req.slave_address,
                        "paramR1": req.param_r1,
                        "paramR2": req.param_r2
                    })
                }
                Err(e) => {
                    return error_response(format!("Invalid Modbus payload: {}", e), Some(400));
                }
            }
        }
        "dlt645_2007" => {
            match serde_json::from_value::<PlcDLGroupReq>(payload.clone()) {
                Ok(req) => {
                    // Build forward payload
                    json!({
                        "gid": req.gid,
                        "uuid": req.uuid,
                        "name": req.name,
                        "pid": req.pid,
                        "protocol": req.protocol,
                        "slaveAddress": req.slave_address,
                        "paramR1": req.param_r1,
                        "paramR2": req.param_r2
                    })
                }
                Err(e) => {
                    return error_response(
                        format!("Invalid dlt645_2007 payload: {}", e),
                        Some(400),
                    );
                }
            }
        }
        "s7" => {
            match serde_json::from_value::<PlcS7GroupReq>(payload.clone()) {
                Ok(req) => {
                    // Perform any additional validation if needed
                    // Example: req.protocol must be "s7"
                    if req.protocol != "s7" {
                        return error_response(
                            "Protocol must be 's7' for S7 requests".to_string(),
                            Some(400),
                        );
                    }
                    // Build forward payload
                    json!({
                        "gid": req.gid,
                        "uuid": req.uuid,
                        "name": req.name,
                        "pid": req.pid,
                        "protocol": req.protocol,
                        "rack": req.rack,
                        "slot": req.slot
                    })
                }
                Err(e) => return error_response(format!("Invalid S7 payload: {}", e), Some(400)),
            }
        }
        "mc" => {
            match serde_json::from_value::<PlcMcGroupReq>(payload.clone()) {
                Ok(req) => {
                    // Perform any additional validation if needed
                    // Example: req.protocol must be "mc"
                    if req.protocol != "mc" {
                        return error_response(
                            "Protocol must be 'mc' for MC requests".to_string(),
                            Some(400),
                        );
                    }
                    // Build forward payload
                    json!({
                        "gid": req.gid,
                        "uuid": req.uuid,
                        "name": req.name,
                        "pid": req.pid,
                        "protocol": req.protocol,
                        "frame": req.frame,
                        "mode": req.mode
                    })
                }
                Err(e) => return error_response(format!("Invalid MC payload: {}", e), Some(400)),
            }
        }
        "opcua" => {
            match serde_json::from_value::<PlcOpcuaGroupReq>(payload.clone()) {
                Ok(req) => {
                    // Perform any additional validation if needed
                    // Example: req.protocol must be "opcua"
                    if req.protocol != "opcua" {
                        return error_response(
                            "Protocol must be 'opcua' for OPCUA requests".to_string(),
                            Some(400),
                        );
                    }
                    // Build forward payload
                    json!({
                        "gid": req.gid,
                        "uuid": req.uuid,
                        "name": req.name,
                        "pid": req.pid,
                        "protocol": req.protocol,
                        "path": req.path,
                    })
                }
                Err(e) => {
                    return error_response(format!("Invalid OPCUA payload: {}", e), Some(400));
                }
            }
        }
        _ => return error_response("Unsupported protocol: {}".to_string(), Some(400)),
    };

    log::info!("add plc config group: ip={}, payload={:?}", ip, forward);

    let url = format!("http://{}:80/plc/api/v1/group/{}", &ip, uuid);
    call_device(url, forward, Some("PUT")).await
}
