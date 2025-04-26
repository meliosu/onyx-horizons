use axum::{
    extract::{Query, State},
    response::Html,
    routing::get,
    Router,
};
use serde::Deserialize;
use askama::Template;
use chrono::Utc;
use sqlx::FromRow;

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
    // Fetch key metrics from database
    let active_sites = match get_active_sites_count(&database).await {
        Ok(count) => count,
        Err(_) => 0,
    };
    
    let total_departments = match get_departments_count(&database).await {
        Ok(count) => count,
        Err(_) => 0,
    };
    
    let total_areas = match get_areas_count(&database).await {
        Ok(count) => count,
        Err(_) => 0,
    };
    
    let active_personnel = match get_active_personnel_count(&database).await {
        Ok(count) => count, 
        Err(_) => 0,
    };
    
    let equipment_utilization = match get_equipment_utilization(&database).await {
        Ok(utilization) => utilization,
        Err(_) => 0.0,
    };
    
    // Create template with the fetched data
    let template = DashboardTemplate {
        active_sites,
        total_departments,
        total_areas,
        active_personnel,
        equipment_utilization,
    };
    
    // Render the template
    match template.render() {
        Ok(html) => Html(html),
        Err(_) => Html(String::from("Failed to render dashboard template")),
    }
}

async fn not_found_page() -> Html<String> {
    let template = NotFoundTemplate {};
    match template.render() {
        Ok(html) => Html(html),
        Err(_) => Html(String::from("404 - Page Not Found")),
    }
}

async fn server_error_page() -> Html<String> {
    let template = ServerErrorTemplate {};
    match template.render() {
        Ok(html) => Html(html),
        Err(_) => Html(String::from("500 - Server Error")),
    }
}

// HTMX Endpoints
async fn alerts_component(
    State(database): State<Database>,
) -> Html<String> {
    // Fetch alerts from database
    let alerts = match get_recent_alerts(&database).await {
        Ok(alerts) => alerts,
        Err(_) => Vec::new(),
    };
    
    // Create template with the fetched data
    let template = AlertsTemplate { alerts };
    
    // Render the template
    match template.render() {
        Ok(html) => Html(html),
        Err(_) => Html(String::from("Failed to render alerts component")),
    }
}

async fn stats_component(
    State(database): State<Database>,
) -> Html<String> {
    // Fetch stats from database
    let stats = match get_dashboard_stats(&database).await {
        Ok(stats) => stats,
        Err(_) => Vec::new(),
    };
    
    // Create template with the fetched data
    let template = StatsTemplate { stats };
    
    // Render the template
    match template.render() {
        Ok(html) => Html(html),
        Err(_) => Html(String::from("Failed to render stats component")),
    }
}

async fn activity_component(
    State(database): State<Database>,
) -> Html<String> {
    // Fetch activities from database
    let activities = match get_recent_activities(&database).await {
        Ok(activities) => activities,
        Err(_) => Vec::new(),
    };
    
    // Create template with the fetched data
    let template = ActivityTemplate { activities };
    
    // Render the template
    match template.render() {
        Ok(html) => Html(html),
        Err(_) => Html(String::from("Failed to render activity component")),
    }
}

