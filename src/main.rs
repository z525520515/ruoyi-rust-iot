use backend::http_server;
use backend::modules::system::service::CONTEXT;
use backend::util::mqtt::MqttManager;
use netcore::error::Result;
use netcore::transport::Bootstrap;

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

    // 启动 TCP 服务器
    tokio::spawn(async {
        if let Err(e) = start_tcp_server().await {
            eprintln!("TCP Server error: {}", e);
        }
    });

    http_server::build_server(&CONTEXT.config.base_api, mqtt_manager).await
}

async fn start_tcp_server() -> Result<()> {
    let bootstrap = Bootstrap::new();
    let listener = bootstrap.bind("0.0.0.0:8080").await?;
    println!("TCP Server listening on 127.0.0.1:8080");
    bootstrap.start(listener).await
}
