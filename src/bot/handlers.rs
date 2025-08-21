use super::TokenApiClientKey;
use crate::api::traits::TokenApiClient;
use crate::db::database::Database;
use crate::utils::{
    message_parser::{extract_sol_token_address, extract_token_symbol},
    token_message_builder::TokenMessageBuilder,
};
use serenity::{
    all::{Context, EventHandler, Message, Ready},
    async_trait,
};
use std::sync::Arc;
use tracing::{error, info};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, new_message: Message) {
        let author_id = new_message.author.id;
        let guild_id = match new_message.guild_id {
            Some(id) => i64::from(id),
            None => {
                error!("Failed to get \"Guild Id\" from message");
                return;
            }
        };

        match ctx.http.get_current_user().await {
            Ok(current_user) => {
                if current_user.id == author_id {
                    return;
                }
            }
            Err(error) => {
                error!("{}", error);
                return;
            }
        }

        let msg_content = &new_message.content;

        if let Some(token_identifier) =
            extract_token_symbol(msg_content).or_else(|| extract_sol_token_address(msg_content))
        {
            let data = ctx.data.read().await;
            let token_api_client: Arc<dyn TokenApiClient> = Arc::clone(
                data.get::<TokenApiClientKey>()
                    .expect("Expected Jupiter Api Client in TypeMap"),
            );
            let database = Arc::clone(
                &data
                    .get::<Database>()
                    .expect("Expected Database in TypeMap"),
            );

            let token_info_opt = match token_api_client.fetch_token_info(token_identifier).await {
                Ok(value) => value,
                Err(error) => {
                    error!("{}", error);
                    return;
                }
            };

            if let Some(token_info) = token_info_opt {
                let token_msg_builder = TokenMessageBuilder::new(
                    Arc::clone(&ctx.http),
                    token_info,
                    Arc::clone(&database),
                );

                match token_msg_builder
                    .build(guild_id as u64, &new_message.author)
                    .await
                {
                    Ok(msg) => {
                        if let Err(error) = new_message
                            .channel_id
                            .send_message(&ctx.http, msg.reference_message(&new_message))
                            .await
                        {
                            error!("{}", error);
                            //SEND EPHEMERAL
                        }
                    }
                    Err(e) => {
                        error!("{}", e);
                        //SEND EPHEMERAL
                    }
                }
            }
        }

        if msg_content == "!test" {
            if let Err(why) = new_message.reply(&ctx.http, "pong").await {
                error!("{}", why);
            }
        }
    }

    async fn ready(&self, _ctx: Context, _data_about_bot: Ready) {
        info!("Discord Bot is ready");
        //TODO ADD INVITE LINK TO LOGS
    }
}
