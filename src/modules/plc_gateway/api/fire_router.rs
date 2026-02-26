use axum::{
    Router,
    routing::{post},
};
use crate::modules::plc_gateway::api::fire_info::handle_get_fire_info;
use crate::modules::plc_gateway::api::fire_info_push::handle_fire_info_push;
/// 创建远程管理相关的路由
pub fn create_fire_router() -> Router {
    Router::new()
    .route("/api/fire/{fire_type}", post(handle_get_fire_info))
    .route("/api/firePush/{fire_type}", post(handle_fire_info_push))
}