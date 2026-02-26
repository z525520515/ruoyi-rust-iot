use serde::{Deserialize, Serialize};
use serde::{ Serializer};
use chrono::{Local, TimeZone};
use uuid::Uuid;
// 把 i64 秒时间戳序列化成 "%Y-%m-%d %H:%M:%S"
// fn fmt_i64_as_datetime<S>(ts: &i64, ser: S) -> Result<S::Ok, S::Error>
// where
//     S: Serializer,
// {
//     let dt = Local.timestamp_opt(*ts, 0).single(); // 按秒解析
//     match dt {
//         Some(d) => ser.serialize_str(&d.format("%Y-%m-%d %H:%M:%S").to_string()),
//         None => ser.serialize_str("1970-01-01 00:00:00"), // 兜底
//     }
// }

fn fmt_i64_as_datetime<S>(ts: &i64, ser: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    // 2100-01-01 00:00:00 UTC 的时间戳（秒）
    const TS_2100: i64 = 4_102_444_800;

    // 1. 先按秒解
    let mut ts_sec = *ts;
    if ts_sec > TS_2100 {
        // 2. 超过 2100 年，当成毫秒转秒
        ts_sec /= 1_000;
    }

    match Local.timestamp_opt(ts_sec, 0).single() {
        Some(dt) => ser.serialize_str(&dt.format("%Y-%m-%d %H:%M:%S").to_string()),
        None => ser.serialize_str("1970-01-01 00:00:00"),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FireAlarm {
    pub id: Uuid,
    pub device_id: String,
    pub gateway_id: String,
    pub device_type: String,
    pub extension_tag: String,
    // 只改序列化表现，仍然用 i64 存
    #[serde(serialize_with = "fmt_i64_as_datetime")]
    pub event_time: i64,
    pub location: String,
    pub value: String,
    pub alarm_condition: String,
    pub alarm_status: String,
    pub master_machine: String,
    pub alarm_remark: String,
}


pub trait ToFireAlarm {
    fn to_fire_alarm(&self) -> FireAlarm;
}