use crate::modules::plc_gateway::api::remote_response::{PendingResponses, Response, Sender};
use crate::modules::plc_gateway::api::remote_util::generate_token;
use anyhow::{Result, anyhow};
use axum::{
    Json,
    extract::{Extension, Path},
    http::StatusCode,
};
use chrono::Utc;
use log::{error, info};
use rumqttc::{AsyncClient, QoS};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;
use tokio::sync::oneshot;
use tokio::time::{Duration, timeout};
// === 结构体定义 ===
// #[derive(Serialize, Deserialize, Debug)]
// pub struct RebootReq {
//     #[serde(rename = "clientToken")]
//     pub client_token: String,
//     pub data: serde_json::Value,
// }

// 重启请求的 JSON 结构体
#[derive(Serialize, Deserialize, Debug)]
pub struct RebootResp {
    // #[serde(rename = "clientToken")]
    // pub client_token: String,
    pub code: i32,
    pub message: String,
    pub timestamp: i64,
}

pub async fn publish_reboot(
    client: &AsyncClient,
    pending: PendingResponses,
    sn: &str,
    // params: RebootReq,
) -> Result<RebootResp> {
    // 新增pending param
    info!("正在发送重启指令至设备 '{}'...", sn);

    let topic_req = format!("$iothub/things/{}/methods/reboot/req", sn);
    let topic_resp = format!("$iothub/things/{}/methods/reboot/resp", sn);
    let token = generate_token();

    let request_payload = json!({
        "clientToken": token,
        "data": {}
    });
    // let token = params.client_token.clone();
    // let request_payload = serde_json::to_string(&params)?;
    // 订阅resp（如果还没）
    client.subscribe(&topic_resp, QoS::AtLeastOnce).await?;

    // 发布req
    client
        .publish(
            &topic_req,
            QoS::AtLeastOnce,
            false,
            request_payload.to_string(),
        )
        .await?;
    info!("📤 已发送重启请求到 {}, 等待响应...", topic_req);

    // 新增：创建oneshot channel
    let (tx, rx) = oneshot::channel::<Response>();
    {
        let mut pending_guard = pending.lock().unwrap();
        pending_guard.insert(token.clone(), Sender::Oneshot(tx)); // 用 enum
    }

    // 等待响应（带15s超时）
    match timeout(Duration::from_secs(15), rx).await {
        Ok(Ok(Response::Reboot(resp))) => {
            info!(
                "✅ 设备响应成功: Code={}, Message='{}'",
                resp.code, resp.message
            );
            client.unsubscribe(&topic_resp).await?;
            Ok(resp)
        }
        Ok(Ok(other)) => {
            error!("Received unexpected response type: {:?}", other);
            client.unsubscribe(&topic_resp).await?;
            Err(anyhow!("Unexpected response type for reboot"))
        }
        Ok(Err(e)) => {
            error!("Receiver error: {:?}", e);
            client.unsubscribe(&topic_resp).await?;
            Err(anyhow!("重启响应接收失败"))
        }
        Err(_) => {
            println!("❌ 等待重启响应超时");
            client.unsubscribe(&topic_resp).await?;
            Err(anyhow!("重启失败或超时"))
        }
    }
}

pub async fn handle_reboot(
    Extension(mqtt_client): Extension<Arc<AsyncClient>>,
    Extension(pending_responses): Extension<PendingResponses>,
    Path(sn): Path<String>,
    // Json(req): Json<RebootReq>,
) -> std::result::Result<Json<serde_json::Value>, StatusCode> {
    match publish_reboot(&mqtt_client, pending_responses.clone(), &sn).await {
        Ok(resp) => Ok(Json(
            serde_json::to_value(resp).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
        )),
        Err(e) => {
            println!("重启失败: {:?}", e);
            Ok(Json(serde_json::json!({
                "code": 500,
                "message": format!("重启设备{}失败: {}", sn, e),
                "timestamp": Utc::now().timestamp_millis() as u64,
            })))
        }
    }
}
