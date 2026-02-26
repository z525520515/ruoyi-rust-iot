use crate::modules::plc_gateway::api::remote_log::GetLogResp;
use crate::modules::plc_gateway::api::remote_ota::OtaResp;
use crate::modules::plc_gateway::api::remote_ota::{OtaTaskQueue, OtaTaskStatus};
use crate::modules::plc_gateway::api::remote_reboot::RebootResp;
use crate::modules::plc_gateway::api::remote_response::{PendingResponses, Response, Sender};
use crate::modules::plc_gateway::api::remote_sysinfo::SysInfoResp;
use anyhow::Result;
use config::Config;
use log::{error, info};
use rumqttc::{AsyncClient, Event, MqttOptions, Packet, Publish};
use sea_orm::{ConnectionTrait, Database, DatabaseConnection, DbErr};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{fs, time::Duration};
use tokio::time::sleep;
// ========== 配置和工具函数 ==========
#[derive(Serialize, Debug, Deserialize)]
struct MqttConfig {
    client_id: String,
    host: String,
    username: String,
    password: String,
}
#[derive(Debug, Deserialize)]
pub struct DbConfig {
    url: String,
}


// Topic 路由映射
const TOPIC_ROUTES: &[(&str, fn(&str) -> bool)] = &[
    ("/reboot/resp", |t| t.contains("/reboot/resp")),
    ("/getLog/resp", |t| t.contains("/getLog/resp")),
    ("/ota/task/result", |t| t.contains("/ota/task/result")),
    ("/getSysinfo/resp", |t| t.contains("/getSysinfo/resp")),
];

// 解析 SN
fn parse_sn_from_topic(topic: &str) -> Option<String> {
    topic
        .strip_prefix("$iothub/things/")
        .and_then(|parts| parts.split_once('/'))
        .map(|(sn, _)| sn.to_string())
}

// ========== 统一的消息解析器（修复：返回 token）==========
async fn parse_response(topic: &str, payload: &str) -> Result<(Response, String, bool), String> {
    let value: Value = serde_json::from_str(payload).map_err(|e| format!("JSON解析失败: {}", e))?;

    // ✅ 修复：正确获取 token
    let token = value
        .get("clientToken")
        .and_then(|v| v.as_str())
        .ok_or("缺少clientToken".to_string())?
        .to_string();

    // 根据 topic 智能解析
    let response = match TOPIC_ROUTES
        .iter()
        .find(|(_, matcher)| matcher(topic))
        .map(|(route, _)| route)
    {
        Some(&"/reboot/resp") => {
            let resp: RebootResp =
                serde_json::from_str(payload).map_err(|e| format!("解析RebootResp失败: {}", e))?;
            Response::Reboot(resp)
        }
        Some(&"/getLog/resp") => {
            let resp: GetLogResp =
                serde_json::from_str(payload).map_err(|e| format!("解析GetLogResp失败: {}", e))?;
            Response::GetLog(resp)
        }
        Some(&"/ota/task/result") => {
            let resp: OtaResp =
                serde_json::from_str(payload).map_err(|e| format!("解析OtaResp失败: {}", e))?;
            Response::Ota(resp)
        }
        Some(&"/getSysinfo/resp") => {
            let resp: SysInfoResp =
                serde_json::from_str(payload).map_err(|e| format!("解析SysInfoResp失败: {}", e))?;
            Response::SysInfo(resp)
        }
        _ => return Err(format!("未知topic: {}", topic)),
    };

    let is_ota = topic.contains("/ota/task/result");
    Ok((response, token, is_ota)) // ✅ 返回 token
}

// ========== 处理 OTA 响应的专用函数 ==========
async fn handle_ota_response(queue: &OtaTaskQueue, topic: &str, ota_resp: &OtaResp) {
    let Some(sn) = parse_sn_from_topic(topic) else {
        error!("无法解析SN: {}", topic);
        return;
    };

    match ota_resp {
        OtaResp::Ongoing {
            task_id, progress, ..
        } => {
            queue
                .update_task_status(&sn, *task_id, OtaTaskStatus::Ongoing(*progress))
                .await;
            info!(
                "✅ OTA进度: sn={}, task_id={}, progress={}%",
                sn, task_id, progress
            );
        }
        OtaResp::Done { task_id, .. } => {
            queue
                .update_task_status(&sn, *task_id, OtaTaskStatus::Done)
                .await;
            let queue_clone = queue.clone();
            let sn_clone = sn.clone();
            let task_id_clone = *task_id;
            tokio::spawn(async move {
                sleep(Duration::from_secs(60)).await;
                queue_clone.remove_task(&sn_clone, task_id_clone).await;
            });
            info!("✅ OTA完成: sn={}, task_id={}", sn, task_id);
        }
        OtaResp::Failed {
            task_id,
            err_code,
            err_msg,
            ..
        } => {
            queue
                .update_task_status(
                    &sn,
                    *task_id,
                    OtaTaskStatus::Failed(*err_code, err_msg.clone()),
                )
                .await;
            let queue_clone = queue.clone();
            let sn_clone = sn.clone();
            let task_id_clone = *task_id;
            tokio::spawn(async move {
                sleep(Duration::from_secs(60)).await;
                queue_clone.remove_task(&sn_clone, task_id_clone).await;
            });
            error!(
                "❌ OTA失败: sn={}, task_id={}, code={}, msg={}",
                sn, task_id, err_code, err_msg
            );
        }
    }
}

