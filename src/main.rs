use log::*;

use onyx_horizons::router;

#[tokio::main]
async fn main() {
    env_logger::init();

    if let Err(e) = run().await {
        error!("{e}");
    }
}

async fn run() -> anyhow::Result<()> {
    let port = std::env::var("LISTEN_PORT").unwrap_or(LISTEN_PORT.to_string());
    let pg_url = std::env::var("POSTGRES_URL").unwrap_or(POSTGRES_URL.to_string());

    let listen_addr = format!("localhost:{port}");
    let listener = tokio::net::TcpListener::bind(listen_addr).await?;

    let router = router(&pg_url).await?;

    axum::serve(listener, router.into_make_service()).await?;

    Ok(())
}

const LISTEN_PORT: &str = "7999";
const POSTGRES_URL: &str = "postgresql://postgres";
