use crate::modules::equipment_data::domain::table::tables::EquipmentData;
use crate::modules::equipment_data::domain::dto::equipment_data::EquipmentDaumPageDTO;

// 保持基础增删改查针对原表
crud!(EquipmentData {});

// 修改分页查询：将数据源从表改为视图 `view_device_status`
impl_select_page!(EquipmentData{select_page(limit_sql:&str, dto: &EquipmentDaumPageDTO) =>
    "where 1=1
    if dto.equipmentNumber != '' && dto.equipmentNumber != None:
      ` and equipment_number like #{'%%'+dto.equipmentNumber+'%%'}`
    if dto.equipment_name != '' && dto.equipment_name != None:
      ` and equipment_name like #{'%%'+dto.equipment_name+'%%'}`
    if dto.deviceCategory != '' && dto.deviceCategory != None:
      ` and device_category = #{dto.deviceCategory}`
    if dto.onlineState != '' && dto.onlineState != None:
      ` and online_state_calc = #{dto.onlineState}`
    if !sql.contains('count'):
     ` order by upload_time desc`"
} );