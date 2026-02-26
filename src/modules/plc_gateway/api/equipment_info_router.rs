use axum::{
    Router,
    extract::{Extension, Path, Query},
    response::Json,
    routing::{delete, get, post, put},
};
use uuid::Uuid;

use crate::entity;
use crate::modules::plc_gateway::unified::response::{error_response, success_response};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    QuerySelect, Set,
}; // Added PaginatorTrait and QuerySelect
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::sync::Arc;

/* ---------- 路由挂接 ---------- */
pub fn equipment_routes() -> Router {
    Router::new()
        .route("/equipInfo/all", get(all))
        .route("/equipInfo/page", get(page))
        .route("/equipInfo/{id}", get(single))
        .route("/equipInfo", post(create))
        .route("/equipInfo/{id}", put(update))
        .route("/equipInfo/{id}", delete(remove))
}

/* ---------- dto ---------- */
#[derive(Deserialize, Debug)]
pub struct PageParams {
    start: Option<u64>,
    end: Option<u64>,
    page_size: Option<u64>,
}

#[derive(Serialize)]
pub struct PageResp {
    list: Vec<Resp>,
    total: u64,
    start: u64,
    end: u64,
    page_size: u64,
}

#[derive(Serialize)]
struct Resp {
    pub id: uuid::Uuid,
    pub equipment_number: String,
    pub project_num: Option<String>,
    pub equipment_name: Option<String>,
    pub equipment_model: Option<String>,
    pub equipment_model_label: Option<String>,
    pub equipment_type: Option<String>,
    pub equipment_status: Option<String>,
    pub manufacturer_name: Option<String>,
    pub dict_value: Option<String>,
    pub communication_means: Option<String>,
    pub addr: Option<String>,
    pub location_id: Option<String>,
    pub installation_drawing: Option<String>,
    pub remark: Option<String>,
    pub ip: Option<String>,
    pub sim: Option<String>,
}

#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct UpsertReq {
    equipment_number: Option<String>,
    project_num: Option<String>,
    equipment_name: Option<String>,
    equipment_model: Option<String>,
    equipment_model_label: Option<String>,
    equipment_type: Option<String>,
    equipment_status: Option<String>,
    manufacturer_name: Option<String>,
    dict_value: Option<String>,
    communication_means: Option<String>,
    addr: Option<String>,
    location_id: Option<String>,
    installation_drawing: Option<String>,
    remark: Option<String>,
     ip: Option<String>,
     sim: Option<String>,
}

/* ---------- 1. 查所有 ---------- */
async fn all(db: Extension<Arc<DatabaseConnection>>) -> Json<Value> {
    let list = entity::equipment_info::Entity::find()
        .all(db.as_ref())
        .await
        .map_err(|e| error_response(e.to_string(), Some(500)))
        .unwrap();
    let resp = json!(
        list.into_iter()
            .map(|m| Resp {
                id: m.id,
                equipment_number: m.equipment_number,
                project_num: m.project_num,
                equipment_name: m.equipment_name,
                equipment_model: m.equipment_model,
                equipment_model_label: m.equipment_model_label,
                equipment_type: m.equipment_type,
                equipment_status: m.equipment_status,
                manufacturer_name: m.manufacturer_name,
                dict_value: m.dict_value,
                communication_means: m.communication_means,
                addr: m.addr,
                location_id: m.location_id,
                installation_drawing: m.installation_drawing,
                remark: m.remark,
                ip: m.ip,
                sim: m.sim
            })
            .collect::<Vec<_>>()
    );
    success_response(resp, None)
}

/* ---------- 分页查询 ---------- */
async fn page(
    db: Extension<Arc<DatabaseConnection>>,
    Query(params): Query<PageParams>,
) -> Json<Value> {
    // 参数处理，设置默认值
    let start = params.start.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(30);
    let end = params.end.unwrap_or(start + page_size - 1);

    // 验证参数合法性
    if start < 1 || page_size < 1 || end < start {
        return error_response(
            "参数错误：start和pageSize必须大于0，且end必须大于等于start".to_string(),
            Some(400),
        );
    }

    // 计算偏移量
    let offset = (start - 1) * page_size;

    // 查询总数
    let total = entity::equipment_info::Entity::find()
        .count(db.as_ref())
        .await
        .map_err(|e| error_response(e.to_string(), Some(500)))
        .unwrap();

    // 分页查询数据
    let list = entity::equipment_info::Entity::find()
        .offset(offset)
        .limit(page_size)
        .all(db.as_ref())
        .await
        .map_err(|e| error_response(e.to_string(), Some(500)))
        .unwrap();

    // 转换响应格式
    let data_list = list
        .into_iter()
        .map(|m| Resp {
            id: m.id,
            equipment_number: m.equipment_number,
            project_num: m.project_num,
            equipment_name: m.equipment_name,
            equipment_model: m.equipment_model,
            equipment_model_label: m.equipment_model_label,
            equipment_type: m.equipment_type,
            equipment_status: m.equipment_status,
            manufacturer_name: m.manufacturer_name,
            dict_value: m.dict_value,
            communication_means: m.communication_means,
            addr: m.addr,
            location_id: m.location_id,
            installation_drawing: m.installation_drawing,
            remark: m.remark,
            ip: m.ip,
            sim : m.sim
        })
        .collect::<Vec<_>>();

    // 构建分页响应
    let page_resp = PageResp {
        list: data_list,
        total,
        start,
        end: std::cmp::min(end, total), // Ensure end doesn't exceed total
        page_size,
    };

    success_response(json!(page_resp), None)
}