// ========== 发送响应的统一函数 ==========
fn send_response(pending: &PendingResponses, token: String, response: Response) {
    let sender = pending.lock().unwrap().remove(&token);

    match sender {
        Some(Sender::Oneshot(tx)) => {
            if tx.send(response).is_err() {
                error!("Oneshot发送失败: {}", token);
            } else {
                info!("✅ Oneshot响应发送成功: {}", token);
            }
        }
        Some(Sender::Mpsc(tx)) => {
            // MPSC 发送需异步，但当前函数非 async，故移出到外部异步任务中处理
            // ✅ Arc 允许多线程共享，无需克隆
            let token = token.clone(); // 克隆一份给 spawn
            tokio::spawn(async move {
                if tx.send(response).await.is_err() {
                    error!("MPSC发送失败: {}", token);
                } else {
                    info!("✅ MPSC响应发送成功: {}", token);
                }
            });
        }
        None => info!("⏳ 无等待者: {}", token),
    }
}

// ========== 修复后的消息处理函数 ==========
async fn handle_mqtt_message(queue: OtaTaskQueue, pending: PendingResponses, publish: Publish) {
    let topic = publish.topic.clone();
    let payload_bytes = publish.payload.to_vec();
    // println!("payload_bytes: {:?}", payload_bytes);
    info!(
        "MQTT收到: topic={}, payload={}",
        topic,
        String::from_utf8_lossy(&payload_bytes)
    );

    // 只处理方法响应
    if !TOPIC_ROUTES.iter().any(|(_, matcher)| matcher(&topic)) {
        return;
    }

    // ✅ 修复：正确的 UTF8 + 解析逻辑
    let payload_str = match String::from_utf8(payload_bytes) {
        Ok(payload) => payload,
        Err(e) => {
            error!("非UTF8 payload: {:?}", e);
            return;
        }
    };

    // ✅ 修复：正确的 Result 处理
    let (response, token, is_ota) = match parse_response(&topic, &payload_str).await {
        Ok(result) => result,
        Err(err) => {
            error!("消息解析失败: {}", err); // ✅ err 来自这里
            return;
        }
    };

    // 处理 OTA 状态更新
    if is_ota {
        if let Response::Ota(ref ota_resp) = response {
            handle_ota_response(&queue, &topic, ota_resp).await;
        }
    }

    // ✅ 修复：使用正确的 token
    send_response(&pending, token, response);
}

// ========== 优化的初始化函数 ==========
pub async fn init_services(
    pending_responses: PendingResponses,
    task_queue: OtaTaskQueue,
) -> Result<(DatabaseConnection, AsyncClient)> {
    // 1. 加载配置
    let conf = Config::builder()
        .add_source(config::File::with_name(".config/config"))
        .build()?;
    let mqtt_conf: MqttConfig = conf.get("mqtt")?;
    let db_conf: DbConfig = conf.get("database")?;
    let db = Database::connect(db_conf.url).await?;
    info!("✅ 数据库连接成功");
    // 2. 初始化 MQTT
    let mut mqttoptions = MqttOptions::new(&mqtt_conf.client_id, &mqtt_conf.host, 1883);
    mqttoptions.set_keep_alive(Duration::from_secs(30));
    if !mqtt_conf.username.is_empty() {
        mqttoptions.set_credentials(&mqtt_conf.username, &mqtt_conf.password);
    }
    let (mqtt_client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
    info!("MQTT客户端初始化完成: {}", mqtt_conf.client_id);

    // 3. 只克隆一次
    let pending_clone = pending_responses.clone();
    let queue_clone = task_queue.clone();

    // 4. 启动事件循环
    tokio::spawn(async move {
        loop {
            match eventloop.poll().await {
                Ok(Event::Incoming(Packet::Publish(publish))) => {
                    // println!("publish: {:?}", publish.payload);
                    handle_mqtt_message(queue_clone.clone(), pending_clone.clone(), publish).await;
                }
                Ok(_event) => {
                    // info!("MQTT事件: {:?}", event);
                }
                Err(e) => {
                    error!("MQTT错误: {:?}, 5秒后重连", e);
                    sleep(Duration::from_secs(5)).await;
                }
            }
        }
    });

    Ok((db, mqtt_client))
}

// ========== 数据库工具 ==========
pub async fn load_sql(db: &DatabaseConnection, path: &str) -> Result<(), DbErr> {
    let sql = fs::read_to_string(path).unwrap();
    db.execute_unprepared(&sql).await?;
    info!("✅ 执行SQL: {}", path);
    Ok(())
}

pub async fn table_exists(db: &DatabaseConnection, table_name: &str) -> Result<bool, DbErr> {
    let sql = format!(
        "SELECT EXISTS (SELECT 1 FROM information_schema.tables WHERE table_name = '{}')",
        table_name
    );
    let result = db
        .query_one(sea_orm::Statement::from_string(
            db.get_database_backend(),
            sql,
        ))
        .await?;

    Ok(result
        .map(|row| row.try_get("", "exists").unwrap_or(false))
        .unwrap_or(false))
}
