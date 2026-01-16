// installation_controller
use actix_web::{delete, get, post, put, web, HttpRequest, Responder};
use permit_lib::has_permit;
// 引用 数据表相关对象
use crate::modules::equipment::domain::dto::equipment_installation::{
    EquipmentInstallationAddDTO, EquipmentInstallationPageDTO, EquipmentInstallationUpdateDTO,
};
use crate::modules::equipment::domain::table::tables::EquipmentInstallation;
use crate::modules::system::domain::vo::{PageVO, RespVO};
use crate::modules::system::service::{ServiceContext, CONTEXT};

/// installation controller
// 分页查询方法 传入EquipmentInstallationPageDTO,返回 Page<EquipmentInstallationVo> json
#[get("/installation/list")]
#[has_permit("equipment:installation:query")]
pub async fn page(query: web::Query<EquipmentInstallationPageDTO>) -> impl Responder {
    let data = CONTEXT.equipment_installation_service.page(&query).await;
    PageVO::from_result(&data).resp_json()
}
// 根据id返回一条
#[get("/installation/{id}")]
#[has_permit("equipment:installation:query")]
pub async fn detail(id: web::Path<i64>) -> impl Responder {
    let id = id.into_inner();
    let row_vo = CONTEXT
        .equipment_installation_service
        .detail(&id.to_string())
        .await;
    RespVO::from_result(&row_vo).resp_json()
}
// post add 一条
#[post("/installation")]
#[has_permit("equipment:installation:add")]
pub async fn add(arg: web::Json<EquipmentInstallationAddDTO>) -> impl Responder {
    let mut data = EquipmentInstallation::from(arg.0);
    // 创建记录人员
    data.create_by = Some(crate::web_data::get_user_name());
    // 根据需求可以使给id填业务流水号
    //let a = ServiceContext::get_table_maxid("id","equipment_installation").await ;
    //data.id= Some(a);
    let res = EquipmentInstallation::from(data);
    let data = CONTEXT
        .equipment_installation_service
        .add(&res)
        .await
        .unwrap();
    RespVO::<u64>::judge(data, "".to_string(), "保存失败！".to_string()).resp_json()
}
// put update 一条
#[put("/installation")]
#[has_permit("equipment:installation:edit")]
pub async fn update(arg: web::Json<EquipmentInstallationUpdateDTO>) -> impl Responder {
    let mut data = EquipmentInstallation::from(arg.0);
    data.update_by = Some(crate::web_data::get_user_name());
    let data = CONTEXT
        .equipment_installation_service
        .update(data)
        .await
        .unwrap();
    RespVO::<u64>::judge(data, "".to_string(), "更新失败！".to_string()).resp_json()
}
// delete 删除 一条
#[delete("/installation/{id}")]
#[has_permit("equipment:installation:remove")]
pub async fn remove(id: web::Path<String>) -> impl Responder {
    let id = id.into_inner();
    let data = CONTEXT
        .equipment_installation_service
        .remove(&id)
        .await
        .unwrap();
    RespVO::<u64>::judge(data, "".to_string(), "删除失败！".to_string()).resp_json()
}
