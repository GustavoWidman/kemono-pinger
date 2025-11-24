use std::ops::Sub;

use chrono::Utc;
use colored::Colorize;
use eyre::Result;
use log::{debug, info};
use tokio::time::sleep;

use crate::bot::notifier::Notifier;
use crate::bot::requester::Requester;
use crate::utils::config::Config;

pub struct Manager {
    config: Config,
    requester: Requester,
    notifier: Notifier,
}

impl Manager {
    pub async fn new(config: Config) -> Result<Self> {
        let manager = Self {
            requester: Requester::new(config.clone())?,
            notifier: Notifier::new(&config).await?,
            config,
        };

        info!("{}   initialized successfully", "manager".red());

        Ok(manager)
    }

    pub async fn run(mut self) -> Result<()> {
        loop {
            let tick_start = Utc::now();
            debug!(
                "starting tick at {}",
                tick_start.format("%Y-%m-%d %H:%M:%S.%6f").to_string()
            );

            if let Some(event) = self.requester.tick().await {
                debug!("notifying event: {:?}", event);
                self.notifier.notify(event).await?;
            }

            let tick_duration = Utc::now().sub(tick_start).to_std()?;

            debug!(
                "tick duration: {}ms",
                tick_duration.as_millis().to_string().cyan()
            );

            let delay = self.config.requester.delay_ms.sub(tick_duration);

            debug!(
                "tick complete, sleeping for {}ms",
                delay.as_millis().to_string().cyan()
            );

            sleep(delay).await;
        }
    }
}
