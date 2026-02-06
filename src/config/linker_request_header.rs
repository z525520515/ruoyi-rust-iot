use axum::extract::State;
use axum::Json;
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::oneshot;
use rumqttc::{AsyncClient, Event, MqttOptions, Packet, QoS};
use log::error;
use chrono::Utc;
use rbatis::snowflake::new_snowflake_id;

// ========== 1. 协议模型 (基于 README.md) ==========

#[derive(Debug, Serialize, Deserialize)]
pub struct LinkerRequestHeader {
    pub action: String,
    pub message_type: String, // 固定为 "request"
    pub method: String,
    pub source: String,
    pub target: String,
    pub timestamp: i64,
    pub transaction_id: i64,
    pub version: String,
    pub timeout: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LinkerResponseHeader {
    pub action: String,
    pub is_error: bool,
    pub message_type: String,
    pub method: String,
    pub source: String,
    pub target: String,
    pub transaction_id: i64,
}

// HTTP 接收的请求载体
#[derive(Debug, Deserialize)]
pub struct HttpInvokeParams {
    pub protocol_type: String, // e.g., Control
    pub stack_name: String,    // e.g., ReservedControlProtocolStack
    pub action: String,        // e.g., system
    pub method: String,        // e.g., deviceUniqueID
    pub params: Value,         // 具体的参数内容
}

// ========== 1.1 README 协议相关常规值 ==========

pub const DEFAULT_CLIENT_NAME: &str = "test_mqtt";
pub const DEFAULT_REQUEST_SOURCE: &str = "control";
pub const DEFAULT_PROTOCOL_VERSION: &str = "1.0.0";
pub const DEFAULT_TIMEOUT_MS: u64 = 3000;

pub fn generate_transaction_id() -> i64 {
    new_snowflake_id() as i64
}

pub fn current_timestamp_ms() -> i64 {
    Utc::now().timestamp_millis()
}

pub fn build_topic_prefix(device_hash: &str, client_name: &str) -> String {
    format!("{}/{}", urlencoding::encode(device_hash), urlencoding::encode(client_name))
}

pub fn build_request_topic(
    topic_prefix: &str,
    protocol_type: &str,
    stack_name: &str,
    action: &str,
) -> String {
    format!(
        "{}/protocol_stack/{}/{}/{}/request",
        topic_prefix,
        urlencoding::encode(protocol_type),
        urlencoding::encode(stack_name),
        urlencoding::encode(action)
    )
}

pub fn build_response_topic(
    topic_prefix: &str,
    protocol_type: &str,
    stack_name: &str,
    action: &str,
) -> String {
    format!(
        "{}/protocol_stack/{}/{}/{}/response",
        topic_prefix,
        urlencoding::encode(protocol_type),
        urlencoding::encode(stack_name),
        urlencoding::encode(action)
    )
}

pub fn build_request_header(
    action: &str,
    method: &str,
    source: &str,
    target: &str,
    timestamp: i64,
    transaction_id: i64,
    timeout: u64,
) -> LinkerRequestHeader {
    LinkerRequestHeader {
        action: action.to_string(),
        message_type: "request".to_string(),
        method: method.to_string(),
        source: source.to_string(),
        target: target.to_string(),
        timestamp,
        transaction_id,
        version: DEFAULT_PROTOCOL_VERSION.to_string(),
        timeout,
    }
}

pub fn build_request_payload(header: LinkerRequestHeader, params: Value) -> Value {
    json!([header, { "params": params }])
}

// ========== 2. 全局状态管理 ==========

type PendingRequests = Arc<DashMap<i64, oneshot::Sender<Value>>>;

pub struct AppState {
    pub mqtt_client: AsyncClient,
    pub pending_requests: PendingRequests,
    pub device_hash: String,
}

// ========== 3. MQTT 管理器改进版 ==========

pub struct MqttManager;

impl MqttManager {
    pub async fn init(
        host: &str,
        port: u16,
        device_id: &str,
        device_hash: &str,
        pending: PendingRequests
    ) -> AsyncClient {
        let mut mqtt_options = MqttOptions::new("Linker-HTTP-Proxy", host, port);
        mqtt_options.set_keep_alive(Duration::from_secs(30));
        mqtt_options.set_credentials(device_hash, device_id);

        let (client, mut eventloop) = AsyncClient::new(mqtt_options, 10);

        let client_clone = client.clone();
        tokio::spawn(async move {
            loop {
                match eventloop.poll().await {
                    Ok(event) => {
                        if let Event::Incoming(Packet::Publish(publish)) = event {
                            Self::handle_incoming(publish, &pending);
                        }
                    }
                    Err(e) => {
                        error!("MQTT Connection Error: {:?}", e);
                        tokio::time::sleep(Duration::from_secs(5)).await;
                    }
                }
            }
        });

        client_clone
    }

