use backend::modules::system::service::CONTEXT;
use backend::http_server;
use backend::util::mqtt::MqttManager; // 添加这行导入

#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_doc_comments)]
/// use tokio,because Rbatis specifies the runtime-tokio
#[tokio::main]
async fn main() -> std::io::Result<()> {
    //日志追加器
    backend::config::log::init_log();

    //连接数据库
    CONTEXT.init_pool().await;
    dbg!("init_pool ok! 11");
    let mqtt_config = CONTEXT.config.mqtt.clone();
    let mqtt_manager = MqttManager::init(mqtt_config).await;
    CONTEXT.sys_dict_data_service.update_cache().await?;
    // dbg!("init sys_dict data ok! 22");
    CONTEXT.sys_config_service.loading_config_cache().await?;
    //dbg!("init sys_config data ok! 33");
    CONTEXT.sys_menu_service.update_cache().await?;
    //dbg!("init sys_menu data ok! 44");
    http_server::build_server(&CONTEXT.config.base_api).await

}
