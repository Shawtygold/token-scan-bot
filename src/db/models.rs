use chrono::{DateTime, Utc};
use validator::Validate;

#[derive(Debug, Validate, Clone)]
pub struct TokenScan {
    pub id: i64,
    #[validate(range(min = 1))]
    pub guild_id: u64,
    #[validate(range(min = 1))]
    pub user_id: u64,
    #[validate(length(min = 32, max = 44))]
    pub token_id: String,
    #[validate(range(min = 0.0))]
    pub fdv: f64,
    pub scanned_at: DateTime<Utc>,
}

#[derive(Debug, Validate)]
pub struct NewTokenScan {
    #[validate(range(min = 1))]
    pub guild_id: u64,
    #[validate(range(min = 1))]
    pub user_id: u64,
    #[validate(length(min = 32, max = 44))]
    pub token_id: String,
    #[validate(range(min = 0.0))]
    pub fdv: f64,
}

#[derive(Debug, Validate)]
pub struct Guild {
    #[validate(range(min = 1))]
    pub guild_id: u64,
}

#[derive(Debug, Validate)]
pub struct Token {
    #[validate(length(min = 32, max = 44))]
    pub token_id: String,
    #[validate(length(max = 100))]
    pub name: String,
    #[validate(length(max = 20))]
    pub symbol: String,
}

#[derive(Debug, Validate)]
pub struct User {
    #[validate(range(min = 1))]
    pub user_id: u64,
}
