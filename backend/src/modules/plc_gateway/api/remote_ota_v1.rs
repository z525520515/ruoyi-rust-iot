use crate::modules::plc_gateway::api::remote_response::{PendingResponses, Response, Sender};
use crate::modules::plc_gateway::api::remote_util::{calculate_md5_from_url, generate_token, get_remote_file_size};
use anyhow::{Result, anyhow};
use axum::{
    Json,
    extract::{Extension, Path},
    http::StatusCode,
};
use rumqttc::{AsyncClient, QoS};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::time::Duration;
use tokio::time::timeout;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "status", rename_all = "camelCase")]
pub enum OtaResp {
    #[serde(rename_all = "camelCase")]
    Ongoing {
        client_token: String,
        task_id: u8,
        #[serde(rename = "progress")]
        progress: u8,
        #[serde(rename = "timestamp")]
        timestamp: u64,
    },
    #[serde(rename_all = "camelCase")]
    Failed {
        client_token: String,
        task_id: u8,
        err_code: u8,
        err_msg: String,
        #[serde(rename = "timestamp")]
        timestamp: u64,
    },
    #[serde(rename_all = "camelCase")]
    Done {
        client_token: String,
        task_id: u8,
        #[serde(rename = "timestamp")]
        timestamp: u64,
    },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserOtaReq {
    pub sn: String,
    pub task_id: u8,
    pub file_url: String,
}

pub async fn publish_ota(
    client: &AsyncClient,
    pending: PendingResponses,
    sn: &str,
    task_id: u8,
    file_url: &str,
) -> Result<()> {
    println!("正在发送OTA指令至设备 '{}'...", sn);

    let req_topic = format!("$iothub/things/{}/ota/task", sn);
    let resp_topic = format!("$iothub/things/{}/ota/task/result", sn);
    let token = generate_token();

    let md5 = calculate_md5_from_url(file_url)
        .await
        .map_err(|e| anyhow!("获取文件MD5失败: {:?}", e))?;
    let size = get_remote_file_size(file_url)
        .await
        .map_err(|e| anyhow!("获取文件大小失败: {:?}", e))?;

    let payload = json!({
        "clientToken": token,
        "taskId": task_id,
        "type": "file",
        "meta": {
            "version": "1.0.0",
            "md5": md5,
            "content-type": "application/octet-stream",
            "content-length": size
        },
        "fileUrl": file_url
    });

    client.subscribe(&resp_topic, QoS::AtLeastOnce).await?;
    client
        .publish(&req_topic, QoS::AtLeastOnce, false, payload.to_string())
        .await?;
    println!("📤 已发送OTA请求到 {}, 等待响应...", req_topic);

    let (tx, mut rx) = mpsc::channel::<Response>(10);
    {
        let mut pending_guard = pending.lock().unwrap();
        pending_guard.insert(token.clone(), Sender::Mpsc(tx));
    }

    let timeout_duration = Duration::from_secs(30);
    let mut success = false;

    match timeout(timeout_duration, async {
        while let Some(response) = rx.recv().await {
            match response {
                Response::Ota(OtaResp::Ongoing { progress, .. }) => {
                    println!("✅ 设备OTA进度: {}%", progress);
                }
                Response::Ota(OtaResp::Done { .. }) => {
                    println!("✅ 设备已确认OTA接收成功！");
                    success = true;
                    break;
                }
                Response::Ota(OtaResp::Failed {
                    err_code, err_msg, ..
                }) => {
                    println!("❌ 设备OTA失败，错误码: {}, 原因: {}", err_code, err_msg);
                    return Err(anyhow!("OTA失败: 错误码 {}, 原因: {}", err_code, err_msg));
                }
                other => {
                    println!("⚠️ 收到意外响应类型: {:?}", other);
                }
            }
        }
        Ok(())
    })
    .await
    {
        Ok(Ok(())) => {
            if success {
                client.unsubscribe(&resp_topic).await?;
                Ok(())
            } else {
                client.unsubscribe(&resp_topic).await?;
                Err(anyhow!("OTA未确认成功，请检查设备是否在线"))
            }
        }
        Ok(Err(e)) => {
            client.unsubscribe(&resp_topic).await?;
            Err(e)
        }
        Err(_) => {
            println!("❌ 等待OTA响应超时");
            client.unsubscribe(&resp_topic).await?;
            Err(anyhow!("OTA响应超时"))
        }
    }
}

pub async fn handle_ota(
    Extension(mqtt_client): Extension<Arc<AsyncClient>>,
    Extension(pending_responses): Extension<PendingResponses>,
    Json(req): Json<UserOtaReq>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let sn = req.sn.clone();
    let file_url = req.file_url.clone();
    let task_id = req.task_id;
    
    match publish_ota(&mqtt_client, pending_responses.clone(), &sn, task_id, &file_url).await {
        Ok(_) => Ok(Json(serde_json::json!({
            "code": 200,
            "message": "OTA指令发送成功",
            "data": { "sn": sn }
        }))),
        Err(e) => {
            println!("OTA失败: {:?}", e);
            Ok(Json(serde_json::json!({
                "code": 500,
                "message": format!("OTA失败: {}", e),
                "data": { "sn": sn }
            })))
        }
    }
}


// pub async fn handle_ota(
//     Extension(mqtt_client): Extension<Arc<AsyncClient>>,
//     Extension(pending_responses): Extension<PendingResponses>,
//     Path(sn): Path<String>,
//     Path(file_url): Path<String>,
// ) -> Result<Json<serde_json::Value>, StatusCode> {
//     match publish_ota(&mqtt_client, &sn, &file_url, pending_responses.clone()).await {
//         Ok(_) => Ok(Json(serde_json::json!({
//             "code": 200,
//             "message": "OTA指令发送成功",
//             "data": { "sn": sn }
//         }))),
//         Err(e) => {
//             println!("OTA失败: {:?}", e);
//             Ok(Json(serde_json::json!({
//                 "code": 500,
//                 "message": format!("OTA失败: {}", e),
//                 "data": { "sn": sn }
//             })))
//         }
//     }
// }
