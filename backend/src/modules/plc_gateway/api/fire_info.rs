use crate::modules::plc_gateway::entity::firealarm::FireAlarm;
use anyhow::Result;
use axum::{Json, extract::Path};

use crate::modules::plc_gateway::data_process::data_filter::transform_json;
use crate::modules::plc_gateway::unified::response::{error_response, success_response};
// use serde::{Deserialize, Serialize};
use log::error;
use serde_json::{Value, json};
/*
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SlaverReq {
    pub status: String,
    pub status_type: String,
    pub device_type: String,
    pub extension_tag: String,
    pub partition: String,
    #[serde(rename = "type")] // 保留字，必须单独改
    pub point_type: String,
    pub node_number: String,
    pub loop_number: String,
    pub point_number: String,
    pub time: String,
    pub ds_num: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MasterReq {
    pub location: String,
    pub master_machine: String,
    pub equipment: String,
    pub n: String,
    pub v: i32,
    pub t: String,
}

pub async fn get_fire_master(fire_type: &str, json_data: &mut MasterReq) -> Result<FireAlarm> {
    // 1. 清洗时间字符串
    let time_str_clean = json_data
        .t
        .replace(" (中国标准时间)", "")
        .replace("GMT", "");

    if let Ok(dt) = chrono::DateTime::parse_from_str(&time_str_clean, "%a %b %d %Y %H:%M:%S %z") {
        json_data.t = dt.timestamp().to_string(); // 秒级时间戳
    } else {
        eprintln!("时间解析失败: {}", time_str_clean);
    }

    // 2. 序列化
    let json_str = serde_json::to_string_pretty(json_data)?; // ← 关键：借引用
    let fire_alarm = transform_json(&json_str, fire_type);

    Ok(fire_alarm)
}
 */

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
                    error!("时间解析失败: {}", time_str_clean);
                }
            }
        }
        // 2. 补上缺失的 masterMachine
        json_data["masterMachine"] = json!("火灾自动报警系统");
    }
    // slaver类型无需处理时间戳，直接返回
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

pub async fn get_fire_info(fire_type: &str, json_data: serde_json::Value) -> Result<FireAlarm> {
    // 处理时间戳
    let processed_data = process_time_stamp(json_data, fire_type)?;

    // 转换为FireAlarm对象
    let fire_alarm: FireAlarm = transform_json(processed_data.to_string().as_str(), fire_type);

    Ok(fire_alarm)
}

// 批量处理函数
async fn process_batch(fire_type: &str, json_data: &Value) -> Result<Vec<FireAlarm>, String> {
    let arr = serde_json::from_value::<Vec<Value>>(json_data.clone())
        .map_err(|e| format!("解析批量JSON失败: {:?}", e))?;

    let mut fire_alarms = Vec::with_capacity(arr.len());

    for item in arr {
        match get_fire_info(fire_type, item).await {
            Ok(fire_alarm) => fire_alarms.push(fire_alarm),
            Err(e) => {
                eprintln!("处理批量项目失败: {}", e);
                // 继续处理其他项目
            }
        }
    }

    if fire_alarms.is_empty() {
        return Err("批量处理失败：所有项目均处理失败".to_string());
    }

    Ok(fire_alarms)
}

pub async fn handle_get_fire_info(
    Path(fire_type): Path<String>,
    Json(json_data): Json<Value>,
) -> Json<Value> {
    // 1. 校验 fire_type 参数 - 默认400
    if fire_type != "master" && fire_type != "slaver" {
        return error_response(
            "fire_type参数无效，必须是'master'或'slaver'".to_string(),
            None,
        );
    }

    // 2. 校验 JSON 数据是否为空 - 默认400
    if json_data.is_null() || (json_data.is_object() && json_data.as_object().unwrap().is_empty()) {
        return error_response("请求JSON数据不能为空".to_string(), None);
    }

    // 3. 根据 fire_type 处理
    match fire_type.as_str() {
        "master" => {
            match get_fire_info(&fire_type, json_data).await {
                Ok(fire_alarm) => match serde_json::to_value(fire_alarm) {
                    Ok(json_value) => success_response(json_value, None),
                    Err(e) => error_response(
                        format!("序列化{}类型数据失败: {:?}", fire_type, e),
                        None, // 默认400
                    ),
                },
                Err(e) => error_response(
                    format!("处理{}类型火灾信息失败: {}", fire_type, e),
                    None, // 默认400
                ),
            }
        }
        "slaver" => {
            match process_batch(&fire_type, &json_data).await {
                Ok(fire_alarms) => match serde_json::to_value(fire_alarms) {
                    Ok(json_value) => success_response(json_value, None),
                    Err(e) => error_response(
                        format!("序列化批量数据失败: {:?}", e),
                        None, // 默认400
                    ),
                },
                Err(e) => error_response(e, None), // 默认400
            }
        }
        _ => error_response("未知的fire_type参数".to_string(), None),
    }
}
