use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize, de::Error};

pub mod manager;
pub mod notifier;
pub mod requester;

fn deserialize_utc_from_missing_z<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = String::deserialize(deserializer)?;
    // Append Z and let chrono handle the rest
    Ok(DateTime::parse_from_rfc3339(&format!("{s}Z"))
        .map_err(Error::custom)?
        .with_timezone(&Utc))
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Response {
    id: String,
    name: String,
    service: String,
    #[serde(deserialize_with = "deserialize_utc_from_missing_z")]
    indexed: DateTime<Utc>,
    #[serde(deserialize_with = "deserialize_utc_from_missing_z")]
    updated: DateTime<Utc>,
    public_id: String,
    relation_id: u64,
    has_chats: bool,
    post_count: u64,
    dm_count: u64,
    share_count: u64,
    chat_count: u64,
}
