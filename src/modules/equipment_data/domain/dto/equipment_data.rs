use crate::modules::equipment_data::domain::table::tables::EquipmentData;
use rbatis::snowflake::new_snowflake_id;
use rbatis::rbdc::datetime::DateTime;
use rbatis::plugin::page::PageRequest;
use serde::{Deserialize, Serialize};
    
    
/// page DTO
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EquipmentDaumPageDTO {
    #[serde(rename(deserialize = "pageNum"))]
    pub page_no: Option<u64>,
    #[serde(rename(deserialize = "pageSize"))]
    pub page_size: Option<u64>,
    pub equipment_number: Option<String>,
    pub equipment_name: Option<String>,
    pub device_category: Option<String>,
    pub battery_level: Option<f64>,
    pub signal_strength: Option<i32>,
    pub alarm_status: Option<i32>,
    pub payload: Option<serde_json::Value>,
    pub dept_id: Option<i32>,
    pub remark: Option<String>,
}
//    实现从 dto对象 到 PageRequest
impl From<EquipmentDaumPageDTO> for PageRequest {
    fn from(arg: EquipmentDaumPageDTO) -> Self {
        PageRequest::new(arg.page_no.unwrap_or(1), arg.page_size.unwrap_or(10))
    }
}
//    实现从 &dto对象 到 PageRequest
impl From<&EquipmentDaumPageDTO> for PageRequest {
    fn from(arg: &EquipmentDaumPageDTO) -> Self {
        PageRequest::new(arg.page_no.unwrap_or(1), arg.page_size.unwrap_or(10))
    }
}
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EquipmentDaumAddDTO {
    pub equipment_number: Option<String>,//设备唯一编号 (对应原表 equipment_number 或 deviceID)
    pub equipment_name: Option<String>,//设备名称
    pub device_category: Option<String>,//设备分类标签 (electric:电表, water:水表, terminal:终端, alarm:报警器, wireless:无线终端)
    pub upload_time: Option<DateTime>,//统一上报/触发时间 (时序索引核心字段)
    pub battery_level: Option<f64>,//剩余电量百分比/数值
    pub signal_strength: Option<i32>,//信号强度 (对应原表 rssi 或 signal_strength)
    pub alarm_status: Option<i32>,//报警状态标识 (0:正常, 1:报警或异常)
    pub payload: Option<serde_json::Value>,
    pub dept_id: Option<i32>,
    pub remark: Option<String>,
}
//    实现从 table对象 到 AddDTO对象的复制创建
impl From<EquipmentDaumAddDTO> for EquipmentData {
    fn from(arg: EquipmentDaumAddDTO) -> Self {
        Self {
            id: Some(new_snowflake_id() as i64),
            equipment_number: arg.equipment_number,
            equipment_name: arg.equipment_name,
            device_category: arg.device_category,
            upload_time: arg.upload_time,
            battery_level: arg.battery_level,
            signal_strength: arg.signal_strength,
            alarm_status: arg.alarm_status,
            payload: arg.payload,
            dept_id: arg.dept_id,
            remark: arg.remark,
            create_time: DateTime::now().into(),
        }
    }
}
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EquipmentDaumUpdateDTO {
    pub id: Option<i64>,//主键ID（自增）
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
}
//    实现从 table对象 到 UpdateDTO对象的复制创建
impl From<EquipmentDaumUpdateDTO> for EquipmentData {
    fn from(arg: EquipmentDaumUpdateDTO) -> Self {
        Self {
            id: arg.id,
            equipment_number: arg.equipment_number,
            equipment_name: arg.equipment_name,
            device_category: arg.device_category,
            upload_time: arg.upload_time,
            battery_level: arg.battery_level,
            signal_strength: arg.signal_strength,
            alarm_status: arg.alarm_status,
            payload: arg.payload,
            dept_id: arg.dept_id,
            remark: arg.remark,
            create_time: None,
        }
    }
}
