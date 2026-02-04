use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IotData {
    #[serde(rename = "deviceId")]
    pub device_id: Option<String>,

    #[serde(rename = "gatewayId")]
    pub gateway_id: Option<String>,

    #[serde(rename = "deviceType")]
    pub device_type: Option<String>,

    #[serde(rename = "extensionTag")]
    pub extension_tag: Option<String>,

    #[serde(rename = "eventTime")]
    pub event_time: Option<String>,

    #[serde(rename = "location")]
    pub location: Option<String>,

    #[serde(rename = "value")]
    pub value: Option<String>,

    #[serde(rename = "alarmCondition")]
    pub alarm_condition: Option<String>,

    #[serde(rename = "alarmStatus")]
    pub alarm_status: Option<String>,

    #[serde(rename = "masterMachine")]
    pub master_machine: Option<String>,

    #[serde(rename = "alarmRemark")]
    pub alarm_remark: Option<String>,

    #[serde(rename = "battery")]
    pub battery: Option<String>,

    #[serde(rename = "rssi")]
    pub rssi: Option<String>,
}

impl IotData {
    pub fn new() -> Self {
        Self {
            device_id: None,
            gateway_id: None,
            device_type: None,
            extension_tag: None,
            event_time: None,
            location: None,
            value: None,
            alarm_condition: None,
            alarm_status: None,
            master_machine: None,
            alarm_remark: None,
            battery: None,
            rssi: None,
        }
    }
}
