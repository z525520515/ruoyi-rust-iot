use crate::modules::plc_gateway::api::remote_response::{PendingResponses, Response,Sender};
use crate::modules::plc_gateway::api::remote_util::generate_token;
use anyhow::{Result, anyhow};
use axum::{
    Json,
    extract::{Extension, Path},
    http::StatusCode,
};
use log::{error, info};
use rumqttc::{AsyncClient, QoS};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::oneshot;
use tokio::time::timeout;
// === 结构体定义 ===
// #[derive(Serialize, Deserialize, Debug)]
// pub struct SysInfoReq {
//     #[serde(rename = "clientToken")]
//     pub client_token: String,
//     pub data: serde_json::Value,
// }

#[derive(Serialize, Deserialize, Debug)]
pub struct SysInfoResp {
    // #[serde(rename = "clientToken")]
    // pub client_token: String,
    pub code: i32,
    pub message: String,
    pub timestamp: u64,
    pub data: SysInfoData,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SysInfoData {
    pub app_name: String,
    pub app_status: String,
    pub app_uptime: String,
    pub app_version: String,
    pub os_name: String,
    pub os_uptime: String,
    pub os_version: String,
    pub network_interfaces: HashMap<String, Vec<NetworkInterface>>,

    // 可选字段
    pub wan: Option<Wan>,
    pub eth0: Option<SimpleNetIf>,
    pub eth1: Option<SimpleNetIf>,
    pub wifi: Option<SimpleNetIf>,
    pub modem0: Option<SimpleNetIf>,
    pub bs_location: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NetworkInterface {
    pub family: String,
    pub internal: bool,
    pub address: String,
    pub netmask: Option<String>,
    pub cidr: Option<String>,
    pub mac: Option<String>,
    pub scopeid: Option<u32>, // IPv6 特有字段
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Wan {
    pub ip: String,
    pub mac: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SimpleNetIf {
    pub ip: Option<String>,
    pub mac: Option<String>,
}

//
pub async fn get_sysinfo(
    client: &AsyncClient,
    pending: PendingResponses,
     sn: &str,
    //   params: SysInfoReq,
) -> Result<SysInfoResp> {
    println!("正在发送系统信息请求至设备 '{}'...", sn);

    let topic_req = format!("$iothub/things/{}/methods/getSysinfo/req", sn);
    let topic_resp = format!("$iothub/things/{}/methods/getSysinfo/resp", sn);
    let token = generate_token();


    let request_payload = json!({
        "clientToken": token,
        "data": {}
    });
//  let token = params.client_token.clone();
//     let request_payload = serde_json::to_string(&params)?;

    // 订阅响应topic
    client.subscribe(&topic_resp, QoS::AtLeastOnce).await?;
    // 发布请求
    client
        .publish(
            &topic_req,
            QoS::AtLeastOnce,
            false,
            request_payload.to_string(),
        )
        .await?;
    println!("📤 已发送系统信息请求到 {}, 等待响应...", topic_req);

    // 创建oneshot通道
  let (tx, rx) = oneshot::channel::<Response>();
{
    let mut pending_guard = pending.lock().unwrap();
    pending_guard.insert(token.clone(), Sender::Oneshot(tx));  // 用 enum
}

    // 等待响应，10秒超时
    match timeout(Duration::from_secs(10), rx).await {
        Ok(Ok(Response::SysInfo(resp))) => {
            info!(
                "✅ 系统信息响应成功: Code={}, Message='{}'",
                resp.code, resp.message
            );
            info!("📋 系统信息: {:#?}", resp.data);
            client.unsubscribe(&topic_resp).await?;
            Ok(resp)
        }
        Ok(Ok(other)) => {
            error!("Received unexpected response type: {:?}", other);
            client.unsubscribe(&topic_resp).await?;
            Err(anyhow!("Unexpected response type for sysinfo"))
        }
        Ok(Err(e)) => {
            error!("Receiver error: {:?}", e);
            client.unsubscribe(&topic_resp).await?;
            Err(anyhow!("系统信息响应接收失败"))
        }
        Err(_) => {
            error!("❌ 等待系统信息响应超时");
            client.unsubscribe(&topic_resp).await?;
            Err(anyhow!("获取系统信息超时"))
        }
    }
}

pub async fn handle_sysinfo(
    Extension(mqtt_client): Extension<Arc<AsyncClient>>,
    Extension(pending_responses): Extension<PendingResponses>,
    Path(sn): Path<String>,
    // Json(req): Json<SysInfoReq>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    match get_sysinfo(&mqtt_client,  pending_responses.clone(),&sn).await {
        Ok(resp) => Ok(Json(serde_json::to_value(resp).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?)),
        Err(e) => {
            error!("获取系统信息失败: {:?}", e);
            Ok(Json(serde_json::json!({
                "code": 500,
                "message": format!("获取设备{}系统信息失败: {}", sn, e),
                "timestamp": std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as i64
            })))
        }
    }
}
