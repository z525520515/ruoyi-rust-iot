use crate::modules::plc_gateway::api::config_util::call_device;
use crate::modules::plc_gateway::entity::plc_config::{BasePlcConfig, PlcConfigRequest};
use crate::modules::plc_gateway::unified::response::error_response;
use axum::{extract::Path, response::Json};
use serde_json::{Value, json};
use thiserror::Error;

use validator::Validate;

#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("Validation error: {0}")]
    Validation(String),
    #[error("Protocol to type conversion error: {0}")]
    TypeConversion(String),
}

pub struct ConfigValidator;

impl ConfigValidator {
    pub fn validate_request(request: &PlcConfigRequest) -> Result<(), ValidationError> {
        match request {
            PlcConfigRequest::SiemensS7(config) => {
                config
                    .validate()
                    .map_err(|e| ValidationError::Validation(e.to_string()))?;
                Self::validate_base_config(&config.base)?;
            }
            PlcConfigRequest::MitsubishiMc(config) => {
                config
                    .validate()
                    .map_err(|e| ValidationError::Validation(e.to_string()))?;
                Self::validate_base_config(&config.base)?;
            }
            PlcConfigRequest::OpcUa(config) => {
                config
                    .validate()
                    .map_err(|e| ValidationError::Validation(e.to_string()))?;
                Self::validate_base_config(&config.base)?;
            }
            PlcConfigRequest::ModbusTcp(config) => {
                config
                    .validate()
                    .map_err(|e| ValidationError::Validation(e.to_string()))?;
                Self::validate_base_config(&config.base)?;
            }
            PlcConfigRequest::ModbusRtuOverTcp(config) => {
                config
                    .validate()
                    .map_err(|e| ValidationError::Validation(e.to_string()))?;
                Self::validate_base_config(&config.base)?;
            }
            // 为 ModbusRTU 和 DLT 类型创建通用的端口验证函数
            PlcConfigRequest::ModbusRtu(config) => {
                Self::validate_rs485_port(&config.base)?;
            }
            PlcConfigRequest::DLT(config) => {
                Self::validate_rs485_port(&config.base)?;
            }
        }
        Ok(())
    }

    fn validate_base_config(base: &BasePlcConfig) -> Result<(), ValidationError> {
        base.validate()
            .map_err(|e| ValidationError::Validation(e.to_string()))?;

        // Validate IP format (basic validation)
        if base.ip.is_empty() {
            return Err(ValidationError::Validation(
                "IP address is required".to_string(),
            ));
        }

        // Validate interface-specific rules
        if base.interface.starts_with("eth") && base.ip.is_empty() {
            return Err(ValidationError::Validation(
                "IP address is required for Ethernet interfaces".to_string(),
            ));
        }

        Ok(())
    }

    /// 验证RS485接口的端口配置
    fn validate_rs485_port(base: &BasePlcConfig) -> Result<(), ValidationError> {
        // Validate port based on interface for RS485
        if base.interface == "rs485-1" && base.port != 102 {
            return Err(ValidationError::Validation(
                "For rs485-1 interface, port must be 502".to_string(),
            ));
        }
        if base.interface == "rs485-2" && base.port != 502 {
            return Err(ValidationError::Validation(
                "For rs485-2 interface, port must be 102".to_string(),
            ));
        }
        Ok(())
    }

