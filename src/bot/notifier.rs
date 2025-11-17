use chrono::Utc;
use colored::Colorize;
use eyre::{Report, Result};
use log::info;
use serenity::all::{CreateEmbed, CreateEmbedFooter, ExecuteWebhook, Http, Webhook};

use crate::utils::config::Config;

#[derive(Debug)]
pub enum Event {
    Error(String),
    Updated,
    Indexed,
    Other,
}

impl Into<CreateEmbed> for Event {
    fn into(self) -> CreateEmbed {
        let builder = CreateEmbed::new()
            .timestamp(Utc::now())
            .footer(CreateEmbedFooter::new("kemono-pinger by r3dlust"));

        let builder = match self {
            Event::Error(err) => builder
                .color(15158332)
                .title("error while querying")
                .description(format!("the requester ran into an error:\n```{err}```")),
            Event::Indexed => builder
                .color(3066993)
                .title("service reindexed")
                .description("the requester has detected a service index update, go check it out"),
            Event::Updated => builder
                .color(3066993)
                .title("service updated")
                .description("the requester has detected a service update, go check it out"),
            Event::Other => builder
                .color(3066993)
                .title("unknown update")
                .description(
                    "the requester has detected an update but could not make out what was updated. go check it out",
                ),
        };

        builder
    }
}

impl From<Report> for Event {
    fn from(value: Report) -> Self {
        Self::Error(value.to_string())
    }
}

pub struct Notifier {
    webhook: Webhook,
    http: Http,
}

impl Notifier {
    pub async fn new(config: &Config) -> Result<Self> {
        let http = Http::new("");
        let notifier = Self {
            webhook: Webhook::from_url(&http, &config.webhook.url.as_str()).await?,
            http,
        };

        info!("{}  initialized successfully", "notifier".blue());

        Ok(notifier)
    }

    pub async fn notify(&self, event: Event) -> Result<()> {
        self.webhook
            .execute(&self.http, true, ExecuteWebhook::new().embed(event.into()))
            .await
            .map_err(Into::into)
            .map(|_| ())
    }
}
