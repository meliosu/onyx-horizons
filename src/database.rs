use std::sync::Arc;

use sqlx::{migrate::MigrateDatabase, PgPool};

#[derive(Clone)]
pub(crate) struct Database {
    pub pool: Arc<PgPool>,
}

impl Database {
    pub async fn new(postgres_url: &str) -> sqlx::Result<Self> {
        // Create database if it doesn't exist
        if !sqlx::Postgres::database_exists(postgres_url).await? {
            sqlx::Postgres::create_database(postgres_url).await?;
        }
        
        let pool = PgPool::connect(postgres_url).await?;
        
        // Run migrations
        sqlx::migrate!("./migrations")
            .run(&pool)
            .await?;

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
