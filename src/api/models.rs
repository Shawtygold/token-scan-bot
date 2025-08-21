use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TokenInfo {
    pub id: String,
    pub name: String,
    pub symbol: String,
    pub twitter: Option<String>,
    pub telegram: Option<String>,
    pub website: Option<String>,
    pub dev: Option<String>,
    #[serde(rename = "circSupply")]
    pub circ_supply: Option<f64>,
    #[serde(rename = "totalSupply")]
    pub total_supply: Option<f64>,
    pub launchpad: Option<String>,
    #[serde(rename = "holderCount")]
    pub holder_count: Option<usize>,
    pub fdv: Option<f64>,
    pub mcap: Option<f64>,
    #[serde(rename = "usdPrice")]
    pub usd_price: Option<f64>,
    pub liquidity: Option<f64>,
    pub stats1h: Option<Stats>,
    #[serde(rename = "firstPool")]
    pub first_pool: Option<FirstPool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Stats {
    #[serde(rename = "priceChange")]
    pub price_change: Option<f64>,
    #[serde(rename = "volumeChange")]
    pub volume_change: Option<f64>,
    #[serde(rename = "buyVolume")]
    pub buy_volume: Option<f64>,
    #[serde(rename = "sellVolume")]
    pub sell_volume: Option<f64>,
    #[serde(rename = "numBuys")]
    pub num_buys: Option<i64>,
    #[serde(rename = "numSells")]
    pub num_sells: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FirstPool {
    pub id: String,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
}
