use crate::modules::system::domain::vo::{RespVO, CODE_FAIL};
use crate::modules::system::service::CONTEXT;
use crate::modules::gateway::domain::dto::gateway_invoke::{
    GatewayInvokeVO,
    SystemInfoParams,
    ChangePasswordParams,
    SetTimeParams,
    SetTimeZoneParams,
    DeviceUniqueIDParams,
    DeviceUniqueIDHashParams,
    ApplicationInfoParams,
    AvailableTimeZoneIDsParams,
    SystemTimeZoneIDParams,
    ICCIDsParams,
    PerformRebootParams,
    AvailableSerialPortsParams,
    SetBaudRateParams,
    SetDataBitsParams,
    SetParityParams,
    SetStopBitsParams,
    SetFlowControlParams,
    AvailableDataBitsParams,
    AvailableParitiesParams,
    AvailableStopBitsParams,
    AvailableFlowControlsParams,
};
use actix_web::{post, web, Responder};
use serde::{Serialize, Deserialize};
use serde_json::{json, Value};
use crate::util::device_unique_id::device_unique_id_hash_from_sn;

/// Linker 网关管理控制器
/// 基于 README 协议规范实现 MQTT 协议栈调用

/// ========== 通用网关调用接口 ==========

#[derive(Debug, Serialize, Deserialize)]
pub struct GenericInvokeRequest {
    pub device_hash: String,
    pub protocol_type: String,
    pub stack_name: String,
    pub action: String,
    pub method: String,
    #[serde(default)]
    pub params: serde_json::Value,
    #[serde(default)]
    pub source: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceUniqueIdHashRequest {
    pub sn: String,
}

#[post("/gateway/invoke")]
pub async fn gateway_invoke(req: web::Json<GenericInvokeRequest>) -> impl Responder {
    let vo = req.into_inner();

    if let Some(manager) = CONTEXT.mqtt_manager.get() {
        let source = vo
            .source
            .as_deref()
            .map(str::trim)
            .filter(|s| !s.is_empty());
        let result = manager.gateway_invoke_with_options(
            &vo.device_hash,
            &vo.protocol_type,
            &vo.stack_name,
            &vo.action,
            &vo.method,
            vo.params,
            source,
            None,
            None,
        ).await;
        let final_result: Result<serde_json::Value, crate::error::Error> = result
            .map_err(|e| crate::error::Error::from(e.to_string()));
        RespVO::from_result(&final_result).resp_json()
    } else {
        RespVO::<()>::from_error_info(CODE_FAIL, "MQTT 客户端未初始化").resp_json()
    }
}

#[post("/gateway/system/deviceUniqueIDHashBySN")]
pub async fn system_device_unique_id_hash_by_sn(
    req: web::Json<DeviceUniqueIdHashRequest>,
) -> impl Responder {
    let sn = req.sn.trim();
    if sn.is_empty() {
        return RespVO::<()>::from_error_info(CODE_FAIL, "缺少参数 sn").resp_json();
    }

    match device_unique_id_hash_from_sn(sn) {
        Ok(hash) => RespVO::from(&hash).resp_json(),
        Err(e) => RespVO::<()>::from_error_info(CODE_FAIL, &e.to_string()).resp_json(),
    }
}


fn get_string_param(params: &Value, keys: &[&str]) -> Option<String> {
    for key in keys {
        if let Some(value) = params.get(*key).and_then(|v| v.as_str()) {
            let trimmed = value.trim();
            if !trimmed.is_empty() {
                return Some(trimmed.to_string());
            }
        }
    }
    None
}

fn get_u64_param(params: &Value, keys: &[&str]) -> Option<u64> {
    for key in keys {
        if let Some(value) = params.get(*key) {
            if let Some(num) = value.as_u64() {
                return Some(num);
            }
            if let Some(num) = value.as_i64() {
                if num >= 0 {
                    return Some(num as u64);
                }
            }
        }
    }
    None
}

fn params_or_empty(params: Value) -> Value {
    if params.is_null() {
        json!({})
    } else {
        params
    }
}

/// ========== System 模块接口 ==========

/// 获取设备唯一标识符
#[post("/gateway/system/deviceUniqueID")]
pub async fn system_device_unique_id(req: web::Json<GatewayInvokeVO>) -> impl Responder {
    let vo = req.into_inner();
    if let Some(manager) = CONTEXT.mqtt_manager.get() {
        let params = DeviceUniqueIDParams {};
        let result = manager.gateway_invoke(
            &vo.device_hash,
            &vo.protocol_type,
            &vo.stack_name,
            &vo.action,
            &vo.method,
            json!(params)
        ).await;
        let final_result: Result<serde_json::Value, crate::error::Error> = result
            .map_err(|e| crate::error::Error::from(e.to_string()));
        RespVO::from_result(&final_result).resp_json()
    } else {
        RespVO::<()>::from_error_info(CODE_FAIL, "MQTT 客户端未初始化").resp_json()
    }
}

/// 获取设备唯一标识符哈希
#[post("/gateway/system/deviceUniqueIDHash")]
pub async fn system_device_unique_id_hash(req: web::Json<GatewayInvokeVO>) -> impl Responder {
    let vo = req.into_inner();
    if let Some(manager) = CONTEXT.mqtt_manager.get() {
        let params = DeviceUniqueIDHashParams {};
        let result = manager.gateway_invoke(
            &vo.device_hash,
            &vo.protocol_type,
            &vo.stack_name,
            &vo.action,
            &vo.method,
            json!(params)
        ).await;
        let final_result: Result<serde_json::Value, crate::error::Error> = result
            .map_err(|e| crate::error::Error::from(e.to_string()));
        RespVO::from_result(&final_result).resp_json()
    } else {
        RespVO::<()>::from_error_info(CODE_FAIL, "MQTT 客户端未初始化").resp_json()
    }
}

/// 获取设备信息
#[post("/gateway/system/info")]
pub async fn system_info(req: web::Json<GatewayInvokeVO>) -> impl Responder {
    let vo = req.into_inner();
    if let Some(manager) = CONTEXT.mqtt_manager.get() {
        let params = SystemInfoParams {};
        let result = manager.gateway_invoke(
            &vo.device_hash,
            &vo.protocol_type,
            &vo.stack_name,
            &vo.action,
            &vo.method,
            json!(params)
        ).await;
        let final_result: Result<serde_json::Value, crate::error::Error> = result
            .map_err(|e| crate::error::Error::from(e.to_string()));
        RespVO::from_result(&final_result).resp_json()
    } else {
        RespVO::<()>::from_error_info(CODE_FAIL, "MQTT 客户端未初始化").resp_json()
    }
}

/// 更改设备密码
#[post("/gateway/system/changePassword")]
pub async fn system_change_password(req: web::Json<GatewayInvokeVO>) -> impl Responder {
    let vo = req.into_inner();
    if let Some(manager) = CONTEXT.mqtt_manager.get() {
        let current_password = get_string_param(
            &vo.params,
            &["currentPassword", "current_password"],
        )
        .unwrap_or_default();
        let new_password = get_string_param(
            &vo.params,
            &["newPassword", "new_password"],
        )
        .unwrap_or_default();

        if current_password.is_empty() || new_password.is_empty() {
            return RespVO::<()>::from_error_info(CODE_FAIL, "缺少参数 current_password 或 new_password").resp_json();
        }

        let params = ChangePasswordParams { current_password, new_password };
        let result = manager.gateway_invoke(
            &vo.device_hash,
            &vo.protocol_type,
            &vo.stack_name,
            &vo.action,
            &vo.method,
            json!(params)
        ).await;
        let final_result: Result<serde_json::Value, crate::error::Error> = result
            .map_err(|e| crate::error::Error::from(e.to_string()));
        RespVO::from_result(&final_result).resp_json()
    } else {
        RespVO::<()>::from_error_info(CODE_FAIL, "MQTT 客户端未初始化").resp_json()
    }
}

/// 获取应用程序信息
#[post("/gateway/system/applicationInfo")]
pub async fn system_application_info(req: web::Json<GatewayInvokeVO>) -> impl Responder {
    let vo = req.into_inner();
    if let Some(manager) = CONTEXT.mqtt_manager.get() {
        let params = ApplicationInfoParams {};
        let result = manager.gateway_invoke(
            &vo.device_hash,
            &vo.protocol_type,
            &vo.stack_name,
            &vo.action,
            &vo.method,
            json!(params)
        ).await;
        let final_result: Result<serde_json::Value, crate::error::Error> = result
            .map_err(|e| crate::error::Error::from(e.to_string()));
        RespVO::from_result(&final_result).resp_json()
    } else {
        RespVO::<()>::from_error_info(CODE_FAIL, "MQTT 客户端未初始化").resp_json()
    }
}

/// 获取 ICCID 列表
#[post("/gateway/system/iccIDs")]
pub async fn system_iccid(req: web::Json<GatewayInvokeVO>) -> impl Responder {
    let vo = req.into_inner();
    if let Some(manager) = CONTEXT.mqtt_manager.get() {
        let params = ICCIDsParams {};
        let result = manager.gateway_invoke(
            &vo.device_hash,
            &vo.protocol_type,
            &vo.stack_name,
            &vo.action,
            &vo.method,
            json!(params)
        ).await;
        let final_result: Result<serde_json::Value, crate::error::Error> = result
            .map_err(|e| crate::error::Error::from(e.to_string()));
        RespVO::from_result(&final_result).resp_json()
    } else {
        RespVO::<()>::from_error_info(CODE_FAIL, "MQTT 客户端未初始化").resp_json()
    }
}

/// 设置设备时间
#[post("/gateway/system/setTime")]
pub async fn system_set_time(req: web::Json<GatewayInvokeVO>) -> impl Responder {
    let vo = req.into_inner();
    if let Some(manager) = CONTEXT.mqtt_manager.get() {
        let timestamp = if vo.params.is_i64() || vo.params.is_u64() {
            vo.params.as_i64().unwrap_or_else(|| chrono::Utc::now().timestamp_millis())
        } else {
            chrono::Utc::now().timestamp_millis()
        };
        let params = SetTimeParams(timestamp);
        let result = manager.gateway_invoke(
            &vo.device_hash,
            &vo.protocol_type,
            &vo.stack_name,
            &vo.action,
            &vo.method,
            json!(params)
        ).await;
        let final_result: Result<serde_json::Value, crate::error::Error> = result
            .map_err(|e| crate::error::Error::from(e.to_string()));
        RespVO::from_result(&final_result).resp_json()
    } else {
        RespVO::<()>::from_error_info(CODE_FAIL, "MQTT 客户端未初始化").resp_json()
    }
}

/// 设置设备时区
#[post("/gateway/system/setTimeZone")]
pub async fn system_set_timezone(req: web::Json<GatewayInvokeVO>) -> impl Responder {
    let vo = req.into_inner();
    if let Some(manager) = CONTEXT.mqtt_manager.get() {
        let timezone = if let Some(value) = vo.params.as_str() {
            value.trim().to_string()
        } else {
            get_string_param(&vo.params, &["timeZone", "timezone"]).unwrap_or_default()
        };

        if timezone.is_empty() {
            return RespVO::<()>::from_error_info(CODE_FAIL, "缺少参数 timezone").resp_json();
        }

        let params = SetTimeZoneParams(timezone);
        let result = manager.gateway_invoke(
            &vo.device_hash,
            &vo.protocol_type,
            &vo.stack_name,
            &vo.action,
            &vo.method,
            json!(params)
        ).await;
        let final_result: Result<serde_json::Value, crate::error::Error> = result
            .map_err(|e| crate::error::Error::from(e.to_string()));
        RespVO::from_result(&final_result).resp_json()
    } else {
        RespVO::<()>::from_error_info(CODE_FAIL, "MQTT 客户端未初始化").resp_json()
    }
}

/// 获取可用时区列表
#[post("/gateway/system/availableTimeZoneIDs")]
pub async fn system_available_timezones(req: web::Json<GatewayInvokeVO>) -> impl Responder {
    let vo = req.into_inner();
    if let Some(manager) = CONTEXT.mqtt_manager.get() {
        let params = AvailableTimeZoneIDsParams {};
        let result = manager.gateway_invoke(
            &vo.device_hash,
            &vo.protocol_type,
            &vo.stack_name,
            &vo.action,
            &vo.method,
            json!(params)
        ).await;
        let final_result: Result<serde_json::Value, crate::error::Error> = result
            .map_err(|e| crate::error::Error::from(e.to_string()));
        RespVO::from_result(&final_result).resp_json()
    } else {
        RespVO::<()>::from_error_info(CODE_FAIL, "MQTT 客户端未初始化").resp_json()
    }
}

/// 获取当前系统时区
#[post("/gateway/system/systemTimeZoneID")]
pub async fn system_current_timezone(req: web::Json<GatewayInvokeVO>) -> impl Responder {
    let vo = req.into_inner();
    if let Some(manager) = CONTEXT.mqtt_manager.get() {
        let params = SystemTimeZoneIDParams {};
        let result = manager.gateway_invoke(
            &vo.device_hash,
            &vo.protocol_type,
            &vo.stack_name,
            &vo.action,
            &vo.method,
            json!(params)
        ).await;
        let final_result: Result<serde_json::Value, crate::error::Error> = result
            .map_err(|e| crate::error::Error::from(e.to_string()));
        RespVO::from_result(&final_result).resp_json()
    } else {
        RespVO::<()>::from_error_info(CODE_FAIL, "MQTT 客户端未初始化").resp_json()
    }
}

/// 重启设备 (危险操作)
#[post("/gateway/system/performReboot")]
pub async fn system_reboot(req: web::Json<GatewayInvokeVO>) -> impl Responder {
    let vo = req.into_inner();
    if let Some(manager) = CONTEXT.mqtt_manager.get() {
        let params = PerformRebootParams {};
        let result = manager.gateway_invoke(
            &vo.device_hash,
            &vo.protocol_type,
            &vo.stack_name,
            &vo.action,
            &vo.method,
            json!(params)
        ).await;
        let final_result: Result<serde_json::Value, crate::error::Error> = result
            .map_err(|e| crate::error::Error::from(e.to_string()));
        RespVO::from_result(&final_result).resp_json()
    } else {
        RespVO::<()>::from_error_info(CODE_FAIL, "MQTT 客户端未初始化").resp_json()
    }
}

/// ========== Serial Port Manager 模块接口 ==========

/// 获取可用串口列表
#[post("/gateway/serial/availableSerialPorts")]
pub async fn serial_available_ports(req: web::Json<GatewayInvokeVO>) -> impl Responder {
    let vo = req.into_inner();
    if let Some(manager) = CONTEXT.mqtt_manager.get() {
        let params = AvailableSerialPortsParams {};
        let result = manager.gateway_invoke(
            &vo.device_hash,
            &vo.protocol_type,
            &vo.stack_name,
            &vo.action,
            &vo.method,
            json!(params)
        ).await;
        let final_result: Result<serde_json::Value, crate::error::Error> = result
            .map_err(|e| crate::error::Error::from(e.to_string()));
        RespVO::from_result(&final_result).resp_json()
    } else {
        RespVO::<()>::from_error_info(CODE_FAIL, "MQTT 客户端未初始化").resp_json()
    }
}

/// 打开串口
#[post("/gateway/serial/openSerialPort")]
pub async fn serial_open_port(req: web::Json<GatewayInvokeVO>) -> impl Responder {
    let vo = req.into_inner();
    if let Some(manager) = CONTEXT.mqtt_manager.get() {
        let port_name = if let Some(value) = vo.params.as_str() {
            value.trim().to_string()
        } else {
            get_string_param(&vo.params, &["portName", "port_name"]).unwrap_or_default()
        };

        if port_name.is_empty() {
            return RespVO::<()>::from_error_info(CODE_FAIL, "缺少参数 port_name").resp_json();
        }

        let params = port_name;
        let result = manager.gateway_invoke(
            &vo.device_hash,
            &vo.protocol_type,
            &vo.stack_name,
            &vo.action,
            &vo.method,
            json!(params)
        ).await;
        let final_result: Result<serde_json::Value, crate::error::Error> = result
            .map_err(|e| crate::error::Error::from(e.to_string()));
        RespVO::from_result(&final_result).resp_json()
    } else {
        RespVO::<()>::from_error_info(CODE_FAIL, "MQTT 客户端未初始化").resp_json()
    }
}

/// 关闭串口
#[post("/gateway/serial/closeSerialPort")]
pub async fn serial_close_port(req: web::Json<GatewayInvokeVO>) -> impl Responder {
    let vo = req.into_inner();
    if let Some(manager) = CONTEXT.mqtt_manager.get() {
        let port_name = if let Some(value) = vo.params.as_str() {
            value.trim().to_string()
        } else {
            get_string_param(&vo.params, &["portName", "port_name"]).unwrap_or_default()
        };

        if port_name.is_empty() {
            return RespVO::<()>::from_error_info(CODE_FAIL, "缺少参数 port_name").resp_json();
        }

        let params = port_name;
        let result = manager.gateway_invoke(
            &vo.device_hash,
            &vo.protocol_type,
            &vo.stack_name,
            &vo.action,
            &vo.method,
            json!(params)
        ).await;
        let final_result: Result<serde_json::Value, crate::error::Error> = result
            .map_err(|e| crate::error::Error::from(e.to_string()));
        RespVO::from_result(&final_result).resp_json()
    } else {
        RespVO::<()>::from_error_info(CODE_FAIL, "MQTT 客户端未初始化").resp_json()
    }
}

/// 设置串口波特率
#[post("/gateway/serial/setBaudRate")]
pub async fn serial_set_baudrate(req: web::Json<GatewayInvokeVO>) -> impl Responder {
    let vo = req.into_inner();
    if let Some(manager) = CONTEXT.mqtt_manager.get() {
        let port_name = get_string_param(&vo.params, &["portName", "port_name"]).unwrap_or_default();
        let baud_rate = get_u64_param(&vo.params, &["baudRate", "baud_rate"]).unwrap_or(115200) as u32;

        if port_name.is_empty() {
            return RespVO::<()>::from_error_info(CODE_FAIL, "缺少参数 port_name").resp_json();
        }

        let params = SetBaudRateParams { port_name, baud_rate };
        let result = manager.gateway_invoke(
            &vo.device_hash,
            &vo.protocol_type,
            &vo.stack_name,
            &vo.action,
            &vo.method,
            json!(params)
        ).await;
        let final_result: Result<serde_json::Value, crate::error::Error> = result
            .map_err(|e| crate::error::Error::from(e.to_string()));
        RespVO::from_result(&final_result).resp_json()
    } else {
        RespVO::<()>::from_error_info(CODE_FAIL, "MQTT 客户端未初始化").resp_json()
    }
}

/// 获取支持的数据位选项
#[post("/gateway/serial/availableDataBits")]
pub async fn serial_available_databits(req: web::Json<GatewayInvokeVO>) -> impl Responder {
    let vo = req.into_inner();
    if let Some(manager) = CONTEXT.mqtt_manager.get() {
        let params = AvailableDataBitsParams {};
        let result = manager.gateway_invoke(
            &vo.device_hash,
            &vo.protocol_type,
            &vo.stack_name,
            &vo.action,
            &vo.method,
            json!(params)
        ).await;
        let final_result: Result<serde_json::Value, crate::error::Error> = result
            .map_err(|e| crate::error::Error::from(e.to_string()));
        RespVO::from_result(&final_result).resp_json()
    } else {
        RespVO::<()>::from_error_info(CODE_FAIL, "MQTT 客户端未初始化").resp_json()
    }
}

/// 获取支持的校验位选项
#[post("/gateway/serial/availableParities")]
pub async fn serial_available_parities(req: web::Json<GatewayInvokeVO>) -> impl Responder {
    let vo = req.into_inner();
    if let Some(manager) = CONTEXT.mqtt_manager.get() {
        let params = AvailableParitiesParams {};
        let result = manager.gateway_invoke(
            &vo.device_hash,
            &vo.protocol_type,
            &vo.stack_name,
            &vo.action,
            &vo.method,
            json!(params)
        ).await;
        let final_result: Result<serde_json::Value, crate::error::Error> = result
            .map_err(|e| crate::error::Error::from(e.to_string()));
        RespVO::from_result(&final_result).resp_json()
    } else {
        RespVO::<()>::from_error_info(CODE_FAIL, "MQTT 客户端未初始化").resp_json()
    }
}

/// 获取支持的停止位选项
#[post("/gateway/serial/availableStopBits")]
pub async fn serial_available_stopbits(req: web::Json<GatewayInvokeVO>) -> impl Responder {
    let vo = req.into_inner();
    if let Some(manager) = CONTEXT.mqtt_manager.get() {
        let params = AvailableStopBitsParams {};
        let result = manager.gateway_invoke(
            &vo.device_hash,
            &vo.protocol_type,
            &vo.stack_name,
            &vo.action,
            &vo.method,
            json!(params)
        ).await;
        let final_result: Result<serde_json::Value, crate::error::Error> = result
            .map_err(|e| crate::error::Error::from(e.to_string()));
        RespVO::from_result(&final_result).resp_json()
    } else {
        RespVO::<()>::from_error_info(CODE_FAIL, "MQTT 客户端未初始化").resp_json()
    }
}

/// 获取支持的流控选项
#[post("/gateway/serial/availableFlowControls")]
pub async fn serial_available_flowcontrols(req: web::Json<GatewayInvokeVO>) -> impl Responder {
    let vo = req.into_inner();
    if let Some(manager) = CONTEXT.mqtt_manager.get() {
        let params = AvailableFlowControlsParams {};
        let result = manager.gateway_invoke(
            &vo.device_hash,
            &vo.protocol_type,
            &vo.stack_name,
            &vo.action,
            &vo.method,
            json!(params)
        ).await;
        let final_result: Result<serde_json::Value, crate::error::Error> = result
            .map_err(|e| crate::error::Error::from(e.to_string()));
        RespVO::from_result(&final_result).resp_json()
    } else {
        RespVO::<()>::from_error_info(CODE_FAIL, "MQTT 客户端未初始化").resp_json()
    }
}

#[post("/gateway/serial/setDataBits")]
pub async fn serial_set_databits(req: web::Json<GatewayInvokeVO>) -> impl Responder {
    let vo = req.into_inner();
    if let Some(manager) = CONTEXT.mqtt_manager.get() {
        let port_name = get_string_param(&vo.params, &["portName", "port_name"]).unwrap_or_default();
        let data_bits = get_string_param(&vo.params, &["dataBits", "data_bits"]).unwrap_or_default();

        if port_name.is_empty() || data_bits.is_empty() {
            return RespVO::<()>::from_error_info(CODE_FAIL, "缺少参数 portName 或 dataBits").resp_json();
        }

        let params = SetDataBitsParams { port_name, data_bits };
        let result = manager.gateway_invoke(
            &vo.device_hash,
            &vo.protocol_type,
            &vo.stack_name,
            &vo.action,
            &vo.method,
            json!(params)
        ).await;
        let final_result: Result<serde_json::Value, crate::error::Error> = result
            .map_err(|e| crate::error::Error::from(e.to_string()));
        RespVO::from_result(&final_result).resp_json()
    } else {
        RespVO::<()>::from_error_info(CODE_FAIL, "MQTT 瀹㈡埛绔湭鍒濩?)").resp_json()
    }
}

#[post("/gateway/serial/setParity")]
pub async fn serial_set_parity(req: web::Json<GatewayInvokeVO>) -> impl Responder {
    let vo = req.into_inner();
    if let Some(manager) = CONTEXT.mqtt_manager.get() {
        let port_name = get_string_param(&vo.params, &["portName", "port_name"]).unwrap_or_default();
        let parity = get_string_param(&vo.params, &["parity"]).unwrap_or_default();

        if port_name.is_empty() || parity.is_empty() {
            return RespVO::<()>::from_error_info(CODE_FAIL, "缺少参数 portName 或 parity").resp_json();
        }

        let params = SetParityParams { port_name, parity };
        let result = manager.gateway_invoke(
            &vo.device_hash,
            &vo.protocol_type,
            &vo.stack_name,
            &vo.action,
            &vo.method,
            json!(params)
        ).await;
        let final_result: Result<serde_json::Value, crate::error::Error> = result
            .map_err(|e| crate::error::Error::from(e.to_string()));
        RespVO::from_result(&final_result).resp_json()
    } else {
        RespVO::<()>::from_error_info(CODE_FAIL, "MQTT 瀹㈡埛绔湭鍒濩?)").resp_json()
    }
}

#[post("/gateway/serial/setStopBits")]
pub async fn serial_set_stopbits(req: web::Json<GatewayInvokeVO>) -> impl Responder {
    let vo = req.into_inner();
    if let Some(manager) = CONTEXT.mqtt_manager.get() {
        let port_name = get_string_param(&vo.params, &["portName", "port_name"]).unwrap_or_default();
        let stop_bits = get_string_param(&vo.params, &["stopBits", "stop_bits"]).unwrap_or_default();

        if port_name.is_empty() || stop_bits.is_empty() {
            return RespVO::<()>::from_error_info(CODE_FAIL, "缺少参数 portName 或 stopBits").resp_json();
        }

        let params = SetStopBitsParams { port_name, stop_bits };
        let result = manager.gateway_invoke(
            &vo.device_hash,
            &vo.protocol_type,
            &vo.stack_name,
            &vo.action,
            &vo.method,
            json!(params)
        ).await;
        let final_result: Result<serde_json::Value, crate::error::Error> = result
            .map_err(|e| crate::error::Error::from(e.to_string()));
        RespVO::from_result(&final_result).resp_json()
    } else {
        RespVO::<()>::from_error_info(CODE_FAIL, "MQTT 瀹㈡埛绔湭鍒濩?)").resp_json()
    }
}

#[post("/gateway/serial/setFlowControl")]
pub async fn serial_set_flowcontrol(req: web::Json<GatewayInvokeVO>) -> impl Responder {
    let vo = req.into_inner();
    if let Some(manager) = CONTEXT.mqtt_manager.get() {
        let port_name = get_string_param(&vo.params, &["portName", "port_name"]).unwrap_or_default();
        let flow_control = get_string_param(&vo.params, &["flowControl", "flow_control"]).unwrap_or_default();

        if port_name.is_empty() || flow_control.is_empty() {
            return RespVO::<()>::from_error_info(CODE_FAIL, "缺少参数 portName 或 flowControl").resp_json();
        }

        let params = SetFlowControlParams { port_name, flow_control };
        let result = manager.gateway_invoke(
            &vo.device_hash,
            &vo.protocol_type,
            &vo.stack_name,
            &vo.action,
            &vo.method,
            json!(params)
        ).await;
        let final_result: Result<serde_json::Value, crate::error::Error> = result
            .map_err(|e| crate::error::Error::from(e.to_string()));
        RespVO::from_result(&final_result).resp_json()
    } else {
        RespVO::<()>::from_error_info(CODE_FAIL, "MQTT 瀹㈡埛绔湭鍒濩?)").resp_json()
    }
}

#[post("/gateway/logger/availableLogLevels")]
pub async fn logger_available_levels(req: web::Json<GatewayInvokeVO>) -> impl Responder {
    let vo = req.into_inner();
    if let Some(manager) = CONTEXT.mqtt_manager.get() {
        let result = manager.gateway_invoke(
            &vo.device_hash,
            &vo.protocol_type,
            &vo.stack_name,
            &vo.action,
            &vo.method,
            params_or_empty(vo.params)
        ).await;
        let final_result: Result<serde_json::Value, crate::error::Error> = result
            .map_err(|e| crate::error::Error::from(e.to_string()));
        RespVO::from_result(&final_result).resp_json()
    } else {
        RespVO::<()>::from_error_info(CODE_FAIL, "MQTT 客户端未初始化").resp_json()
    }
}

#[post("/gateway/logger/openLogChannel")]
pub async fn logger_open_channel(req: web::Json<GatewayInvokeVO>) -> impl Responder {
    let vo = req.into_inner();
    if let Some(manager) = CONTEXT.mqtt_manager.get() {
        let result = manager.gateway_invoke(
            &vo.device_hash,
            &vo.protocol_type,
            &vo.stack_name,
            &vo.action,
            &vo.method,
            params_or_empty(vo.params)
        ).await;
        let final_result: Result<serde_json::Value, crate::error::Error> = result
            .map_err(|e| crate::error::Error::from(e.to_string()));
        RespVO::from_result(&final_result).resp_json()
    } else {
        RespVO::<()>::from_error_info(CODE_FAIL, "MQTT 客户端未初始化").resp_json()
    }
}

#[post("/gateway/logger/closeLogChannel")]
pub async fn logger_close_channel(req: web::Json<GatewayInvokeVO>) -> impl Responder {
    let vo = req.into_inner();
    if let Some(manager) = CONTEXT.mqtt_manager.get() {
        let result = manager.gateway_invoke(
            &vo.device_hash,
            &vo.protocol_type,
            &vo.stack_name,
            &vo.action,
            &vo.method,
            params_or_empty(vo.params)
        ).await;
        let final_result: Result<serde_json::Value, crate::error::Error> = result
            .map_err(|e| crate::error::Error::from(e.to_string()));
        RespVO::from_result(&final_result).resp_json()
    } else {
        RespVO::<()>::from_error_info(CODE_FAIL, "MQTT 客户端未初始化").resp_json()
    }
}

#[post("/gateway/logger/globalLogLevel")]
pub async fn logger_global_level(req: web::Json<GatewayInvokeVO>) -> impl Responder {
    let vo = req.into_inner();
    if let Some(manager) = CONTEXT.mqtt_manager.get() {
        let result = manager.gateway_invoke(
            &vo.device_hash,
            &vo.protocol_type,
            &vo.stack_name,
            &vo.action,
            &vo.method,
            params_or_empty(vo.params)
        ).await;
        let final_result: Result<serde_json::Value, crate::error::Error> = result
            .map_err(|e| crate::error::Error::from(e.to_string()));
        RespVO::from_result(&final_result).resp_json()
    } else {
        RespVO::<()>::from_error_info(CODE_FAIL, "MQTT 客户端未初始化").resp_json()
    }
}

#[post("/gateway/logger/setGlobalLogLevel")]
pub async fn logger_set_global_level(req: web::Json<GatewayInvokeVO>) -> impl Responder {
    let vo = req.into_inner();
    if let Some(manager) = CONTEXT.mqtt_manager.get() {
        let result = manager.gateway_invoke(
            &vo.device_hash,
            &vo.protocol_type,
            &vo.stack_name,
            &vo.action,
            &vo.method,
            params_or_empty(vo.params)
        ).await;
        let final_result: Result<serde_json::Value, crate::error::Error> = result
            .map_err(|e| crate::error::Error::from(e.to_string()));
        RespVO::from_result(&final_result).resp_json()
    } else {
        RespVO::<()>::from_error_info(CODE_FAIL, "MQTT 客户端未初始化").resp_json()
    }
}

#[post("/gateway/logger/maxLogsFileSize")]
pub async fn logger_max_file_size(req: web::Json<GatewayInvokeVO>) -> impl Responder {
    let vo = req.into_inner();
    if let Some(manager) = CONTEXT.mqtt_manager.get() {
        let result = manager.gateway_invoke(
            &vo.device_hash,
            &vo.protocol_type,
            &vo.stack_name,
            &vo.action,
            &vo.method,
            params_or_empty(vo.params)
        ).await;
        let final_result: Result<serde_json::Value, crate::error::Error> = result
            .map_err(|e| crate::error::Error::from(e.to_string()));
        RespVO::from_result(&final_result).resp_json()
    } else {
        RespVO::<()>::from_error_info(CODE_FAIL, "MQTT 客户端未初始化").resp_json()
    }
}

#[post("/gateway/logger/setMaxLogsFileSize")]
pub async fn logger_set_max_file_size(req: web::Json<GatewayInvokeVO>) -> impl Responder {
    let vo = req.into_inner();
    if let Some(manager) = CONTEXT.mqtt_manager.get() {
        let result = manager.gateway_invoke(
            &vo.device_hash,
            &vo.protocol_type,
            &vo.stack_name,
            &vo.action,
            &vo.method,
            params_or_empty(vo.params)
        ).await;
        let final_result: Result<serde_json::Value, crate::error::Error> = result
            .map_err(|e| crate::error::Error::from(e.to_string()));
        RespVO::from_result(&final_result).resp_json()
    } else {
        RespVO::<()>::from_error_info(CODE_FAIL, "MQTT 客户端未初始化").resp_json()
    }
}

#[post("/gateway/maintenance/addRepository")]
pub async fn maintenance_add_repository(req: web::Json<GatewayInvokeVO>) -> impl Responder {
    let vo = req.into_inner();
    if let Some(manager) = CONTEXT.mqtt_manager.get() {
        let result = manager.gateway_invoke(
            &vo.device_hash,
            &vo.protocol_type,
            &vo.stack_name,
            &vo.action,
            &vo.method,
            params_or_empty(vo.params)
        ).await;
        let final_result: Result<serde_json::Value, crate::error::Error> = result
            .map_err(|e| crate::error::Error::from(e.to_string()));
        RespVO::from_result(&final_result).resp_json()
    } else {
        RespVO::<()>::from_error_info(CODE_FAIL, "MQTT 客户端未初始化").resp_json()
    }
}

#[post("/gateway/maintenance/addTempRepository")]
pub async fn maintenance_add_temp_repository(req: web::Json<GatewayInvokeVO>) -> impl Responder {
    let vo = req.into_inner();
    if let Some(manager) = CONTEXT.mqtt_manager.get() {
        let result = manager.gateway_invoke(
            &vo.device_hash,
            &vo.protocol_type,
            &vo.stack_name,
            &vo.action,
            &vo.method,
            params_or_empty(vo.params)
        ).await;
        let final_result: Result<serde_json::Value, crate::error::Error> = result
            .map_err(|e| crate::error::Error::from(e.to_string()));
        RespVO::from_result(&final_result).resp_json()
    } else {
        RespVO::<()>::from_error_info(CODE_FAIL, "MQTT 客户端未初始化").resp_json()
    }
}

#[post("/gateway/maintenance/checkUpdates")]
pub async fn maintenance_check_updates(req: web::Json<GatewayInvokeVO>) -> impl Responder {
    let vo = req.into_inner();
    if let Some(manager) = CONTEXT.mqtt_manager.get() {
        let result = manager.gateway_invoke(
            &vo.device_hash,
            &vo.protocol_type,
            &vo.stack_name,
            &vo.action,
            &vo.method,
            params_or_empty(vo.params)
        ).await;
        let final_result: Result<serde_json::Value, crate::error::Error> = result
            .map_err(|e| crate::error::Error::from(e.to_string()));
        RespVO::from_result(&final_result).resp_json()
    } else {
        RespVO::<()>::from_error_info(CODE_FAIL, "MQTT 客户端未初始化").resp_json()
    }
}

#[post("/gateway/maintenance/update")]
pub async fn maintenance_update(req: web::Json<GatewayInvokeVO>) -> impl Responder {
    let vo = req.into_inner();
    if let Some(manager) = CONTEXT.mqtt_manager.get() {
        let result = manager.gateway_invoke(
            &vo.device_hash,
            &vo.protocol_type,
            &vo.stack_name,
            &vo.action,
            &vo.method,
            params_or_empty(vo.params)
        ).await;
        let final_result: Result<serde_json::Value, crate::error::Error> = result
            .map_err(|e| crate::error::Error::from(e.to_string()));
        RespVO::from_result(&final_result).resp_json()
    } else {
        RespVO::<()>::from_error_info(CODE_FAIL, "MQTT 客户端未初始化").resp_json()
    }
}

#[post("/gateway/network/allInterfaces")]
pub async fn network_all_interfaces(req: web::Json<GatewayInvokeVO>) -> impl Responder {
    let vo = req.into_inner();
    if let Some(manager) = CONTEXT.mqtt_manager.get() {
        let result = manager.gateway_invoke(
            &vo.device_hash,
            &vo.protocol_type,
            &vo.stack_name,
            &vo.action,
            &vo.method,
            params_or_empty(vo.params)
        ).await;
        let final_result: Result<serde_json::Value, crate::error::Error> = result
            .map_err(|e| crate::error::Error::from(e.to_string()));
        RespVO::from_result(&final_result).resp_json()
    } else {
        RespVO::<()>::from_error_info(CODE_FAIL, "MQTT 客户端未初始化").resp_json()
    }
}

#[post("/gateway/network/setInterfaceUp")]
pub async fn network_set_interface_up(req: web::Json<GatewayInvokeVO>) -> impl Responder {
    let vo = req.into_inner();
    if let Some(manager) = CONTEXT.mqtt_manager.get() {
        let result = manager.gateway_invoke(
            &vo.device_hash,
            &vo.protocol_type,
            &vo.stack_name,
            &vo.action,
            &vo.method,
            params_or_empty(vo.params)
        ).await;
        let final_result: Result<serde_json::Value, crate::error::Error> = result
            .map_err(|e| crate::error::Error::from(e.to_string()));
        RespVO::from_result(&final_result).resp_json()
    } else {
        RespVO::<()>::from_error_info(CODE_FAIL, "MQTT 客户端未初始化").resp_json()
    }
}

#[post("/gateway/network/setInterfaceDown")]
pub async fn network_set_interface_down(req: web::Json<GatewayInvokeVO>) -> impl Responder {
    let vo = req.into_inner();
    if let Some(manager) = CONTEXT.mqtt_manager.get() {
        let result = manager.gateway_invoke(
            &vo.device_hash,
            &vo.protocol_type,
            &vo.stack_name,
            &vo.action,
            &vo.method,
            params_or_empty(vo.params)
        ).await;
        let final_result: Result<serde_json::Value, crate::error::Error> = result
            .map_err(|e| crate::error::Error::from(e.to_string()));
        RespVO::from_result(&final_result).resp_json()
    } else {
        RespVO::<()>::from_error_info(CODE_FAIL, "MQTT 客户端未初始化").resp_json()
    }
}

#[post("/gateway/network/enableInterfaceDHCPClient")]
pub async fn network_enable_dhcp(req: web::Json<GatewayInvokeVO>) -> impl Responder {
    let vo = req.into_inner();
    if let Some(manager) = CONTEXT.mqtt_manager.get() {
        let result = manager.gateway_invoke(
            &vo.device_hash,
            &vo.protocol_type,
            &vo.stack_name,
            &vo.action,
            &vo.method,
            params_or_empty(vo.params)
        ).await;
        let final_result: Result<serde_json::Value, crate::error::Error> = result
            .map_err(|e| crate::error::Error::from(e.to_string()));
        RespVO::from_result(&final_result).resp_json()
    } else {
        RespVO::<()>::from_error_info(CODE_FAIL, "MQTT 客户端未初始化").resp_json()
    }
}

#[post("/gateway/network/disableInterfaceDHCPClient")]
pub async fn network_disable_dhcp(req: web::Json<GatewayInvokeVO>) -> impl Responder {
    let vo = req.into_inner();
    if let Some(manager) = CONTEXT.mqtt_manager.get() {
        let result = manager.gateway_invoke(
            &vo.device_hash,
            &vo.protocol_type,
            &vo.stack_name,
            &vo.action,
            &vo.method,
            params_or_empty(vo.params)
        ).await;
        let final_result: Result<serde_json::Value, crate::error::Error> = result
            .map_err(|e| crate::error::Error::from(e.to_string()));
        RespVO::from_result(&final_result).resp_json()
    } else {
        RespVO::<()>::from_error_info(CODE_FAIL, "MQTT 客户端未初始化").resp_json()
    }
}

#[post("/gateway/network/addInterfaceAddressEntry")]
pub async fn network_add_address(req: web::Json<GatewayInvokeVO>) -> impl Responder {
    let vo = req.into_inner();
    if let Some(manager) = CONTEXT.mqtt_manager.get() {
        let result = manager.gateway_invoke(
            &vo.device_hash,
            &vo.protocol_type,
            &vo.stack_name,
            &vo.action,
            &vo.method,
            params_or_empty(vo.params)
        ).await;
        let final_result: Result<serde_json::Value, crate::error::Error> = result
            .map_err(|e| crate::error::Error::from(e.to_string()));
        RespVO::from_result(&final_result).resp_json()
    } else {
        RespVO::<()>::from_error_info(CODE_FAIL, "MQTT 客户端未初始化").resp_json()
    }
}

#[post("/gateway/network/removeInterfaceAddressEntry")]
pub async fn network_remove_address(req: web::Json<GatewayInvokeVO>) -> impl Responder {
    let vo = req.into_inner();
    if let Some(manager) = CONTEXT.mqtt_manager.get() {
        let result = manager.gateway_invoke(
            &vo.device_hash,
            &vo.protocol_type,
            &vo.stack_name,
            &vo.action,
            &vo.method,
            params_or_empty(vo.params)
        ).await;
        let final_result: Result<serde_json::Value, crate::error::Error> = result
            .map_err(|e| crate::error::Error::from(e.to_string()));
        RespVO::from_result(&final_result).resp_json()
    } else {
        RespVO::<()>::from_error_info(CODE_FAIL, "MQTT 客户端未初始化").resp_json()
    }
}

#[post("/gateway/network/setInterfaceAddressEntry")]
pub async fn network_set_address(req: web::Json<GatewayInvokeVO>) -> impl Responder {
    let vo = req.into_inner();
    if let Some(manager) = CONTEXT.mqtt_manager.get() {
        let result = manager.gateway_invoke(
            &vo.device_hash,
            &vo.protocol_type,
            &vo.stack_name,
            &vo.action,
            &vo.method,
            params_or_empty(vo.params)
        ).await;
        let final_result: Result<serde_json::Value, crate::error::Error> = result
            .map_err(|e| crate::error::Error::from(e.to_string()));
        RespVO::from_result(&final_result).resp_json()
    } else {
        RespVO::<()>::from_error_info(CODE_FAIL, "MQTT 客户端未初始化").resp_json()
    }
}

#[post("/gateway/tcp/availableTCPClients")]
pub async fn tcp_available_clients(req: web::Json<GatewayInvokeVO>) -> impl Responder {
    let vo = req.into_inner();
    if let Some(manager) = CONTEXT.mqtt_manager.get() {
        let result = manager.gateway_invoke(
            &vo.device_hash,
            &vo.protocol_type,
            &vo.stack_name,
            &vo.action,
            &vo.method,
            params_or_empty(vo.params)
        ).await;
        let final_result: Result<serde_json::Value, crate::error::Error> = result
            .map_err(|e| crate::error::Error::from(e.to_string()));
        RespVO::from_result(&final_result).resp_json()
    } else {
        RespVO::<()>::from_error_info(CODE_FAIL, "MQTT 客户端未初始化").resp_json()
    }
}

#[post("/gateway/tcp/registerTCPClient")]
pub async fn tcp_register_client(req: web::Json<GatewayInvokeVO>) -> impl Responder {
    let vo = req.into_inner();
    if let Some(manager) = CONTEXT.mqtt_manager.get() {
        let result = manager.gateway_invoke(
            &vo.device_hash,
            &vo.protocol_type,
            &vo.stack_name,
            &vo.action,
            &vo.method,
            params_or_empty(vo.params)
        ).await;
        let final_result: Result<serde_json::Value, crate::error::Error> = result
            .map_err(|e| crate::error::Error::from(e.to_string()));
        RespVO::from_result(&final_result).resp_json()
    } else {
        RespVO::<()>::from_error_info(CODE_FAIL, "MQTT 客户端未初始化").resp_json()
    }
}

#[post("/gateway/tcp/unregisterTCPClient")]
pub async fn tcp_unregister_client(req: web::Json<GatewayInvokeVO>) -> impl Responder {
    let vo = req.into_inner();
    if let Some(manager) = CONTEXT.mqtt_manager.get() {
        let result = manager.gateway_invoke(
            &vo.device_hash,
            &vo.protocol_type,
            &vo.stack_name,
            &vo.action,
            &vo.method,
            params_or_empty(vo.params)
        ).await;
        let final_result: Result<serde_json::Value, crate::error::Error> = result
            .map_err(|e| crate::error::Error::from(e.to_string()));
        RespVO::from_result(&final_result).resp_json()
    } else {
        RespVO::<()>::from_error_info(CODE_FAIL, "MQTT 客户端未初始化").resp_json()
    }
}

#[post("/gateway/tcp/setTCPClientAndReconnect")]
pub async fn tcp_set_client_reconnect(req: web::Json<GatewayInvokeVO>) -> impl Responder {
    let vo = req.into_inner();
    if let Some(manager) = CONTEXT.mqtt_manager.get() {
        let result = manager.gateway_invoke(
            &vo.device_hash,
            &vo.protocol_type,
            &vo.stack_name,
            &vo.action,
            &vo.method,
            params_or_empty(vo.params)
        ).await;
        let final_result: Result<serde_json::Value, crate::error::Error> = result
            .map_err(|e| crate::error::Error::from(e.to_string()));
        RespVO::from_result(&final_result).resp_json()
    } else {
        RespVO::<()>::from_error_info(CODE_FAIL, "MQTT 客户端未初始化").resp_json()
    }
}

#[post("/gateway/mqtt/availableMQTTClients")]
pub async fn mqtt_available_clients(req: web::Json<GatewayInvokeVO>) -> impl Responder {
    let vo = req.into_inner();
    if let Some(manager) = CONTEXT.mqtt_manager.get() {
        let result = manager.gateway_invoke(
            &vo.device_hash,
            &vo.protocol_type,
            &vo.stack_name,
            &vo.action,
            &vo.method,
            params_or_empty(vo.params)
        ).await;
        let final_result: Result<serde_json::Value, crate::error::Error> = result
            .map_err(|e| crate::error::Error::from(e.to_string()));
        RespVO::from_result(&final_result).resp_json()
    } else {
        RespVO::<()>::from_error_info(CODE_FAIL, "MQTT 客户端未初始化").resp_json()
    }
}

#[post("/gateway/mqtt/registerMQTTClient")]
pub async fn mqtt_register_client(req: web::Json<GatewayInvokeVO>) -> impl Responder {
    let vo = req.into_inner();
    if let Some(manager) = CONTEXT.mqtt_manager.get() {
        let result = manager.gateway_invoke(
            &vo.device_hash,
            &vo.protocol_type,
            &vo.stack_name,
            &vo.action,
            &vo.method,
            params_or_empty(vo.params)
        ).await;
        let final_result: Result<serde_json::Value, crate::error::Error> = result
            .map_err(|e| crate::error::Error::from(e.to_string()));
        RespVO::from_result(&final_result).resp_json()
    } else {
        RespVO::<()>::from_error_info(CODE_FAIL, "MQTT 客户端未初始化").resp_json()
    }
}

#[post("/gateway/mqtt/unregisterMQTTClient")]
pub async fn mqtt_unregister_client(req: web::Json<GatewayInvokeVO>) -> impl Responder {
    let vo = req.into_inner();
    if let Some(manager) = CONTEXT.mqtt_manager.get() {
        let result = manager.gateway_invoke(
            &vo.device_hash,
            &vo.protocol_type,
            &vo.stack_name,
            &vo.action,
            &vo.method,
            params_or_empty(vo.params)
        ).await;
        let final_result: Result<serde_json::Value, crate::error::Error> = result
            .map_err(|e| crate::error::Error::from(e.to_string()));
        RespVO::from_result(&final_result).resp_json()
    } else {
        RespVO::<()>::from_error_info(CODE_FAIL, "MQTT 客户端未初始化").resp_json()
    }
}

#[post("/gateway/mqtt/setIsSSLAndReconnect")]
pub async fn mqtt_set_ssl_reconnect(req: web::Json<GatewayInvokeVO>) -> impl Responder {
    let vo = req.into_inner();
    if let Some(manager) = CONTEXT.mqtt_manager.get() {
        let result = manager.gateway_invoke(
            &vo.device_hash,
            &vo.protocol_type,
            &vo.stack_name,
            &vo.action,
            &vo.method,
            params_or_empty(vo.params)
        ).await;
        let final_result: Result<serde_json::Value, crate::error::Error> = result
            .map_err(|e| crate::error::Error::from(e.to_string()));
        RespVO::from_result(&final_result).resp_json()
    } else {
        RespVO::<()>::from_error_info(CODE_FAIL, "MQTT 客户端未初始化").resp_json()
    }
}

#[post("/gateway/mqtt/setMQTTClientAndReconnect")]
pub async fn mqtt_set_client_reconnect(req: web::Json<GatewayInvokeVO>) -> impl Responder {
    let vo = req.into_inner();
    if let Some(manager) = CONTEXT.mqtt_manager.get() {
        let result = manager.gateway_invoke(
            &vo.device_hash,
            &vo.protocol_type,
            &vo.stack_name,
            &vo.action,
            &vo.method,
            params_or_empty(vo.params)
        ).await;
        let final_result: Result<serde_json::Value, crate::error::Error> = result
            .map_err(|e| crate::error::Error::from(e.to_string()));
        RespVO::from_result(&final_result).resp_json()
    } else {
        RespVO::<()>::from_error_info(CODE_FAIL, "MQTT 客户端未初始化").resp_json()
    }
}

#[post("/gateway/ntp/availableNTPClients")]
pub async fn ntp_available_clients(req: web::Json<GatewayInvokeVO>) -> impl Responder {
    let vo = req.into_inner();
    if let Some(manager) = CONTEXT.mqtt_manager.get() {
        let result = manager.gateway_invoke(
            &vo.device_hash,
            &vo.protocol_type,
            &vo.stack_name,
            &vo.action,
            &vo.method,
            params_or_empty(vo.params)
        ).await;
        let final_result: Result<serde_json::Value, crate::error::Error> = result
            .map_err(|e| crate::error::Error::from(e.to_string()));
        RespVO::from_result(&final_result).resp_json()
    } else {
        RespVO::<()>::from_error_info(CODE_FAIL, "MQTT 客户端未初始化").resp_json()
    }
}

#[post("/gateway/ntp/sendNTPRequestsToAll")]
pub async fn ntp_send_requests(req: web::Json<GatewayInvokeVO>) -> impl Responder {
    let vo = req.into_inner();
    if let Some(manager) = CONTEXT.mqtt_manager.get() {
        let result = manager.gateway_invoke(
            &vo.device_hash,
            &vo.protocol_type,
            &vo.stack_name,
            &vo.action,
            &vo.method,
            params_or_empty(vo.params)
        ).await;
        let final_result: Result<serde_json::Value, crate::error::Error> = result
            .map_err(|e| crate::error::Error::from(e.to_string()));
        RespVO::from_result(&final_result).resp_json()
    } else {
        RespVO::<()>::from_error_info(CODE_FAIL, "MQTT 客户端未初始化").resp_json()
    }
}

#[post("/gateway/ntp/registerNTPClient")]
pub async fn ntp_register_client(req: web::Json<GatewayInvokeVO>) -> impl Responder {
    let vo = req.into_inner();
    if let Some(manager) = CONTEXT.mqtt_manager.get() {
        let result = manager.gateway_invoke(
            &vo.device_hash,
            &vo.protocol_type,
            &vo.stack_name,
            &vo.action,
            &vo.method,
            params_or_empty(vo.params)
        ).await;
        let final_result: Result<serde_json::Value, crate::error::Error> = result
            .map_err(|e| crate::error::Error::from(e.to_string()));
        RespVO::from_result(&final_result).resp_json()
    } else {
        RespVO::<()>::from_error_info(CODE_FAIL, "MQTT 客户端未初始化").resp_json()
    }
}

#[post("/gateway/ntp/unregisterNTPClient")]
pub async fn ntp_unregister_client(req: web::Json<GatewayInvokeVO>) -> impl Responder {
    let vo = req.into_inner();
    if let Some(manager) = CONTEXT.mqtt_manager.get() {
        let result = manager.gateway_invoke(
            &vo.device_hash,
            &vo.protocol_type,
            &vo.stack_name,
            &vo.action,
            &vo.method,
            params_or_empty(vo.params)
        ).await;
        let final_result: Result<serde_json::Value, crate::error::Error> = result
            .map_err(|e| crate::error::Error::from(e.to_string()));
        RespVO::from_result(&final_result).resp_json()
    } else {
        RespVO::<()>::from_error_info(CODE_FAIL, "MQTT 客户端未初始化").resp_json()
    }
}

#[post("/gateway/ntp/setNTPClient")]
pub async fn ntp_set_client(req: web::Json<GatewayInvokeVO>) -> impl Responder {
    let vo = req.into_inner();
    if let Some(manager) = CONTEXT.mqtt_manager.get() {
        let result = manager.gateway_invoke(
            &vo.device_hash,
            &vo.protocol_type,
            &vo.stack_name,
            &vo.action,
            &vo.method,
            params_or_empty(vo.params)
        ).await;
        let final_result: Result<serde_json::Value, crate::error::Error> = result
            .map_err(|e| crate::error::Error::from(e.to_string()));
        RespVO::from_result(&final_result).resp_json()
    } else {
        RespVO::<()>::from_error_info(CODE_FAIL, "MQTT 客户端未初始化").resp_json()
    }
}

#[post("/gateway/protocol/availableProtocolStacks")]
pub async fn protocol_available_stacks(req: web::Json<GatewayInvokeVO>) -> impl Responder {
    let vo = req.into_inner();
    if let Some(manager) = CONTEXT.mqtt_manager.get() {
        let result = manager.gateway_invoke(
            &vo.device_hash,
            &vo.protocol_type,
            &vo.stack_name,
            &vo.action,
            &vo.method,
            params_or_empty(vo.params)
        ).await;
        let final_result: Result<serde_json::Value, crate::error::Error> = result
            .map_err(|e| crate::error::Error::from(e.to_string()));
        RespVO::from_result(&final_result).resp_json()
    } else {
        RespVO::<()>::from_error_info(CODE_FAIL, "MQTT 客户端未初始化").resp_json()
    }
}

#[post("/gateway/protocol/availableProtocolType")]
pub async fn protocol_available_types(req: web::Json<GatewayInvokeVO>) -> impl Responder {
    let vo = req.into_inner();
    if let Some(manager) = CONTEXT.mqtt_manager.get() {
        let result = manager.gateway_invoke(
            &vo.device_hash,
            &vo.protocol_type,
            &vo.stack_name,
            &vo.action,
            &vo.method,
            params_or_empty(vo.params)
        ).await;
        let final_result: Result<serde_json::Value, crate::error::Error> = result
            .map_err(|e| crate::error::Error::from(e.to_string()));
        RespVO::from_result(&final_result).resp_json()
    } else {
        RespVO::<()>::from_error_info(CODE_FAIL, "MQTT 客户端未初始化").resp_json()
    }
}

#[post("/gateway/protocol/availableDataFormat")]
pub async fn protocol_available_formats(req: web::Json<GatewayInvokeVO>) -> impl Responder {
    let vo = req.into_inner();
    if let Some(manager) = CONTEXT.mqtt_manager.get() {
        let result = manager.gateway_invoke(
            &vo.device_hash,
            &vo.protocol_type,
            &vo.stack_name,
            &vo.action,
            &vo.method,
            params_or_empty(vo.params)
        ).await;
        let final_result: Result<serde_json::Value, crate::error::Error> = result
            .map_err(|e| crate::error::Error::from(e.to_string()));
        RespVO::from_result(&final_result).resp_json()
    } else {
        RespVO::<()>::from_error_info(CODE_FAIL, "MQTT 客户端未初始化").resp_json()
    }
}

#[post("/gateway/protocol/registerProtocolStack")]
pub async fn protocol_register_stack(req: web::Json<GatewayInvokeVO>) -> impl Responder {
    let vo = req.into_inner();
    if let Some(manager) = CONTEXT.mqtt_manager.get() {
        let result = manager.gateway_invoke(
            &vo.device_hash,
            &vo.protocol_type,
            &vo.stack_name,
            &vo.action,
            &vo.method,
            params_or_empty(vo.params)
        ).await;
        let final_result: Result<serde_json::Value, crate::error::Error> = result
            .map_err(|e| crate::error::Error::from(e.to_string()));
        RespVO::from_result(&final_result).resp_json()
    } else {
        RespVO::<()>::from_error_info(CODE_FAIL, "MQTT 客户端未初始化").resp_json()
    }
}

#[post("/gateway/protocol/unregisterProtocolStack")]
pub async fn protocol_unregister_stack(req: web::Json<GatewayInvokeVO>) -> impl Responder {
    let vo = req.into_inner();
    if let Some(manager) = CONTEXT.mqtt_manager.get() {
        let result = manager.gateway_invoke(
            &vo.device_hash,
            &vo.protocol_type,
            &vo.stack_name,
            &vo.action,
            &vo.method,
            params_or_empty(vo.params)
        ).await;
        let final_result: Result<serde_json::Value, crate::error::Error> = result
            .map_err(|e| crate::error::Error::from(e.to_string()));
        RespVO::from_result(&final_result).resp_json()
    } else {
        RespVO::<()>::from_error_info(CODE_FAIL, "MQTT 客户端未初始化").resp_json()
    }
}

#[post("/gateway/protocol/setProtocolStack")]
pub async fn protocol_set_stack(req: web::Json<GatewayInvokeVO>) -> impl Responder {
    let vo = req.into_inner();
    if let Some(manager) = CONTEXT.mqtt_manager.get() {
        let result = manager.gateway_invoke(
            &vo.device_hash,
            &vo.protocol_type,
            &vo.stack_name,
            &vo.action,
            &vo.method,
            params_or_empty(vo.params)
        ).await;
        let final_result: Result<serde_json::Value, crate::error::Error> = result
            .map_err(|e| crate::error::Error::from(e.to_string()));
        RespVO::from_result(&final_result).resp_json()
    } else {
        RespVO::<()>::from_error_info(CODE_FAIL, "MQTT 客户端未初始化").resp_json()
    }
}

