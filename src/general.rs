use axum::{
    extract::{Query, State},
    response::Html,
    routing::get,
    Router,
};
use serde::Deserialize;
use askama::Template;

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

// Define templates
#[derive(Template)]
#[template(path = "dashboard.html")]
struct DashboardTemplate {}

#[derive(Template)]
#[template(path = "error/404.html")]
struct NotFoundTemplate {}

#[derive(Template)]
#[template(path = "error/500.html")]
struct ServerErrorTemplate {}

#[derive(Template)]
#[template(path = "components/stats.html")]
struct StatsTemplate {
    active_sites: usize,
    departments: usize,
    areas: usize,
    active_personnel: usize,
    equipment_utilization: usize,
}

#[derive(Template)]
#[template(path = "components/alerts.html")]
struct AlertsTemplate {
    alerts: Vec<Alert>,
}

#[derive(Template)]
#[template(path = "components/activity.html")]
struct ActivityTemplate {
    activities: Vec<Activity>,
}

#[derive(Template)]
#[template(path = "components/search-results.html")]
struct SearchResultsTemplate {
    query: String,
    results: Vec<SearchResult>,
    more_results: bool,
}

// Define data structs for templates
#[derive(Debug)]
pub struct Alert {
    pub title: String,
    pub message: String,
    pub type_: String,
    pub link: Option<String>,
}

#[derive(Debug)]
pub struct Activity {
    pub type_: String,
    pub description: String,
    pub timestamp: String,
    pub time_ago: String,
}

#[derive(Debug)]
pub struct SearchResult {
    pub type_: String,
    pub title: String,
    pub subtitle: String,
    pub link: String,
}

// Page Endpoints
async fn dashboard_page(
    State(database): State<Database>,
) -> Html<String> {
    // Render dashboard page
    let template = DashboardTemplate {};
    Html(template.render().unwrap_or_else(|_| String::from("Error rendering dashboard template")))
}

async fn not_found_page() -> Html<String> {
    // Render 404 page
    let template = NotFoundTemplate {};
    Html(template.render().unwrap_or_else(|_| String::from("Error rendering 404 template")))
}

async fn server_error_page() -> Html<String> {
    // Render 500 page
    let template = ServerErrorTemplate {};
    Html(template.render().unwrap_or_else(|_| String::from("Error rendering 500 template")))
}

// HTMX Endpoints
async fn alerts_component(
    State(database): State<Database>,
) -> Html<String> {
    // Fetch dynamic alerts for dashboard
    // In a real implementation, we would query the database for alerts
    let alerts = vec![
        Alert {
            title: "Overdue Task".to_string(),
            message: "Foundation work at Riverfront Housing is 3 days overdue".to_string(),
            type_: "warning".to_string(),
            link: Some("/tasks/123".to_string()),
        },
        Alert {
            title: "Material Excess".to_string(),
            message: "Cement usage at Downtown Bridge exceeds estimate by 15%".to_string(),
            type_: "error".to_string(),
            link: Some("/sites/456/materials".to_string()),
        },
    ];
    
    let template = AlertsTemplate { alerts };
    Html(template.render().unwrap_or_else(|_| String::from("Error rendering alerts template")))
}

async fn stats_component(
    State(database): State<Database>,
) -> Html<String> {
    // Fetch quick stats for dashboard
    // In a real implementation, we would query the database for stats
    let stats = StatsTemplate {
        active_sites: 14,
        departments: 5,
        areas: 12,
        active_personnel: 128,
        equipment_utilization: 73,
    };
    
    Html(stats.render().unwrap_or_else(|_| String::from("Error rendering stats template")))
}

async fn activity_component(
    State(database): State<Database>,
) -> Html<String> {
    // Fetch recent activity timeline
    // In a real implementation, we would query the database for recent activities
    let activities = vec![
        Activity {
            type_: "site".to_string(),
            description: "New site 'Central Park Renovation' was created".to_string(),
            timestamp: "2023-11-15T14:32:00Z".to_string(),
            time_ago: "2 hours ago".to_string(),
        },
        Activity {
            type_: "task".to_string(),
            description: "Task 'Foundation Work' was completed at Downtown Bridge".to_string(),
            timestamp: "2023-11-15T12:15:00Z".to_string(),
            time_ago: "4 hours ago".to_string(),
        },
        Activity {
            type_: "brigade".to_string(),
            description: "Brigade #12 was assigned to Riverfront Housing".to_string(),
            timestamp: "2023-11-15T09:45:00Z".to_string(),
            time_ago: "7 hours ago".to_string(),
        },
    ];
    
    let template = ActivityTemplate { activities };
    Html(template.render().unwrap_or_else(|_| String::from("Error rendering activity template")))
}

async fn global_search(
    Query(params): Query<SearchParams>,
    State(database): State<Database>,
) -> Html<String> {
    // Global search function
    // In a real implementation, we would query the database for search results
    let query = params.q.unwrap_or_default();
    
    let results = if !query.is_empty() {
        vec![
            SearchResult {
                type_: "site".to_string(),
                title: "Riverfront Housing".to_string(),
                subtitle: "In Progress • Housing".to_string(),
                link: "/sites/123".to_string(),
            },
            SearchResult {
                type_: "department".to_string(),
                title: "North Regional Department".to_string(),
                subtitle: "5 areas • 12 sites".to_string(),
                link: "/departments/45".to_string(),
            },
            SearchResult {
                type_: "personnel".to_string(),
                title: "John Smith".to_string(),
                subtitle: "Engineer • Project Manager".to_string(),
                link: "/personnel/technical/78".to_string(),
            },
        ]
    } else {
        vec![]
    };
    
    let template = SearchResultsTemplate {
        query,
        results,
        more_results: false,
    };
    
    Html(template.render().unwrap_or_else(|_| String::from("Error rendering search results")))
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
