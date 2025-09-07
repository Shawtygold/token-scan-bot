use crate::api::models::{SolTokenInfo, TokenLinks, TokenStats1H};
use crate::db::models::ScanType;
use anyhow::Error;
use chrono::{DateTime, Utc};
use serenity::{
    all::{Color, CreateEmbed, CreateEmbedFooter, CreateMessage, User, UserId},
    http,
};
use std::sync::Arc;

pub struct TokenMessageBuilder {
    pub http: Arc<http::Http>,
    pub token_info: SolTokenInfo,
}

impl TokenMessageBuilder {
    pub fn new(http: Arc<http::Http>, token_info: SolTokenInfo) -> Self {
        Self { http, token_info }
    }

    pub async fn build(&self, author: &User, scan_type: ScanType) -> Result<CreateMessage, Error> {
        let content = self.build_content();
        let description = self.build_description()?;
        let footer = self.build_footer(author, scan_type).await?;

        let embed = CreateEmbed::new()
            .color(Color::BLURPLE)
            .description(description)
            .footer(footer);

        Ok(CreateMessage::new().content(content).add_embed(embed))
    }

    fn build_content(&self) -> String {
        let launchpad_icon: Option<&str> = if let Some(launchpad) = &self.token_info.launchpad {
            match launchpad.to_lowercase() {
                l if l.contains("pump") => Some("💊"),
                l if l.contains("bonk") => Some("🐶"),
                _ => None,
            }
        } else {
            None
        };

        let mut content = format!(
            "**{} [{}/{:.1}%] - ${}**",
            self.token_info.name,
            Self::to_short_scale(self.token_info.fully_diluted_value),
            self.token_info.stats_24h.price_percent_change,
            self.token_info.symbol
        );

        if let Some(icon) = launchpad_icon {
            content.insert_str(0, &format!("{} ", icon));
        }

        content
    }

    fn build_description(&self) -> Result<String, Error> {
        let mut embed_fields: Vec<String> = vec![];

        let mint = &self.token_info.mint;
        let token_links = &self.token_info.links;
        let dev_address = self.token_info.dev.as_ref();
        let holder_count = self.token_info.holder_count;
        let fdv = self.token_info.fully_diluted_value;
        let usd_price = self.token_info.usd_price;
        let liquidity_usd = self.token_info.liquidity_usd;
        let token_stats_1h = &self.token_info.stats_1h;
        let exchange_name = &self.token_info.token_pair_exchange_name;

        embed_fields.push(format!("🌐 Solana @ {}", exchange_name));
        embed_fields.push(format!("💰 USD: `${}`", Self::format_price(usd_price)));
        embed_fields.push(format!("💎 FDV: `${}`", Self::to_short_scale(fdv)));
        embed_fields.push(format!(
            "💦 Liq: `${}`",
            Self::to_short_scale(liquidity_usd)
        ));
        // embed_fields.push(format!("🕰️ Age: `{}`", token_age));
        embed_fields.push(Self::format_hourly_stats(token_stats_1h));
        embed_fields.push(String::from(""));
        embed_fields.push(format!(
            "🤝 Total: `{}`",
            Self::to_short_scale(holder_count as f64)
        ));

        if let Some(socials) = Self::format_socials(token_links, dev_address) {
            embed_fields.push(format!("💼 Socials: {}", socials));
        }

        embed_fields.push(format!(
            "💹 Chart: [DEX](https://dexscreener.com/solana/{}) ⋅ [DEF](https://www.defined.fi/sol/{})",
            mint, mint
        ));

        let mut description = embed_fields.join("\n");
        description.push_str(format!("\n\n`{}`", mint).as_str());

        Ok(description)
    }

    async fn build_footer(
        &self,
        author: &User,
        scan_type: ScanType,
    ) -> Result<CreateEmbedFooter, Error> {
        let mut footer = match scan_type {
            ScanType::FirstScan(token_scan) => CreateEmbedFooter::new(format!(
                "{} 💨 You are first! @ {}",
                author.display_name(),
                Self::to_short_scale(token_scan.fdv)
            )),
            ScanType::Scanned(token_scan) => CreateEmbedFooter::new(format!(
                "{} 🏆 {} @ {} ⋅ {}",
                author.display_name(),
                self.http.get_user(UserId::from(token_scan.user_id)).await?.display_name(),
                Self::to_short_scale(token_scan.fdv),
                Self::format_duration(&token_scan.scanned_at)
            )),
        };

        if let Some(avatar_url) = author.avatar_url() {
            footer = footer.icon_url(avatar_url);
        }

        Ok(footer)
    }

    fn format_hourly_stats(pair_stats_1h: &TokenStats1H) -> String {
        let price_change = pair_stats_1h.price_percent_change;
        let buy_volume = pair_stats_1h.buy_volume;
        let sell_volume = pair_stats_1h.sell_volume;
        let volume = Self::to_short_scale(buy_volume + sell_volume);
        let buys = pair_stats_1h.buys;
        let sells = pair_stats_1h.sells;

        format!(
            "📈 1H: `{:.1}%` ⋅ `${}` 🅑 `{}` Ⓢ `{}`",
            price_change, volume, buys, sells
        )
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
            n if n > 1000000.0 => format!("{:.1}M", n / 1000000.0),
            n if n > 1000.0 => format!("{:.1}K", n / 1000.0),
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

    fn format_socials(token_links: &TokenLinks, dev_address: Option<&String>) -> Option<String> {
        let mut links: Vec<String> = vec![];

        if let Some(twitter) = &token_links.twitter {
            links.push(format!("[𝕏]({})", twitter));
        }

        if let Some(discord) = &token_links.discord {
            links.push(format!("[𝕏]({})", discord));
        }

        if let Some(reddit) = &token_links.reddit {
            links.push(format!("[𝕏]({})", reddit));
        }

        if let Some(telegram) = &token_links.telegram {
            links.push(format!("[Tg]({})", telegram));
        }

        if let Some(website) = &token_links.website {
            links.push(format!("[Web]({})", website));
        }

        if let Some(dev_wallet) = dev_address {
            links.push(format!("[Dev](https://solscan.io/account/{})", dev_wallet));
        }

        if links.is_empty() {
            return None;
        }

        Some(links.join(" ⋅ "))
    }
}
