use crate::modules::equipment::domain::table::tables::EquipmentInstallation;
use rbatis::object_id::ObjectId;
use rbatis::snowflake::new_snowflake_id;
use rbatis::rbdc::datetime::DateTime;
use rbatis::plugin::page::PageRequest;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EquipmentInstallationImportDTO {
    pub equipment_number: Option<String>,
    pub equipment_name: Option<String>,
    pub equipment_type: Option<String>,
    pub equipment_model_label: Option<String>,
    pub equipment_status: Option<String>,
    pub communication_means: Option<String>,
    pub addr: Option<String>,
    pub location_id: Option<String>,
    pub lower_equipment: Option<String>,
    pub project_num: Option<String>,
}

impl From<EquipmentInstallationImportDTO> for EquipmentInstallation {
    fn from(arg: EquipmentInstallationImportDTO) -> Self {
        Self {
            id: Some(new_snowflake_id() as i64),
            dept_id: None,
            remark: None,
            expand: None,
            spare1: None,
            equipment_number: arg.equipment_number,
            equipment_name: arg.equipment_name,
            installation_drawing: None,
            equipment_type: arg.equipment_type,
            dict_value: None,
            equipment_model_label: arg.equipment_model_label,
            equipment_status: arg.equipment_status,
            communication_means: arg.communication_means,
            addr: arg.addr,
            location_id: arg.location_id,
            lower_equipment: arg.lower_equipment,
            project_num: arg.project_num,
            create_by: None,
            create_time: DateTime::now().into(),
            update_by: None,
            update_time: None,
        }
    }
}
    
    
/// page DTO
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EquipmentInstallationPageDTO {
    #[serde(rename(deserialize = "pageNum"))]
    pub page_no: Option<u64>,
    #[serde(rename(deserialize = "pageSize"))]
    pub page_size: Option<u64>,
    pub create_by: Option<String>,//
    pub update_by: Option<String>,//
    pub dept_id: Option<i64>,//部门ID
    pub remark: Option<String>,//
    pub expand: Option<String>,//扩展(json)
    pub spare1: Option<String>,//
    pub equipment_number: Option<String>,//
    pub equipment_name: Option<String>,//
    pub installation_drawing: Option<String>,//
    pub equipment_type: Option<String>,//
    pub dict_value: Option<String>,//
    pub equipment_model_label: Option<String>,//
    pub equipment_status: Option<String>,//
    pub communication_means: Option<String>,//
    pub addr: Option<String>,//
    pub location_id: Option<String>,//
    pub lower_equipment: Option<String>,//
    pub project_num: Option<String>,//
}
//    实现从 dto对象 到 PageRequest
impl From<EquipmentInstallationPageDTO> for PageRequest {
    fn from(arg: EquipmentInstallationPageDTO) -> Self {
        PageRequest::new(arg.page_no.unwrap_or(1), arg.page_size.unwrap_or(10))
    }
}
//    实现从 &dto对象 到 PageRequest
impl From<&EquipmentInstallationPageDTO> for PageRequest {
    fn from(arg: &EquipmentInstallationPageDTO) -> Self {
        PageRequest::new(arg.page_no.unwrap_or(1), arg.page_size.unwrap_or(10))
    }
}
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EquipmentInstallationAddDTO {
    pub dept_id: Option<i64>,//部门ID
    pub remark: Option<String>,//
    pub expand: Option<String>,//扩展(json)
    pub spare1: Option<String>,//
    pub equipment_number: Option<String>,//
    pub equipment_name: Option<String>,//
    pub installation_drawing: Option<String>,//
    pub equipment_type: Option<String>,//
    pub dict_value: Option<String>,//
    pub equipment_model_label: Option<String>,//
    pub equipment_status: Option<String>,//
    pub communication_means: Option<String>,//
    pub addr: Option<String>,//
    pub location_id: Option<String>,//
    pub lower_equipment: Option<String>,//
    pub project_num: Option<String>,//
}
//    实现从 table对象 到 AddDTO对象的复制创建
impl From<EquipmentInstallationAddDTO> for EquipmentInstallation {
    fn from(arg: EquipmentInstallationAddDTO) -> Self {
        Self {
            id: Some(new_snowflake_id() as i64),
            dept_id: arg.dept_id,
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
            create_by: None,
            create_time: DateTime::now() .into(),
            update_by: None,
            update_time: None,
        }
    }
}
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EquipmentInstallationUpdateDTO {
    pub id: Option<i64>,//
    pub dept_id: Option<i64>,
    pub remark: Option<String>,
    pub expand: Option<String>,
    pub spare1: Option<String>,
    pub equipment_number: Option<String>,
    pub equipment_name: Option<String>,
    pub installation_drawing: Option<String>,
    pub equipment_type: Option<String>,
    pub dict_value: Option<String>,
    pub equipment_model_label: Option<String>,
    pub equipment_status: Option<String>,
    pub communication_means: Option<String>,
    pub addr: Option<String>,
    pub location_id: Option<String>,
    pub lower_equipment: Option<String>,
    pub project_num: Option<String>,
}
//    实现从 table对象 到 UpdateDTO对象的复制创建
impl From<EquipmentInstallationUpdateDTO> for EquipmentInstallation {
    fn from(arg: EquipmentInstallationUpdateDTO) -> Self {
        Self {
            id: arg.id,
            dept_id: arg.dept_id,
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
            create_by: None,
            create_time: None,
            update_by: None,
            update_time: DateTime::now() .into(),
        }
    }
}
