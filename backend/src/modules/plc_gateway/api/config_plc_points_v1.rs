use crate::modules::plc_gateway::api::config_util::call_device;
use crate::modules::plc_gateway::unified::response::error_response;
use axum::{extract::Path, response::Json};
use serde::Deserialize;
use serde_json::{Value, json};

#[derive(Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct PlcModbusNodePointReq {
    pub id: Option<u16>,
    pub name: String,
    pub addr: String,
    pub datatype: String,
    pub protocol: String,
    /* ===== 选填，全部用 Option ===== */
    pub reg_swap: Option<String>,
    pub param_k: Option<u16>,
    pub multiples: Option<u16>,
    pub precision: Option<u16>,
    pub effective: Option<u16>,
    pub reg_num: Option<u16>,
    pub bit_offset: Option<u16>,
    pub table: Option<String>,
    pub field: Option<String>,
    pub remark: Option<String>,
    // extra 字段
    pub trigger: Option<String>,
    pub condition: Option<String>,
    pub value: Option<u16>,
}

/// 提供添加分组后测点的配置，由于缺乏数据，目前只测试了协议类型为"modbus_tcp"的添加测试，后续如果有了其他协议类型，需要更新测试这里的函数
/// gid 是 pcl的uuid
/// 请求示例：/plc/points/10.0.0.5/7clxbl7h
pub async fn add_plc_config_group_points(
    Path((ip, uuid)): Path<(String, String)>,
    Json(params): Json<PlcModbusNodePointReq>,
) -> Json<Value> {
    log::info!("add plc modbus points: ip={}, params={:#?}", ip, params);
    let extra = json!({
        "trigger": params.trigger.as_deref().unwrap_or(""),
        "condition": params.condition.as_deref().unwrap_or(""),
        "value": params.value.unwrap_or(0),
    });
    // 数据校验
    let datatype = &params.datatype;
    let alt_types = vec!["UINT16", "INT16", "UINT32", "INT32", "FLOAT"];
    let is_alt_type = alt_types.contains(&datatype.as_str());

    // trigger 字段校验
    if let Some(trigger) = &params.trigger {
        let valid_triggers = vec!["onChange", "onValue"];
        if !valid_triggers.contains(&trigger.as_str()) {
            return error_response(
                "trigger must be 'onChange' or 'onValue'".to_string(),
                Some(400),
            );
        }
    }

    // condition 字段校验
    if let Some(condition) = &params.condition {
        let valid_conditions = vec!["eq", "ne", "ge", "gt", "le", "lt"];
        if !valid_conditions.contains(&condition.as_str()) {
            return error_response(
                "condition must be one of: eq, ne, ge, gt, le, lt".to_string(),
                Some(400),
            );
        }
    }

    match datatype.to_uppercase().as_str() {
        "BIT" => {
            if params.reg_swap.is_none() {
                return error_response("regSwap is required for BIT".to_string(), Some(400));
            }
            if params.param_k.is_none() {
                return error_response("paramK is required for BIT".to_string(), Some(400));
            }
        }
        "BOOL" => {}
        "UINT16" | "INT16" | "UINT32" | "INT32" | "FLOAT" => {
            if params.reg_swap.is_none() {
                return error_response("regSwap is required".to_string(), Some(400));
            }
            if params.multiples.is_none() {
                return error_response("multiples is required".to_string(), Some(400));
            }
            if params.precision.is_some() && params.effective.is_some() {
                return error_response(
                    "cannot have both precision and effective".to_string(),
                    Some(400),
                );
            }
            if params.precision.is_none() && params.effective.is_none() {
                return error_response(
                    "must provide either precision or effective".to_string(),
                    Some(400),
                );
            }
        }
        "STRING" => {
            if params.reg_swap.is_none() {
                return error_response("regSwap is required for STRING".to_string(), Some(400));
            }
            if params.reg_num.is_none() {
                return error_response("regNum is required for STRING".to_string(), Some(400));
            }
        }
        _ => {
            return error_response("unknown datatype".to_string(), Some(400));
        }
    }

    // 构建 forward 对象
    let mut point = json!({
        "id": 0,
        "name": params.name,
        "datatype": params.datatype,
        "addr": params.addr,
        "regSwap": params.reg_swap.as_deref().unwrap_or("AB"),
        "paramK": params.param_k.unwrap_or(0),
        "multiples": params.multiples.unwrap_or(1),
        "table": params.table.as_deref().unwrap_or(""),
        "field": params.field.as_deref().unwrap_or(""),
        "remark": params.remark.as_deref().unwrap_or(""),
        "extra": extra.to_string(),
        "bitOffset": params.bit_offset.unwrap_or(0),
        "protocol": params.protocol,
    });

    if is_alt_type {
        if let Some(precision) = params.precision {
            point["precision"] = json!(precision);
        }
        if let Some(effective) = params.effective {
            point["effective"] = json!(effective);
        }
    } else {
        point["precision"] = json!(params.precision.unwrap_or(3));
    }

    if let Some(reg_num) = params.reg_num {
        point["regNum"] = json!(reg_num);
    }

    let forward = json!([point]);

    let url = format!("http://{}:80/plc/api/v1/group/{}/node", &ip, uuid);
    call_device(url, forward, Some("POST")).await
}

