// 数据过滤模块
// 该模块负责根据设备类型和数据类型对数据进行转发前的过滤，转化为标准的JSON格式

// 来一个json数据，转化为标准的JSON格式 data_type 为要处理的数据结构格式，对应于device_mappings.yml中的device_name
use crate::modules::plc_gateway::data_process::dynamic_mapping::load_mappings;
use crate::modules::plc_gateway::entity::firealarm::FireAlarm;
use crate::modules::plc_gateway::entity::firealarm::ToFireAlarm;
use serde::Deserialize;
use serde_json::{self, Value};
use std::collections::HashMap;
use uuid::Uuid;

// Structure to hold the input JSON and device name
#[derive(Debug, Deserialize)]
pub struct InputJson {
    pub json: Value,
    pub device_name: String,
}

// Implementation to transform input JSON to FireAlarm
impl ToFireAlarm for InputJson {
    fn to_fire_alarm(&self) -> FireAlarm {
        // 从缓存获取YAML映射，避免重复加载文件
        let mappings = load_mappings();

        // Find the mapping for the specified device_name
        let device_mapping = mappings
            .mappings
            .get(&self.device_name)
            .expect("Device name not found in mappings");

        // Create a new HashMap to store the standardized JSON
        let mut standardized_json = HashMap::new();

        // Iterate through the mappings to transform the input JSON
        for (standard_key, input_key) in device_mapping.iter() {
            // Handle nested keys (e.g., "alarm:alerts")
            if input_key.contains(':') {
                let keys: Vec<&str> = input_key.split(':').collect();
                if keys.len() == 2 {
                    let parent_key = keys[0];
                    let child_key = keys[1];
                    if let Some(parent) = self.json.get(parent_key) {
                        if let Some(value) = parent.get(child_key) {
                            // Special handling for alarm_condition (serialize as JSON string)
                            if standard_key == "alarm_condition" {
                                let serialized_value = serde_json::to_string(value)
                                    .expect("Failed to serialize alarm_condition");
                                standardized_json
                                    .insert(standard_key.clone(), Value::String(serialized_value));
                            } else {
                                standardized_json.insert(standard_key.clone(), value.clone());
                            }
                        }
                    }
                }
            } else {
                // Handle top-level keys
                if let Some(value) = self.json.get(input_key) {
                    standardized_json.insert(standard_key.clone(), value.clone());
                }
            }
        }
        // println!("standardized_json: {:#?}", standardized_json);
        // Convert the standardized JSON to a FireAlarm object
        let mut fire_alarm = FireAlarm {
            id: Uuid::new_v4(),
            device_id: String::new(),
            gateway_id: String::new(), // Default or configurable
            device_type: String::new(),
            extension_tag: String::new(),
            event_time: 0,
            location: String::new(),
            value: String::new(),
            alarm_condition: String::new(),
            alarm_status: String::new(),
            master_machine: String::new(),
            alarm_remark: String::new(),
        };

        // Populate FireAlarm fields from standardized JSON
        if let Some(device_id) = standardized_json.get("device_id") {
            fire_alarm.device_id = device_id.as_str().unwrap_or_default().to_string();
        }
        if let Some(gateway_id) = standardized_json.get("gateway_id") {
            fire_alarm.gateway_id = gateway_id.as_str().unwrap_or_default().to_string();
        }
        if let Some(device_type) = standardized_json.get("device_type") {
            fire_alarm.device_type = device_type.as_str().unwrap_or_default().to_string();
        }
        if let Some(extension_tag) = standardized_json.get("extension_tag") {
            fire_alarm.extension_tag = extension_tag.as_str().unwrap_or_default().to_string();
        }
        if let Some(event_time) = standardized_json.get("event_time") {
            fire_alarm.event_time = event_time.as_i64().unwrap_or_default();
        }
        if let Some(location) = standardized_json.get("location") {
            fire_alarm.location = location.as_str().unwrap_or_default().to_string();
        }
        if let Some(value) = standardized_json.get("value") {
            fire_alarm.value = value.as_str().unwrap_or_default().to_string();
        }
        if let Some(alarm_condition) = standardized_json.get("alarm_condition") {
            fire_alarm.alarm_condition = alarm_condition.as_str().unwrap_or_default().to_string();
        }
        if let Some(alarm_status) = standardized_json.get("alarm_status") {
            fire_alarm.alarm_status = alarm_status.as_str().unwrap_or_default().to_string();
        }
        if let Some(master_machine) = standardized_json.get("master_machine") {
            fire_alarm.master_machine = master_machine.as_str().unwrap_or_default().to_string();
        }
        let alarm_value = standardized_json.get("alarm_remark").unwrap_or(&Value::Null);
        // 2. 统一转成 Vec<String>
        fire_alarm.alarm_remark = match alarm_value {
            // 情况 A：已经是数组
            Value::Array(arr) => arr
                .iter()
                .filter_map(|v| v.as_str()) // 只保留字符串元素
                .collect::<Vec<&str>>() // 先收集到 Vec<&str>
                .join(","), // 再用逗号拼接成一个 String
            // 情况 B：单个字符串
            Value::String(s) => s.clone(),
            // 情况 C：其它（缺失/null/数字等）当空字符串
            _ => String::new(),
        };
        fire_alarm
    }
}

// Function to transform JSON and return FireAlarm
pub fn transform_json(json_str: &str, device_name: &str) -> FireAlarm {
    let json: Value = serde_json::from_str(json_str).expect("Failed to parse JSON");
    let input = InputJson {
        json,
        device_name: device_name.to_string(),
    };

    input.to_fire_alarm()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_master() {
        // 处理时间字符串转换为时间戳
        let mut json_data = json!({
            "status":"火灾报警",
            "statusType":"",
            "deviceType":"探测器",
            "extensionTag":"L01D32",
            "partition":"",
            "type":"定温感温头",
            "nodeNumber":"1",
            "loopNumber":"1",
            "pointNumber":"32",
            "time":"Mon Oct 22 2007 13:14:29 GMT+0800 (中国标准时间)",
            "dsNum":"N001L01D032"
        });

        let time_str_opt = json_data
            .get("time")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        if let Some(time_str) = time_str_opt {
            let time_str_clean = time_str.replace(" (中国标准时间)", "").replace("GMT", "");
            println!("清理后的时间字符串: {}", time_str_clean);
            if let Ok(dt) =
                chrono::DateTime::parse_from_str(&time_str_clean, "%a %b %d %Y %H:%M:%S %z")
            {
                // let timestamp = dt.timestamp_millis(); //毫秒
                let timestamp = dt.timestamp(); //  秒
                json_data["time"] = json!(timestamp); // 现在可以安全修改
                println!("原时间: {}", time_str);
                println!("时间戳: {}", timestamp);
            } else {
                println!("时间解析失败: {}", time_str_clean);
            }
        }

        let fire_alarm = transform_json(json_data.to_string().as_str(), "master");
        println!("{:#?}", serde_json::to_string_pretty(&fire_alarm).unwrap());
    }

    #[test]
    fn test_slaver() {
        let json_data = json!(
                {
            "location": "九龙湖南站",
            "masterMachine": "消防主机电源",
            "equipment": "V13xd_XDSE002",
            "n": "主电状态",
            "v": 0,
            "t": 1725004463130 as i64
        }
            );
        let fire_alarm = transform_json(json_data.to_string().as_str(), "slaver");
        // assert_eq!(fire_alarm.alarm_type, vec!["火灾报警"]);
        println!("{:#?}", serde_json::to_string_pretty(&fire_alarm).unwrap());
    }
}
