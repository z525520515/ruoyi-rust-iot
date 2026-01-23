use crate::modules::system::domain::vo::{RespVO, CODE_FAIL};
use crate::modules::system::service::CONTEXT;
use actix_web::{post, web, Responder};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug, Serialize, Deserialize)]
pub struct GatewayInvokeVO {
    pub device_hash: String,
    pub protocol_type: String,
    pub action: String,
    pub method: String,
    pub params: Value,
    pub stack_name: String,
}

#[post("/gateway/invoke")]
pub async fn gateway_invoke(req: web::Json<GatewayInvokeVO>) -> impl Responder {
    let vo = req.into_inner();

    if let Some(ref manager) = CONTEXT.mqtt_manager {
        let result = manager.gateway_invoke(
            &vo.device_hash,
            &vo.protocol_type,
            &vo.stack_name,
            &vo.action,
            &vo.method,
            vo.params
        ).await;
        let final_result: Result<serde_json::Value, crate::error::Error> = result
            .map_err(|e| crate::error::Error::from(e.to_string()));
        RespVO::from_result(&final_result).resp_json()
    } else {
        RespVO::<()>::from_error_info(CODE_FAIL, "MQTT 客户端未初始化").resp_json()
    }
}