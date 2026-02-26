use crate::modules::plc_gateway::api::remote_response::{PendingResponses, Response, Sender};
use crate::modules::plc_gateway::api::remote_util::generate_token;
use anyhow::{Result, anyhow};
use axum::{
    Json,
    extract::{Extension, Path},
    http::StatusCode,
};
use log::error;
use rumqttc::{AsyncClient, QoS};
use serde::{Deserialize, Serialize};
// use serde_json::json;
use tokio::sync::oneshot; // 新增
use tokio::time::Duration;
use tokio::time::timeout;
use chrono::Utc;
use std::sync::Arc;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetLogData {
    context: LogContext,
    #[serde(skip_serializing_if = "Option::is_none")]
    up_size: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    down_size: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetLogReq {
    #[serde(rename = "clientToken")]
    client_token: String,
    data: GetLogData,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LogContext {
    pub log_size: Option<u64>,
    pub log_start_addr: Option<u64>,
    pub log_end_addr: Option<u64>,
    pub position: Option<u64>,
    pub direction: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RespLogContext {
    pub pos: i64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetLogRespData {
    pub content: String,
    pub context: RespLogContext,
    pub size: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetLogResp {
    // #[serde(rename = "clientToken")]
    // pub client_token: String,
    pub code: i32,
    pub message: String,
    pub data: GetLogRespData,
    pub timestamp: u64,
}


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserGetLogReq {
   pub direction: String,
}

pub async fn get_device_log(
    client: &AsyncClient,
    pending: PendingResponses,
    sn: &str,
    params: GetLogReq,
) -> Result<GetLogResp> {
    println!("正在发送获取日志指令至设备 '{}'...", sn);

    let topic_req = format!("$iothub/things/{}/methods/getLog/req", sn);
    let topic_resp = format!("$iothub/things/{}/methods/getLog/resp", sn);
    // let token = generate_token();

    // let context = json!({
    //     "logSize": 3133192,
    //     "logStartAddr": 1212006,
    //     "logEndAddr": 12116744,
    //     "position": 8192,
    //     "direction": "up"
    // });

    // let request_payload = json!({
    //     "clientToken": token,
    //     "data": {
    //         "context": context,
    //         "upSize": 8192
    //     }
    // });
    let token = params.client_token.clone();
    let request_payload = serde_json::to_string(&params)?;

    client.subscribe(&topic_resp, QoS::AtLeastOnce).await?;
    client
        .publish(
            &topic_req,
            QoS::AtLeastOnce,
            false,
            request_payload.to_string(),
        )
        .await?;
    println!("📤 已发送日志请求到 {}, 等待响应...", topic_req);

    let (tx, rx) = oneshot::channel::<Response>();
    {
        let mut pending_guard = pending.lock().unwrap();
        pending_guard.insert(token.clone(), Sender::Oneshot(tx)); // 用 enum
    }

    match timeout(Duration::from_secs(15), rx).await {
        Ok(Ok(Response::GetLog(resp))) => {
            println!(
                "✅ 设备响应成功: Code={}, Message='{}'",
                resp.code, resp.message
            );
            println!("📄 日志大小: {} 字节", resp.data.size);
            println!(
                "📜 内容预览:\n{}",
                &resp.data.content[..200.min(resp.data.content.len())]
            );
            client.unsubscribe(&topic_resp).await?;
            Ok(resp)
        }
        Ok(Ok(other)) => {
            error!("Received unexpected response type: {:?}", other);
            client.unsubscribe(&topic_resp).await?;
            Err(anyhow!("Unexpected response type for get_log"))
        }
        Ok(Err(e)) => {
            error!("Receiver error: {:?}", e);
            client.unsubscribe(&topic_resp).await?;
            Err(anyhow!(format!("获取设备日志{}响应接收失败: {:?}", sn, e)))
        }
        Err(_) => {
            println!("❌ 等待获取日志响应超时");
            client.unsubscribe(&topic_resp).await?;
            Err(anyhow!(format!("获取设备日志{}超时", sn)))
        }
    }
}

pub async fn handle_get_log(
    Extension(mqtt_client): Extension<Arc<AsyncClient>>,
    Extension(pending_responses): Extension<PendingResponses>,
    Path(sn): Path<String>,
    Json(req): Json<UserGetLogReq>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let token = generate_token();
    let direction = req.direction.clone();
    
    // 验证direction参数只能是"up"或"down"
    let (up_size, down_size) = match direction.as_str() {
        "up" => (Some(8192), None), // 默认8192字节
        "down" => (None, Some(8192)), // 默认8192字节
        _ => {
            // 参数错误，直接返回
            return Ok(Json(serde_json::json!({
                "code": 400,
                "message": "参数错误：direction只能是'up'或'down'",
                "timestamp": Utc::now().timestamp_millis() as u64,
            })));
        }
    };
    
    let params = GetLogReq {
        client_token: token.clone(),
        data: GetLogData {
            context: LogContext {
                log_size: Some(3133192),
                log_start_addr: Some(1212006),
                log_end_addr: Some(12116744),
                position: Some(8192),
                direction: Some(direction),
            },
            up_size,
            down_size,
        },
    };
    match get_device_log(&mqtt_client, pending_responses.clone(), &sn, params).await {
        Ok(resp) => Ok(Json(
            serde_json::to_value(resp).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
        )),
        Err(e) => {
            println!("获取日志失败: {:?}", e);
            Ok(Json(serde_json::json!({
                "code": 500,
                "message": format!("获取设备{}日志失败: {}", sn, e),
                "timestamp": Utc::now().timestamp_millis() as u64,
            })))
        }
    }
}
