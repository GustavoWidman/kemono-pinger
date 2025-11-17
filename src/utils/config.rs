use std::sync::Arc;
use std::{path::PathBuf, time::Duration};

use easy_config_store::ConfigStore;
use eyre::Result;
use log::{debug, info};
use serde::{Deserialize, Serialize};
use url::Url;

pub type Config = Arc<ConfigStore<ConfigInner>>;
pub fn config(path: PathBuf) -> Result<Config> {
    let config = ConfigStore::<ConfigInner>::read(path, "settings".to_string())?;

    info!("config parsing successful");
    debug!(
        "loaded configuration:\n{}",
        toml::to_string_pretty(&*config)?
    );

    Ok(config.arc())
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ConfigInner {
    pub webhook: WebhookConfig,
    pub requester: RequesterConfig,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct WebhookConfig {
    pub url: Url,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct RequesterConfig {
    pub service: String,
    pub creator_id: String,

    /// the delay between each requester tick
    #[serde(
        deserialize_with = "millis_to_duration",
        serialize_with = "duration_to_millis"
    )]
    pub delay_ms: Duration,
}

impl Default for ConfigInner {
    fn default() -> Self {
        let cfg = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/config.default.toml",));

        toml::from_str(&cfg).unwrap() // should be okay
    }
}

fn millis_to_duration<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
    D: serde::Deserializer<'de>,
{
    // You can change u64 to u32/f64/etc. depending on your expected range
    u64::deserialize(deserializer).map(Duration::from_millis)
}
fn duration_to_millis<S>(dur: &Duration, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let millis = dur.as_millis() as u64; // or .as_millis() as u128 if you need huge values
    millis.serialize(serializer)
}
