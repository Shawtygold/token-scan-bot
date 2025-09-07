use anyhow::{Error, Result};
use dotenv::dotenv;
use std::env;
use tracing::{Level, info};
use tracing_subscriber::FmtSubscriber;

mod api;
mod bot;
mod db;
mod utils;
mod errors;

struct Config {
    discord_token: String,
    moralis_api_key: String,
}

impl Config {
    async fn load() -> Result<Self, Error> {
        dotenv().ok();

        let discord_token = env::var("DISCORD_TOKEN")
            .expect("Missing `DISCORD_TOKEN` env var, see README for more information.");

        let moralis_api_key = env::var("MORALIS_API_KEY").expect("Missing Moralis Api Key");

        Ok(Self {
            discord_token,
            moralis_api_key,
        })
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
    bot::run(cfg.discord_token, cfg.moralis_api_key).await?;

    Ok(())
}
