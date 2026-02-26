use crate::modules::plc_gateway::api::{config_util::call_device, };
use crate::modules::plc_gateway::unified::response::error_response;
use axum::{
    extract::{Extension, Path},
    response::Json,
};
use sea_orm::DatabaseConnection;
use serde::Deserialize;
use serde_json::{Value, json};
use std::sync::Arc;

/* ==================== 1. 接收的请求体 ==================== */
#[derive(Debug, Deserialize)]
pub struct ConfigReq {
    eth0: EthConfigInput,
    eth1: Eth1ConfigInput,
    wifi: WifiConfigInput,
}

#[derive(Debug, Deserialize)]
pub struct EthConfigInput {
    #[serde(rename = "staticIp")]
    static_ip: bool,
    ip: String,
    mask: String,
    gw: String,
}

#[derive(Debug, Deserialize)]
pub struct Eth1ConfigInput {
    ip: String,
    mask: String,
    gw: String,
}

#[derive(Debug, Deserialize)]
pub struct WifiConfigInput {
    enable: bool,
    ssid: String,
    password: String,
    #[serde(rename = "staticIp")]
    static_ip: bool,
    ip: String,
    mask: String,
    gw: String,
}

/* ==================== 2. 处理函数 ==================== */
/// 修改信息配置
/// 请求示例:
/// ```json
/// {
///     "eth0": {
///         "staticIp": true,
///         "ip": "192.168.1.100",
///         "mask": "255.255.255.0",
///         "gw": "192.168.1.1"
///     },
///     "eth1": {
///         "ip": "192.168.78.1",
///         "mask": "255.255.255.0",
///         "gw": "0.0.0.0"
///     },
///     "wifi": {
///         "enable": false,
///         "ssid": "",
///         "password": "",
///         "staticIp": false,
///         "ip": "192.168.3.100",
///         "mask": "255.255.255.0",
///         "gw": "192.168.3.1"
///     }
/// }
/// ```
/// 响应示例：
/// ```json
/// {
///     "code": 200,
///     "data": {},
///     "message": "success",
///     "timestamp": 1761719745019
/// }
/// ```

pub async fn set_network_config(
    _db: Extension<Arc<DatabaseConnection>>,
    Path(ip): Path<String>,
    Json(payload): Json<ConfigReq>,
) -> Json<Value> {
    // 业务校验
    if payload.wifi.static_ip
        && (payload.wifi.ip.trim().is_empty()
            || payload.wifi.mask.trim().is_empty()
            || payload.wifi.gw.trim().is_empty())
    {
        return error_response(
            "wifi.staticIp 为 true 时，ip、mask、gw 必须提供".to_string(),
            Some(400),
        );
    }

    // 组装转发体
    let forward = json!({
        "type": "system.setNetwork",
        "networkInfo": {
            "eth0": {
                "staticIp": payload.eth0.static_ip,
                "ip": payload.eth0.ip,
                "mask": payload.eth0.mask,
                "gw": payload.eth0.gw
            },
            "eth1": {
                "staticIp": true,
                "ip": payload.eth1.ip,
                "mask": payload.eth1.mask,
                "gw": payload.eth1.gw
            },
            "wifi": {
                "enable": payload.wifi.enable,
                "ssid": payload.wifi.ssid,
                "password": payload.wifi.password,
                "staticIp": payload.wifi.static_ip,
                "ip": payload.wifi.ip,
                "mask": payload.wifi.mask,
                "gw": payload.wifi.gw
            }
        }
    });
    let url = format!("http://{}:80/api/call", ip);
    call_device(url, forward, Some("POST")).await
}

