use std::sync::Arc;

use sqlx::PgPool;

#[derive(Clone)]
pub(crate) struct Database {
    pool: Arc<PgPool>,
}

impl Database {
    pub async fn new(postgres_url: &str) -> sqlx::Result<Self> {
        let pool = PgPool::connect(postgres_url).await?;

        Ok(Self {
            pool: Arc::new(pool),
        })
    }
}

impl std::ops::Deref for Database {
    type Target = PgPool;

    fn deref(&self) -> &Self::Target {
        &self.pool
    }
}
