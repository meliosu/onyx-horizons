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
#[template(path = "dashboard.html")]
pub struct DashboardTemplate {
    pub active_sites: usize,
    pub total_departments: usize,
    pub total_areas: usize,
    pub active_personnel: usize,
    pub equipment_utilization: f64,
}

#[derive(Template)]
#[template(path = "errors/404.html")]
pub struct NotFoundTemplate {}

#[derive(Template)]
#[template(path = "errors/500.html")]
pub struct ServerErrorTemplate {}

// Component template data types
#[derive(Template)]
#[template(path = "components/alerts.html")]
pub struct AlertsTemplate {
    pub alerts: Vec<Alert>,
}

#[derive(Template)]
#[template(path = "components/stats.html")]
pub struct StatsTemplate {
    pub stats: Vec<Stat>,
}

#[derive(Template)]
#[template(path = "components/activity.html")]
pub struct ActivityTemplate {
    pub activities: Vec<ActivityItem>,
}

#[derive(Template)]
#[template(path = "components/search_results.html")]
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
    let template = DashboardTemplate {
        active_sites: 12,
        total_departments: 5,
        total_areas: 18,
        active_personnel: 243,
        equipment_utilization: 78.5,
    };
    
    match template.render() {
        Ok(html) => Html(html),
        Err(err) => {
            eprintln!("Template error: {}", err);
            Html("Error rendering dashboard template".to_string())
        }
    }
}

async fn not_found_page() -> Html<String> {
    // 404 page
    // Return the rendered NotFoundTemplate
    let template = NotFoundTemplate {};
    
    match template.render() {
        Ok(html) => Html(html),
        Err(err) => {
            eprintln!("Template error: {}", err);
            Html("404 - Page Not Found".to_string())
        }
    }
}

async fn server_error_page() -> Html<String> {
    // 500 page
    // Return the rendered ServerErrorTemplate
    let template = ServerErrorTemplate {};
    
    match template.render() {
        Ok(html) => Html(html),
        Err(err) => {
            eprintln!("Template error: {}", err);
            Html("500 - Server Error".to_string())
        }
    }
}

// HTMX Endpoints
async fn alerts_component(
    State(database): State<Database>,
) -> Html<String> {
    // Fetch dynamic alerts for dashboard
    // Return the rendered AlertsTemplate
    
    // In a real app, this would query the database
    let alerts = vec![
        Alert {
            id: "alert1".to_string(),
            title: "Task Overdue".to_string(),
            message: "Foundation work at Riverside Apartments is 2 days overdue".to_string(),
            severity: AlertSeverity::Warning,
            timestamp: Utc::now(),
            is_read: false,
            link: Some("/tasks/123".to_string()),
        },
        Alert {
            id: "alert2".to_string(),
            title: "Material Shortage".to_string(),
            message: "Cement stocks are running low for Park Avenue Bridge project".to_string(),
            severity: AlertSeverity::Error,
            timestamp: Utc::now(),
            is_read: false,
            link: Some("/materials/cement".to_string()),
        },
        Alert {
            id: "alert3".to_string(),
            title: "New Client".to_string(),
            message: "Metro City Council has registered as a new client".to_string(),
            severity: AlertSeverity::Info,
            timestamp: Utc::now(),
            is_read: true,
            link: Some("/clients/456".to_string()),
        },
        Alert {
            id: "alert4".to_string(),
            title: "Task Completed".to_string(),
            message: "Electrical wiring task completed at Central Park project".to_string(),
            severity: AlertSeverity::Success,
            timestamp: Utc::now(),
            is_read: false,
            link: None,
        },
    ];
    
    let template = AlertsTemplate { alerts };
    
    match template.render() {
        Ok(html) => Html(html),
        Err(err) => {
            eprintln!("Template error: {}", err);
            Html("Error loading alerts".to_string())
        }
    }
}

async fn stats_component(
    State(database): State<Database>,
) -> Html<String> {
    // Fetch quick stats for dashboard
    // Return the rendered StatsTemplate
    
    // In a real app, this would query the database
    let stats = vec![
        Stat {
            id: "stat1".to_string(),
            title: "Active Sites".to_string(),
            value: "12".to_string(),
            icon: "fas fa-hard-hat".to_string(),
            change_percentage: Some(8.5),
            change_direction: Some(ChangeDirection::Up),
            link: Some("/sites?status=active".to_string()),
        },
        Stat {
            id: "stat2".to_string(),
            title: "Active Personnel".to_string(),
            value: "243".to_string(),
            icon: "fas fa-users".to_string(),
            change_percentage: Some(2.1),
            change_direction: Some(ChangeDirection::Up),
            link: Some("/personnel".to_string()),
        },
        Stat {
            id: "stat3".to_string(),
            title: "Overdue Tasks".to_string(),
            value: "7".to_string(),
            icon: "fas fa-exclamation-triangle".to_string(),
            change_percentage: Some(15.0),
            change_direction: Some(ChangeDirection::Down),
            link: Some("/tasks?status=overdue".to_string()),
        },
        Stat {
            id: "stat4".to_string(),
            title: "Material Budget".to_string(),
            value: "$1.2M".to_string(),
            icon: "fas fa-coins".to_string(),
            change_percentage: None,
            change_direction: None,
            link: Some("/materials/budget".to_string()),
        },
    ];
    
    let template = StatsTemplate { stats };
    
    match template.render() {
        Ok(html) => Html(html),
        Err(err) => {
            eprintln!("Template error: {}", err);
            Html("Error loading stats".to_string())
        }
    }
}

