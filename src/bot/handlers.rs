use crate::api::{
    jupiter::{jupiter_api_client::JupiterApiClient, models::TokenData},
    models::{SolTokenInfo, TokenLinks, TokenStats1H, TokenStats24H},
    moralis::{
        constants::{PUMP_SWAP_ADDRESS, RAYDIUM_CPMM_ADDRESS},
        models::TokenPair,
        moralis_api_client::MoralisApiClient,
    },
};
use crate::db::{
    database::Database,
    models::{Guild, NewTokenScan, ScanType, Token, User},
};
use crate::utils::{
    message_parser::{extract_sol_token_address, extract_token_symbol},
    token_message_builder::TokenMessageBuilder,
};
use serenity::{
    all::{Context, EventHandler, Message, Ready},
    async_trait,
};
use std::{sync::Arc, time::Instant};
use tokio::join;
use tracing::{error, info};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, new_message: Message) {
        let author_id = new_message.author.id;
        let guild_id = match new_message.guild_id {
            Some(id) => u64::from(id),
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

        let token_address_opt = match extract_sol_token_address(msg_content)
            // .or(extract_token_symbol(msg_content)) 
            {
                Ok(token_address) => token_address,
                Err(e) => {
                    error!("{}", e);
                    None
                }
            };

        if let Some(token_address) = token_address_opt {
            let data = ctx.data.read().await;
            let moralis_api_client: Arc<MoralisApiClient> = Arc::clone(
                data.get::<MoralisApiClient>()
                    .expect("Expected Moralis Api Client in TypeMap"),
            );
            let jupiter_api_client: Arc<JupiterApiClient> = Arc::clone(
                data.get::<JupiterApiClient>()
                    .expect("Expected Jupiter Api Client in TypeMap"),
            );
            let database = Arc::clone(
                data.get::<Database>()
                    .expect("Expected Database in TypeMap"),
            );

            let start = Instant::now();

            let client = Arc::clone(&moralis_api_client);
            let token_address1 = token_address.clone();
            let fut_metadata = async move { client.get_token_metadata(&token_address1).await };

            let client = Arc::clone(&moralis_api_client);
            let token_address2 = token_address.clone();
            let fut_pair_stats = async move {
                let primary_token_pair = client
                    .get_primary_token_pair_by_address(&token_address2)
                    .await?;          

                client
                    .get_token_pair_stats(&primary_token_pair.pair_address)
                    .await
            };

            let client = Arc::clone(&moralis_api_client);
            let token_address3 = token_address.clone();
            let fut_holder_stats = async move { client.get_token_holders(&token_address3).await };

            let client = Arc::clone(&jupiter_api_client);
            let token_address4 = token_address.clone();
            let fut_jup_token_data = async move { client.fetch_token_info(&token_address4).await };

            let (
                token_metadata_res,
                token_pair_stats_res,
                token_holder_stats_res,
                jup_token_data_res,
            ) = join!(
                fut_metadata,
                fut_pair_stats,
                fut_holder_stats,
                fut_jup_token_data
            );

            let jup_token_data = match jup_token_data_res {
                Ok(token_data) => token_data,
                Err(e) => {
                    error!("{}", e);
                    TokenData::default()
                }
            };

            let token_metadata = match token_metadata_res {
                Ok(metadata) => metadata,
                Err(e) => {
                    error!("{}", e);
                    return;
                }
            };

            let token_pair_stats = match token_pair_stats_res {
                Ok(token_pair_stats) => token_pair_stats,
                Err(e) => {
                    error!("{}", e);
                    return;
                }
            };

            let token_holder_stats = match token_holder_stats_res {
                Ok(holder_stats) => holder_stats,
                Err(e) => {
                    error!("{}", e);
                    return;
                }
            };

            let duration = start.elapsed();
            println!("{:?}", duration);

            let sol_token_info = SolTokenInfo {
                mint: token_metadata.mint,
                name: token_metadata.name,
                symbol: token_metadata.symbol,
                logo: token_metadata.logo,
                links: TokenLinks {
                    discord: token_metadata.links.discord,
                    telegram: token_metadata.links.telegram,
                    reddit: token_metadata.links.reddit,
                    twitter: token_metadata.links.twitter,
                    website: token_metadata.links.website,
                },
                dev: jup_token_data.dev,
                launchpad: jup_token_data.launchpad,
                token_pair_exchange_name: token_pair_stats.exchange,
                holder_count: token_holder_stats.total_holders,
                fully_diluted_value: token_metadata.fully_diluted_value.parse::<f64>().unwrap(),
                liquidity_usd: token_pair_stats.total_iquidity_usd.parse::<f64>().unwrap(),
                usd_price: token_pair_stats.current_usd_price.parse::<f64>().unwrap(),
                stats_1h: TokenStats1H {
                    buys: token_pair_stats.buys.h1,
                    sells: token_pair_stats.sells.h1,
                    buy_volume: token_pair_stats.buy_volume.h1,
                    sell_volume: token_pair_stats.sell_volume.h1,
                    price_percent_change: token_pair_stats.price_percent_change.h1,
                },
                stats_24h: TokenStats24H {
                    price_percent_change: token_pair_stats.price_percent_change.h24,
                },
            };

            let scan_type = match database
                .get_token_scan(&sol_token_info.mint, guild_id)
                .await
            {
                Ok(token_scan_opt) => match token_scan_opt {
                    Some(token_scan) => ScanType::Scanned(token_scan),
                    None => {
                        let token_info = sol_token_info.clone();

                        let guild = Guild { guild_id };

                        let user = User {
                            user_id: u64::from(new_message.author.id),
                        };

                        let token = Token {
                            token_id: token_info.mint,
                            name: token_info.name,
                            symbol: token_info.symbol,
                        };

                        let new_token_scan = NewTokenScan {
                            user_id: user.user_id,
                            guild_id: guild.guild_id,
                            token_id: token.token_id.clone(),
                            fdv: token_info.fully_diluted_value,
                        };

                        if let Err(e) = database
                            .insert_token_scan(&guild, &user, &token, &new_token_scan)
                            .await
                        {
                            error!("{}", e);
                            return;
                        }

                        ScanType::FirstScan(new_token_scan)
                    }
                },
                Err(e) => {
                    error!("{}", e);
                    return;
                }
            };

            let token_msg_builder = TokenMessageBuilder::new(Arc::clone(&ctx.http), sol_token_info);

            match token_msg_builder
                .build(&new_message.author, scan_type)
                .await
            {
                Ok(msg) => {
                    if let Err(error) = new_message
                        .channel_id
                        .send_message(&ctx.http, msg.reference_message(&new_message))
                        .await
                    {
                        error!("{}", error);
                    }
                }
                Err(e) => {
                    error!("{}", e);
                    return;
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
    }
}
