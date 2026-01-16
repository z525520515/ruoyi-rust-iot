use crate::modules::equipment::domain::table::tables::EquipmentInstallation;
use crate::modules::equipment::domain::dto::equipment_installation::EquipmentInstallationPageDTO;
crud!(EquipmentInstallation {});
impl_select_page!(EquipmentInstallation{select_page(limit_sql:&str,dto: &EquipmentInstallationPageDTO) =>
    "`where 1=1 `
    if dto.createBy != '' && dto.createBy != None:
      ` and create_by like #{'%'+dto.createBy+'%'}`
    if dto.updateBy != '' && dto.updateBy != None:
      ` and update_by like #{'%'+dto.updateBy+'%'}`
    if dto.remark != '' && dto.remark != None:
      ` and remark like #{'%'+dto.remark+'%'}`
    if dto.spare1 != '' && dto.spare1 != None:
      ` and spare1 like #{'%'+dto.spare1+'%'}`
    if dto.equipmentNumber != '' && dto.equipmentNumber != None:
      ` and equipment_number like #{'%'+dto.equipmentNumber+'%'}`
    if dto.equipmentName != '' && dto.equipmentName != None:
      ` and equipment_name like #{'%'+dto.equipmentName+'%'}`
    if dto.installationDrawing != '' && dto.installationDrawing != None:
      ` and installation_drawing like #{'%'+dto.installationDrawing+'%'}`
    if dto.equipmentType != '' && dto.equipmentType != None:
      ` and equipment_type like #{'%'+dto.equipmentType+'%'}`
    if dto.dictValue != '' && dto.dictValue != None:
      ` and dict_value like #{'%'+dto.dictValue+'%'}`
    if dto.equipmentModelLabel != '' && dto.equipmentModelLabel != None:
      ` and equipment_model_label like #{'%'+dto.equipmentModelLabel+'%'}`
    if dto.equipmentStatus != '' && dto.equipmentStatus != None:
      ` and equipment_status like #{'%'+dto.equipmentStatus+'%'}`
    if dto.communicationMeans != '' && dto.communicationMeans != None:
      ` and communication_means like #{'%'+dto.communicationMeans+'%'}`
    if dto.addr != '' && dto.addr != None:
      ` and addr like #{'%'+dto.addr+'%'}`
    if dto.locationId != '' && dto.locationId != None:
      ` and location_id like #{'%'+dto.locationId+'%'}`
    if dto.lowerEquipment != '' && dto.lowerEquipment != None:
      ` and lower_equipment like #{'%'+dto.lowerEquipment+'%'}`
    if dto.projectNum != '' && dto.projectNum != None:
      ` and project_num like #{'%'+dto.projectNum+'%'}`
    if !sql.contains('count'):
     ` order by create_time desc`"} );