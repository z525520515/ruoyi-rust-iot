use rbatis::rbdc::datetime::DateTime;
use crate::modules::equipment_data::domain::table::tables::EquipmentData;
//  Vo 对象 可以根据需要增加交互所需的属性
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EquipmentDaumVo {
    pub id: Option<i64>, // 主键ID（自增）
    pub equipment_number: Option<String>, // 设备唯一编号 (对应原表 equipment_number 或 deviceID)
    pub equipment_name: Option<String>, // 设备名称
    pub device_category: Option<String>, // 设备分类标签 (electric:电表, water:水表, terminal:终端, alarm:报警器, wireless:无线终端)
    pub upload_time: Option<DateTime>, // 统一上报/触发时间 (时序索引核心字段)
    pub battery_level: Option<f64>, // 剩余电量百分比/数值
    pub signal_strength: Option<i32>, // 信号强度 (对应原表 rssi 或 signal_strength)
    pub alarm_status: Option<i32>, // 报警状态标识 (0:正常, 1:报警或异常)
    pub payload: Option<serde_json::Value>,
    pub dept_id: Option<i32>,
    pub dept_name: Option<String>, // 部门名称（关联 sys_dept 回填）
    pub remark: Option<String>,
    pub create_time: Option<DateTime>,
}
//    实现从 table对象 到 Vo对象的复制创建
impl From<EquipmentData> for EquipmentDaumVo {
    fn from(arg: EquipmentData) -> Self {
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
            dept_name: None,
            remark: arg.remark,
            create_time: arg.create_time,
        }
    }
}
