use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use chrono_tz::Asia::Shanghai;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Timestamp {
    #[serde(serialize_with = "serialize_timestamp_seconds")]
    #[serde(deserialize_with = "deserialize_timestamp_seconds")]
    pub seconds: u32,
}

impl Timestamp {
    pub fn new(seconds: u32) -> Timestamp {
        Timestamp { seconds }
    }

    pub fn from_bytes(bytes: [u8; 4]) -> Timestamp {
        let seconds = u32::from_be_bytes(bytes);
        Timestamp::new(seconds)
    }

    pub fn to_bytes(&self) -> [u8; 4] {
        self.seconds.to_be_bytes()
    }

    fn to_datetime(&self) -> chrono::DateTime<chrono_tz::Tz> {
        chrono::Utc
            .timestamp_opt(self.seconds as i64, 0)
            .unwrap()
            .with_timezone(&Shanghai)
    }

    pub fn to_naive_datetime(&self) -> NaiveDateTime {
        Utc.timestamp_opt(self.seconds as i64, 0)
            .unwrap()
            .naive_utc()
    }

    pub fn to_formatted_string(&self) -> String {
        self.to_datetime()
            .format("%Y-%m-%d %H:%M:%S CST")
            .to_string()
    }

    pub fn from_naive_datetime(dt: NaiveDateTime) -> Self {
        let utc_datetime = DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc);
        let seconds = utc_datetime.timestamp() as u32;
        Timestamp::new(seconds)
    }

    pub fn from_datetime(datetime: chrono::DateTime<chrono_tz::Tz>) -> Self {
        let seconds = datetime.timestamp() as u32;
        Timestamp::new(seconds)
    }

    pub fn from_now() -> Self {
        let now = chrono::Utc::now().with_timezone(&chrono_tz::Asia::Shanghai);
        Self::from_datetime(now)
    }
}

/// 序列化Timestamp的seconds字段为NaiveDateTime
fn serialize_timestamp_seconds<S>(seconds: &u32, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let timestamp = Timestamp::new(*seconds);
    let naive_dt = timestamp.to_naive_datetime();
    NaiveDateTime::serialize(&naive_dt, serializer)
}

/// 从NaiveDateTime反序列化为Timestamp的seconds字段
fn deserialize_timestamp_seconds<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    let naive_dt = NaiveDateTime::deserialize(deserializer)?;
    let timestamp = Timestamp::from_naive_datetime(naive_dt);
    Ok(timestamp.seconds)
}

/// 序列化Timestamp为NaiveDateTime
fn serialize_timestamp<S>(timestamp: &Timestamp, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let naive_dt = timestamp.to_naive_datetime();
    NaiveDateTime::serialize(&naive_dt, serializer)
}

/// 从NaiveDateTime反序列化Timestamp
fn deserialize_timestamp<'de, D>(deserializer: D) -> Result<Timestamp, D::Error>
where
    D: Deserializer<'de>,
{
    let naive_dt = NaiveDateTime::deserialize(deserializer)?;
    Ok(Timestamp::from_naive_datetime(naive_dt))
}

/// 支持序列化和反序列化的Timestamp包装器
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SerializableTimestamp {
    #[serde(serialize_with = "serialize_timestamp")]
    #[serde(deserialize_with = "deserialize_timestamp")]
    pub timestamp: Timestamp,
}

impl From<Timestamp> for SerializableTimestamp {
    fn from(timestamp: Timestamp) -> Self {
        Self { timestamp }
    }
}

impl From<SerializableTimestamp> for Timestamp {
    fn from(serializable: SerializableTimestamp) -> Self {
        serializable.timestamp
    }
}

impl std::default::Default for Timestamp {
    fn default() -> Self {
        Timestamp::from_now()
    }
}

impl std::fmt::Display for Timestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_formatted_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;
    use serde_json;

    #[test]
    fn test_timestamp_to_naive_datetime() {
        // 创建一个已知的时间戳：2023-01-01 00:00:00 UTC
        let expected_dt = NaiveDate::from_ymd_opt(2023, 1, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();

        // 计算这个时间的Unix时间戳
        let expected_timestamp = expected_dt.and_utc().timestamp() as u32;
        let timestamp = Timestamp::new(expected_timestamp);

        let actual_dt = timestamp.to_naive_datetime();
        assert_eq!(expected_dt, actual_dt);
    }

    #[test]
    fn test_from_naive_datetime() {
        let dt = NaiveDate::from_ymd_opt(2023, 6, 15)
            .unwrap()
            .and_hms_opt(12, 30, 45)
            .unwrap();

        let timestamp = Timestamp::from_naive_datetime(dt);
        let roundtrip_dt = timestamp.to_naive_datetime();

        assert_eq!(dt, roundtrip_dt);
    }

    #[test]
    fn test_serializable_timestamp_json() {
        let dt = NaiveDate::from_ymd_opt(2023, 6, 15)
            .unwrap()
            .and_hms_opt(12, 30, 45)
            .unwrap();

        let timestamp = Timestamp::from_naive_datetime(dt);
        let serializable = SerializableTimestamp::from(timestamp);

        // 序列化为JSON
        let json_str = serde_json::to_string(&serializable).unwrap();
        println!("Serialized JSON: {}", json_str);

        // 从JSON反序列化
        let deserialized: SerializableTimestamp = serde_json::from_str(&json_str).unwrap();
        let roundtrip_timestamp = Timestamp::from(deserialized);

        assert_eq!(timestamp, roundtrip_timestamp);
        assert_eq!(dt, roundtrip_timestamp.to_naive_datetime());
    }

    #[test]
    fn test_serializable_timestamp_roundtrip() {
        let original = Timestamp::from_now();
        let serializable = SerializableTimestamp::from(original);

        // 序列化
        let json = serde_json::to_value(&serializable).unwrap();

        // 反序列化
        let deserialized: SerializableTimestamp = serde_json::from_value(json).unwrap();
        let recovered = Timestamp::from(deserialized);

        assert_eq!(original, recovered);
    }

    #[test]
    fn test_direct_timestamp_serialization() {
        let timestamp = Timestamp::from_now();

        // Test direct serialization/deserialization
        let json = serde_json::to_string(&timestamp).unwrap();
        println!("Direct timestamp JSON: {}", json);

        let deserialized: Timestamp = serde_json::from_str(&json).unwrap();
        assert_eq!(timestamp, deserialized);
    }

    #[test]
    fn test_timestamp_bytes_conversion() {
        let original_seconds = 1672531200; // 2023-01-01 00:00:00 UTC
        let timestamp = Timestamp::new(original_seconds);

        let bytes = timestamp.to_bytes();
        let recovered = Timestamp::from_bytes(bytes);

        assert_eq!(original_seconds, recovered.seconds);
        assert_eq!(timestamp, recovered);
    }
}
