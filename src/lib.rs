use axum::response::IntoResponse;
use axum::routing::get;

use database::Database;

pub mod database;

pub async fn router(postgres_url: &str) -> anyhow::Result<axum::Router> {
    let database = Database::new(postgres_url).await?;
    let router = axum::Router::new()
        .route("/", get(index))
        .with_state(database);

    Ok(router)
}

pub async fn index() -> impl IntoResponse {
    ()
}
