use rbatis::rbdc::datetime::DateTime;
//   在tables.rs 增加
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct EquipmentData {
    pub id: Option<i64>,
    pub equipment_number: Option<String>,
    pub equipment_name: Option<String>,
    pub device_category: Option<String>,
    pub upload_time: Option<DateTime>,
    pub battery_level: Option<f64>,
    pub signal_strength: Option<i32>,
    pub alarm_status: Option<i32>,
    pub payload: Option<serde_json::Value>,
    pub dept_id: Option<i32>,
    pub remark: Option<String>,
    pub create_time: Option<DateTime>,
}
