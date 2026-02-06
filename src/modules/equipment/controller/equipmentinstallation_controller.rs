// installation_controller
use std::io::Cursor;
use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse, Responder};
use actix_multipart::Multipart;
use permit_lib::has_permit;
use futures_util::stream::StreamExt;
// 引用 数据表相关对象
use crate::modules::equipment::domain::dto::equipment_installation::{
    EquipmentInstallationAddDTO, EquipmentInstallationPageDTO, EquipmentInstallationUpdateDTO, EquipmentInstallationImportDTO,
};
use crate::modules::equipment::domain::table::tables::EquipmentInstallation;
use crate::modules::system::domain::vo::{PageVO, RespVO};
use crate::modules::system::service::{ServiceContext, CONTEXT};
use crate::modules::equipment_data::domain::dto::equipment_data::EquipmentDaumPageDTO;

/// installation controller
// 分页查询方法 传入EquipmentInstallationPageDTO,返回 Page<EquipmentInstallationVo> json
#[get("/installation/list")]
#[has_permit("equipment:installation:query")]
pub async fn page(query: web::Query<EquipmentInstallationPageDTO>) -> impl Responder {
    let data = CONTEXT.equipment_installation_service.page(&query).await;
    PageVO::from_result(&data).resp_json()
}
// Excel模板导出
#[get("/export")]
#[has_permit("equipment:installation:query")]
pub async fn template_export() -> impl Responder {
    let data = CONTEXT
        .equipment_installation_service
        .export_template()
        .await;
    match data {
        Ok(bytes) => {
            HttpResponse::Ok()
                .content_type("application/vnd.openxmlformats-officedocument.spreadsheetml.sheet")
                .append_header(("Content-Disposition", "attachment; filename=equipment_installation_template.xlsx"))
                .body(bytes)
        }
        Err(e) => {
            RespVO::<u64>::from_error_info(500, &format!("导出模板失败: {}", e)).resp_json()
        }
    }
}
// Excel导入
#[post("/import")]
#[has_permit("equipment:installation:add")]
pub async fn import_excel(mut payload: Multipart) -> impl Responder {
    let _import_list: Vec<()> = Vec::new();
    
    while let Some(item) = payload.next().await {
        let mut field = match item {
            Ok(f) => f,
            Err(e) => {
                return RespVO::<u64>::from_error_info(400, &format!("读取表单失败: {}", e)).resp_json();
            }
        };

        if field.name() == Some("file") {
            let mut bytes = Vec::new();
            while let Some(chunk) = field.next().await {
                let chunk = match chunk {
                    Ok(c) => c,
                    Err(e) => {
                        return RespVO::<u64>::from_error_info(400, &format!("读取文件失败: {}", e)).resp_json();
                    }
                };
                bytes.extend_from_slice(&chunk);
            }

            if bytes.is_empty() {
                return RespVO::<u64>::from_error_info(400, "上传文件为空").resp_json();
            }

            match CONTEXT.equipment_installation_service.import_excel(&bytes).await {
                Ok(count) => {
                    return RespVO::<u64>::judge(1, format!("成功导入 {} 条数据", count), "导入失败".to_string()).resp_json();
                }
                Err(e) => {
                    return RespVO::<u64>::from_error_info(500, &format!("导入失败: {}", e)).resp_json();
                }
            }
        }
    }

    RespVO::<u64>::from_error_info(400, "未找到上传文件").resp_json()
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
pub async fn remove(id: web::Path<i64>) -> impl Responder {
    let id_val = id.into_inner();
    let data = CONTEXT
        .equipment_installation_service
        .remove(&id_val.to_string())
        .await
        .unwrap();
    RespVO::<u64>::judge(data, "".to_string(), "删除失败！".to_string()).resp_json()
}
#[get("/status/list")]
// #[has_permit("equipment:installation:query")]
pub async fn status_list(query: web::Query<EquipmentDaumPageDTO>) -> impl Responder {
    let data = CONTEXT.equipment_installation_service.get_status_by_sql(&query).await;
    PageVO::from_result(&data).resp_json()
}
