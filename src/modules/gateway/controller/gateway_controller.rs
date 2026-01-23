use crate::modules::gateway::domain::dto::gateway_invoke::GatewayInvokeVO;
use crate::modules::system::domain::vo::RespVO;
use crate::util::mqtt::MqttManager;
use actix_web::{dev::ServiceRequest, post, web, Responder};

#[post("/gateway/invoke")]
pub async fn gateway_invoke(
    req_inner: ServiceRequest,
    req: web::Json<GatewayInvokeVO>,
) -> impl Responder {
    let vo = req.into_inner();

    // TODO: 这里使用 match 匹配覆盖错误处理
    let mqtt_manager = req_inner.app_data::<MqttManager>().unwrap();

    let result = mqtt_manager
        .gateway_invoke(
            &vo.device_hash,
            &vo.protocol_type,
            &vo.action,
            "ReservedControlProtocolStack",
            &vo.action,
            &vo.method,
            vo.params,
        )
        .await;

    match result {
        Ok(data) => RespVO::from_result(data).resp_json(),
        Err(e) => RespVO::from_result(e.to_string()).resp_json(),
    }
}