    pub fn protocol_to_type(protocol: &str) -> Result<String, ValidationError> {
        match protocol {
            "s7" => Ok("s7".to_string()),
            "mc" => Ok("mc".to_string()),
            "opcua" => Ok("opcua".to_string()),
            "modbus_tcp" => Ok("modbusTcp".to_string()),
            "modbus_rtu_over_tcp" => Ok("modbusRtuOverTcp".to_string()),
            "modbus_rtu" => Ok("modbusRtu".to_string()),
            _ => Err(ValidationError::TypeConversion(format!(
                "Unknown protocol: {}",
                protocol
            ))),
        }
    }
}
pub async fn get_forward(request: &PlcConfigRequest) -> Value {
    // Build forward JSON based on request type
    let forward = match request {
        PlcConfigRequest::SiemensS7(config) => {
            json!({
                "uuid": config.base.uuid,
                "name": config.base.name,
                "type": "s7",
                "protocol": "s7",
                "interface": config.base.interface,
                "disable": config.base.disable,
                "ip": config.base.ip,
                "port": config.base.port,
                "rack": config.rack,
                "slot": config.slot
            })
        }
        PlcConfigRequest::MitsubishiMc(config) => {
            json!({
                "uuid": config.base.uuid,
                "name": config.base.name,
                "type": "mc",
                "protocol": "mc",
                "interface": config.base.interface,
                "disable": config.base.disable,
                "ip": config.base.ip,
                "port": config.base.port,
                "frame": config.frame,
                "mode": config.mode
            })
        }
        PlcConfigRequest::OpcUa(config) => {
            json!({
                "uuid": config.base.uuid,
                "name": config.base.name,
                "type": "opcua",
                "protocol": "opcua",
                "interface": config.base.interface,
                "disable": config.base.disable,
                "ip": config.base.ip,
                "port": config.base.port,
                "path": config.path
            })
        }
        PlcConfigRequest::ModbusTcp(config) => {
            json!({
                "uuid": config.base.uuid,
                "name": config.base.name,
                "type": "modbusTcp",
                "protocol": "modbus_tcp",
                "interface": config.base.interface,
                "disable": config.base.disable,
                "ip": config.base.ip,
                "port": config.base.port
            })
        }
        PlcConfigRequest::ModbusRtuOverTcp(config) => {
            json!({
                "uuid": config.base.uuid,
                "name": config.base.name,
                "type": "modbusRtuOverTcp",
                "protocol": "modbus_rtu_over_tcp",
                "interface": config.base.interface,
                "disable": config.base.disable,
                "ip": config.base.ip,
                "port": config.base.port
            })
        }
        PlcConfigRequest::ModbusRtu(config) => {
            json!({
                "uuid": config.base.uuid,
                "name": config.base.name,
                "type": "modbusRtu",
                "protocol": "modbus_rtu",
                "interface": config.base.interface,
                "disable": config.base.disable,
                "ip": config.base.ip,
                "port": config.base.port,
                "baudRate": config.baud_rate,
                "dataBits": config.data_bits,
                "parity": config.parity,
                "stopBits": config.stop_bits
            })
        }
        PlcConfigRequest::DLT(config) => {
            json!({
                "uuid": config.base.uuid,
                "name": config.base.name,
                "type": "dlt645-2007",  //这里要特别注意，type和上面的写法不一样，巨坑
                "protocol": "dlt645_2007",
                "interface": config.base.interface,
                "disable": config.base.disable,
                "ip": config.base.ip,
                "port": config.base.port,
                "baudRate": config.baud_rate,
                "dataBits": config.data_bits,
                "parity": config.parity,
                "stopBits": config.stop_bits
            })
        }
    };
    forward
}

/// 统一的 PLC 配置处理入口
async fn handle_plc_config(
    ip: String,
    mut params: Value,
    // 差异点通过闭包提供
    config: impl FnOnce(&mut Value, &PlcConfigRequest) -> Result<(String, String), Json<Value>>,
) -> Json<Value> {
    log::info!("handle plc config: ip={}, params={:#?}", ip, params);

    // ---------- 1. 协议前置处理 ----------
    if let Some(protocol) = params
        .get("protocol")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
    {
        if protocol == "modbus_rtu" || protocol == "dlt645_2007" {
            if let Some(interface) = params.get("interface").and_then(|v| v.as_str()) {
                if !["rs485-1", "rs485-2"].contains(&interface) {
                    return error_response(
                        "interface must be rs485-1 or rs485-2".to_string(),
                        Some(400),
                    );
                }
                params["port"] = match interface {
                    "rs485-1" => json!(102),
                    "rs485-2" => json!(502),
                    _ => unreachable!(),
                };
            }
            params["ip"] = json!("");
        }
    }

    // ---------- 2. 解析 ----------
    let request: PlcConfigRequest = match serde_json::from_value(params.clone()) {
        Ok(r) => r,
        Err(e) => {
            log::error!("Failed to parse request: {}", e);
            return error_response(format!("Invalid request format: {}", e), Some(400));
        }
    };

    // ---------- 3. 两层验证 ----------
    if let Err(errors) = request.validate() {
        return error_response(format!("Validation failed: {:?}", errors), Some(400));
    }
    if let Err(e) = ConfigValidator::validate_request(&request) {
        log::error!("Validation failed: {}", e);
        return error_response(e.to_string(), Some(400));
    }

    // ---------- 4. 转发 JSON ----------
    let forward = get_forward(&request).await;

    // ---------- 5. 差异点闭包 ----------
    let (url, method) = match config(&mut params, &request) {
        Ok(v) => v,
        Err(e) => return e,
    };

    // ---------- 6. 调用设备 ----------
    call_device(url, forward, Some(&method)).await
}

