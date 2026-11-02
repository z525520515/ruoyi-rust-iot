use rbatis::rbdc::datetime::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeviceStatusDTO {
    pub equipment_number: Option<String>,
    pub equipment_name: Option<String>,
    pub device_category: Option<String>,
    pub last_online_time: Option<DateTime>,
    pub report_interval: Option<i32>,
    pub is_online: Option<bool>,
    pub online_state_calc: Option<String>,
    pub battery_level: Option<f64>,
    pub signal_strength: Option<i32>,
    pub reported: Option<serde_json::Value>,
}