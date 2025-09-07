use crate::api::moralis::moralis_api_client::{self, MoralisApiClient};
use crate::{api::jupiter::jupiter_api_client::JupiterApiClient, db::database::Database};
use anyhow::Result;
use handlers::Handler;
use serenity::prelude::TypeMapKey;
use serenity::{Client, all::GatewayIntents};
use std::sync::Arc;

struct Data {}

// Error type produced by commands
type Error = Box<dyn std::error::Error + Send + Sync>;

// Context type for commands
type Context<'a> = poise::Context<'a, Data, Error>;

mod commands;
mod handlers;

pub async fn run(discord_token: String, moralis_api_key: String) -> Result<(), anyhow::Error> {
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![commands::ping()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let intents = GatewayIntents::all();

    let handler = Handler {};

    let mut client = Client::builder(discord_token, intents)
        .framework(framework)
        .event_handler(handler)
        .await?;

    let database: Arc<Database> = Arc::new(Database::new(
        "postgresql://postgres:CyfQRh0SGG5g4@localhost/TokenScansDB".to_string(),
    )?);

    let jupiter_api_client: Arc<JupiterApiClient> = Arc::new(JupiterApiClient::new());
    let moralis_api_client: Arc<MoralisApiClient> =
        Arc::new(MoralisApiClient::new(&moralis_api_key)?);

    {
        let mut data = client.data.write().await;
        data.insert::<JupiterApiClient>(jupiter_api_client);
        data.insert::<MoralisApiClient>(moralis_api_client);
        data.insert::<Database>(database);
    }

    client.start().await?;

    Ok(())
}

impl TypeMapKey for MoralisApiClient {
    type Value = Arc<MoralisApiClient>;
}

impl TypeMapKey for JupiterApiClient {
    type Value = Arc<JupiterApiClient>;
}

impl TypeMapKey for Database {
    type Value = Arc<Database>;
}
