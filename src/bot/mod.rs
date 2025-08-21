use crate::{
    api::{jupiter_api_client::JupiterApiClient, traits::TokenApiClient},
    db::database::Database,
};
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

pub async fn run(token: String) -> Result<(), anyhow::Error> {
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

    let mut client = Client::builder(token, intents)
        .framework(framework)
        .event_handler(handler)
        .await?;

    let database: Arc<Database> = Arc::new(Database::new(
        "postgresql://postgres:CyfQRh0SGG5g4@localhost/TokenScansDB".to_string(),
    )?);

    let token_api_client: Arc<dyn TokenApiClient> = Arc::new(JupiterApiClient::new());

    {
        let mut data = client.data.write().await;
        data.insert::<TokenApiClientKey>(token_api_client);
        data.insert::<Database>(database);
    }

    client.start().await?;

    Ok(())
}

struct TokenApiClientKey {}

impl TypeMapKey for TokenApiClientKey {
    type Value = Arc<dyn TokenApiClient>;
}

impl TypeMapKey for Database {
    type Value = Arc<Database>;
}