pub async fn add_plc_config(Path(ip): Path<String>, Json(mut params): Json<Value>) -> Json<Value> {
    let ip_clone = ip.clone();
    params["uuid"] = json!("");
    params["disable"] = json!(false);
    // 统一的处理函数
    handle_plc_config(ip, params, |_, _| {
        // add 特有补全
        let url = format!("http://{}:80/plc/api/v1/plc", ip_clone);
        Ok((url, "POST".to_string()))
    })
    .await
}

pub async fn update_plc_config(Path(ip): Path<String>, Json(mut params): Json<Value>) -> Json<Value> {
    let ip_clone = ip.clone();
    // uuid 必填检查
    let uuid = params
        .get("uuid")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
       
    if uuid.is_empty() {
        return error_response("uuid is required".to_string(), Some(400));
    }
    // disable 默认 false（如果缺失或空串）
    if params
        .get("disable")
        .and_then(|v| v.as_bool())
        .unwrap_or(false)
    {
        params["disable"] = json!(true);
    } else {
        params["disable"] = json!(false);
    }

    handle_plc_config(ip, params, move |_, _| {
        let url = format!("http://{}:80/plc/api/v1/plc/{}", ip_clone, uuid);
        Ok((url, "PUT".to_string()))
    })
    .await
}

/// 获取所有PLC数据,包含所有分组和测点
pub async fn get_all_plc_config(Path(ip): Path<String>) -> Json<Value> {
    let url = format!("http://{}:80/plc/api/v1/plc", &ip);
    call_device(url, json!([]), Some("GET")).await
}

/// 获取实时监控的数据 Real-time monitoring
pub async fn get_all_data_real_time_monitoring(Path(ip): Path<String>) -> Json<Value> {
    let url = format!("http://{}:80/plc/api/v1/data", &ip);
    call_device(url, json!([]), Some("GET")).await
}

