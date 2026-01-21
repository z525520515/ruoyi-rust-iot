use rbatis::rbdc::datetime::DateTime;
//   在tables.rs 增加
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct EquipmentData {
    pub id: Option<i64>, // 主键ID（自增）
    pub equipment_number: Option<String>, // 设备唯一编号 (对应原表 equipment_number 或 deviceID)
    pub equipment_name: Option<String>, // 设备名称
    pub device_category: Option<String>, // 设备分类标签 (electric:电表, water:水表, terminal:终端, alarm:报警器, wireless:无线终端)
    pub upload_time: Option<DateTime>, // 统一上报/触发时间 (时序索引核心字段)
    pub battery_level: Option<f64>, // 剩余电量百分比/数值
    pub signal_strength: Option<i32>, // 信号强度 (对应原表 rssi 或 signal_strength)
    pub alarm_status: Option<i32>, // 报警状态标识 (0:正常, 1:报警或异常)
    pub payload: Option<serde_json::Value>, // 核心测量数据 (JSONB 格式，存储各设备特有的电压、电流、开关状态等指标)
    pub dept_id: Option<i32>, // 部门ID，用于权限管理
    pub remark: Option<String>, // 备注描述
    pub create_time: Option<DateTime>, // 数据入库时间 (系统自动生成)
    pub online_state: Option<String>,
    pub online_state_calc: Option<String>,
}