async fn global_search(
    Query(params): Query<SearchParams>,
    State(database): State<Database>,
) -> Html<String> {
    // Get pagination parameters
    let page = params.pagination.page.unwrap_or(1);
    let per_page = params.pagination.per_page.unwrap_or(10);
    let query = params.q.clone();
    
    // Fetch search results from database
    let (results, total_results) = match search_entities(&database, &params).await {
        Ok(result) => result,
        Err(_) => (Vec::new(), 0),
    };
    
    // Calculate total pages
    let total_pages = if total_results == 0 {
        1
    } else {
        (total_results + per_page - 1) / per_page
    };
    
    // Create template with the fetched data
    let template = SearchResultsTemplate {
        results,
        query,
        total_results,
        current_page: page,
        total_pages,
    };
    
    // Render the template
    match template.render() {
        Ok(html) => Html(html),
        Err(_) => Html(String::from("Failed to render search results")),
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

// Database functions to fetch data

// Derive database models
#[derive(FromRow)]
struct SiteCount {
    count: i64,
}

#[derive(FromRow)]
struct DepartmentCount {
    count: i64,
}

#[derive(FromRow)]
struct AreaCount {
    count: i64,
}

#[derive(FromRow)]
struct PersonnelCount {
    count: i64,
}

#[derive(FromRow)]
struct EquipmentUtilization {
    utilization: f64,
}

#[derive(FromRow)]
struct AlertRow {
    id: String,
    title: String,
    message: String,
    severity: String,
    timestamp: chrono::DateTime<chrono::Utc>,
    is_read: bool,
    link: Option<String>,
}

#[derive(FromRow)]
struct StatRow {
    id: String,
    title: String,
    value: String,
    icon: String,
    change_percentage: Option<f64>,
    change_direction: Option<String>,
    link: Option<String>,
}

#[derive(FromRow)]
struct ActivityRow {
    id: String,
    activity_type: String,
    description: String,
    timestamp: chrono::DateTime<chrono::Utc>,
    user: Option<String>,
    link: Option<String>,
    entity_id: Option<String>,
    entity_type: Option<String>,
}

#[derive(FromRow)]
struct SearchResultRow {
    id: String,
    entity_type: String,
    title: String,
    description: Option<String>,
    matching_field: Option<String>,
    link: String,
    total_count: Option<i64>,
}

// Dashboard data fetching functions
async fn get_active_sites_count(db: &Database) -> anyhow::Result<usize> {
    let count: SiteCount = sqlx::query_as(
        "SELECT COUNT(*) as count FROM site"
    )
    .fetch_one(&*db.pool)
    .await?;
    
    Ok(count.count as usize)
}

async fn get_departments_count(db: &Database) -> anyhow::Result<usize> {
    let count: DepartmentCount = sqlx::query_as(
        "SELECT COUNT(*) as count FROM department"
    )
    .fetch_one(&*db.pool)
    .await?;
    
    Ok(count.count as usize)
}

async fn get_areas_count(db: &Database) -> anyhow::Result<usize> {
    let count: AreaCount = sqlx::query_as(
        "SELECT COUNT(*) as count FROM area"
    )
    .fetch_one(&*db.pool)
    .await?;
    
    Ok(count.count as usize)
}

async fn get_active_personnel_count(db: &Database) -> anyhow::Result<usize> {
    let count: PersonnelCount = sqlx::query_as(
        "SELECT COUNT(*) as count FROM employee"
    )
    .fetch_one(&*db.pool)
    .await?;
    
    Ok(count.count as usize)
}

async fn get_equipment_utilization(db: &Database) -> anyhow::Result<f64> {
    // Calculate equipment utilization as:
    // (allocated_equipment / total_equipment) * 100
    let utilization: EquipmentUtilization = sqlx::query_as(
        r#"
        WITH total_equipment AS (
            SELECT SUM(amount) as total FROM equipment
        ),
        allocated_equipment AS (
            SELECT SUM(amount) as allocated FROM equipment_allocation 
            WHERE period_end >= CURRENT_DATE
        )
        SELECT 
            CASE 
                WHEN (SELECT total FROM total_equipment) = 0 THEN 0
                ELSE (SELECT allocated FROM allocated_equipment) * 100.0 / (SELECT total FROM total_equipment)
            END as utilization
        "#
    )
    .fetch_one(&*db.pool)
    .await?;
    
    Ok(utilization.utilization)
}

// Alert data fetching
async fn get_recent_alerts(db: &Database) -> anyhow::Result<Vec<Alert>> {
    // For demonstration, we'll generate alerts based on database state
    // In a real system, you might have a dedicated alerts table
    let rows: Vec<AlertRow> = sqlx::query_as(
        r#"
        WITH task_alerts AS (
            SELECT 
                'task_' || t.id as id,
                'Task Overdue' as title,
                'Task "' || t.name || '" is overdue' as message,
                'error' as severity,
                t.expected_period_end as timestamp,
                false as is_read,
                '/tasks/' || t.id as link
            FROM task t
            WHERE t.expected_period_end < CURRENT_DATE
            AND t.actual_period_end IS NULL
            LIMIT 5
        ),
        material_alerts AS (
            SELECT 
                'material_' || e.material_id || '_' || e.task_id as id,
                'Material Excess' as title,
                'Material exceeded estimate on task "' || t.name || '"' as message,
                'warning' as severity,
                CURRENT_TIMESTAMP as timestamp,
                false as is_read,
                '/tasks/' || t.id as link
            FROM expenditure e
            JOIN task t ON e.task_id = t.id
            WHERE e.actuial_amount > e.expected_amount
            LIMIT 5
        )
        SELECT * FROM task_alerts
        UNION ALL
        SELECT * FROM material_alerts
        ORDER BY timestamp DESC
        LIMIT 10
        "#
    )
    .fetch_all(&*db.pool)
    .await?;
    
    let alerts = rows.into_iter().map(|row| {
        let severity = match row.severity.as_str() {
            "info" => AlertSeverity::Info,
            "warning" => AlertSeverity::Warning,
            "error" => AlertSeverity::Error,
            "success" => AlertSeverity::Success,
            _ => AlertSeverity::Info,
        };
        
        Alert {
            id: row.id,
            title: row.title,
            message: row.message,
            severity,
            timestamp: row.timestamp,
            is_read: row.is_read,
            link: row.link,
        }
    }).collect();
    
    Ok(alerts)
}

// Stats data fetching
async fn get_dashboard_stats(db: &Database) -> anyhow::Result<Vec<Stat>> {
    let mut stats = Vec::new();
    
    // Active Sites Stat
    let active_sites = get_active_sites_count(db).await?;
    stats.push(Stat {
        id: "active-sites".to_string(),
        title: "Active Sites".to_string(),
        value: active_sites.to_string(),
        icon: "building-2".to_string(),
        change_percentage: Some(5.0), // Example, in real app would calculate from historical data
        change_direction: Some(ChangeDirection::Up),
        link: Some("/sites".to_string()),
    });
    
    // Active Personnel Stat
    let active_personnel = get_active_personnel_count(db).await?;
    stats.push(Stat {
        id: "active-personnel".to_string(),
        title: "Active Personnel".to_string(),
        value: active_personnel.to_string(),
        icon: "users".to_string(),
        change_percentage: Some(2.5),
        change_direction: Some(ChangeDirection::Up),
        link: Some("/personnel".to_string()),
    });
    
    // Equipment Utilization Stat
    let equipment_utilization = get_equipment_utilization(db).await?;
    stats.push(Stat {
        id: "equipment-utilization".to_string(),
        title: "Equipment Utilization".to_string(),
        value: format!("{:.1}%", equipment_utilization),
        icon: "truck".to_string(),
        change_percentage: Some(1.2),
        change_direction: Some(ChangeDirection::Up),
        link: Some("/equipment".to_string()),
    });
    
    // Task Completion Rate
    #[derive(FromRow)]
    struct TaskCompletionRate {
        rate: Option<f64>,
    }

    let task_completion: TaskCompletionRate = sqlx::query_as(
        r#"
        WITH total_tasks AS (
            SELECT COUNT(*) as count FROM task
            WHERE expected_period_end <= CURRENT_DATE
        ),
        completed_tasks AS (
            SELECT COUNT(*) as count FROM task
            WHERE actual_period_end IS NOT NULL
            AND expected_period_end <= CURRENT_DATE
        )
        SELECT 
            CASE 
                WHEN (SELECT count FROM total_tasks) = 0 THEN 0
                ELSE (SELECT count FROM completed_tasks) * 100.0 / (SELECT count FROM total_tasks)
            END as rate
        "#
    )
    .fetch_one(&*db.pool)
    .await?;
    
    stats.push(Stat {
        id: "task-completion".to_string(),
        title: "Task Completion Rate".to_string(),
        value: format!("{:.1}%", task_completion.rate.unwrap_or(0.0)),
        icon: "check-circle".to_string(),
        change_percentage: Some(-0.8),
        change_direction: Some(ChangeDirection::Down),
        link: Some("/tasks".to_string()),
    });
    
    Ok(stats)
}

// Activity data fetching
async fn get_recent_activities(db: &Database) -> anyhow::Result<Vec<ActivityItem>> {
    // Generate activity feed from recent database changes
    // In a real system, you might track activities in a dedicated table
    let rows: Vec<ActivityRow> = sqlx::query_as(
        r#"
        WITH site_activities AS (
            SELECT 
                's_' || id as id,
                'SiteCreated' as activity_type,
                'New site "' || name || '" created' as description,
                CURRENT_TIMESTAMP as timestamp,
                NULL as user,
                '/sites/' || id as link,
                id::text as entity_id,
                'Site' as entity_type
            FROM site
            ORDER BY id DESC
            LIMIT 3
        ),
        task_activities AS (
            SELECT 
                't_' || t.id as id,
                CASE 
                    WHEN t.actual_period_end IS NOT NULL THEN 'TaskCompleted'
                    WHEN t.expected_period_end < CURRENT_DATE THEN 'TaskOverdue'
                    ELSE 'TaskCompleted'
                END as activity_type,
                CASE 
                    WHEN t.actual_period_end IS NOT NULL THEN 'Task "' || t.name || '" completed at site "' || s.name || '"'
                    WHEN t.expected_period_end < CURRENT_DATE THEN 'Task "' || t.name || '" is overdue at site "' || s.name || '"'
                    ELSE 'Task "' || t.name || '" in progress at site "' || s.name || '"'
                END as description,
                COALESCE(t.actual_period_end, CURRENT_TIMESTAMP) as timestamp,
                NULL as user,
                '/tasks/' || t.id as link,
                t.id::text as entity_id,
                'Task' as entity_type
            FROM task t
            JOIN site s ON t.site_id = s.id
            ORDER BY timestamp DESC
            LIMIT 3
        ),
        brigade_activities AS (
            SELECT 
                'b_' || b.id as id,
                'BrigadeAssigned' as activity_type,
                'Brigade led by ' || CONCAT(e.first_name, ' ', e.last_name) || ' assigned to new task' as description,
                CURRENT_TIMESTAMP as timestamp,
                CONCAT(e.first_name, ' ', e.last_name) as user,
                '/brigades/' || b.id as link,
                b.id::text as entity_id,
                'Brigade' as entity_type
            FROM brigade b
            JOIN worker w ON b.brigadier_id = w.id
            JOIN employee e ON w.id = e.id
            ORDER BY b.id DESC
            LIMIT 3
        ),
        equipment_activities AS (
            SELECT 
                'e_' || ea.equipment_id || '_' || ea.department_id || '_' || COALESCE(ea.site_id, 0) as id,
                'EquipmentAllocated' as activity_type,
                'Equipment "' || e.name || '" allocated to ' || 
                CASE 
                    WHEN ea.site_id IS NOT NULL THEN 'site "' || s.name || '"'
                    ELSE 'department "' || d.name || '"'
                END as description,
                ea.period_start as timestamp,
                NULL as user,
                CASE 
                    WHEN ea.site_id IS NOT NULL THEN '/sites/' || ea.site_id
                    ELSE '/departments/' || ea.department_id
                END as link,
                e.id::text as entity_id,
                'Equipment' as entity_type
            FROM equipment_allocation ea
            JOIN equipment e ON ea.equipment_id = e.id
            JOIN department d ON ea.department_id = d.id
            LEFT JOIN site s ON ea.site_id = s.id
            ORDER BY ea.period_start DESC
            LIMIT 3
        )
        SELECT * FROM site_activities
        UNION ALL
        SELECT * FROM task_activities
        UNION ALL
        SELECT * FROM brigade_activities
        UNION ALL
        SELECT * FROM equipment_activities
        ORDER BY timestamp DESC
        LIMIT 10
        "#
    )
    .fetch_all(&*db.pool)
    .await?;
    
    let activities = rows.into_iter().map(|row| {
        let activity_type = match row.activity_type.as_str() {
            "SiteCreated" => ActivityType::SiteCreated,
            "TaskCompleted" => ActivityType::TaskCompleted,
            "MaterialDelivered" => ActivityType::MaterialDelivered,
            "EquipmentAllocated" => ActivityType::EquipmentAllocated,
            "BrigadeAssigned" => ActivityType::BrigadeAssigned,
            "ReportSubmitted" => ActivityType::ReportSubmitted,
            "TaskOverdue" => ActivityType::TaskOverdue,
            "MaterialExceeded" => ActivityType::MaterialExceeded,
            _ => ActivityType::SiteCreated,
        };
        
        ActivityItem {
            id: row.id,
            activity_type,
            description: row.description,
            timestamp: row.timestamp,
            user: row.user,
            link: row.link,
            entity_id: row.entity_id,
            entity_type: row.entity_type,
        }
    }).collect();
    
    Ok(activities)
}

// Search function
async fn search_entities(db: &Database, params: &SearchParams) -> anyhow::Result<(Vec<SearchResult>, usize)> {
    let query = params.q.as_deref().unwrap_or("").to_lowercase();
    let page = params.pagination.page.unwrap_or(1);
    let per_page = params.pagination.per_page.unwrap_or(10);
    let offset = (page - 1) * per_page;
    
    // If query is empty, return empty results
    if query.is_empty() {
        return Ok((Vec::new(), 0));
    }
    
    // Build search query
    let rows: Vec<SearchResultRow> = sqlx::query_as(
        r#"
        WITH site_results AS (
            SELECT 
                id::text as id,
                'Site' as entity_type,
                name as title,
                description,
                CASE 
                    WHEN position(lower($1) in lower(name)) > 0 THEN 'name'
                    WHEN description IS NOT NULL AND position(lower($1) in lower(description)) > 0 THEN 'description'
                    ELSE NULL
                END as matching_field,
                '/sites/' || id as link
            FROM site
            WHERE position(lower($1) in lower(name)) > 0
               OR (description IS NOT NULL AND position(lower($1) in lower(description)) > 0)
        ),
        department_results AS (
            SELECT 
                id::text as id,
                'Department' as entity_type,
                name as title,
                NULL as description,
                'name' as matching_field,
                '/departments/' || id as link
            FROM department
            WHERE position(lower($1) in lower(name)) > 0
        ),
        area_results AS (
            SELECT 
                id::text as id,
                'Area' as entity_type,
                name as title,
                NULL as description,
                'name' as matching_field,
                '/areas/' || id as link
            FROM area
            WHERE position(lower($1) in lower(name)) > 0
        ),
        personnel_results AS (
            SELECT 
                id::text as id,
                'Personnel' as entity_type,
                concat(first_name, ' ', last_name) as title,
                concat('Phone: ', phone_number) as description,
                CASE 
                    WHEN position(lower($1) in lower(concat(first_name, ' ', last_name))) > 0 THEN 'name'
                    WHEN position(lower($1) in lower(phone_number)) > 0 THEN 'phone'
                    ELSE NULL
                END as matching_field,
                '/personnel/' || 
                CASE 
                    WHEN class = 'worker' THEN 'workers/' 
                    ELSE 'technical/'
                END || id as link
            FROM employee
            WHERE position(lower($1) in lower(concat(first_name, ' ', last_name))) > 0
               OR position(lower($1) in lower(phone_number)) > 0
        ),
        client_results AS (
            SELECT 
                id::text as id,
                'Client' as entity_type,
                name as title,
                address as description,
                CASE 
                    WHEN position(lower($1) in lower(name)) > 0 THEN 'name'
                    WHEN position(lower($1) in lower(address)) > 0 THEN 'address'
                    WHEN position(lower($1) in lower(contact_person_name)) > 0 THEN 'contact'
                    ELSE NULL
                END as matching_field,
                '/clients/' || id as link
            FROM client
            WHERE position(lower($1) in lower(name)) > 0
               OR position(lower($1) in lower(address)) > 0
               OR position(lower($1) in lower(contact_person_name)) > 0
        ),
        task_results AS (
            SELECT 
                id::text as id,
                'Task' as entity_type,
                name as title,
                description,
                CASE 
                    WHEN position(lower($1) in lower(name)) > 0 THEN 'name'
                    WHEN description IS NOT NULL AND position(lower($1) in lower(description)) > 0 THEN 'description'
                    ELSE NULL
                END as matching_field,
                '/tasks/' || id as link
            FROM task
            WHERE position(lower($1) in lower(name)) > 0
               OR (description IS NOT NULL AND position(lower($1) in lower(description)) > 0)
        ),
        all_results AS (
            SELECT * FROM site_results
            UNION ALL
            SELECT * FROM department_results
            UNION ALL
            SELECT * FROM area_results
            UNION ALL
            SELECT * FROM personnel_results
            UNION ALL
            SELECT * FROM client_results
            UNION ALL
            SELECT * FROM task_results
        ),
        counted_results AS (
            SELECT COUNT(*) as total FROM all_results
        )
        SELECT 
            r.*,
            (SELECT total FROM counted_results) as total_count
        FROM all_results r
        ORDER BY 
            CASE entity_type
                WHEN 'Site' THEN 1
                WHEN 'Department' THEN 2
                WHEN 'Area' THEN 3
                WHEN 'Personnel' THEN 4
                WHEN 'Client' THEN 5
                WHEN 'Task' THEN 6
                ELSE 99
            END,
            CASE 
                WHEN position(lower($1) in lower(title)) = 1 THEN 1
                WHEN position(lower($1) in lower(title)) > 0 THEN 2
                ELSE 3
            END
        LIMIT $2 OFFSET $3
        "#
    )
    .bind(query)
    .bind(per_page as i64)
    .bind(offset as i64)
    .fetch_all(&*db.pool)
    .await?;
    
    // Calculate total count from the first row
    let total_results = if rows.is_empty() { 0 } else { rows[0].total_count.unwrap_or(0) as usize };
    
    // Map rows to SearchResult
    let results: Vec<SearchResult> = rows.into_iter().map(|row| {
        let entity_type = match row.entity_type.as_str() {
            "Site" => EntityType::Site,
            "Department" => EntityType::Department,
            "Area" => EntityType::Area,
            "Personnel" => EntityType::Personnel,
            "Equipment" => EntityType::Equipment,
            "Brigade" => EntityType::Brigade,
            "Task" => EntityType::Task,
            "Material" => EntityType::Material,
            "Client" => EntityType::Client,
            _ => EntityType::Site,
        };
        
        SearchResult {
            id: row.id,
            entity_type,
            title: row.title,
            description: row.description,
            matching_field: row.matching_field,
            link: row.link,
        }
    }).collect();
    
    Ok((results, total_results))
}
