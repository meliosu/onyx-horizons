use database::Database;

mod database;

pub async fn router(postgres_url: &str) -> anyhow::Result<axum::Router> {
    let database = Database::new(postgres_url).await?;
    let router = axum::Router::new()
        .merge(general::router())
        .merge(departments::router())
        .merge(sites::router())
        .merge(personnel::router())
        .merge(equipment::router())
        .merge(clients::router())
        .merge(brigades::router())
        .merge(tasks::router())
        .merge(reports::router())
        .with_state(database);

    Ok(router)
}

mod sites;
mod departments;
mod general;
mod personnel;
mod equipment;
mod clients;
mod brigades;
mod tasks;
mod reports;