use humantime::parse_duration;
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
        Some(s) => Ok(Some(
            parse_duration(&s).expect("不正确的Duration字符串，支持的格式如5s、3m、6h"),
        )),
        None => Ok(None),
    }
}
