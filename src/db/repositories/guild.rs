use super::traits::GuildRepository;
use crate::db::models::Guild;
use anyhow::Error;
use async_trait::async_trait;
use deadpool_postgres::Pool;

pub struct PgGuildRepository {
    pool: Pool,
}

#[async_trait]
impl GuildRepository for PgGuildRepository {
    async fn insert(&self, guild: &Guild) -> Result<(), Error> {
        let client = self.pool.get().await?;
        client.execute(
            "INSERT INTO guilds (guild_id) VALUES ($1) ON CONFLICT ON CONSTRAINT guilds_pkey DO NOTHING",
            &[&(guild.guild_id as i64)],
        ).await?;

        Ok(())
    }
}

impl PgGuildRepository {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }
}
