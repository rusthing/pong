use serde::{Deserialize, Deserializer, Serializer};
use std::time::Duration;

pub fn serialize<S>(dur: &Option<Duration>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match dur {
        Some(d) => serializer.serialize_str(&format!("{}s", d.as_secs())),
        None => serializer.serialize_none(),
    }
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Duration>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    match s {
        Some(s) => {
            Ok(Some(if s.ends_with("ms") {
                let sec = s.trim_end_matches("ms").parse::<u64>().unwrap();
                Duration::from_millis(sec)
            } else if s.ends_with('s') {
                let sec = s.trim_end_matches('s').parse::<u64>().unwrap();
                Duration::from_secs(sec)
            } else if s.ends_with('m') {
                let sec = s.trim_end_matches('s').parse::<u64>().unwrap();
                Duration::from_secs(sec * 60)
            } else if s.ends_with('h') {
                let sec = s.trim_end_matches('h').parse::<u64>().unwrap();
                Duration::from_secs(sec * 3600)
            } else {
                // 默认按秒解析
                let sec = s.parse::<u64>().unwrap();
                Duration::from_secs(sec)
            }))
        }
        None => Ok(None),
    }
}
