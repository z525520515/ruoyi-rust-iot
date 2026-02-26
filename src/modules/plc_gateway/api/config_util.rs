use axum::{

    response::Json,
};

use serde_json::{Value, json};

use crate::modules::plc_gateway::unified::response::{error_response, success_response};


/// 通用转发函数，支持不同的HTTP方法
pub async fn call_device(url: String, body: Value, method: Option<&str>) -> Json<Value> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(8))
        .build()
        .map_err(|e| error_response(e.to_string(), Some(500)))
        .unwrap();

    // 确定HTTP方法，默认为POST以保持向后兼容
    let http_method = method.unwrap_or("POST");
    
    // 根据方法创建不同的请求
    let request_builder = match http_method.to_uppercase().as_str() {
        "GET" => client.get(&url),
        "POST" => client.post(&url).json(&body),
        "PUT" => client.put(&url).json(&body),
        "DELETE" => client.delete(&url),
        _ => return error_response(format!("不支持的HTTP方法: {}", http_method), Some(400)),
    };
    
    let resp = request_builder
        .send()
        .await
        .map_err(|e| error_response(format!("请求设备失败: {}", e), Some(502)))
        .unwrap();

    let status = resp.status();
    let text = resp
        .text()
        .await
        .map_err(|e| error_response(e.to_string(), Some(500)))
        .unwrap();

    if status.is_success() {
        match serde_json::from_str::<Value>(&text) {
            Ok(v) => success_response(v, None),
            Err(_) => success_response(json!(text), None),
        }
    } else {
        error_response(
            format!("设备返回错误 {}: {}", status.as_u16(), text),
            Some(status.as_u16() as u16),
        )
    }
}
