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
use chrono::Utc;

// 假设您的配置路径如下，请根据实际情况调整
use crate::config::config::MqttConfig;

// ========== 全局状态：用于匹配 MQTT 异步回包 ==========
// Key: transaction_id, Value: 发送结果的信道
static PENDING_REQUESTS: Lazy<DashMap<i64, oneshot::Sender<Value>>> = Lazy::new(DashMap::new);

// ========== MQTT 公共管理类 ==========
pub struct MqttManager {
    pub client: AsyncClient,
}

impl MqttManager {
    /// 初始化 MQTT 服务 (保持您原始的 init 逻辑不变)
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
                                // 核心修改：调用处理函数检查是否有 HTTP 请求在等待此消息
                                Self::handle_raw_message(publish);
                            }
                            Event::Incoming(Packet::ConnAck(_)) => {
                                info!("✅ MQTT 连接成功 (ConnAck)");
                                // 可以在这里添加您的自动订阅逻辑
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

    /// 订阅逻辑
    pub async fn subscribe(&self, topic: &str, qos: QoS) -> Result<()> {
        self.client.subscribe(topic, qos).await?;
        info!("📢 订阅成功: {}", topic);
        Ok(())
    }

    /// 发布逻辑
    pub async fn publish(&self, topic: &str, payload: Vec<u8>, qos: QoS) -> Result<()> {
        self.client.publish(topic, qos, false, payload).await?;
        Ok(())
    }

    // ================== 新增方法 1: 供 HTTP 调用的封装方法 ==================

    /// 封装网关协议并同步等待结果
    pub async fn gateway_invoke(
        &self,
        device_hash: &str,    // README 中的 deviceUniqueIDHash (Base64)
        protocol_type: &str, // 如 "Control"
        stack_name: &str,    // 如 "ReservedControlProtocolStack"
        action: &str,        // 指令动作
        method: &str,        // 指令方法
        params: Value,       // 参数
    ) -> Result<Value> {
        // 1. 生成事务 ID (微秒级时间戳)
        let tid = (Utc::now().timestamp_nanos_opt().unwrap_or(0) / 1000) as i64;

        // 2. 构造 README 要求的 [Header, Params] 数组格式
        let header = json!({
            "action": action,
            "message_type": "request",
            "method": method,
            "source": "HTTP_BACKEND",
            "target": device_hash,
            "timestamp": Utc::now().timestamp_millis(),
            "transaction_id": tid,
            "version": "1.0.0"
        });
        let full_payload = json!([header, { "params": params }]);
        let payload_bytes = serde_json::to_vec(&full_payload)?;

        // 3. 构造 Topic (对 Hash 进行 URL 编码以防特殊字符)
        let encoded_hash = urlencoding::encode(device_hash);
        let topic = format!(
            "{}/ReservedClient/protocol_stack/{}/{}/{}/request",
            encoded_hash, protocol_type, stack_name, action
        );

        // 4. 在全局等待队列中注册
        let (tx, rx) = oneshot::channel();
        PENDING_REQUESTS.insert(tid, tx);

        // 5. 发布 MQTT 消息
        self.client.publish(&topic, QoS::AtLeastOnce, false, payload_bytes).await?;

        // 6. 等待回包 (设置 10 秒超时)
        match timeout(Duration::from_secs(10), rx).await {
            Ok(Ok(response_body)) => Ok(response_body),
            Ok(Err(_)) => Err(anyhow::anyhow!("Internal communication channel closed")),
            Err(_) => {
                PENDING_REQUESTS.remove(&tid);
                Err(anyhow::anyhow!("Gateway response timeout"))
            }
        }
    }

    // ================== 新增方法 2: 修改后的处理函数 ==================

    /// 处理收到的原始消息，并尝试唤醒等待的 HTTP 请求
    fn handle_raw_message(publish: Publish) {
        let payload_bytes = &publish.payload;

        // 尝试解析 [Header, Result] 数组
        if let Ok(Value::Array(arr)) = serde_json::from_slice::<Value>(payload_bytes) {
            if arr.len() >= 2 {
                // 提取 Header 中的 transaction_id
                if let Some(tid) = arr[0].get("transaction_id").and_then(|v| v.as_i64()) {
                    // 检查是否有对应的 HTTP 请求在等待
                    if let Some((_, tx)) = PENDING_REQUESTS.remove(&tid) {
                        info!("🎯 匹配到网关响应: tid={}", tid);
                        let _ = tx.send(arr[1].clone()); // 发送 Result 部分
                        return;
                    }
                }
            }
        }

        // 如果没有匹配到，则走普通日志逻辑
        let payload_str = String::from_utf8_lossy(payload_bytes);
        info!("📩 收到普通 MQTT 消息: Topic={}, Payload={}", publish.topic, payload_str);
    }
}