/// 获取分组下的所有测点, gid 是group的uuid
pub async fn get_plc_config_group_points(Path((ip, uuid)): Path<(String, String)>) -> Json<Value> {
    let url = format!("http://{}:80/plc/api/v1/group/{}/node", &ip, uuid);
    call_device(url, json!([]), Some("GET")).await
}

pub async fn del_plc_config_group_points(
    Path((ip, uuid, id)): Path<(String, String, String)>,
) -> Json<Value> {
    let url = format!("http://{}:80/plc/api/v1/group/{}/node/{}", &ip, uuid, id);
    call_device(url, json!([]), Some("DELETE")).await
}

pub async fn update_plc_config_group_points(
    Path((ip, uuid)): Path<(String, String)>,
    Json(params): Json<PlcModbusNodePointReq>,
) -> Json<Value> {
    log::info!(
        "update plc config group points: ip={}, params={:#?}",
        ip,
        params
    );

    // id 字段校验,必须存在,且必须是u16类型
    if params.id.is_none(){
        return error_response("id is required".to_string(), Some(400));
    }

    let id = params.id.unwrap_or(0);
    let extra = json!({
        "trigger": params.trigger.as_deref().unwrap_or(""),
        "condition": params.condition.as_deref().unwrap_or(""),
        "value": params.value.unwrap_or(0),
    });
    // 数据校验
    let datatype = &params.datatype;
    let alt_types = vec!["UINT16", "INT16", "UINT32", "INT32", "FLOAT"];
    let is_alt_type = alt_types.contains(&datatype.as_str());

    // trigger 字段校验
    if let Some(trigger) = &params.trigger {
        let valid_triggers = vec!["onChange", "onValue"];
        if !valid_triggers.contains(&trigger.as_str()) {
            return error_response(
                "trigger must be 'onChange' or 'onValue'".to_string(),
                Some(400),
            );
        }
    }

    // condition 字段校验
    if let Some(condition) = &params.condition {
        let valid_conditions = vec!["eq", "ne", "ge", "gt", "le", "lt"];
        if !valid_conditions.contains(&condition.as_str()) {
            return error_response(
                "condition must be one of: eq, ne, ge, gt, le, lt".to_string(),
                Some(400),
            );
        }
    }

    match datatype.to_uppercase().as_str() {
        "BIT" => {
            if params.reg_swap.is_none() {
                return error_response("regSwap is required for BIT".to_string(), Some(400));
            }
            if params.param_k.is_none() {
                return error_response("paramK is required for BIT".to_string(), Some(400));
            }
        }
        "BOOL" => {}
        "UINT16" | "INT16" | "UINT32" | "INT32" | "FLOAT" => {
            if params.reg_swap.is_none() {
                return error_response("regSwap is required".to_string(), Some(400));
            }
            if params.multiples.is_none() {
                return error_response("multiples is required".to_string(), Some(400));
            }
            if params.precision.is_some() && params.effective.is_some() {
                return error_response(
                    "cannot have both precision and effective".to_string(),
                    Some(400),
                );
            }
            if params.precision.is_none() && params.effective.is_none() {
                return error_response(
                    "must provide either precision or effective".to_string(),
                    Some(400),
                );
            }
        }
        "STRING" => {
            if params.reg_swap.is_none() {
                return error_response("regSwap is required for STRING".to_string(), Some(400));
            }
            if params.reg_num.is_none() {
                return error_response("regNum is required for STRING".to_string(), Some(400));
            }
        }
        _ => {
            return error_response("unknown datatype".to_string(), Some(400));
        }
    }

    // 构建 forward 对象
    let mut point = json!({
        "id": 0,
        "name": params.name,
        "datatype": params.datatype,
        "addr": params.addr,
        "regSwap": params.reg_swap.as_deref().unwrap_or("AB"),
        "paramK": params.param_k.unwrap_or(0),
        "multiples": params.multiples.unwrap_or(1),
        "table": params.table.as_deref().unwrap_or(""),
        "field": params.field.as_deref().unwrap_or(""),
        "remark": params.remark.as_deref().unwrap_or(""),
        "extra": extra.to_string(),
        "bitOffset": params.bit_offset.unwrap_or(0),
        "protocol": params.protocol,
    });

    if is_alt_type {
        if let Some(precision) = params.precision {
            point["precision"] = json!(precision);
        }
        if let Some(effective) = params.effective {
            point["effective"] = json!(effective);
        }
    } else {
        point["precision"] = json!(params.precision.unwrap_or(3));
    }

    if let Some(reg_num) = params.reg_num {
        point["regNum"] = json!(reg_num);
    }

    let forward = json!([point]);

    let url = format!("http://{}:80/plc/api/v1/group/{}/node/{}", &ip, uuid, id);
    call_device(url, forward, Some("POST")).await
}