pub async fn set_network_config_eth0(
    _db: Extension<Arc<DatabaseConnection>>,
    Path(ip): Path<String>,
    Json(params): Json<EthConfigInput>,
) -> Json<Value> {
    // 组装转发体
    let forward = json!({
        "type": "system.setNetwork",
        "networkInfo": {
            "eth0": {
                "staticIp": params.static_ip,
                "ip": params.ip,
                "mask": params.mask,
                "gw": params.gw
            },
            "eth1": {
                "staticIp": true,
                "ip": "192.168.78.1",
                "mask": "255.255.255.0",
                "gw": "0.0.0.0"
            },
            "wifi": {
                "enable": false,
                "ssid": "",
                "password": "",
                "staticIp": false,
                "ip": "",
                "mask": "",
                "gw": "0.0.0.0"
            }
        }
    });
    let url = format!("http://{}:80/api/call", ip);
    call_device(url, forward, Some("POST")).await
}
/// 修改网关的网关名称
///
/// 请求示例：/network/name/10.0.0.5/MyWiFi
///
/// 响应示例：
/// ```json
/// {
///     "code": 200,
///     "data": {},
///     "message": "success",
///     "timestamp": 1761719745019
/// }
/// ```
pub async fn reset_network_name(Path((ip, name)): Path<(String, String)>) -> Json<Value> {
    log::info!("reset_network_name: ip={}, name={}", ip, name);
    let forward = json!({
        "type": "system.rename",
        "name": name
    });

    let url = format!("http://{}:80/api/call", ip);
    call_device(url, forward, Some("POST")).await
}
/// 修改网关的网关位置
///
/// 请求示例：/network/location/10.0.0.5/wuhu
///
/// 响应示例：
/// ```json
/// {
///     "code": 200,
///     "data": {},
///     "message": "success",
///     "timestamp": 1761719745019
/// }
/// ```
pub async fn reset_network_set_location(
    Path((ip, position)): Path<(String, String)>,
) -> Json<Value> {
    log::info!(
        "reset_network_set_location: ip={}, position={}",
        ip,
        position
    );
    let forward = json!({
        "type": "system.setLocation",
        "position": position
    });
    let url = format!("http://{}:80/api/call", ip);
    call_device(url, forward, Some("POST")).await
}

#[derive(Debug, Deserialize)]
pub struct TioGwhubReq {
    pub url: String,
}

/// 修改网关的TIO等信息
/// 请求示例:
/// ```json
/// {
///     "url": "mqtt://14.103.235.181:1883"
/// }
/// ```
/// 响应示例：
/// ```json
/// {
///     "code": 200,
///     "data": {},
///     "message": "success",
///     "timestamp": 1761719745019
/// }
/// ```
/// 注意：此操作后，要生效的话，设备需要重启才能生效
pub async fn reset_network_set_platform_tio(
    Path(ip): Path<String>,
    Json(payload): Json<TioGwhubReq>,
) -> Json<Value> {
    log::info!(
        "reset_network_set_platform: ip={}, payload={:?}",
        ip,
        payload
    );
    let forward = json!({
        "type": "system.setPlatformConfig",
        "enable": true,
        "platform": {
            "type": "tio",
            "url": payload.url,
        }
    });
    let url = format!("http://{}:80/api/call", ip);
    call_device(url, forward, Some("POST")).await
}

/// 修改网关的Gateway Hub信息
/// payload 示例:
/// ```json
/// {
///     "url": "ws://gateway.example.com:8080"
/// }
/// ```
/// 注意：此操作后，要生效的话，设备需要重启才能生效
pub async fn reset_network_set_platform_gwhub(
    Path(ip): Path<String>,
    Json(payload): Json<TioGwhubReq>,
) -> Json<Value> {
    log::info!(
        "reset_network_set_platform: ip={}, payload={:?}",
        ip,
        payload
    );
    let forward = json!({
        "type": "system.setPlatformConfig",
        "enable": true,
        "platform": {
            "type": "gwhub",
            "url": payload.url,
        }
    });
    let url = format!("http://{}:80/api/call", ip);
    call_device(url, forward, Some("POST")).await
}

#[derive(Debug, Deserialize)]
pub struct ExplorerReq {
    pub host: String,
    pub port: u64,
}
/// 修改网关的explorer信息
/// payload 示例:
/// ```json
/// {
///      "host":"explorer.example.com",
///      "port":443
/// }
/// ```
/// 响应示例：
/// ```json
/// {
///     "code": 200,
///     "data": {},
///     "message": "success",
///     "timestamp": 1761719745019
/// }
/// ```
/// 注意：此操作后，要生效的话，设备需要重启才能生效
pub async fn reset_network_set_platform_explorer(
    Path(ip): Path<String>,
    Json(payload): Json<ExplorerReq>,
) -> Json<Value> {
    log::info!(
        "reset_network_set_platform: ip={}, payload={:?}",
        ip,
        payload
    );
    let forward = json!({
            "type":"system.setPlatformConfig",
            "enable":true,
            "platform":{
                "type":"explorer",
                "host":payload.host,
                "port":payload.port
            }
    });
    let url = format!("http://{}:80/api/call", ip);
    call_device(url, forward, Some("POST")).await
}
