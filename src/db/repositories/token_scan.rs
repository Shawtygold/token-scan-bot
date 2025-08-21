use super::traits::TokenScanRepository;
use crate::db::models::{NewTokenScan, TokenScan};
use anyhow::Error;
use async_trait::async_trait;
use deadpool_postgres::Pool;

pub struct PgTokenScanRepository {
    pool: Pool,
}

#[async_trait]
impl TokenScanRepository for PgTokenScanRepository {
    async fn insert(&self, token_scan: &NewTokenScan) -> Result<(), Error> {
        let client = self.pool.get().await?;
        client.execute(
            "INSERT INTO token_scans (guild_id, user_id, token_id, fdv) VALUES ($1, $2, $3, $4) ON CONFLICT ON CONSTRAINT idx_unique_token_guild DO NOTHING",
            &[
                &(token_scan.guild_id as i64),
                &(token_scan.user_id as i64),
                &token_scan.token_id,
                &token_scan.fdv,
            ],
        ).await?;

        Ok(())
    }

    async fn get(&self, token_id: &str, guild_id: u64) -> Result<Vec<TokenScan>, Error> {
        let client = self.pool.get().await?;
        let rows = client
            .query(
                "SELECT * FROM token_scans WHERE token_id = $1 and guild_id = $2",
                &[&token_id, &(guild_id as i64)],
            )
            .await?;

        let scans: Vec<TokenScan> = rows
            .into_iter()
            .map(|row| TokenScan {
                id: row.get(0),
                guild_id: row.get::<_, i64>(1) as u64,
                user_id: row.get::<_, i64>(2) as u64,
                token_id: row.get(3),
                fdv: row.get(4),
                scanned_at: row.get(5),
            })
            .collect();

        return Ok(scans);
    }
}

impl PgTokenScanRepository {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }
}
