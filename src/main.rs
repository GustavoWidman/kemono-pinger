mod bot;
mod utils;

use clap::Parser;
use colored::Colorize;
use eyre::Result;
use log::info;

use crate::bot::manager::Manager;
use crate::utils::cli::Args;
use crate::utils::config::{Config, config};
use crate::utils::log::Logger;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    Logger::init(args.verbosity);

    info!(
        "starting kemono-pinger {}",
        format!("v{}", env!("CARGO_PKG_VERSION")).magenta()
    );

    let config: Config = config(args.config)?;
    let manager = Manager::new(config).await?;

    manager.run().await?;

    Ok(())
}
