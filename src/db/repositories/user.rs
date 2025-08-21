use super::traits::UserRepository;
use crate::db::models::User;
use anyhow::Error;
use async_trait::async_trait;
use deadpool_postgres::Pool;

pub struct PgUserRepository {
    pool: Pool,
}

#[async_trait]
impl UserRepository for PgUserRepository {
    async fn insert(&self, user: &User) -> Result<(), Error> {
        let client = self.pool.get().await?;
        client
            .execute("INSERT INTO users (user_id) VALUES ($1) ON CONFLICT ON CONSTRAINT users_pkey DO NOTHING", &[&(user.user_id as i64)])
            .await?;

        Ok(())
    }
}

impl PgUserRepository {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }
}