/* ---------- 2. 查单条 ---------- */
async fn single(db: Extension<Arc<DatabaseConnection>>, Path(id): Path<String>) -> Json<Value> {
    let id_uuid = Uuid::parse_str(&id)
        .map_err(|_| error_response("无效的 UUID 格式".to_string(), Some(400)))
        .unwrap();
    let m = entity::equipment_info::Entity::find()
        .filter(entity::equipment_info::Column::Id.eq(id_uuid))
        .one(db.as_ref())
        .await
        .map_err(|e| error_response(e.to_string(), Some(500)))
        .unwrap()
        .ok_or_else(|| error_response("设备信息不存在".to_string(), Some(404)))
        .unwrap();
    let data = Resp {
        id: m.id,
        equipment_number: m.equipment_number,
        project_num: m.project_num,
        equipment_name: m.equipment_name,
        equipment_model: m.equipment_model,
        equipment_model_label: m.equipment_model_label,
        equipment_type: m.equipment_type,
        equipment_status: m.equipment_status,
        manufacturer_name: m.manufacturer_name,
        dict_value: m.dict_value,
        communication_means: m.communication_means,
        addr: m.addr,
        location_id: m.location_id,
        installation_drawing: m.installation_drawing,
        remark: m.remark,
        ip: m.ip,
        sim: m.sim
    };
    success_response(json!(data), None)
}

/* ---------- 3. 新增 ---------- */
async fn create(db: Extension<Arc<DatabaseConnection>>, Json(req): Json<UpsertReq>) -> Json<Value> {
    println!("Request body: {:?}", req);
    let id_uuid = Uuid::new_v4();
    let am = entity::equipment_info::ActiveModel {
        id: Set(id_uuid), // 使用路径中的 UUID
        equipment_number: Set(req.equipment_number.unwrap_or_default()),
        project_num: Set(req.project_num),
        equipment_name: Set(req.equipment_name),
        equipment_model: Set(req.equipment_model),
        equipment_model_label: Set(req.equipment_model_label),
        equipment_type: Set(req.equipment_type),
        equipment_status: Set(req.equipment_status),
        manufacturer_name: Set(req.manufacturer_name),
        dict_value: Set(req.dict_value),
        communication_means: Set(req.communication_means),
        addr: Set(req.addr),
        location_id: Set(req.location_id),
        installation_drawing: Set(req.installation_drawing),
        remark: Set(req.remark),
        ..Default::default()
    };
    am.insert(db.as_ref())
        .await
        .map_err(|e| {
            println!("Insert error: {:?}", e);
            match e {
                sea_orm::DbErr::Query(sea_orm::error::RuntimeErr::SqlxError(
                    sqlx::Error::Database(db_err),
                )) if db_err.constraint().is_some() => {
                    error_response("设备编号重复".to_string(), Some(409))
                }
                _ => error_response(e.to_string(), Some(500)),
            }
        })
        .unwrap();

    success_response(json!({}), None)
}

/* ---------- 4. 更新 ---------- */
async fn update(
    db: Extension<Arc<DatabaseConnection>>,
    Path(id): Path<String>,
    Json(req): Json<UpsertReq>,
) -> Json<Value> {
    let id_uuid = Uuid::parse_str(&id)
        .map_err(|_| error_response("无效的 UUID 格式".to_string(), Some(400)))
        .unwrap();
    let model = entity::equipment_info::Entity::find()
        .filter(entity::equipment_info::Column::Id.eq(id_uuid))
        .one(db.as_ref())
        .await
        .map_err(|e| error_response(e.to_string(), Some(500)))
        .unwrap()
        .ok_or_else(|| error_response("设备信息不存在".to_string(), Some(404)))
        .unwrap();
    let mut am: entity::equipment_info::ActiveModel = model.into();
    am.equipment_number = Set(req.equipment_number.unwrap_or_default());
    am.project_num = Set(req.project_num);
    am.equipment_name = Set(req.equipment_name);
    am.equipment_model = Set(req.equipment_model);
    am.equipment_model_label = Set(req.equipment_model_label);
    am.equipment_type = Set(req.equipment_type);
    am.equipment_status = Set(req.equipment_status);
    am.manufacturer_name = Set(req.manufacturer_name);
    am.dict_value = Set(req.dict_value);
    am.communication_means = Set(req.communication_means);
    am.addr = Set(req.addr);
    am.location_id = Set(req.location_id);
    am.installation_drawing = Set(req.installation_drawing);
    am.remark = Set(req.remark);

    am.update(db.as_ref())
        .await
        .map_err(|e| {
            println!("Update error: {:?}", e);
            match e {
                sea_orm::DbErr::Query(sea_orm::error::RuntimeErr::SqlxError(
                    sqlx::Error::Database(db_err),
                )) if db_err.constraint().is_some() => {
                    error_response("设备编号重复".to_string(), Some(409))
                }
                _ => error_response(e.to_string(), Some(500)),
            }
        })
        .unwrap();

    success_response(json!({}), None)
}

/* ---------- 5. 删除 ---------- */
async fn remove(db: Extension<Arc<DatabaseConnection>>, Path(id): Path<String>) -> Json<Value> {
    let id_uuid = Uuid::parse_str(&id)
        .map_err(|_| error_response("无效的 UUID 格式".to_string(), Some(400)))
        .unwrap();
    let res = entity::equipment_info::Entity::delete_many()
        .filter(entity::equipment_info::Column::Id.eq(id_uuid))
        .exec(db.as_ref())
        .await
        .map_err(|e| error_response(e.to_string(), Some(500)))
        .unwrap();
    if res.rows_affected > 0 {
        success_response(json!({}), None)
    } else {
        error_response("设备信息不存在".to_string(), Some(404))
    }
}
