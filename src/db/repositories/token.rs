use super::traits::TokenRepository;
use crate::db::models::Token;
use anyhow::Error;
use async_trait::async_trait;
use deadpool_postgres::Pool;

pub struct PgTokenRepository {
    pool: Pool,
}

#[async_trait]
impl TokenRepository for PgTokenRepository {
    async fn insert(&self, token: &Token) -> Result<(), Error> {
        let client = self.pool.get().await?;
        client.execute(
            "INSERT INTO tokens (token_id, name, symbol) VALUES ($1, $2, $3) ON CONFLICT ON CONSTRAINT tokens_pkey DO NOTHING",
            &[&token.token_id, &token.name, &token.symbol],
        ).await?;

        Ok(())
    }
}

impl PgTokenRepository {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }
}
