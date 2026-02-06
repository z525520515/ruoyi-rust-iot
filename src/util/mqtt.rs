use anyhow::Result;
use log::{error, info};
use rumqttc::{AsyncClient, Event, MqttOptions, Packet, Publish, QoS};
use std::time::Duration;
use tokio::time::{sleep, timeout};
use dashmap::DashMap;
use tokio::sync::oneshot;
use serde_json::{json, Value};
use once_cell::sync::Lazy;
use rbatis::rbdc::datetime::DateTime;
use rbatis::snowflake::new_snowflake_id;

// 假设您的配置路径如下，请根据实际情况调整
use crate::config::config::MqttConfig;
use crate::modules::equipment_data::domain::table::tables::EquipmentData;
use crate::config::linker_request_header::{
    build_request_header,
    build_request_payload,
    build_request_topic,
    build_response_topic,
    build_topic_prefix,
    current_timestamp_ms,
    generate_transaction_id,
    DEFAULT_CLIENT_NAME,
    DEFAULT_REQUEST_SOURCE,
    DEFAULT_TIMEOUT_MS,
};

// DEVICE_PROTO 协议解析
use DEVICE_PROTO::dev_frame::DevFrame;

use crate::util::iot_data::IotData;

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
                                Self::handle_raw_message(&client, publish);
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
        let tid = generate_transaction_id();

        // 2. 构造 README 要求的 [Header, Params] 数组格式
        let header = build_request_header(
            action,
            method,
            DEFAULT_REQUEST_SOURCE,
            device_hash,
            current_timestamp_ms(),
            tid,
            DEFAULT_TIMEOUT_MS,
        );
        let full_payload = build_request_payload(header, params);
        let payload_bytes = serde_json::to_vec(&full_payload)?;

        // 3. 构造 Topic (对 Hash 进行 URL 编码以防特殊字符)
        let topic_prefix = build_topic_prefix(device_hash, DEFAULT_CLIENT_NAME);
        let request_topic = build_request_topic(
            &topic_prefix,
            protocol_type,
            stack_name,
            action,
        );
        let response_topic = build_response_topic(
            &topic_prefix,
            protocol_type,
            stack_name,
            action,
        );

        // 4. 在全局等待队列中注册
        let (tx, rx) = oneshot::channel();
        PENDING_REQUESTS.insert(tid, tx);

        // 5. 发布 MQTT 消息
        self.client.subscribe(&response_topic, QoS::AtMostOnce).await?;
        self.client.publish(&request_topic, QoS::AtLeastOnce, false, payload_bytes).await?;

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
    fn handle_raw_message(client: &AsyncClient, publish: Publish) {
        let payload_bytes = &publish.payload;
        if let Ok(Value::Array(arr)) = serde_json::from_slice::<Value>(payload_bytes) {
            if arr.len() >= 2 {
                // 提取 Header 中的 transaction_id
                if let Some(tid) = arr[0].get("transaction_id").and_then(|v| v.as_i64()) {
                    // 检查是否有对应的 HTTP 请求在等待
                    if let Some((_, tx)) = PENDING_REQUESTS.remove(&tid) {
                        info!("🎯 匹配到网关响应: tid={}", tid);
                        let _ = tx.send(arr[1].clone());
                        return;
                    }
                }
            }
        }

        // 尝试作为 DEVICE_PROTO 协议解析
        if let Some(parsed_data) = Self::try_parse_dev_proto(payload_bytes) {
            info!("✅ DEVICE_PROTO 解析成功: device_id={:?}", parsed_data.device_id);
            Self::process_device_data(client, parsed_data);
            return;
        }

        // 普通文本消息
        let payload_str = String::from_utf8_lossy(payload_bytes);
        info!("📩 收到普通 MQTT 消息: Topic={}, Payload={}", publish.topic, payload_str);
    }

    fn try_parse_dev_proto(payload: &[u8]) -> Option<DevFrame> {
        // 尝试将 payload 解析为 DEVICE_PROTO 帧
        match DevFrame::from_bytes(payload) {
            Ok(frame) => Some(frame),
            Err(e) => {
                // 如果是十六进制字符串格式，尝试转换后再解析
                if let Ok(hex_str) = std::str::from_utf8(payload) {
                    if let Ok(bytes) = hex_from_str(hex_str) {
                        return match DevFrame::from_bytes(&bytes) {
                            Ok(frame) => Some(frame),
                            Err(_) => None,
                        };
                    }
                }
                None
            }
        }
    }

    fn process_device_data(client: &AsyncClient, frame: DevFrame) {
        let device_id_str = frame.device_id.to_string();
        let device_type_name = frame.device_type.info().name;

        info!("📡 收到设备数据: id={}, type={}", device_id_str, device_type_name);

        let signal_dbm = frame.signal_strength.get_dbm();
        let power_level = frame.power_status.level.get_level();
        let is_external_power = frame.power_status.is_external_power;
        let alarm_status = frame.alarm_status.to_u8();

        let alarm_info = frame.alarm_status.interpret(frame.device_type.clone());

        let data_record_interval = frame.data_record_interval.get_interval();
        let data_record_count = frame.data_record_count.get_count();

        let current_data_value = match &frame.current_device_data {
            either::Either::Left(device_data) => {
                let params: Vec<serde_json::Value> = device_data
                    .get_parameters()
                    .iter()
                    .map(|p| json!({
                        "name": p.parameter_name,
                        "physical_value": p.physical_value,
                        "unit": p.unit,
                        "raw_value": format!("{:?}", p.raw_value),
                    }))
                    .collect();
                json!({
                    "type": "device_data",
                    "parameters": params
                })
            }
            either::Either::Right(modbus_data) => {
                json!({
                    "type": "modbus_data",
                    "data_length": modbus_data.data_length(),
                    "data_bytes": format!("{:?}", modbus_data.to_bytes()),
                })
            }
        };

        let timestamp_str = frame.timestamp.to_formatted_string();

        let cache_data_json: Vec<serde_json::Value> = frame
            .cache_device_data
            .iter()
            .zip(frame.cache_timestamp.iter())
            .map(|(data, ts)| {
                let data_info = match data {
                    either::Either::Left(device_data) => {
                        let params: Vec<serde_json::Value> = device_data
                            .get_parameters()
                            .iter()
                            .map(|p| json!({
                                "name": p.parameter_name,
                                "physical_value": p.physical_value,
                                "unit": p.unit,
                            }))
                            .collect();
                        json!({
                            "type": "device_data",
                            "parameters": params
                        })
                    }
                    either::Either::Right(modbus_data) => {
                        json!({
                            "type": "modbus_data",
                            "data_length": modbus_data.data_length(),
                        })
                    }
                };
                json!({
                    "timestamp": ts.to_formatted_string(),
                    "data": data_info
                })
            })
            .collect();

        let full_device_data = json!({
             "frame_info": {
                 "command_type": format!("0x{:02X}", frame.command_type),
                 "frame_length": frame.frame_length.len(),
                 "crc": frame.crc,
             },
             "device": {
                 "id": device_id_str,
                 "type_code": frame.device_type.to_u8(),
                 "type_name": device_type_name,
             },
             "power": {
                 "level": power_level,
                 "is_external_power": is_external_power,
                 "source": format!("{:?}", frame.power_status.source),
             },
             "signal": {
                 "value": signal_dbm,
                 "unit": "dBm",
             },
             "alarm": {
                 "status": alarm_status,
                 "info": format!("{:?}", alarm_info),
                 "parameters": {
                     "alarm_group": frame.alarm_parameters.get_alarm_group(),
                     "lower_limit_0": frame.alarm_parameters.get_alarm_lower_limit(0).map(|v| format!("{:?}", v)),
                     "upper_limit_0": frame.alarm_parameters.get_alarm_upper_limit(0).map(|v| format!("{:?}", v)),
                     "lower_limit_1": frame.alarm_parameters.get_alarm_lower_limit(1).map(|v| format!("{:?}", v)),
                     "upper_limit_1": frame.alarm_parameters.get_alarm_upper_limit(1).map(|v| format!("{:?}", v)),
                 }
             },
             "data_record": {
                 "interval_seconds": data_record_interval,
                 "count": data_record_count,
             },
             "current_data": current_data_value,
             "timestamp": timestamp_str,
             "cache_data": cache_data_json,
         });

        info!("📊 完整设备数据:\n{}", serde_json::to_string_pretty(&full_device_data).unwrap());

        let equipment_data = EquipmentData {
             id: Some(new_snowflake_id() as i64),
             equipment_number: Some(device_id_str.clone()),
             equipment_name: Some(format!("设备-{}", &device_id_str[..8])),
             device_category: Some(device_type_name.to_string()),
             upload_time: Some(DateTime::now().into()),
             battery_level: Some(power_level as f64),
             signal_strength: Some(signal_dbm as i32),
             alarm_status: Some(alarm_status as i32),
             payload: Some(full_device_data.clone()),
             dept_id: None,
             remark: None,
             create_time: Some(DateTime::now().into()),
         };

        let iot_client = client.clone();
        let iot_device_id = device_id_str.clone();
        let iot_device_type = device_type_name;
        let iot_equipment_name = format!("设备-{}", &device_id_str[..8]);
        let iot_signal = signal_dbm;
        let iot_battery = power_level;
        let iot_alarm_status = alarm_status;
        let iot_alarm_info = format!("{:?}", alarm_info);

        let iot_value = match &frame.current_device_data {
            either::Either::Left(device_data) => {
                let first_param = device_data.get_parameters().first();
                if let Some(p) = first_param {
                    Some(format!("{:.3} {}", p.physical_value, p.unit))
                } else {
                    Some(String::new())
                }
            }
            either::Either::Right(_) => {
                Some(format!("{:?}", frame.current_device_data))
            }
        };

        tokio::spawn(async move {
            if let Err(e) = EquipmentData::insert(crate::pool!(), &equipment_data).await {
                error!("❌ 保存设备数据失败: {}", e);
            } else {
                info!("✅ 设备数据已保存: {}", device_id_str);
            }

            let mut iot_data = IotData::new();
            iot_data.device_id = Some(iot_device_id.clone());
            iot_data.gateway_id = None;
            iot_data.device_type = Some(iot_device_type.to_string());
            iot_data.extension_tag = None;
            iot_data.event_time = Some(chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
            iot_data.location = Some(iot_equipment_name);
            iot_data.value = iot_value;
            iot_data.alarm_condition = None;
            iot_data.alarm_status = if iot_alarm_status > 0 { Some("1".to_string()) } else { Some("0".to_string()) };
            iot_data.master_machine = None;
            iot_data.alarm_remark = if iot_alarm_status > 0 { Some(iot_alarm_info) } else { None };
            iot_data.battery = Some(iot_battery.to_string());
            iot_data.rssi = Some(iot_signal.to_string());

            let iot_payload = serde_json::to_string(&iot_data).unwrap_or_default();
            let topic = format!("iot/iot/{}/data", iot_device_id);

            if let Err(e) = iot_client.publish(&topic, QoS::AtLeastOnce, false, iot_payload.into_bytes()).await {
                error!("❌ IotData 转发失败: {}", e);
            } else {
                info!("✅ IotData 转发成功: topic={}", topic);
            }
        });
    }
}

fn hex_from_str(input: &str) -> Result<Vec<u8>, std::num::ParseIntError> {
    input
        .trim()
        .replace(|c: char| !c.is_ascii_hexdigit(), "")
        .as_bytes()
        .chunks(2)
        .map(|chunk| std::str::from_utf8(chunk).unwrap())
        .map(|chunk| u8::from_str_radix(chunk, 16))
        .collect()
}
