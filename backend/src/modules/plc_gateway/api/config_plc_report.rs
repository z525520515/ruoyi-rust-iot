use crate::modules::plc_gateway::api::config_util::call_device;
use crate::modules::plc_gateway::unified::response::error_response;
use axum::{extract::Path, response::Json};
use serde::Deserialize;
use serde_json::{Value, json};

// #[derive(Deserialize)]
// pub struct AddPlcReq {
//     pub name: String,
//     pub protocol: String,
//     pub interface: String,
//     pub ip: String,
//     pub port: u16,
//     pub rack: u8,
//     pub slot: u8,
// }

/// 添加PLC 测点配置
/// 请求示例:
/// ```json
/// {
///      "name": "myname",
///      "protocol": "s7",
///      "interface": "eth0",
///      "ip": "11.11.11.11",
///      "port": 102,
///      "rack": 0,
///      "slot": 2
/// }
/// ```
/// 响应示例：
/// ```json
/// {
///     "code": 200,
///     "data": {
///         "item": {
///             "disable": false,
///             "interface": "eth0",
///             "ip": "11.11.11.11",
///             "name": "myname",
///             "port": 102,
///             "protocol": "s7",
///             "rack": 0,
///             "slot": 2,
///             "type": "s7",
///             "uuid": "m9992avk"
///         },
///         "updateTime": 1761725956469
///     },
///     "message": "success",
///     "timestamp": 1761725956625
/// }
/// ```
// pub async fn add_plc(Path(ip): Path<String>, params: Json<AddPlcReq>) -> Json<Value> {
//     // log::info!("add plc: ip={}, params={}", ip, params);
//     let forward = json!({
//         "uuid":"",
//         "disable": false,
//         "name": params.name,
//         "type": params.protocol,
//         "protocol": params.protocol,
//         "interface": params.interface,
//         "ip": params.ip,
//         "port": params.port,
//         "rack": params.rack,
//         "slot": params.slot,
//     });

//     let url = format!("http://{}/plc/api/v1/plc", &ip);
//     call_device(url, forward, Some("POST")).await
// }

#[derive(Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AddPlcDataReportThirdPartyReq {
    pub http_method: String,
    pub http_url: String,
    /* ===== 选填，全部用 Option ===== */
    pub http_alarm_url: Option<String>,
    pub http_headers: Option<String>,
    pub period: Option<u32>,        // 上报周期，单位秒
    pub sample_period: Option<u32>, // 采样周期，单位秒
}
/// 添加配置数据上报、异常数据上报
/// 请求示例:
/// ```json
/// {
///     "httpMethod": "POST",  // 必填字段
///     "httpUrl": "test",     // 必填字段
///     "httpAlarmUrl": "test",
///     "httpHeaders": "test",
///     "period": 121,
///     "samplePeriod": 11
/// }
/// ```
/// 响应示例：
/// ```json
/// {
///     "code": 200,
///     "data": [
///         {
///             "model": "http",
///             "options": {
///                 "clientId": null,
///                 "host": null,
///                 "httpAlarmUrl": "test",
///                 "httpHeaders": "test",
///                 "httpMethod": "POST",
///                 "httpUrl": "test",
///                 "name": null,
///                 "password": null,
///                 "port": 1883,       
///                 "topic": "",
///                 "username": null
///             },
///             "period": 121,
///             "samplePeriod": 11,
///             "type": "thirdparty",
///             "uuid": "wb10aebwzh"
///         }
///     ],
///     "message": "success",
///     "timestamp": 1761727149722
/// }
/// ```
pub async fn add_plc_data_report_thirdparty(
    Path(ip): Path<String>,
    params: Json<AddPlcDataReportThirdPartyReq>,
) -> Json<Value> {
    log::info!(
        "add plc data report by third party: ip={}, params={:#?}",
        ip,
        params
    );
    let forward = json!(    {
        "uuid": "1",
        "period": params.period.unwrap_or(1),
        "samplePeriod": params.sample_period.unwrap_or(1),
        "type": "thirdparty",
        "model": "http",
        "options": {
            "name": null,
            "topic": "",
            "host": null,
            "port": 1883,
            "username": null,
            "password": null,
            "clientId": null,
            "httpMethod": params.http_method,
            "httpUrl": params.http_url,
            "httpAlarmUrl": params.http_alarm_url.as_deref().unwrap_or(""),
            "httpHeaders":  params.http_headers.as_deref().unwrap_or(""),
        }
    });

    let url = format!("http://{}:80/plc/api/v1/report-config", &ip);
    call_device(url, forward, Some("POST")).await
}