/// 删除某个PLC配置
pub async fn delete_plc_config(Path((ip, uuid)): Path<(String, String)>) -> Json<Value> {
    let url = format!("http://{}/plc/api/v1/plc/{}", &ip, uuid);
    call_device(url, json!([]), Some("DELETE")).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use tokio::test;

    #[test]
    async fn test_params_s7() {
        let params = json!({
            "name": "西门子_test",  //必填
            "protocol": "s7",   //必填
            "interface": "eth0",  //必填, 只可以是eth0或者eth1, 默认eth0
            "ip": "1.1.1.1", //必填
            "port": 1, //必填
            "rack": 1, //必填
            "slot": 1 //必填
        });
        // println!("{:?}", params);

        let request: PlcConfigRequest = serde_json::from_value(params.clone())
            .expect("Failed to deserialize - check field names and #[serde(rename_all)]");
        // 关键：手动触发验证
        if let Err(errors) = request.validate() {
            println!("Validation failed: {:?}", errors);
            return; // 或者 panic!()
        }
        // 手动触发验证 base 配置
        ConfigValidator::validate_request(&request).unwrap();
        let forward = get_forward(&request).await;
        println!(
            "{}",
            serde_json::to_string_pretty(&forward).unwrap_or("Failed to format JSON".to_string())
        );
    }

    #[test]
    async fn test_params_mc() {
        let params = json!({
            "name": "三菱_test", //必填
            "protocol": "mc", //必填
            "interface": "eth0",   //必填, 只可以是eth0或者eth1, 默认eth0
            "ip": "2.2.2.2", //必填
            "port": 2, //必填
            // "frame": "10E",   // 只可以是1E或者3E, 默认1E
            // "mode": "ascii"  // 只可以是ascii 或者 binary，默认binary
        });
        // println!("{:?}", params);
        let request: PlcConfigRequest = serde_json::from_value(params.clone())
            .expect("Failed to deserialize - check field names and #[serde(rename_all)]");
        // 关键：手动触发验证
        if let Err(errors) = request.validate() {
            println!("Validation failed: {:?}", errors);
            return; // 或者 panic!()
        }
        let forward = get_forward(&request).await;
        println!(
            "{}",
            serde_json::to_string_pretty(&forward).unwrap_or("Failed to format JSON".to_string())
        );
    }

    #[test]
    async fn test_params_opcua() {
        // 测试用例 interface 为 eth9 时，是否会报错
        let params = json!({
            "name": "汇川_test",  //必填
            "protocol": "opcua",  //必填
            "interface": "eth9",  //必填, 只可以是eth0或者eth1, 默认eth0
            "ip": "3.3.3.3",   //必填
            "port": 4840,     //必填
        });
        // println!("{:?}", params);
        let request: PlcConfigRequest = serde_json::from_value(params.clone())
            .expect("Failed to deserialize - check field names and #[serde(rename_all)]");
        // 关键：手动触发验证
        if let Err(errors) = request.validate() {
            println!("Validation failed: {:?}", errors);
            return; // 或者 panic!()
        }
        let forward = get_forward(&request).await;
        println!(
            "{}",
            serde_json::to_string_pretty(&forward).unwrap_or("Failed to format JSON".to_string())
        );
    }

    #[test]
    async fn test_params_modbus_tcp() {
        // 测试用例 interface 为 eth9 时，是否会报错
        let params = json!({
            "name": "ModbusTCP_test",  //必填
            "protocol": "modbus_tcp",    //必填
            "interface": "eth0",   //必填, 只可以是eth0或者eth1, 默认eth0
            "ip": "4.4.4.4",  //必填
            "port": 502  //必填
        }
        );
        // println!("{:?}", params);
        let request: PlcConfigRequest = serde_json::from_value(params.clone())
            .expect("Failed to deserialize - check field names and #[serde(rename_all)]");
        // 关键：手动触发验证
        if let Err(errors) = request.validate() {
            println!("Validation failed: {:?}", errors);
            return; // 或者 panic!()
        }
        let forward = get_forward(&request).await;
        println!(
            "{}",
            serde_json::to_string_pretty(&forward).unwrap_or("Failed to format JSON".to_string())
        );
    }

    #[test]
    async fn test_params_modbus_rtu_over_tcp() {
        // 测试用例 interface 为 eth9 时，是否会报错
        let params = json!({
            "name": "ModbusTCPoverTCP_test",  //必填
            "protocol": "modbus_rtu_over_tcp",  //必填
            "interface": "eth10",  //必填, 只可以是eth0或者eth1,
            "ip": "5.5.5.5",   //必填
            "port": 5 //必填
        });
        // println!("{:?}", params);
        let request: PlcConfigRequest = serde_json::from_value(params.clone())
            .expect("Failed to deserialize - check field names and #[serde(rename_all)]");
        // 关键：手动触发验证
        if let Err(errors) = request.validate() {
            println!("Validation failed: {:?}", errors);
            return; // 或者 panic!()
        }
        let forward = get_forward(&request).await;
        println!(
            "{}",
            serde_json::to_string_pretty(&forward).unwrap_or("Failed to format JSON".to_string())
        );
    }

    #[test]
    async fn test_params_modbus_rtu_2() {
        // 测试用例 interface 为 eth9 时，是否会报错
        let mut params = json!({
            "name": "ModbusRTU_test", //必填
            "protocol": "modbus_rtu",  //必填
            "interface": "rs485-1", //必填
            "port": 502, // 如果"interface"是"rs485-1"，port是102。如果"interface"是"rs485-2"，port是502。
            "baudRate": 9600, //必填
            "dataBits": 8, //必填，只能是5，6，7，8中的一个数值
            "parity": "none", //必填，只能是none，add，even这三个的一个
            "stopBits": 1 //必填，只能是1或者2
        });
        if let Some(protocol) = params.get("protocol").and_then(|v| v.as_str()) {
            if protocol == "modbus_rtu" && !params.as_object().unwrap().contains_key("ip") {
                params["ip"] = json!(""); // 添加 ip 字段，默认值为空字符串
            }
        }

        // println!("{:?}", params);
        let request: PlcConfigRequest = serde_json::from_value(params.clone())
            .expect("Failed to deserialize - check field names and #[serde(rename_all)]");
        // 关键：手动触发验证
        if let Err(errors) = request.validate() {
            println!("Validation failed: {:?}", errors);
            return; // 或者 panic!()
        }
        // 手动触发验证 base 配置
        ConfigValidator::validate_request(&request).unwrap();
        let forward = get_forward(&request).await;
        println!(
            "{}",
            serde_json::to_string_pretty(&forward).unwrap_or("Failed to format JSON".to_string())
        );
    }
}
