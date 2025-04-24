use database::Database;

pub mod database;

pub async fn router(postgres_url: &str) -> anyhow::Result<axum::Router> {
    let database = Database::new(postgres_url).await?;
    let router = axum::Router::new().with_state(database);
    Ok(router)
}
