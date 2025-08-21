use crate::api::{
    errors::ApiErrors,
    models::{Stats, TokenInfo},
};
use crate::db::{
    database::Database,
    models::{Guild, NewTokenScan, Token, User as DbUser},
};
use anyhow::Error;
use chrono::{DateTime, Utc};
use serenity::all::{Color, CreateEmbed, CreateEmbedFooter, CreateMessage, User, UserId};
use serenity::http;
use std::sync::Arc;

pub struct TokenMessageBuilder {
    pub http: Arc<http::Http>,
    pub token_info: TokenInfo,
    pub database: Arc<Database>,
}

impl TokenMessageBuilder {
    pub fn new(http: Arc<http::Http>, token_info: TokenInfo, database: Arc<Database>) -> Self {
        Self {
            http,
            token_info,
            database,
        }
    }

    pub async fn build(&self, guild_id: u64, author: &User) -> Result<CreateMessage, Error> {
        let content = format!(
            "**{}** - **${}**",
            self.token_info.name, self.token_info.symbol
        );
        let description = Self::build_description(self)?;
        let footer = Self::build_footer(self, guild_id, author).await?;
        let embed = CreateEmbed::new()
            .color(Color::BLURPLE)
            .description(description)
            .footer(footer);

        Ok(CreateMessage::new().content(content).add_embed(embed))
    }

    fn build_description(&self) -> Result<String, Error> {
        let mut embed_fields: Vec<String> = vec![];

        if let Some(launchpad) = self.token_info.launchpad.clone().as_mut() {
            embed_fields.push(match launchpad {
                launch if launch.as_str() == "pump.fun" => String::from("💊 Solana @ PumpFun"),
                launch if launch.as_str() == "letsbonk.fun" => String::from("🐶 Solana @ LetsBonk"),
                _ => format!("🌐 Solana @ {}", launchpad),
            });
        }

        let usd_price = self
            .token_info
            .usd_price
            .ok_or(ApiErrors::MissingData(String::from("\"Usd Price\"")))?;

        let fdv = self
            .token_info
            .fdv
            .ok_or(ApiErrors::MissingData(String::from("\"FDV\"")))?;

        let liquidity = self
            .token_info
            .liquidity
            .ok_or(ApiErrors::MissingData(String::from("\"Liquidity\"")))?;

        let first_pool = self
            .token_info
            .first_pool
            .as_ref()
            .ok_or(ApiErrors::MissingData(String::from("\"First Pool\"")))?;

        let token_age = Self::format_duration(&first_pool.created_at);

        embed_fields.push(format!("💰 USD: `${}`", Self::format_price(usd_price)));
        embed_fields.push(format!("💎 FDV: `${}`", Self::to_short_scale(fdv)));
        embed_fields.push(format!("💦 Liq: `${}`", Self::to_short_scale(liquidity)));
        embed_fields.push(format!("🕰️ Age: `{}`", token_age));

        if let Some(stats) = self.token_info.stats1h.as_ref() {
            if let Some(stats1h) = Self::format_hourly_stats(stats) {
                embed_fields.push(stats1h);
            }
        }

        if let Some(socials_str) = Self::format_socials(
            self.token_info.twitter.as_ref(),
            self.token_info.telegram.as_ref(),
            self.token_info.website.as_ref(),
            self.token_info.dev.as_ref(),
        ) {
            embed_fields.push(format!("💼 Socials: {}", socials_str));
        }

        embed_fields.push(format!(
            "💹 Chart: [Dex](https://dexscreener.com/solana/{}) ⋅ [DEF](https://www.defined.fi/sol/{})",
            self.token_info.id, self.token_info.id
        ));

        let mut description = embed_fields.join("\n");
        description.push_str(format!("\n\n`{}`", self.token_info.id).as_str());

        Ok(description)
    }

