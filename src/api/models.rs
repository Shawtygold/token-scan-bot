use validator::Validate;

#[derive(Debug, Clone, Validate)]
pub struct SolTokenInfo {
    #[validate(length(min = 32, max = 44))]
    pub mint: String,
    #[validate(length(min = 1))]
    pub name: String,
    #[validate(length(min = 1))]
    pub symbol: String,
    #[validate(url)]
    pub logo: String,
    pub links: TokenLinks,
    pub dev: Option<String>,
    pub launchpad: Option<String>,
    pub token_pair_exchange_name: String,
    pub holder_count: u32,
    #[validate(range(min = 0.0))]
    pub fully_diluted_value: f64,
    #[validate(range(min = 0.0))]
    pub usd_price: f64,
    #[validate(range(min = 0.0))]
    pub liquidity_usd: f64,
    pub stats_1h: TokenStats1H,
    pub stats_24h: TokenStats24H,
}

#[derive(Debug, Clone)]
pub struct TokenLinks {
    pub discord: Option<String>,
    pub telegram: Option<String>,
    pub reddit: Option<String>,
    pub twitter: Option<String>,
    pub website: Option<String>,
}

#[derive(Debug, Clone, Validate)]
pub struct TokenStats1H {
    pub buys: u32,
    pub sells: u32,
    #[validate(range(min = 0.0))]
    pub buy_volume: f64,
    #[validate(range(min = 0.0))]
    pub sell_volume: f64,
    #[validate(range(min = 0.0))]
    pub price_percent_change: f64,
}

#[derive(Debug, Clone, Validate)]
pub struct TokenStats24H {
    #[validate(range(min = 0.0))]
    pub price_percent_change: f64,
}
