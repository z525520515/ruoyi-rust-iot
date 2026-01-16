use rbatis::rbdc::datetime::DateTime;
//   在tables.rs 增加
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct EquipmentInstallation {
    pub id: Option<i64>,                       //
    pub create_by: Option<String>,             //
    pub create_time: Option<DateTime>,         //
    pub update_by: Option<String>,             //
    pub update_time: Option<DateTime>,         //
    pub dept_id: Option<i64>,                  // 部门ID
    pub remark: Option<String>,                //
    pub expand: Option<String>,                // 扩展(json)
    pub spare1: Option<String>,                //
    pub equipment_number: Option<String>,      //
    pub equipment_name: Option<String>,        //
    pub installation_drawing: Option<String>,  //
    pub equipment_type: Option<String>,        //
    pub dict_value: Option<String>,            //
    pub equipment_model_label: Option<String>, //
    pub equipment_status: Option<String>,      //
    pub communication_means: Option<String>,   //
    pub addr: Option<String>,                  //
    pub location_id: Option<String>,           //
    pub lower_equipment: Option<String>,       //
    pub project_num: Option<String>,           //
}
