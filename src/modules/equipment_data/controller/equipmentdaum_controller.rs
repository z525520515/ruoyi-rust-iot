// daum_controller
use actix_web::{get, post, put, delete, web,HttpRequest, Responder};
use permit_lib::has_permit;
// 引用 数据表相关对象
use crate::modules::equipment_data::domain::table::tables::EquipmentData;
use crate::modules::equipment_data::domain::dto::equipment_data::{EquipmentDaumAddDTO, EquipmentDaumUpdateDTO, EquipmentDaumPageDTO};
use crate::modules::system::service::CONTEXT;
use crate::modules::system::domain::vo::{PageVO, RespVO};

/// daum controller
// 分页查询方法 传入EquipmentDaumPageDTO,返回 Page<EquipmentDaumVo> json
#[get("/daum/list")]
// #[has_permit("equipment_data:daum:query")]
pub async fn page(query: web::Query<EquipmentDaumPageDTO>) -> impl Responder {
    let data = CONTEXT.equipment_daum_service.page(&query).await;
    PageVO::from_result(&data).resp_json()
}
// 根据id返回一条
#[get("/daum/{id}")]
#[has_permit("equipment_data:daum:query")]
pub async fn detail(id: web::Path<i64>) -> impl Responder {
    let id = id.into_inner();
    let row_vo = CONTEXT.equipment_daum_service.detail(&id.to_string()).await;
    RespVO::from_result(&row_vo).resp_json()
}
// post add 一条 
#[post("/daum")]
#[has_permit("equipment_data:daum:add")]
pub async fn add(arg: web::Json<EquipmentDaumAddDTO>) -> impl Responder {
    let data = EquipmentData::from(arg.0);
    let res = EquipmentData::from(data);
    let data = CONTEXT.equipment_daum_service.add(&res).await.unwrap();
    RespVO::<u64>::judge(data, "".to_string(), "保存失败！".to_string()).resp_json()
}
// put update 一条 
#[put("/daum")]
#[has_permit("equipment_data:daum:edit")]
pub async fn update(arg: web::Json<EquipmentDaumUpdateDTO>) -> impl Responder {
    let data = EquipmentData::from(arg.0);
    let data = CONTEXT.equipment_daum_service.update(data).await.unwrap();
    RespVO::<u64>::judge(data, "".to_string(), "更新失败！".to_string()).resp_json()
}
// delete 删除 一条 
#[delete("/daum/{id}")]
#[has_permit("equipment_data:daum:remove")]
pub async fn remove(id: web::Path<String>) -> impl Responder {
    let id = id.into_inner();
    let data = CONTEXT
        .equipment_daum_service
        .remove(&id)
        .await.unwrap();
    RespVO::<u64>::judge(data, "".to_string(), "删除失败！".to_string()).resp_json()
}
/// 获取设备状态统计（在线/离线数量）
#[get("/daum/stats")]
#[has_permit("equipment_data:daum:query")]
pub async fn stats() -> impl Responder {
    let res = CONTEXT.equipment_daum_service.get_status_summary().await;
    RespVO::from_result(&res).resp_json()
}