async fn activity_component(
    State(database): State<Database>,
) -> Html<String> {
    // Fetch recent activity timeline
    // Return the rendered ActivityTemplate
    
    // In a real app, this would query the database
    let activities = vec![
        ActivityItem {
            id: "activity1".to_string(),
            activity_type: ActivityType::SiteCreated,
            description: "New site 'Downtown Shopping Center' created".to_string(),
            timestamp: Utc::now(),
            user: Some("Jane Smith".to_string()),
            link: Some("/sites/789".to_string()),
            entity_id: Some("789".to_string()),
            entity_type: Some("site".to_string()),
        },
        ActivityItem {
            id: "activity2".to_string(),
            activity_type: ActivityType::TaskCompleted,
            description: "Foundation work completed at Riverside Apartments".to_string(),
            timestamp: Utc::now().checked_sub_signed(chrono::Duration::hours(3)).unwrap(),
            user: Some("John Doe".to_string()),
            link: Some("/tasks/456".to_string()),
            entity_id: Some("456".to_string()),
            entity_type: Some("task".to_string()),
        },
        ActivityItem {
            id: "activity3".to_string(),
            activity_type: ActivityType::MaterialDelivered,
            description: "Cement delivery arrived for Central Park Bridge".to_string(),
            timestamp: Utc::now().checked_sub_signed(chrono::Duration::hours(5)).unwrap(),
            user: None,
            link: Some("/materials/cement".to_string()),
            entity_id: Some("cement".to_string()),
            entity_type: Some("material".to_string()),
        },
        ActivityItem {
            id: "activity4".to_string(),
            activity_type: ActivityType::EquipmentAllocated,
            description: "Crane #5 allocated to Westside Tower construction".to_string(),
            timestamp: Utc::now().checked_sub_signed(chrono::Duration::hours(8)).unwrap(),
            user: Some("Alice Johnson".to_string()),
            link: Some("/equipment/5".to_string()),
            entity_id: Some("5".to_string()),
            entity_type: Some("equipment".to_string()),
        },
        ActivityItem {
            id: "activity5".to_string(),
            activity_type: ActivityType::BrigadeAssigned,
            description: "Brigade #12 assigned to Solar Farm project".to_string(),
            timestamp: Utc::now().checked_sub_signed(chrono::Duration::hours(12)).unwrap(),
            user: Some("Robert Chen".to_string()),
            link: Some("/brigades/12".to_string()),
            entity_id: Some("12".to_string()),
            entity_type: Some("brigade".to_string()),
        },
    ];
    
    let template = ActivityTemplate { activities };
    
    match template.render() {
        Ok(html) => Html(html),
        Err(err) => {
            eprintln!("Template error: {}", err);
            Html("Error loading activity timeline".to_string())
        }
    }
}

async fn global_search(
    Query(params): Query<SearchParams>,
    State(database): State<Database>,
) -> Html<String> {
    // Global search function
    // Return the rendered SearchResultsTemplate
    
    let query = params.q.clone();
    let page = params.pagination.page.unwrap_or(1);
    let per_page = params.pagination.per_page.unwrap_or(10);
    
    // Create dummy search results for demonstration
    // In a real app, this would query the database with the search term
    let results = if let Some(q) = &query {
        if q.trim().is_empty() {
            vec![]
        } else {
            vec![
                SearchResult {
                    id: "site1".to_string(),
                    entity_type: EntityType::Site,
                    title: format!("Downtown Tower (matches '{}')", q),
                    description: Some("Commercial building with 25 floors".to_string()),
                    matching_field: Some("name".to_string()),
                    link: "/sites/1".to_string(),
                },
                SearchResult {
                    id: "person1".to_string(),
                    entity_type: EntityType::Personnel,
                    title: format!("John Smith (matches '{}')", q),
                    description: Some("Engineer - Project Manager".to_string()),
                    matching_field: Some("name".to_string()),
                    link: "/personnel/technical/1".to_string(),
                },
                SearchResult {
                    id: "equipment1".to_string(),
                    entity_type: EntityType::Equipment,
                    title: format!("Mobile Crane X5000 (matches '{}')", q),
                    description: Some("Heavy-duty crane with 50-ton capacity".to_string()),
                    matching_field: Some("description".to_string()),
                    link: "/equipment/1".to_string(),
                },
                SearchResult {
                    id: "task1".to_string(),
                    entity_type: EntityType::Task,
                    title: format!("Foundation Construction (matches '{}')", q),
                    description: Some("Preparing and pouring concrete foundation".to_string()),
                    matching_field: Some("description".to_string()),
                    link: "/tasks/1".to_string(),
                },
                SearchResult {
                    id: "area1".to_string(),
                    entity_type: EntityType::Area,
                    title: format!("North Region (matches '{}')", q),
                    description: Some("Construction area covering northern districts".to_string()),
                    matching_field: Some("name".to_string()),
                    link: "/areas/1".to_string(),
                },
            ]
        }
    } else {
        vec![]
    };
    
    let total_results = results.len();
    let total_pages = (total_results as f64 / per_page as f64).ceil() as usize;
    
    let template = SearchResultsTemplate {
        results,
        query,
        total_results,
        current_page: page,
        total_pages,
    };
    
    match template.render() {
        Ok(html) => Html(html),
        Err(err) => {
            eprintln!("Template error: {}", err);
            Html("Error performing search".to_string())
        }
    }
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
