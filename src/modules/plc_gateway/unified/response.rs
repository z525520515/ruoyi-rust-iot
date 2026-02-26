use serde_json::{json, Value};
use axum::{ response::Json};
use chrono::Utc;
// 统一的响应格式
fn unified_response(
    code: u16,
    message: String,
    data: Option<Value>,
) -> Json<Value> {
    Json(json!({
        "code": code,
        "message": message,
        "data": data.unwrap_or(json!(null)),
        "timestamp": Utc::now().timestamp_millis()
    }))
}


// 成功响应（code=200）
pub fn success_response(data: Value, code: Option<u16>) -> Json<Value> {
    unified_response(code.unwrap_or(200), "success".to_string(), Some(data))
}

// 错误响应 - code 可选，默认 400
pub fn error_response(message: String, code: Option<u16>) -> Json<Value> {
    unified_response(code.unwrap_or(400), message, None)
}