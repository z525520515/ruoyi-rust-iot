use axum::{
    Router,
    routing::{get, post},
};

use crate::modules::plc_gateway::api::remote_reboot::handle_reboot;
use crate::modules::plc_gateway::api::remote_log::handle_get_log;
use crate::modules::plc_gateway::api::remote_sysinfo::handle_sysinfo;
use crate::modules::plc_gateway::api::remote_ota::handle_ota;

/// 创建远程管理相关的路由
pub fn create_remote_management_router() -> Router {
    Router::new()
    .route("/api/reboot/{sn}", get(handle_reboot))
    .route("/api/reboot/{sn}", post(handle_reboot))
    .route("/api/sysInfo/{sn}", get(handle_sysinfo)) 
    .route("/api/sysInfo/{sn}", post(handle_sysinfo)) 
    .route("/api/getLog/{sn}", post(handle_get_log)) 
    .route("/api/ota", post(handle_ota)) // 新增
}