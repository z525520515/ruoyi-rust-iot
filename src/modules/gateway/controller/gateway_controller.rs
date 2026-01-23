use actix_web::{post, web, Responder};
use crate::util::mqtt::MqttManager;
use crate::modules::gateway::domain::dto::gateway_invoke::GatewayInvokeVO;
use crate::modules::system::domain::vo::{RespVO};

#[post("/gateway/invoke")]
pub async fn gateway_invoke(req: web::Json<GatewayInvokeVO>) -> impl Responder {
    let vo = req.into_inner();

    let result = MqttManager::init().await.gateway_invoke(
        &vo.device_hash,
        &vo.protocol_type,
        &vo.action,
        "ReservedControlProtocolStack",
        &vo.action,
        &vo.method,
        vo.params
    ).await;

    match result {
        Ok(data) => RespVO::from_result(data).resp_json(),
        Err(e) => RespVO::from_result(e.to_string()).resp_json(),
    }
}