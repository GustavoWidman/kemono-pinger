use std::time::Duration;
use backon::{ExponentialBuilder, Retryable};
use colored::Colorize;
use eyre::Result;
use log::{debug, error, info, trace, warn};
use reqwest::Client;

use crate::{
    bot::{Response, notifier::Event},
    utils::{config::Config, gunzip},
};

pub struct Requester {
    last: Option<Response>,
    config: Config,
    client: Client,
}

impl Requester {
    pub fn new(config: Config) -> Result<Self> {
        let requester = Self {
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(30))
                .user_agent("Mozilla/5.0 (compatible; KemonoPinger/1.0)")
                .pool_max_idle_per_host(0)
                .build()?,
            last: None,
            config,
        };

        info!("{} initialized successfully", "requester".green());

        Ok(requester)
    }

    async fn query(&self) -> Result<Response> {
        let response = (|| async {
            self.client
                .get(format!(
                    "https://kemono.cr/api/v1/{}/user/{}/profile",
                    self.config.requester.service, self.config.requester.creator_id
                ))
                .header("Accept", "text/css")
                .header("Accept-Encoding", "gzip, deflate")
                .header("Connection", "close")
                .send()
                .await
        })
        .retry(
            ExponentialBuilder::default()
                .with_max_times(5)
                .with_jitter(),
        )
        .notify(|err: &reqwest::Error, dur: Duration| {
            warn!(
                "failed to send request, retrying after {:?}:\n{:#?}",
                dur, err
            );
        })
        .await
        .map_err(|e| {
            error!("failed to send request, exceeded max retries:\n{:#?}", e);
            e
        })?;

        trace!(
            "response headers:\n{:#?}",
            response.headers().iter().collect::<Vec<_>>()
        );
        debug!("response status: {}", response.status());

        let possible_err = response.error_for_status_ref().map(|_| ());

        let decompressed = gunzip(response).await?;

        trace!("decompressed response:\n{}", decompressed);

        possible_err.map_err(|e| {
            error!(
                "request returned error status {}: {}\nresponse body:\n{}",
                e.status().unwrap_or_default().to_string().red(),
                e.to_string().red(),
                decompressed
            );
            e
        })?;

        serde_json::from_str(&decompressed).map_err(Into::into)
    }

    pub async fn tick(&mut self) -> Option<Event> {
        self.query()
            .await
            .map(|new| {
                self.last
                    .take()
                    .filter(|old| new.ne(old))
                    .map(|old| {
                        info!("took out old {:?}, replacing with {:?}", old, new);

                        old.updated
                            .ne(&new.updated)
                            .then_some(Event::Updated)
                            .or(old.indexed.ne(&new.indexed).then_some(Event::Indexed))
                            .unwrap_or(Event::Other)
                    })
                    .map(|event| {
                        self.last.replace(new);
                        event
                    })
            })
            .unwrap_or_else(|e| Some(e.into()))
    }
}