#[derive(Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AddPlcDataReportMqttReq {
    pub host: String,
    pub port: u16,
    pub topic: String,
    /* ===== 选填，全部用 Option ===== */
    pub period: Option<u32>,        // 上报周期，单位秒
    pub sample_period: Option<u32>, // 采样周期，单位秒
    pub username: Option<String>,
    pub password: Option<String>,
    pub client_id: Option<String>,
}
/// 添加配置数据上报、异常数据上报
/// 请求示例:
/// ```json
/// {
///     "host": "test",   // 必填字段
///     "port": 1883,     // 必填字段
///     "topic": "test",  // 必填字段
///     "period": 121,
///     "samplePeriod": 11,
///     "username": "test",
///     "password": "test",
///     "clientId": "test"
/// }
/// ```
/// 响应示例：
/// ```json
/// {
///     "code": 200,
///     "data": [
///         {
///             "model": "mqtt",
///             "options": {
///                 "clientId": "test",
///                 "host": "mqttmethod",
///                 "httpAlarmUrl": "",
///                 "httpHeaders": "",
///                 "httpMethod": "POST",
///                 "httpUrl": "",
///                 "name": "",
///                 "password": "test",
///                 "port": 1883,
///                 "topic": "testmqtt",
///                 "username": "test"
///             },
///             "period": 121,
///             "samplePeriod": 11,
///             "type": "mqtt",
///             "uuid": "avgy85qfyn"
///         }
///     ],
///     "message": "success",
///     "timestamp": 1761727480480
/// }
pub async fn add_plc_data_report_mqtt(
    Path(ip): Path<String>,
    params: Json<AddPlcDataReportMqttReq>,
) -> Json<Value> {
    log::info!(
        "add plc data report by mqtt: ip={}, params={:#?}",
        ip,
        params
    );
    let forward = json!({
            "uuid":"",
            "period": params.period.unwrap_or(1),
            "samplePeriod": params.sample_period.unwrap_or(1),
            "type":"mqtt",
            "model":"mqtt",
            "options":{
                "name":"",
                "topic": params.topic,
                "host": params.host,
                "port": params.port,
                "username": params.username.as_deref().unwrap_or(""),
                "password": params.password.as_deref().unwrap_or(""),
                "clientId": params.client_id.as_deref().unwrap_or(uuid::Uuid::new_v4().to_string().as_str()),
                "httpMethod":"POST",
                "httpUrl":"",
                "httpAlarmUrl":"",
                "httpHeaders":""
            }
    });

    let url = format!("http://{}:80/plc/api/v1/report-config", &ip);
    call_device(url, forward, Some("POST")).await
}

