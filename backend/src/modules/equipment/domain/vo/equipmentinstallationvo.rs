use rbatis::rbdc::datetime::DateTime;
use crate::modules::equipment::domain::table::tables::EquipmentInstallation;
//  Vo 对象 可以根据需要增加交互所需的属性
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EquipmentInstallationVo {
    pub id: Option<i64>, // 
    pub create_by: Option<String>, // 
    pub create_time: Option<DateTime>, // 
    pub update_by: Option<String>, // 
    pub update_time: Option<DateTime>, // 
    pub dept_id: Option<i64>, // 部门ID
    pub dept_name: Option<String>, // 部门名称（关联 sys_dept 回填）
    pub remark: Option<String>, // 
    pub expand: Option<String>, // 扩展(json)
    pub spare1: Option<String>, // 
    pub equipment_number: Option<String>, // 
    pub equipment_name: Option<String>, // 
    pub installation_drawing: Option<String>, // 
    pub equipment_type: Option<String>, // 
    pub dict_value: Option<String>, // 
    pub equipment_model_label: Option<String>, // 
    pub equipment_status: Option<String>, // 
    pub communication_means: Option<String>, // 
    pub addr: Option<String>, // 
    pub location_id: Option<String>, // 
    pub lower_equipment: Option<String>, // 
    pub project_num: Option<String>, // 
}
//    实现从 table对象 到 Vo对象的复制创建
impl From<EquipmentInstallation> for EquipmentInstallationVo {
    fn from(arg: EquipmentInstallation) -> Self {
        Self {
            id: arg.id,
            create_by: arg.create_by,
            create_time: arg.create_time,
            update_by: arg.update_by,
            update_time: arg.update_time,
            dept_id: arg.dept_id,
            dept_name: None,
            remark: arg.remark,
            expand: arg.expand,
            spare1: arg.spare1,
            equipment_number: arg.equipment_number,
            equipment_name: arg.equipment_name,
            installation_drawing: arg.installation_drawing,
            equipment_type: arg.equipment_type,
            dict_value: arg.dict_value,
            equipment_model_label: arg.equipment_model_label,
            equipment_status: arg.equipment_status,
            communication_means: arg.communication_means,
            addr: arg.addr,
            location_id: arg.location_id,
            lower_equipment: arg.lower_equipment,
            project_num: arg.project_num,
        }
    }
}
