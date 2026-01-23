use anyhow::Result;
use log::{error, info};
use rumqttc::{AsyncClient, Event, MqttOptions, Packet, Publish, QoS};
use std::time::Duration;
use tokio::time::{sleep, timeout};
use std::sync::Arc;
use dashmap::DashMap;
use tokio::sync::oneshot;
use serde_json::{json, Value};
use once_cell::sync::Lazy;

use crate::config::config::MqttConfig;

pub static PENDING_REQUESTS: Lazy<DashMap<i64, oneshot::Sender<Value>>> = Lazy::new(DashMap::new);

// ========== MQTT 公共管理类 ==========
pub struct MqttManager {
    pub client: AsyncClient,
}

impl MqttManager {
    pub async fn init(config: MqttConfig) -> Self {
        let mut mqtt_options = MqttOptions::new(&config.client_id, &config.host, config.port);
        mqtt_options.set_keep_alive(Duration::from_secs(config.keep_alive));
        mqtt_options.set_clean_session(config.clean_session);

        if !config.username.is_empty() {
            mqtt_options.set_credentials(&config.username, &config.password);
        }

        let (client, mut eventloop) = AsyncClient::new(mqtt_options, 10);
        let manager = Self { client: client.clone() };

        // 启动后台事件监听线程
        tokio::spawn(async move {
            loop {
                match eventloop.poll().await {
                    Ok(event) => {
                        match event {
                            Event::Incoming(Packet::Publish(publish)) => {
                                // 这里调用的 handle_raw_message 内部会去检查全局变量
                                Self::handle_raw_message(publish);
                            }
                            Event::Incoming(Packet::ConnAck(_)) => {
                                info!("✅ MQTT 连接成功 (ConnAck)");
                            }
                            _ => {}
                        }
                    }
                    Err(e) => {
                        error!("❌ MQTT 异常: {:?}, 5秒后重连", e);
                        sleep(Duration::from_secs(5)).await;
                    }
                }
            }
        });

        info!("🚀 MQTT 客户端初始化完成: {}", config.client_id);
        manager
    }

    pub async fn subscribe(&self, topic: &str, qos: QoS) -> Result<()> {
        self.client.subscribe(topic, qos).await?;
        info!("📢 订阅成功: {}", topic);
        Ok(())
    }

    pub async fn publish(&self, topic: &str, payload: Vec<u8>, qos: QoS) -> Result<()> {
        self.client.publish(topic, qos, false, payload).await?;
        Ok(())
    }

    // ================== 新增：网关协议封装方法 ==================

    pub async fn gateway_invoke(
        &self,
        device_hash: &str,
        protocol_type: &str,
        stack_name: &str,
        action: &str,
        method: &str,
        params: Value,
    ) -> Result<Value> {
        let tid = (chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0) / 1000) as i64;

        let header = json!({
            "action": action,
            "message_type": "request",
            "method": method,
            "source": "HTTP_Gateway",
            "target": device_hash,
            "timestamp": chrono::Utc::now().timestamp_millis(),
            "transaction_id": tid,
            "version": "1.0.0"
        });
        let full_payload = json!([header, { "params": params }]);
        let payload_bytes = serde_json::to_vec(&full_payload)?;

        let encoded_hash = urlencoding::encode(device_hash);
        let topic = format!(
            "{}/ReservedClient/protocol_stack/{}/{}/{}/request",
            encoded_hash, protocol_type, stack_name, action
        );

        let (tx, rx) = oneshot::channel();
        PENDING_REQUESTS.insert(tid, tx);

        self.publish(&topic, payload_bytes, QoS::AtLeastOnce).await?;

        match timeout(Duration::from_secs(10), rx).await {
            Ok(Ok(response_body)) => Ok(response_body),
            Ok(Err(_)) => Err(anyhow::anyhow!("Internal channel closed")),
            Err(_) => {
                PENDING_REQUESTS.remove(&tid);
                Err(anyhow::anyhow!("Gateway response timeout"))
            }
        }
    }

    fn handle_raw_message(publish: Publish) {
        let payload_bytes = &publish.payload;

        // 尝试解析 JSON 数组格式 [Header, Result]
        if let Ok(Value::Array(arr)) = serde_json::from_slice::<Value>(payload_bytes) {
            if arr.len() >= 2 {
                // 尝试从 Header 提取 transaction_id
                if let Some(tid) = arr[0].get("transaction_id").and_then(|v| v.as_i64()) {
                    // 检查全局 PENDING_REQUESTS 是否有人在等这个 ID
                    if let Some((_, tx)) = PENDING_REQUESTS.remove(&tid) {
                        info!("🎯 成功匹配回包，正在交付: tid={}", tid);
                        let _ = tx.send(arr[1].clone());
                        return;
                    }
                }
            }
        }

        let payload_str = String::from_utf8_lossy(payload_bytes);
        info!("📩 收到普通消息: Topic={}, Payload={}", publish.topic, payload_str);
    }
}