use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct TokenMetadata {
    #[validate(length(min = 32, max = 44))]
    pub mint: String,
    #[validate(length(min = 1))]
    pub name: String,
    #[validate(length(min = 1))]
    pub symbol: String,
    #[validate(url)]
    pub logo: String,
    #[validate(length(min = 1))]
    #[serde(rename = "fullyDilutedValue")]
    pub fully_diluted_value: String,
    pub links: Links,
}

#[derive(Debug, Deserialize)]
pub struct Links {
    pub discord: Option<String>,
    pub telegram: Option<String>,
    pub reddit: Option<String>,
    pub twitter: Option<String>,
    pub website: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TokenPairs {
    pub pairs: Vec<TokenPair>,
}

#[derive(Debug, Deserialize, Validate, Clone)]
pub struct TokenPair {
    #[validate(length(min = 1))]
    #[serde(rename = "exchangeName")]
    pub exchange_name: String,
    #[validate(length(min = 32, max = 44))]
    #[serde(rename = "exchangeAddress")]
    pub exchange_address: String,
    #[validate(length(min = 32, max = 44))]
    #[serde(rename = "pairAddress")]
    pub pair_address: String,
    #[serde(rename = "inactivePair")]
    pub inactive_pair: bool,
}

#[derive(Debug, Deserialize, Validate)]
pub struct TokenPairStats {
    #[validate(length(min = 1))]
    pub exchange: String,
    #[validate(length(min = 1))]
    #[serde(rename = "totalLiquidityUsd")]
    pub total_iquidity_usd: String,
    #[validate(length(min = 1))]
    #[serde(rename = "currentUsdPrice")]
    pub current_usd_price: String,
    #[serde(rename = "pricePercentChange")]
    pub price_percent_change: PricePercentageChange,
    pub buys: Buys,
    pub sells: Sells,
    #[serde(rename = "buyVolume")]
    pub buy_volume: BuyVolume,
    #[serde(rename = "sellVolume")]
    pub sell_volume: SellVolume,
}

#[derive(Debug, Deserialize, Validate)]
pub struct PricePercentageChange {
    #[validate(range(min = 0.0))]
    #[serde(rename = "5min")]
    pub min5: f64,
    #[validate(range(min = 0.0))]
    #[serde(rename = "1h")]
    pub h1: f64,
    #[validate(range(min = 0.0))]
    #[serde(rename = "4h")]
    pub h4: f64,
    #[validate(range(min = 0.0))]
    #[serde(rename = "24h")]
    pub h24: f64,
}

#[derive(Debug, Deserialize)]
pub struct Buys {
    #[serde(rename = "5min")]
    pub min5: u32,
    #[serde(rename = "1h")]
    pub h1: u32,
    #[serde(rename = "4h")]
    pub h4: u32,
    #[serde(rename = "24h")]
    pub h24: u32,
}

#[derive(Debug, Deserialize)]
pub struct Sells {
    #[serde(rename = "5min")]
    pub min5: u32,
    #[serde(rename = "1h")]
    pub h1: u32,
    #[serde(rename = "4h")]
    pub h4: u32,
    #[serde(rename = "24h")]
    pub h24: u32,
}

#[derive(Debug, Deserialize, Validate)]
pub struct BuyVolume {
    #[validate(range(min = 0.0))]
    #[serde(rename = "5min")]
    pub min5: f64,
    #[validate(range(min = 0.0))]
    #[serde(rename = "1h")]
    pub h1: f64,
    #[validate(range(min = 0.0))]
    #[serde(rename = "4h")]
    pub h4: f64,
    #[validate(range(min = 0.0))]
    #[serde(rename = "24h")]
    pub h24: f64,
}

#[derive(Debug, Deserialize, Validate)]
pub struct SellVolume {
    #[validate(range(min = 0.0))]
    #[serde(rename = "5min")]
    pub min5: f64,
    #[validate(range(min = 0.0))]
    #[serde(rename = "1h")]
    pub h1: f64,
    #[validate(range(min = 0.0))]
    #[serde(rename = "4h")]
    pub h4: f64,
    #[validate(range(min = 0.0))]
    #[serde(rename = "24h")]
    pub h24: f64,
}

#[derive(Debug, Deserialize)]
pub struct TokenHolderStats {
    #[serde(rename = "totalHolders")]
    pub total_holders: u32,
}
