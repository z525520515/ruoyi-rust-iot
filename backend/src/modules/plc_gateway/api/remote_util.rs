use md5::Context;
use reqwest::Client;
use std::time::{SystemTime, UNIX_EPOCH};
// 共享的pending responses状态
pub fn generate_token() -> String {
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    format!("token:{}", ts)
}

pub async fn calculate_md5_from_url(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();
    let mut resp = client.get(url).send().await?.error_for_status()?;
    let mut context = Context::new();
    while let Some(chunk) = resp.chunk().await? {
        context.consume(&chunk);
    }
    let digest = context.finalize();
    Ok(format!("{:x}", digest))
}

pub async fn get_remote_file_size(url: &str) -> Result<u64, Box<dyn std::error::Error>> {
    let client = Client::new();
    let response = client.head(url).send().await?.error_for_status()?;
    if let Some(len) = response.content_length() {
        Ok(len)
    } else {
        Err("No Content-Length header found".into())
    }
}
