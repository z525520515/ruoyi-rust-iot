use crate::modules::plc_gateway::data_process::data_filter::transform_json;
use crate::modules::plc_gateway::entity::firealarm::FireAlarm;
use crate::modules::plc_gateway::unified::response::{error_response, success_response};
use anyhow::Result;
use axum::{Extension, Json, extract::Path};
use log::info;
use rumqttc::AsyncClient;
use serde_json::{Value, json};
use std::sync::Arc;

/// 处理火灾信息推送
/// master类型的消息推送到 info/master topic
/// slaver类型的消息推送到 info/slaver topic
pub async fn handle_fire_info_push(
    Extension(mqtt_client): Extension<Arc<AsyncClient>>,
    Path(fire_type): Path<String>,
    Json(json_data): Json<Value>,
) -> Json<Value> {
    // 1. 校验 fire_type 参数
    if fire_type != "master" && fire_type != "slaver" {
        return error_response(
            "fire_type参数无效，必须是'master'或'slaver'".to_string(),
            None,
        );
    }

    // 2. 校验 JSON 数据是否为空
    if json_data.is_null() || (json_data.is_object() && json_data.as_object().unwrap().is_empty()) {
        return error_response("请求JSON数据不能为空".to_string(), None);
    }

    // 3. 根据 fire_type 处理
    match fire_type.as_str() {
        "master" => match process_and_push(&mqtt_client, &fire_type, json_data).await {
            Ok(_) => success_response(json!({}), None),
            Err(e) => error_response(
                format!("处理{}类型火灾信息并推送失败: {}", fire_type, e),
                None,
            ),
        },
        "slaver" => match process_batch_and_push(&mqtt_client, &fire_type, &json_data).await {
            Ok(_) => success_response(json!({}), None),
            Err(e) => error_response(e, None),
        },
        _ => error_response("未知的fire_type参数".to_string(), None),
    }
}

/// 处理单条数据并推送到MQTT
async fn process_and_push(client: &AsyncClient, fire_type: &str, json_data: Value) -> Result<()> {
    // 处理时间戳
    let processed_data = process_time_stamp(json_data, fire_type)?;

    // 转换为FireAlarm对象
    let fire_alarm: FireAlarm = transform_json(processed_data.to_string().as_str(), fire_type);

    // 转换回JSON字符串用于推送
    let payload = serde_json::to_string(&fire_alarm)?;

    // 构建MQTT主题
    // let topic = format!("info/{}", fire_type);
    let topic = "iot/devices/SNDEVICE00000001/data".to_string();
    // println!("topic: {}", topic);
    // 推送到MQTT
    client
        .publish(&topic, rumqttc::QoS::AtLeastOnce, false, payload)
        .await?;
    info!("已成功推送{}类型火灾信息到topic: {}", fire_type, topic);

    Ok(())
}

/// 批量处理数据并推送到MQTT
async fn process_batch_and_push(
    client: &AsyncClient,
    fire_type: &str,
    json_data: &Value,
) -> Result<usize, String> {
    let arr = serde_json::from_value::<Vec<Value>>(json_data.clone())
        .map_err(|e| format!("解析批量JSON失败: {:?}", e))?;

    let mut success_count = 0;

    // 构建MQTT主题
    // let topic = format!("info/{}", fire_type);
    let topic = "iot/devices/SNDEVICE00000001/data".to_string();
    // println!("topic: {}", topic);
    for item in arr {
        match process_time_stamp(item, fire_type) {
            Ok(processed_data) => {
                // 转换为FireAlarm对象
                let fire_alarm: FireAlarm =
                    transform_json(processed_data.to_string().as_str(), fire_type);

                // 转换回JSON字符串用于推送
                if let Ok(payload) = serde_json::to_string(&fire_alarm) {
                    // 推送到MQTT
                    if client
                        .publish(&topic, rumqttc::QoS::AtLeastOnce, false, payload)
                        .await
                        .is_ok()
                    {
                        success_count += 1;
                    }
                }
            }
            Err(e) => {
                eprintln!("处理批量项目失败: {}", e);
                // 继续处理其他项目
            }
        }
    }

    if success_count == 0 {
        return Err("批量处理失败：所有项目均处理失败".to_string());
    }

    info!(
        "已成功推送{}条{}类型火灾信息到topic: {}",
        success_count, fire_type, topic
    );
    Ok(success_count)
}

/// 处理时间戳格式
/// master类型：处理time字段
/// slaver类型：字段t已是时间戳，无需处理
fn process_time_stamp(mut json_data: Value, fire_type: &str) -> Result<Value> {
    // 只对master类型的数据处理time字段
    if fire_type == "master" {
        if let Some(time_value) = json_data.get("time") {
            // 检查是否已经是数字类型（可能已经是时间戳）
            if time_value.is_number() {
                return Ok(json_data);
            }
            // 处理字符串类型的时间
            if let Some(time_str) = time_value.as_str() {
                let time_str_clean = time_str.replace(" (中国标准时间)", "").replace("GMT", "");
                if let Ok(dt) =
                    chrono::DateTime::parse_from_str(&time_str_clean, "%a %b %d %Y %H:%M:%S %z")
                {
                    let timestamp = dt.timestamp(); // 秒级时间戳
                    json_data["time"] = json!(timestamp);
                } else {
                    println!("时间解析失败: {}", time_str_clean);
                }
            }
        }
        // 2. 补上缺失的 masterMachine
        json_data["masterMachine"] = json!("火灾自动报警系统");
        // 获取
        //  if let Some(time_value) = json_data.get("time") {

        //  }
    }
    // slaver时间戳是毫秒，要转换为秒级
    if fire_type == "slaver" {
        if let Some(time_value) = json_data.get("t") {
            if let Some(time_str) = time_value.as_str() {
                let timestamp = time_str.parse::<i64>()? / 1000; // 转换为秒
                json_data["t"] = json!(timestamp);
            }
            if let Some(timestamp) = json_data.get("t").and_then(|v| v.as_i64()) {
                json_data["t"] = json!(timestamp / 1000); // 转换为秒
            }
        }
        if let Some(value) = json_data.get("v") {
            if let Some(v) = value.as_i64() {
                if v == 1 {
                    json_data["alarmStatus"] = json!("报警");
                } else if v == 0 {
                    json_data["alarmStatus"] = json!("未报警");
                }
            }
        }
    }
    Ok(json_data)
}