    fn handle_incoming(publish: rumqttc::Publish, pending: &PendingRequests) {
        let payload = publish.payload.to_vec();
        // 解析规范要求的 JSON 数组格式 [Header, Body/Result]
        if let Ok(Value::Array(arr)) = serde_json::from_slice::<Value>(&payload) {
            if arr.len() >= 2 {
                if let Ok(header) = serde_json::from_value::<LinkerResponseHeader>(arr[0].clone()) {
                    let tid = header.transaction_id;
                    // 如果在等待列表中找到该 ID，则通过 channel 发送结果
                    if let Some((_, tx)) = pending.remove(&tid) {
                        let _ = tx.send(arr[1].clone());
                    }
                }
            }
        }
    }
}

// ========== 4. HTTP 路由处理函数 ==========

#[allow(dead_code)]
async fn invoke_gateway(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<HttpInvokeParams>,
) -> Result<Json<Value>, String> {
    // 1. 生成符合规范的 Transaction ID (i64)
    let tid = generate_transaction_id();

    // 2. 准备主题路径 (README: {topicPrefix}/protocol_stack/${type}/${name}/${action}/request)
    // 注意：topicPrefix 需要百分号编码，此处简化演示
    let topic_prefix = build_topic_prefix(&state.device_hash, DEFAULT_CLIENT_NAME);
    let request_topic = build_request_topic(
        &topic_prefix,
        &payload.protocol_type,
        &payload.stack_name,
        &payload.action,
    );
    let response_topic = build_response_topic(
        &topic_prefix,
        &payload.protocol_type,
        &payload.stack_name,
        &payload.action,
    );

    // 3. 订阅响应主题 (确保能收到回包)
    let _ = state.mqtt_client.subscribe(&response_topic, QoS::AtMostOnce).await;

    // 4. 构建规范要求的 JSON 数组消息 [Header, Params]
    let header = build_request_header(
        &payload.action,
        &payload.method,
        DEFAULT_REQUEST_SOURCE,
        &state.device_hash,
        current_timestamp_ms(),
        tid,
        DEFAULT_TIMEOUT_MS,
    );

    let mqtt_payload = build_request_payload(header, payload.params);
    let payload_bytes = serde_json::to_vec(&mqtt_payload).map_err(|e| e.to_string())?;

    // 5. 创建同步等待信道
    let (tx, rx) = oneshot::channel();
    state.pending_requests.insert(tid, tx);

    // 6. 发布 MQTT 消息
    state.mqtt_client
        .publish(request_topic, QoS::AtLeastOnce, false, payload_bytes)
        .await
        .map_err(|e| e.to_string())?;

    // 7. 等待响应（带超时机制）
    match tokio::time::timeout(Duration::from_secs(6), rx).await {
        Ok(Ok(result)) => Ok(Json(result)),
        Ok(Err(_)) => Err("Internal communication channel closed".to_string()),
        Err(_) => {
            state.pending_requests.remove(&tid);
            Err("Gateway response timeout".to_string())
        }
    }
}
