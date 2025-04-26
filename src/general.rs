use axum::{
    extract::{Query, State},
    response::Html,
    routing::get,
    Router,
};
use serde::Deserialize;
use askama::Template;
use chrono::Utc;

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

// Template data types
#[derive(Template)]
#[template(path = "general/dashboard.html")]
pub struct DashboardTemplate {
    pub active_sites: usize,
    pub total_departments: usize,
    pub total_areas: usize,
    pub active_personnel: usize,
    pub equipment_utilization: f64,
}

#[derive(Template)]
#[template(path = "general/errors/404.html")]
pub struct NotFoundTemplate {}

#[derive(Template)]
#[template(path = "general/errors/500.html")]
pub struct ServerErrorTemplate {}

// Component template data types
#[derive(Template)]
#[template(path = "general/components/alerts.html")]
pub struct AlertsTemplate {
    pub alerts: Vec<Alert>,
}

#[derive(Template)]
#[template(path = "general/components/stats.html")]
pub struct StatsTemplate {
    pub stats: Vec<Stat>,
}

#[derive(Template)]
#[template(path = "general/components/activity.html")]
pub struct ActivityTemplate {
    pub activities: Vec<ActivityItem>,
}

#[derive(Template)]
#[template(path = "general/components/search_results.html")]
pub struct SearchResultsTemplate {
    pub results: Vec<SearchResult>,
    pub query: Option<String>,
    pub total_results: usize,
    pub current_page: usize,
    pub total_pages: usize,
}

// Data types for templates
pub struct Alert {
    pub id: String,
    pub title: String,
    pub message: String,
    pub severity: AlertSeverity,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub is_read: bool,
    pub link: Option<String>,
}

pub enum AlertSeverity {
    Info,
    Warning,
    Error,
    Success,
}

pub struct Stat {
    pub id: String,
    pub title: String,
    pub value: String,
    pub icon: String,
    pub change_percentage: Option<f64>,
    pub change_direction: Option<ChangeDirection>,
    pub link: Option<String>,
}

pub enum ChangeDirection {
    Up,
    Down,
    Neutral,
}

pub struct ActivityItem {
    pub id: String,
    pub activity_type: ActivityType,
    pub description: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub user: Option<String>,
    pub link: Option<String>,
    pub entity_id: Option<String>,
    pub entity_type: Option<String>,
}

pub enum ActivityType {
    SiteCreated,
    TaskCompleted,
    MaterialDelivered,
    EquipmentAllocated,
    BrigadeAssigned,
    ReportSubmitted,
    TaskOverdue,
    MaterialExceeded,
}

pub struct SearchResult {
    pub id: String,
    pub entity_type: EntityType,
    pub title: String,
    pub description: Option<String>,
    pub matching_field: Option<String>,
    pub link: String,
}

pub enum EntityType {
    Site,
    Department,
    Area,
    Personnel,
    Equipment,
    Brigade,
    Task,
    Material,
    Client,
}

// Page Endpoints
async fn dashboard_page(
    State(database): State<Database>,
) -> Html<String> {
    // Dashboard page
    // Return the rendered DashboardTemplate
    Html::from(String::new())
}

async fn not_found_page() -> Html<String> {
    // 404 page
    // Return the rendered NotFoundTemplate
    Html::from(String::new())
}

async fn server_error_page() -> Html<String> {
    // 500 page
    // Return the rendered ServerErrorTemplate
    Html::from(String::new())
}

// HTMX Endpoints
async fn alerts_component(
    State(database): State<Database>,
) -> Html<String> {
    // Fetch dynamic alerts for dashboard
    // Return the rendered AlertsTemplate
    Html::from(String::new())
}

async fn stats_component(
    State(database): State<Database>,
) -> Html<String> {
    // Fetch quick stats for dashboard
    // Return the rendered StatsTemplate
    Html::from(String::new())
}

async fn activity_component(
    State(database): State<Database>,
) -> Html<String> {
    // Fetch recent activity timeline
    // Return the rendered ActivityTemplate
    Html::from(String::new())
}

async fn global_search(
    Query(params): Query<SearchParams>,
    State(database): State<Database>,
) -> Html<String> {
    // Global search function
    // Return the rendered SearchResultsTemplate
    Html::from(String::new())
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
