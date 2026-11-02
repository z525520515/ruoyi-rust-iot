use std::fs;
use merge_yaml_hash::{MergeYamlHash};
use serde::{Serialize, Deserialize};

/// MQTT 专用配置结构
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct MqttConfig {
    pub host: String,
    pub port: u16,
    pub client_id: String,
    pub username: String,
    pub password: String,
    pub keep_alive: u64,
    pub clean_session: bool,
}

/// 完整应用配置
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationConfig {
    pub debug: bool,
    pub server_url: String,
    pub base_api: String,
    pub web_redis_url: String,
    pub database_url: String,
    pub logic_column: String,
    pub logic_un_deleted: i64,
    pub logic_deleted: i64,
    pub log_dir: String,
    pub log_temp_size: String,
    pub log_pack_compress: String,
    pub log_rolling_type: String,
    pub log_chan_len: Option<usize>,
    pub log_level: String,
    pub sms_cache_send_key_prefix: String,
    pub jwt_secret: String,
    pub white_list_api: Vec<String>,
    pub cache_type: String,
    pub login_fail_retry: u64,
    pub login_fail_retry_wait_sec: u64,
    pub captcha_enabled: bool,
    pub captcha_expired_min: u64,
    pub token_expired_min: u64,
    // --- 新增 MQTT 配置字段 ---
    pub mqtt: MqttConfig,
}

impl Default for ApplicationConfig {
    fn default() -> Self {
        // 1. 获取运行环境 profile (dev/prod/...)
        let k = "profile=";
        let profile = std::env::args_os()
            .find(|v| v.to_str().unwrap_or_default().starts_with(k))
            .map(|v| v.to_str().unwrap_or_default().trim_start_matches(k).to_string());

        let profile = match profile {
            Some(o) => Some(o),
            None => std::env::var("profile").ok(),
        };

        let profile = match profile {
            Some(o) => Some(o),
            None => Some("dev".to_string()), // 默认 dev
        };

        let profile = profile.unwrap_or_else(|| "prod".to_string());
        let profile = profile.trim().to_string();
        println!("loading profile: {}", profile);

        // 2. 加载基础配置 application.yml
        let yml_data = include_str!("../../application.yml");
        let mut hash = MergeYamlHash::new();
        hash.merge(yml_data);

        // 3. 合并环境特有配置 application-{profile}.yml
        if !profile.is_empty() {
            match fs::read_to_string(format!("application-{profile}.yml")) {
                Ok(s) => {
                    hash.merge(&s);
                }
                Err(_) => {
                    println!("Warning: Can't loading application-{profile}.yml, using base config.");
                }
            }
        }

        // 4. 解析为结构体
        // 注意：由于 MqttConfig 字段在 ApplicationConfig 中是非 Option 的，
        // 确保你的 yml 文件中一定包含 mqtt: 节点及相关子项。
        let mut result: ApplicationConfig =
            serde_yaml::from_str(&*hash.to_string()).expect("load config file fail: Make sure 'mqtt' section exists in yml");

        // 支持通过环境变量覆盖敏感配置，避免在源码中硬编码真实凭据
        if let Ok(v) = std::env::var("APP_DATABASE_URL") {
            result.database_url = v;
        }
        if let Ok(v) = std::env::var("APP_REDIS_URL") {
            result.web_redis_url = v;
        }
        if let Ok(v) = std::env::var("APP_JWT_SECRET") {
            result.jwt_secret = v;
        }
        if let Ok(v) = std::env::var("APP_MQTT_HOST") {
            result.mqtt.host = v;
        }
        if let Ok(v) = std::env::var("APP_MQTT_PORT") {
            if let Ok(port) = v.parse::<u16>() {
                result.mqtt.port = port;
            }
        }
        if let Ok(v) = std::env::var("APP_MQTT_CLIENT_ID") {
            result.mqtt.client_id = v;
        }
        if let Ok(v) = std::env::var("APP_MQTT_USERNAME") {
            result.mqtt.username = v;
        }
        if let Ok(v) = std::env::var("APP_MQTT_PASSWORD") {
            result.mqtt.password = v;
        }

        if result.debug {
            println!("[ruoyi_rust] ///////////////////// Start On Debug Mode ////////////////////////////");
        } else {
            println!("[ruoyi_rust] ///////////////////// Start On Release Mode ////////////////////////////");
        }

        result
    }
}