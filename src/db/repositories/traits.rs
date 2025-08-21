use crate::db::models::{Guild, NewTokenScan, Token, TokenScan, User};
use anyhow::Error;
use async_trait::async_trait;

#[async_trait]
pub trait TokenScanRepository: Send + Sync {
    async fn insert(&self, token_scan: &NewTokenScan) -> Result<(), Error>;
    async fn get(&self, token_id: &str, guild_id: u64) -> Result<Vec<TokenScan>, Error>;
}

#[async_trait]
pub trait GuildRepository: Send + Sync {
    async fn insert(&self, guild: &Guild) -> Result<(), Error>;
}

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn insert(&self, user: &User) -> Result<(), Error>;
}

#[async_trait]
pub trait TokenRepository: Send + Sync {
    async fn insert(&self, token: &Token) -> Result<(), Error>;
}
