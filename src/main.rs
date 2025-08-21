use anyhow::{Error, Result};
use dotenv::dotenv;
use std::env;
use tracing::{Level, info};
use tracing_subscriber::FmtSubscriber;

mod api;
mod bot;
mod db;
mod utils;

struct Config {
    token: String,
}

impl Config {
    async fn load() -> Result<Self, Error> {
        dotenv().ok();

        let token = env::var("DISCORD_TOKEN")
            .expect("Missing `DISCORD_TOKEN` env var, see README for more information.");

        Ok(Self { token })
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let fmt_subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(fmt_subscriber).unwrap();

    let cfg = Config::load().await?;

    info!("Running discord bot");
    bot::run(cfg.token).await?;

    Ok(())
}
