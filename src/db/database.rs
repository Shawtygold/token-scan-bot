use super::models::{Guild, NewTokenScan, Token, TokenScan, User};
use super::repositories::{
    guild::PgGuildRepository,
    token::PgTokenRepository,
    token_scan::PgTokenScanRepository,
    traits::{GuildRepository, TokenRepository, TokenScanRepository, UserRepository},
    user::PgUserRepository,
};
use anyhow::Error;
use deadpool_postgres::{Config, Runtime};
use std::sync::Arc;
use tokio_postgres::NoTls;

pub struct Database {
    guild_repository: Arc<dyn GuildRepository>,
    user_repository: Arc<dyn UserRepository>,
    token_repository: Arc<dyn TokenRepository>,
    token_scan_repository: Arc<dyn TokenScanRepository>,
}

impl Database {
    pub fn new(db_url: String) -> Result<Self, Error> {
        let mut config = Config::new();
        config.url = Some(db_url);
        let pool = config.create_pool(Some(Runtime::Tokio1), NoTls)?;
        Ok(Self {
            guild_repository: Arc::new(PgGuildRepository::new(pool.clone())),
            user_repository: Arc::new(PgUserRepository::new(pool.clone())),
            token_repository: Arc::new(PgTokenRepository::new(pool.clone())),
            token_scan_repository: Arc::new(PgTokenScanRepository::new(pool.clone())),
        })
    }

    pub async fn insert_token_scan(
        &self,
        guild: &Guild,
        user: &User,
        token: &Token,
        token_scan: &NewTokenScan,
    ) -> Result<(), Error> {
        self.guild_repository.insert(guild).await?;
        self.user_repository.insert(user).await?;
        self.token_repository.insert(token).await?;
        self.token_scan_repository.insert(token_scan).await?;
        Ok(())
    }

    pub async fn get_token_scan(
        &self,
        token_id: &str,
        guild_id: u64,
    ) -> Result<Option<TokenScan>, Error> {
        let scans = self.token_scan_repository.get(token_id, guild_id).await?;
        if let Some(scan) = scans.into_iter().nth(0) {
            Ok(Some(scan))
        } else {
            Ok(None)
        }
    }
}
