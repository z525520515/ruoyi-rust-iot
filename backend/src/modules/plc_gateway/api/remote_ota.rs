use crate::modules::plc_gateway::api::remote_response::{PendingResponses, Response, Sender};
use crate::modules::plc_gateway::api::remote_util::{calculate_md5_from_url, generate_token, get_remote_file_size};
use anyhow::{Result, anyhow};
use axum::{
    Json,
    extract::{Extension},
    http::StatusCode,
};
use chrono::Utc;
use log::{error, info};
use rumqttc::{AsyncClient, QoS};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::sync::{Arc};
use tokio::sync::RwLock;
use tokio::sync::mpsc;
use tokio::task;
use tokio::time::Duration;
use tokio::time::{sleep};

// OTA任务状态
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum OtaTaskStatus {
    Ongoing(u8), // progress 0-100
    Done,
    Failed(u8, String), // err_code, err_msg
}

// OTA任务信息
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OtaTaskInfo {
    pub sn: String,
    pub task_id: u8,
    pub client_token: String,
    pub status: OtaTaskStatus,
    pub timestamp: u64, // ms
    pub file_url: Option<String>,
    pub md5: Option<String>,
    pub size: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserOtaReq {
    pub sn: String,
    pub task_id: u8,
    #[serde(default)]
    pub file_url: Option<String>, // None表示查询进度
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OtaProgressResp {
    pub client_token: String,
    pub task_id: u8,
    pub status: String, // "ongoing", "done", "failed"
    pub timestamp: u64,
    pub progress: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub err_code: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub err_msg: Option<String>,
}

impl From<&OtaTaskInfo> for OtaProgressResp {
    fn from(task: &OtaTaskInfo) -> Self {
        let (status, progress, err_code, err_msg) = match &task.status {
            OtaTaskStatus::Ongoing(p) => ("ongoing".to_string(), Some(*p), None, None),
            OtaTaskStatus::Done => ("done".to_string(), Some(100), None, None),
            OtaTaskStatus::Failed(code, msg) => {
                ("failed".to_string(), None, Some(*code), Some(msg.clone()))
            }
        };

        Self {
            client_token: task.client_token.clone(),
            task_id: task.task_id,
            status,
            timestamp: task.timestamp,
            progress: progress.unwrap_or(0),
            err_code,
            err_msg,
        }
    }
}

// 任务队列管理器
#[derive(Clone)]
pub struct OtaTaskQueue {
    tasks: Arc<RwLock<HashMap<(String, u8), OtaTaskInfo>>>, // key: (sn, task_id)
}

impl OtaTaskQueue {
    pub fn new() -> Self {
        Self {
            tasks: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    // 添加或更新任务
    pub async fn upsert_task(&self, task: OtaTaskInfo) -> bool {
        let key = (task.sn.clone(), task.task_id);
        let mut tasks = self.tasks.write().await;
        let is_new = !tasks.contains_key(&key);
        tasks.insert(key, task);
        is_new
    }

    // 获取任务状态
    pub async fn get_task(&self, sn: &str, task_id: u8) -> Option<OtaTaskInfo> {
        let tasks = self.tasks.read().await;
        tasks.get(&(sn.to_string(), task_id)).cloned()
    }

    // 更新任务状态
    pub async fn update_task_status(
        &self,
        sn: &str,
        task_id: u8,
        status: OtaTaskStatus,
    ) -> Option<OtaTaskInfo> {
        // 返回拥有数据
        let key = (sn.to_string(), task_id);
        let mut tasks = self.tasks.write().await;
        if let Some(task) = tasks.get_mut(&key) {
            task.status = status.clone();
            task.timestamp = Utc::now().timestamp_millis() as u64;
            Some(task.clone()) // 克隆返回拥有数据
        } else {
            None
        }
    }

    // 移除任务（完成或失败后）
    // pub async fn remove_task(&self, sn: &str, task_id: u8) {
    //     let key = (sn.to_string(), task_id);
    //     let mut tasks = self.tasks.write().await;
    //     tasks.remove(&key);
    // }
    pub async fn remove_task(&self, sn: &str, task_id: u8) {
        // 立即标记为"已完成/失败"状态（但不删除）
        // 注意：这里已经在 update_task_status 中设置了最终状态

        // 启动后台任务，60秒后删除
        let queue_clone = self.clone();
        let sn_clone = sn.to_string();
        let task_id_clone = task_id;
        let miao = 120;
        task::spawn(async move {
            sleep(Duration::from_secs(miao)).await;
            queue_clone._do_remove_task(&sn_clone, task_id_clone).await;
            info!(
                "🗑️ 延迟移除OTA任务: sn={}, task_id={}",
                sn_clone, task_id_clone
            );
        });

        info!(
            "⏳ OTA任务标记为完成/失败，将在{}秒后清理: sn={}, task_id={}",
            miao, sn, task_id
        );
    }
    pub async fn remove_task_immediately(&self, sn: &str, task_id: u8) {
        self._do_remove_task(sn, task_id).await;
    }
    // 内部方法：真正执行删除
    async fn _do_remove_task(&self, sn: &str, task_id: u8) {
        let key = (sn.to_string(), task_id);
        let mut tasks = self.tasks.write().await;
        if tasks.remove(&key).is_some() {
            info!("🗑️ OTA任务已从队列删除: sn={}, task_id={}", sn, task_id);
        }
    }

    // 检查任务是否存在
    pub async fn has_task(&self, sn: &str, task_id: u8) -> bool {
        let tasks = self.tasks.read().await;
        tasks.contains_key(&(sn.to_string(), task_id))
    }
}

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

pub async fn send_ota_command(
    client: &AsyncClient,
    pending: PendingResponses,
    queue: OtaTaskQueue,
    sn: &str,
    task_id: u8,
    file_url: &str,
) -> Result<String, anyhow::Error> {
    let req_topic = format!("$iothub/things/{}/ota/task", sn);
    let resp_topic = format!("$iothub/things/{}/ota/task/result", sn);
    let client_token = generate_token();

    // 计算文件信息
    let md5 = calculate_md5_from_url(file_url)
        .await
        .map_err(|e| anyhow!("获取文件MD5失败: {:?}", e))?;
    let size = get_remote_file_size(file_url)
        .await
        .map_err(|e| anyhow!("获取文件大小失败: {:?}", e))?;

    // 构建任务信息并加入队列
    let task_info = OtaTaskInfo {
        sn: sn.to_string(),
        task_id,
        client_token: client_token.clone(),
        status: OtaTaskStatus::Ongoing(0),
        timestamp: Utc::now().timestamp_millis() as u64,
        file_url: Some(file_url.to_string()),
        md5: Some(md5.clone()),
        size: Some(size),
    };

    queue.upsert_task(task_info).await;

    let payload = json!({
        "clientToken": client_token,
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

    // 订阅响应主题
    client.subscribe(&resp_topic, QoS::AtLeastOnce).await?;

    // 发送消息
    client
        .publish(&req_topic, QoS::AtLeastOnce, false, payload.to_string())
        .await?;

    info!("📤 OTA指令已发送至设备: {}", sn);

    // 注册pending响应（用于后续状态更新）
    let (tx, _rx) = mpsc::channel::<Response>(10);
    {
        let mut pending_guard = pending.lock().unwrap();
        pending_guard.insert(client_token.clone(), Sender::Mpsc(tx));
    }

    // **立即返回成功**，不再等待完整流程
    info!("✅ OTA指令发送成功: sn={}, task_id={}", sn, task_id);
    Ok(client_token)
}
// pub async fn wait_ota_completion(
//     client: &AsyncClient,
//     pending: PendingResponses,
//     queue: OtaTaskQueue,
//     sn: &str,
//     task_id: u8,
//     client_token: &str,
// ) -> Result<OtaTaskStatus, anyhow::Error> {
//     let resp_topic = format!("$iothub/things/{}/ota/task/result", sn);

// }
pub async fn query_ota_progress(
    queue: &OtaTaskQueue,
    sn: &str,
    task_id: u8,
) -> Result<OtaProgressResp, anyhow::Error> {
    let task = queue
        .get_task(sn, task_id)
        .await
        .ok_or_else(|| anyhow!("未找到OTA任务: sn={}, task_id={}", sn, task_id))?;

    let resp: OtaProgressResp = (&task).into();
    println!(
        "📊 实时OTA进度: sn={}, task_id={}, status={}, progress={:?}",
        sn, task_id, resp.status, resp.progress
    );

    Ok(resp)
}
pub async fn handle_ota(
    Extension(mqtt_client): Extension<Arc<AsyncClient>>,
    Extension(pending_responses): Extension<PendingResponses>,
    Extension(task_queue): Extension<OtaTaskQueue>,
    Json(req): Json<UserOtaReq>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let sn = req.sn.clone();
    let task_id = req.task_id;
    let timestamp = Utc::now().timestamp_millis() as u64;

    // 检查任务是否已存在（仅用于下发）
    if req.file_url.is_some() && task_queue.has_task(&sn, task_id).await {
        info!("⚠️ 检测到重复OTA任务: sn={}, task_id={}", sn, task_id);
        return Ok(Json(serde_json::json!({
            "code": 201,
            "message": format!("OTA任务正在进行中，请勿重新发送: sn={}, task_id={}", sn, task_id),
            "data": {  },
            "timestamp": timestamp
        })));
    }

    match req.file_url {
        // 要求1: 下发OTA任务
        Some(file_url) => {
            match send_ota_command(
                &mqtt_client,
                pending_responses.clone(),
                task_queue.clone(),
                &sn,
                task_id,
                &file_url,
            )
            .await
            {
                Ok(client_token) => {
                    info!(
                        "✅ OTA任务下发成功: sn={}, task_id={}, token={}",
                        sn, task_id, client_token
                    );
                    Ok(Json(serde_json::json!({
                        "code": 200,
                        "message": format!("OTA任务下发成功: sn={}, task_id={}, token={}", sn, task_id, client_token),
                        "data": {
                        },
                        "timestamp": timestamp
                    })))
                }
                Err(e) => {
                    error!(
                        "❌ OTA发送失败: sn={}, task_id={}, error={:?}",
                        sn, task_id, e
                    );
                    task_queue.remove_task_immediately(&sn, task_id).await;
                    Ok(Json(serde_json::json!({
                        "code": 500,
                        "message": format!("OTA发送失败: sn={}, task_id={}, error={:?}", sn, task_id, e),
                        "data": { },
                        "timestamp": timestamp
                    })))
                }
            }
        }
        // 要求2: 查询OTA进度 - 使用优化版本
        None => match query_ota_progress(&task_queue, &sn, task_id).await {
            Ok(progress_resp) => Ok(Json(serde_json::json!({
                "code": 200,
                "message": "查询成功",
                "data": progress_resp,
                "timestamp": timestamp
            }))),
            Err(e) => {
                error!(
                    "❌ OTA进度查询失败: sn={}, task_id={}, error={:?}",
                    sn, task_id, e
                );
                Ok(Json(serde_json::json!({
                    "code": 404,
                    "message": format!("查询OTA进度, 未找到OTA任务: sn={}, task_id={}", sn, task_id),
                    "data": {},
                    "timestamp": timestamp
                })))
            }
        },
    }
}
