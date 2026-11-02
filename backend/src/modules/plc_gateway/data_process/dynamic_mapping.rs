// utils/dynamic_mapping.rs

use serde::Deserialize;
use once_cell::sync::Lazy;

use std::collections::HashMap;
use yaml_rust::{Yaml, YamlLoader};
use std::{env, path::Path};

static MAP_FILE: &str = ".config/device_mappings.yml";

// Configuration structure to hold field mappings for each device type
#[derive(Debug, Deserialize)]
pub struct DeviceMappings {
    pub mappings: HashMap<String, HashMap<String, String>>,
}

fn get_config_file_path() -> String {
   
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").ok().unwrap_or_else(|| ".".to_string());
    let mut base = Path::new(&manifest_dir);

    // 最多向上跳两次
    for _ in 0..2 {
        base = base.parent().unwrap_or(base);          // 先往上走一层
        let config = base.join(".config").join("device_mappings.yml");
        if config.exists() {
            return config.to_string_lossy().into_owned();
        }
    }
    
    // 方法3: 默认使用项目根目录下的.config/config.toml
    // 注意：这个相对路径在不同的工作目录下可能会失败
    MAP_FILE.to_string()
}



// 使用Lazy静态变量缓存YAML映射，确保只加载一次
static MAPPINGS_CACHE: Lazy<DeviceMappings> = Lazy::new(|| {
    let file_path = get_config_file_path();
    log::info!("加载设备映射配置文件: {}", file_path);
    let yaml_str = std::fs::read_to_string(file_path).expect("Failed to read YAML file");
    let docs = YamlLoader::load_from_str(yaml_str.as_str()).expect("parse yaml");
    
    let root = &docs[0];                 // 根节点
    let list = root["device_list"].as_vec()
        .expect("device_list must be an array");

    let mut mappings = HashMap::new();

    for item in list {
        let item = item.as_hash().expect("every list item must be a hash");

        // 1. 先拿到 device_name 作为外层 key
        let device_name = item[&Yaml::String("device_name".to_owned())]
            .as_str()
            .expect("device_name must be string")
            .to_string();

        // 2. 把整个 item 变成 HashMap<String,String>（去掉 device_name 也行，随你）
        let mut field_map = HashMap::new();
        for (k, v) in item {
            let k = k.as_str().expect("field key must be string").to_string();
            let v = v.as_str().expect("field value must be string").to_string();
            field_map.insert(k, v);
        }

        mappings.insert(device_name, field_map);
    }

    DeviceMappings { mappings }
});

pub fn load_mappings() -> &'static DeviceMappings {
    // 直接返回缓存的映射，避免重复加载文件
    &MAPPINGS_CACHE
}



mod tests {
  
    #[test]
    fn test_load_mappings() {
        let m = super::load_mappings();
       // 验证一下
        assert!(m.mappings.contains_key("Watch"));
        assert!(m.mappings.contains_key("Subway"));
        assert_eq!(
            m.mappings["Watch"]["device_id"],
            "sn"
        );
        println!("{:#?}", m);
    }
}
