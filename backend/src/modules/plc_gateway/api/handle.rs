use axum::{Router, extract::Extension};

use rumqttc::AsyncClient;

use std::sync::Arc;

use crate::modules::plc_gateway::api::remote_response::PendingResponses;
use crate::modules::plc_gateway::api::remote_ota::OtaTaskQueue;
use sea_orm::DatabaseConnection;

pub fn create_router( db: Arc<DatabaseConnection>,mqtt_client: Arc<AsyncClient>, pending_responses: PendingResponses, task_queue: OtaTaskQueue) -> Router {
    Router::new()
        .merge(crate::modules::plc_gateway::api::remote_router::create_remote_management_router())
        .merge(crate::modules::plc_gateway::api::fire_router::create_fire_router())
        .merge(crate::modules::plc_gateway::api::equipment_info_router::equipment_routes())
        .merge(crate::modules::plc_gateway::api::config_router::network_routes())
        .merge(crate::modules::plc_gateway::api::config_router::plc_routes())
        .layer(Extension(mqtt_client))
        .layer(Extension(pending_responses))
        .layer(Extension(task_queue))
         .layer(Extension(db))
}
