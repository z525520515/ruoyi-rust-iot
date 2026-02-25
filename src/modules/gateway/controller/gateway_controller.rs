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
    OpenSerialPortParams,
    SetBaudRateParams,
    AvailableDataBitsParams,
    AvailableParitiesParams,
    AvailableStopBitsParams,
    AvailableFlowControlsParams,
};
use actix_web::{post, web, Responder};
use serde::{Serialize, Deserialize};
use serde_json::json;

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
        let current_password = vo.params.get("current_password")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let new_password = vo.params.get("new_password")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

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
        let params = SetTimeParams(chrono::Utc::now().timestamp_millis());
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
        let timezone = vo.params.get("timezone")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

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
        let port_name = vo.params.get("port_name")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        if port_name.is_empty() {
            return RespVO::<()>::from_error_info(CODE_FAIL, "缺少参数 port_name").resp_json();
        }

        let params = OpenSerialPortParams { port_name };
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
        let port_name = vo.params.get("port_name")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

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
        let port_name = vo.params.get("port_name")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let baud_rate = vo.params.get("baud_rate")
            .and_then(|v| v.as_u64())
            .unwrap_or(115200) as u32;

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

