use crate::modules::plc_gateway::api::remote_log::GetLogResp;
use crate::modules::plc_gateway::api::remote_ota::OtaResp;
use crate::modules::plc_gateway::api::remote_reboot::RebootResp;
use crate::modules::plc_gateway::api::remote_response::{PendingResponses, Response, Sender};
use crate::modules::plc_gateway::api::remote_sysinfo::SysInfoResp;
use config::Config;
use log::{error, info};
use rumqttc::{AsyncClient, Event, MqttOptions, Packet};
use serde_json::Value;
use std::time::Duration;
use tokio::time::sleep;

use serde::{Deserialize, Serialize};
#[derive(Serialize, Debug, Deserialize)]
pub struct MqttConfig {
    host: String,
    client_id: String,
    username: String,
    password: String,
}

pub async fn init_services(pending_responses: PendingResponses) -> anyhow::Result<AsyncClient> {
    // 1. 加载配置文件
    let conf = Config::builder()
        .add_source(config::File::with_name(".config/config")) // 从.config/conf.toml加载配置
        .build()?;

    // 2. 解析配置信息
    let mqtt_conf: MqttConfig = conf.get::<MqttConfig>("mqtt")?; // MQTT配置

    // 4. 初始化MQTT客户端
    // 创建MQTT选项，设置客户端ID、主机地址和端口(1883)
    let mut mqttoptions = MqttOptions::new(&mqtt_conf.client_id, &mqtt_conf.host, 1883);
    // 设置保活间隔为30秒
    mqttoptions.set_keep_alive(Duration::from_secs(30));
    // 如果配置了用户名和密码，则设置MQTT认证信息
    if !mqtt_conf.username.is_empty() && !mqtt_conf.password.is_empty() {
        mqttoptions.set_credentials(&mqtt_conf.username, &mqtt_conf.password);
    }
    // 创建异步MQTT客户端，缓冲区大小为10
    let (mqtt_client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
    info!("MQTT client initialized with id {}", &mqtt_conf.client_id);

    // 5. 准备异步任务所需的克隆变量
    let pending_clone = pending_responses.clone(); // 新增：克隆共享状态
    // 6. 启动MQTT消息处理的异步任务
    tokio::spawn(async move {
        // 进入事件循环，持续接收和处理MQTT消息
        loop {
            match eventloop.poll().await {
                // 处理接收到的发布消息
                Ok(Event::Incoming(Packet::Publish(publish))) => {
                    info!(
                        "Received publish on topic: {}, payload: {:?}",
                        publish.topic,
                        String::from_utf8_lossy(&publish.payload)
                    );
                    let payload_bytes = publish.payload.to_vec(); // 获取消息载荷
                    let topic = publish.topic.clone(); // 获取消息主题

                    if topic.contains("/methods/") || topic.contains("/ota/task/result") {
                        match String::from_utf8(payload_bytes) {
                            Ok(payload_str) => {
                                match serde_json::from_str::<Value>(&payload_str) {
                                    Ok(value) => {
                                        let token = value
                                            .get("clientToken")
                                            .and_then(|v| v.as_str())
                                            .map(String::from)
                                            .unwrap_or_default();
                                        if token.is_empty() {
                                            error!("No clientToken in response on topic {}", topic);
                                            return;
                                        }

                                        let response = if topic.contains("/reboot/resp") {
                                            match serde_json::from_str::<RebootResp>(&payload_str) {
                                                Ok(resp) => Response::Reboot(resp),
                                                Err(e) => {
                                                    error!("Failed to parse RebootResp: {}", e);
                                                    return;
                                                }
                                            }
                                        } else if topic.contains("/getLog/resp") {
                                            match serde_json::from_str::<GetLogResp>(&payload_str) {
                                                Ok(resp) => Response::GetLog(resp),
                                                Err(e) => {
                                                    error!("Failed to parse GetLogResp: {}", e);
                                                    return;
                                                }
                                            }
                                        } else if topic.contains("/ota/task/result") {
                                            match serde_json::from_str::<OtaResp>(&payload_str) {
                                                Ok(resp) => Response::Ota(resp),
                                                Err(e) => {
                                                    error!("Failed to parse OtaResp: {}", e);
                                                    return;
                                                }
                                            }
                                        } else if topic.contains("/getSysinfo/resp") {
                                            match serde_json::from_str::<SysInfoResp>(&payload_str)
                                            {
                                                Ok(resp) => Response::SysInfo(resp),
                                                Err(e) => {
                                                    error!("Failed to parse SysInfoResp: {}", e);
                                                    return;
                                                }
                                            }
                                        } else {
                                            error!("Unknown method response topic: {}", topic);
                                            return;
                                        };

                                        // Scope the lock to ensure MutexGuard is dropped before await
                                        let sender = {
                                            let mut pending = pending_clone.lock().unwrap();
                                            pending.remove(&token)
                                        };

                                        if let Some(sender) = sender {
                                            match sender {
                                                Sender::Oneshot(tx) => {
                                                    if tx.send(response).is_err() {
                                                        error!(
                                                            "Failed to send oneshot response for token: {}",
                                                            token
                                                        );
                                                    } else {
                                                        info!(
                                                            "Sent oneshot response to waiter for token: {}",
                                                            token
                                                        );
                                                    }
                                                }
                                                Sender::Mpsc(tx) => {
                                                    if tx.send(response).await.is_err() {
                                                        // Await is now safe
                                                        error!(
                                                            "Failed to send mpsc response for token: {}",
                                                            token
                                                        );
                                                    } else {
                                                        info!(
                                                            "Sent mpsc response to waiter for token: {}",
                                                            token
                                                        );
                                                    }
                                                }
                                            }
                                        } else {
                                            info!("No pending waiter for token: {}", token);
                                        }
                                    }
                                    Err(e) => error!("Failed to parse response JSON: {}", e),
                                }
                            }
                            Err(e) => error!("Non-UTF8 resp payload on topic {}: {:?}", topic, e),
                        }
                    }
                }
                // 处理其他MQTT事件
                Ok(other) => info!("MQTT Event: {:?}", other),

                // 处理MQTT错误，实现重连机制
                Err(e) => {
                    error!("MQTT eventloop error: {:?}, reconnecting in 5s...", e);
                    sleep(Duration::from_secs(5)).await;
                    break; // 跳出内部循环，尝试重新订阅
                }
            }
        }
    });

    // 7. 返回初始化成功的数据库连接和MQTT客户端
    Ok(mqtt_client)
}