/// 获取所有配置数据上报、异常数据上报
///
/// 请求示例：/plc/report/data/10.0.0.5
///
/// 响应示例：
/// ```json
/// {
///     "code": 200,
///     "data": {
///         "items": [
///             {
///                 "model": "http",
///                 "options": {
///                     "clientId": null,
///                    "host": null,
///                     "httpAlarmUrl": "test",
///                     "httpHeaders": "test",
///                     "httpMethod": "POST",
///                     "httpUrl": "test",
///                     "name": null,
///                     "password": null,
///                     "port": 1883,
///                     "topic": "",
///                     "username": null
///                 },
///                 "period": 121,
///                 "samplePeriod": 11,
///                 "type": "thirdparty",
///                 "uuid": "wb10aebwzh"
///             },
///             {
///                 "model": "mqtt",
///                 "options": {
///                     "clientId": "test",
///                     "host": "mqttmethod",
///                     "httpAlarmUrl": "",
///                     "httpHeaders": "",
///                     "httpMethod": "POST",
///                     "httpUrl": "",
///                     "name": "",
///                     "password": "test",
///                     "port": 1883,
///                     "topic": "testmqtt",
///                     "username": "test"
///                 },
///                 "period": 121,
///                 "samplePeriod": 11,
///                 "type": "mqtt",
///                 "uuid": "avgy85qfyn"
///             }
///         ],
///         "updateTime": 1761727480426
///     },
///     "message": "success",
///     "timestamp": 1761727734296
/// }   
///```
pub async fn get_plc_data_report_all(Path(ip): Path<String>) -> Json<Value> {
    let url = format!("http://{}:80/plc/api/v1/report-config", &ip);
    call_device(url, json!({}), Some("GET")).await
}

/// 更新配置数据上报、异常数据上报是否启用
/// 请求示例：/plc/report/enable/10.0.0.5/true
/// 响应示例：
/// ```json
/// {
///     "code": 200,
///     "data": true,
///     "message": "success",
///     "timestamp": 1761727734296
/// }
///```
pub async fn update_report_enabled(Path((ip, enabled)): Path<(String, String)>) -> Json<Value> {
    log::info!(
        "update report enabled: ip={}, enabled={}",
        ip,
        enabled.clone()
    );
    let able;
    match enabled.to_lowercase().as_str() {
        "true" => able = true,
        "false" => able = false,
        _ => {
            return error_response("enabled must be true or false".to_string(), Some(500));
        }
    }
    let forward = json!(    {
         "enable": able
    });
    let url = format!("http://{}:80/plc/api/v1/report-enabled", &ip);
    call_device(url, forward, Some("PUT")).await
}

/// 查看 配置数据上报、异常数据上报是否启用的状态
/// 请求示例：/plc/report/enabled/10.0.0.5
/// 响应示例：
/// ```json
/// {
///     "code": 200,
///     "data": true,
///     "message": "success",
///     "timestamp": 1761727734296
/// }
///```
pub async fn get_report_enabled(Path(ip): Path<String>) -> Json<Value> {
    let url = format!("http://{}:80/plc/api/v1/report-enabled", &ip);
    call_device(url, json!({}), Some("GET")).await
}

// #[derive(Deserialize, Debug)]
// #[serde(rename_all = "camelCase")]
// pub struct PlcModbusPointReq {
//     pub name: String,
//     pub interface: u16,
//     pub ip: String,
//     pub port: u16,
// }

// /// 添加modbus类型的 PLC信息配置
// pub async fn add_plc_modbus_points(
//     Path(ip): Path<String>,
//     params: Json<PlcModbusPointReq>,
// ) -> Json<Value> {
//     log::info!("add plc modbus points: ip={}, params={:#?}", ip, params);
//     let forward = json!(    {
//         "uuid": "",
//         "name": params.name,
//         "type": "modbusTcp",
//         "protocol": "modbus_tcp",
//         "interface": params.interface,
//         "disable": false,
//         "ip": params.ip,
//         "port": params.port
//     });
//     let url = format!("http://{}:80/plc/api/v1/plc", &ip);
//     call_device(url, forward, Some("POST")).await
// }







#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;
    #[test]
    async fn test_add_plc_data() {
        let extra = json!({
            "trigger": "onvalue",
            "condition": "eq",
            "value": 1,
        });
        let forward = json!([{
            "extra": extra.to_string(),
        }]);
        println!("forward: {:?}", forward);
    }
}
