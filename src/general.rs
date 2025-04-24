use axum::{
    extract::{Query, State},
    response::Html,
    routing::get,
    Router,
};
use serde::Deserialize;

use crate::database::Database;

// Common query parameters
#[derive(Deserialize)]
pub struct PaginationParams {
    pub page: Option<usize>,
    pub per_page: Option<usize>,
    pub sort_by: Option<String>,
    pub sort_dir: Option<String>,
}

#[derive(Deserialize)]
pub struct SearchParams {
    pub q: Option<String>,
    #[serde(flatten)]
    pub pagination: PaginationParams,
}

// Page Endpoints
async fn dashboard_page(
    State(database): State<Database>,
) -> Html<String> {
    // Dashboard page
    Html(String::new())
}

async fn not_found_page() -> Html<String> {
    // 404 page
    Html(String::new())
}

async fn server_error_page() -> Html<String> {
    // 500 page
    Html(String::new())
}

// HTMX Endpoints
async fn alerts_component(
    State(database): State<Database>,
) -> Html<String> {
    // Fetch dynamic alerts for dashboard
    Html(String::new())
}

async fn stats_component(
    State(database): State<Database>,
) -> Html<String> {
    // Fetch quick stats for dashboard
    Html(String::new())
}

async fn activity_component(
    State(database): State<Database>,
) -> Html<String> {
    // Fetch recent activity timeline
    Html(String::new())
}

async fn global_search(
    Query(params): Query<SearchParams>,
    State(database): State<Database>,
) -> Html<String> {
    // Global search function
    Html(String::new())
}

pub fn router() -> Router<Database> {
    Router::new()
        // Page Endpoints
        .route("/", get(dashboard_page))
        .route("/404", get(not_found_page))
        .route("/500", get(server_error_page))
        // HTMX Endpoints
        .route("/components/alerts", get(alerts_component))
        .route("/components/stats", get(stats_component))
        .route("/components/activity", get(activity_component))
        .route("/search", get(global_search))
}
