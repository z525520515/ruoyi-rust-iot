use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::sync::{mpsc, oneshot};
use crate::modules::plc_gateway::api::remote_reboot::RebootResp;
use crate::modules::plc_gateway::api::remote_log::GetLogResp;
use crate::modules::plc_gateway::api::remote_ota::OtaResp;
use crate::modules::plc_gateway::api::remote_sysinfo::SysInfoResp; // 新增

// 定义通用的Response枚举
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Response {
    Reboot(RebootResp),
    GetLog(GetLogResp),
    Ota(OtaResp),
    SysInfo(SysInfoResp), // 新增
}
// 定义通用的Response枚举（不变）

// 新：用 enum 替换 dyn SenderTrait
pub enum Sender {
    Oneshot(oneshot::Sender<Response>),
    Mpsc(mpsc::Sender<Response>),
}

// 共享的 pending responses（改用 enum，无需 Box<dyn>）
pub type PendingResponses = Arc<Mutex<HashMap<String, Sender>>>;
// // 定义SenderTrait，抽象发送行为
// pub trait SenderTrait: Send + Sync {
//     fn send(&self, response: Response) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
//     fn is_multi_response(&self) -> bool; // 用于区分oneshot和mpsc
// }

// // 为oneshot::Sender实现SenderTrait
// impl SenderTrait for oneshot::Sender<Response> {
//     fn send(&self, response: Response) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//         // 使用完全限定名称调用原始的send方法，避免无限递归
//         oneshot::Sender::send(self, response).map_err(|e| {
//             Box::new(std::io::Error::new(
//                 std::io::ErrorKind::Other,
//                 format!("oneshot send failed: {:?}", e),
//             )) as Box<dyn std::error::Error + Send + Sync>
//         })
//     }
//     fn is_multi_response(&self) -> bool {
//         false
//     }
// }

// // 为mpsc::Sender实现SenderTrait
// impl SenderTrait for mpsc::Sender<Response> {
//     fn send(&self, response: Response) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//         let sender = self.clone();
//         // mpsc::Sender需要异步发送，转换为阻塞操作
//         tokio::runtime::Handle::current().block_on(async move {
//             sender.send(response).await.map_err(|e| {
//                 Box::new(std::io::Error::new(
//                     std::io::ErrorKind::Other,
//                     format!("mpsc send failed: {:?}", e),
//                 )) as Box<dyn std::error::Error + Send + Sync>
//             })
//         })
//     }
//     fn is_multi_response(&self) -> bool {
//         true
//     }
// }

// // 共享的pending responses状态
// pub type PendingResponses = Arc<Mutex<HashMap<String, Box<dyn SenderTrait>>>>;