    async fn build_footer(&self, guild_id: u64, author: &User) -> Result<CreateEmbedFooter, Error> {
        let token_scan_opt = self
            .database
            .get_token_scan(self.token_info.id.as_ref(), guild_id)
            .await?;

        let footer = match token_scan_opt {
            Some(token_scan) => {
                let user = self.http.get_user(UserId::from(token_scan.user_id)).await?;

                let mut footer = CreateEmbedFooter::new(format!(
                    "{} 🏆 {} @ {} ⋅ {}",
                    author.name,
                    user.name,
                    Self::to_short_scale(token_scan.fdv),
                    Self::format_duration(&token_scan.scanned_at)
                ));

                if let Some(icon_url) = user.avatar_url() {
                    footer = footer.icon_url(icon_url);
                }

                footer
            }
            None => {
                let guild = Guild { guild_id };

                let user = DbUser {
                    user_id: u64::from(author.id),
                };

                let token = Token {
                    token_id: self.token_info.id.clone(),
                    name: self.token_info.name.clone(),
                    symbol: self.token_info.symbol.clone(),
                };

                let token_scan = NewTokenScan {
                    user_id: user.user_id,
                    guild_id: guild.guild_id,
                    token_id: token.token_id.clone(),
                    fdv: self
                        .token_info
                        .fdv
                        .ok_or(ApiErrors::MissingData(String::from("\"FDV\"")))?,
                };

                self.database
                    .insert_token_scan(&guild, &user, &token, &token_scan)
                    .await?;

                let mut footer = CreateEmbedFooter::new(format!(
                    "{} 💨 You are first! @ {}",
                    author.name,
                    Self::to_short_scale(token_scan.fdv)
                ));

                if let Some(icon_url) = author.avatar_url() {
                    footer = footer.icon_url(icon_url);
                }

                footer
            }
        };

        Ok(footer)
    }

    fn format_hourly_stats(stats: &Stats) -> Option<String> {
        let price_change = stats.price_change.unwrap_or_default();
        let buy_volume = stats.buy_volume.unwrap_or_default();
        let sell_volume = stats.sell_volume.unwrap_or_default();
        let volume = Self::to_short_scale(buy_volume + sell_volume);
        let num_buys = stats.num_buys.unwrap_or_default();
        let num_sells = stats.num_sells.unwrap_or_default();

        Some(format!(
            "📈 1H: `{:.1}%` ⋅ `${}` 🅑 `{}` Ⓢ `{}`",
            price_change, volume, num_buys, num_sells
        ))
    }

    fn format_price(price: f64) -> String {
        match price {
            p if p >= 1000.0 => format!("{:.0}", p),
            p if p >= 1.0 => format!("{:.2}", p),
            p if p >= 0.1 => format!("{:.4}", p),
            p if p >= 0.01 => format!("{:.5}", p),
            p if p >= 0.001 => format!("{:.6}", p),
            p if p >= 0.0001 => format!("{:.7}", p),
            p if p >= 0.00001 => format!("{:.8}", p),
            _ => format!("{:.9}", price),
        }
    }

    fn to_short_scale(num: f64) -> String {
        match num {
            n if n > 1000000000.0 => format!("{:.1}B", n / 1000000000.0),
            n if n > 1000000.0 => format!("{:.2}M", n / 1000000.0),
            n if n > 1000.0 => format!("{:.2}K", n / 1000.0),
            _ => format!("{:.0}", num),
        }
    }

    fn format_duration(scanned_at: &DateTime<Utc>) -> String {
        let duration = Utc::now().signed_duration_since(scanned_at);
        match duration {
            d if d.num_seconds() < 60 => format!("{}s", duration.num_seconds()),
            d if d.num_minutes() < 60 => format!("{}m", duration.num_minutes()),
            d if d.num_hours() < 24 => format!("{}h", duration.num_hours()),
            d if d.num_days() < 7 => format!("{}d", duration.num_days()),
            d if d.num_weeks() < 5 => format!("{}w", duration.num_weeks()),
            d if d.num_weeks() < 53 => format!("{}mo", duration.num_weeks() / 4),
            _ => format!("{}y", duration.num_weeks() / 52),
        }
    }

    fn format_socials(
        twitter: Option<&String>,
        telegram: Option<&String>,
        website: Option<&String>,
        dev: Option<&String>,
    ) -> Option<String> {
        let mut links: Vec<String> = vec![];

        if let Some(twt) = twitter {
            links.push(format!("[𝕏]({})", twt));
        }

        if let Some(tg) = telegram {
            links.push(format!("[Tg]({})", tg));
        }

        if let Some(web) = website {
            links.push(format!("[Web]({})", web));
        }

        if let Some(dev_wallet) = dev {
            links.push(format!("[Dev](https://solscan.io/account/{})", dev_wallet));
        }

        if links.is_empty() {
            return None;
        }

        Some(links.join(" ⋅ "))
    }
}
