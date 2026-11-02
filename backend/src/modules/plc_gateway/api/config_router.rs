use axum::routing::put;
use axum::{
    Router,
    routing::{delete, get, post},
};

use crate::modules::plc_gateway::api::config_network::{
    reset_network_name, reset_network_set_location, reset_network_set_platform_explorer,
    reset_network_set_platform_gwhub, reset_network_set_platform_tio, set_network_config,
    set_network_config_eth0,
};
use crate::modules::plc_gateway::api::config_plc_report::{
    add_plc_data_report_mqtt,
    add_plc_data_report_thirdparty,
    // add_plc,add_plc_modbus_points,
    get_plc_data_report_all,
    get_report_enabled,
    update_report_enabled,
};
// 提供添加分组后测点的配置
use crate::modules::plc_gateway::api::config_plc_config::{
    add_plc_config, delete_plc_config, get_all_data_real_time_monitoring, get_all_plc_config,
    update_plc_config,
};
use crate::modules::plc_gateway::api::config_plc_group::{
    add_plc_config_group, del_plc_config_group, update_plc_config_group,
};
use crate::modules::plc_gateway::api::config_plc_points::{
    add_plc_config_group_points, del_plc_config_group_points, downlod_plc_config_group_points,
    downlod_points_template, get_plc_config_group_points, update_plc_config_group_points,upload_points_file
};

/* ==================== 3. 路由 ==================== */
pub fn network_routes() -> Router {
    Router::new()
        .route("/network/config/full/{ip}", post(set_network_config))
        .route("/network/config/eth0/{ip}", post(set_network_config_eth0))
        .route("/network/name/{ip}/{name}", post(reset_network_name))
        .route(
            "/network/location/{ip}/{position}",
            post(reset_network_set_location),
        )
        .route(
            "/network/platform/tio/{ip}",
            post(reset_network_set_platform_tio),
        )
        .route(
            "/network/platform/gwhub/{ip}",
            post(reset_network_set_platform_gwhub),
        )
        .route(
            "/network/platform/explorer/{ip}",
            post(reset_network_set_platform_explorer),
        )
}

pub fn plc_data_report() -> Router {
    Router::new()
        .route(
            "/plc/report/data/repeng/{ip}",
            post(add_plc_data_report_thirdparty),
        )
        .route("/plc/report/data/mqtt/{ip}", post(add_plc_data_report_mqtt))
        .route("/plc/report/data/{ip}", get(get_plc_data_report_all))
        .route(
            "/plc/report/enable/{ip}/{enabled}",
            put(update_report_enabled),
        )
        .route("/plc/report/enabled/{ip}", get(get_report_enabled))
}
pub fn plc_data_real_time_monitoring() -> Router {
    Router::new()
        // 获取实时监控的数据 Real-time monitoring
        .route("/plc/data/{ip}", get(get_all_data_real_time_monitoring))
}

pub fn plc_config() -> Router {
    Router::new()
        // 添加PLC配置create_plc_config->添加分组add_plc_modbus_group->添加测点
        .route("/plc/config/{ip}", post(add_plc_config))
        // 获取所有PLC数据
        .route("/plc/config/{ip}", get(get_all_plc_config))
        // 删除某个PLC配置
        .route("/plc/config/{ip}/{uuid}", delete(delete_plc_config))
        // 更新某个PLC配置
        .route("/plc/config/{ip}", put(update_plc_config))
}

pub fn plc_group() -> Router {
    Router::new()
        // 添加PLC分组
        .route("/plc/group/{ip}", post(add_plc_config_group))
        .route("/plc/group/{ip}", put(update_plc_config_group))
        // 删除某个PLC分组
        .route("/plc/group/{ip}/{uuid}", delete(del_plc_config_group))
}

pub fn plc_points() -> Router {
    Router::new()
        // 添加某个分组添加测点
        .route("/plc/points/{ip}/{uuid}", post(add_plc_config_group_points))
        .route("/plc/points/{ip}/{uuid}", get(get_plc_config_group_points))
        // 删除某个分组下的某个测点
        .route(
            "/plc/points/{ip}/{uuid}/{id}",
            delete(del_plc_config_group_points),
        )
        .route(
            "/plc/points/{ip}/{uuid}",
            put(update_plc_config_group_points),
        )
        // 下载某个分组下的所有测点
        .route(
            "/plc/points/download/{ip}/{uuid}/{name}",
            get(downlod_plc_config_group_points),
        )
        // 下载模板
        .route(
            "/plc/points/download/template",
            get(downlod_points_template),
        )
        .route(
            "/plc/points/upload/{ip}/{uuid}/{protocol}",
            post(upload_points_file),
        )
}

pub fn plc_routes() -> Router {
    Router::new()
        .merge(plc_data_report())
        .merge(plc_data_real_time_monitoring())
        .merge(plc_config())
        .merge(plc_group())
        .merge(plc_points())
}
