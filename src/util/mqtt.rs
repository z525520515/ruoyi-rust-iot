use anyhow::Result;
use log::{error, info};
use rumqttc::{AsyncClient, Event, MqttOptions, Packet, Publish, QoS};
use std::time::Duration;
use tokio::time::sleep;

use crate::config::config::MqttConfig;

// ========== MQTT 公共管理类 ==========
pub struct MqttManager {
    pub client: AsyncClient,
}

impl MqttManager {
    /// 初始化 MQTT 服务
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
                                Self::handle_raw_message(publish);
                            }
                            Event::Incoming(Packet::ConnAck(_)) => {
                                info!("✅ MQTT 连接成功 (ConnAck)");

                                // --- 在此处添加自动订阅逻辑 ---
                                // 这样写的好处是：每次重连成功后，都会自动重新订阅主题
                                let sub_client = client.clone();
                                tokio::spawn(async move {
                                    // 你可以在这里添加多个订阅
                                    let topics = vec![
                                        ("GIOT-GW/UL/SHRP25090100", QoS::AtLeastOnce),
                                    ];

                                    for (topic, qos) in topics {
                                        if let Err(e) = sub_client.subscribe(topic, qos).await {
                                            error!("❌ 自动订阅失败: {}, 错误: {:?}", topic, e);
                                        } else {
                                            info!("📢 自动订阅成功: {}", topic);
                                        }
                                    }
                                });
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

    /// 订阅指定主题（手动调用接口）
    pub async fn subscribe(&self, topic: &str, qos: QoS) -> Result<()> {
        self.client.subscribe(topic, qos).await?;
        info!("📢 手动订阅主题成功: {}", topic);
        Ok(())
    }

    /// 发布消息
    pub async fn publish(&self, topic: &str, payload: Vec<u8>, qos: QoS) -> Result<()> {
        self.client.publish(topic, qos, false, payload).await?;
        Ok(())
    }

    /// 统一处理收到的原始消息
    fn handle_raw_message(publish: Publish) {
        let payload = String::from_utf8_lossy(&publish.payload);
        info!("📩 收到消息: Topic={}, Payload={}", publish.topic, payload);
    }
}