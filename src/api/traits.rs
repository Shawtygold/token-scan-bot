use super::models::TokenInfo;
use anyhow::Error;
use async_trait::async_trait;

#[async_trait]
pub trait TokenApiClient: Send + Sync {
    async fn fetch_token_info(&self, token_mint: String) -> Result<Option<TokenInfo>, Error>;
}
