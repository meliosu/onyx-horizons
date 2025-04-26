use axum::{
    extract::{Query, State},
    response::Html,
    routing::get,
    Router,
};
use serde::Deserialize;
use askama::Template;
use chrono::{Utc, NaiveDate};
use sqlx::types::chrono;
use anyhow::Context;

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
    pub page: Option<usize>,
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

#[derive(PartialEq)]
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
) -> Result<Html<String>, axum::http::StatusCode> {
    // Count active sites (sites with at least one task that doesn't have an actual_period_end)
    let active_sites = sqlx::query_scalar!(
        r#"
        SELECT COUNT(DISTINCT s.id)
        FROM site s
        JOIN task t ON s.id = t.site_id
        WHERE t.actual_period_end IS NULL
        "#
    )
    .fetch_one(&database.pool)
    .await
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    // Count departments
    let total_departments = sqlx::query_scalar!(
        r#"SELECT COUNT(*) FROM department"#
    )
    .fetch_one(&database.pool)
    .await
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    // Count areas
    let total_areas = sqlx::query_scalar!(
        r#"SELECT COUNT(*) FROM area"#
    )
    .fetch_one(&database.pool)
    .await
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    // Count active personnel
    let active_personnel = sqlx::query_scalar!(
        r#"SELECT COUNT(*) FROM employee"#
    )
    .fetch_one(&database.pool)
    .await
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    // Calculate equipment utilization (allocated amount / total amount)
    let equipment_utilization = sqlx::query_scalar!(
        r#"
        SELECT COALESCE(
            (SELECT SUM(ea.amount)::float FROM equipment_allocation ea WHERE ea.site_id IS NOT NULL) / 
            (SELECT SUM(e.amount)::float FROM equipment e),
            0.0
        )
        "#
    )
    .fetch_one(&database.pool)
    .await
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    let template = DashboardTemplate {
        active_sites: active_sites.unwrap_or(0) as usize,
        total_departments: total_departments.unwrap_or(0) as usize,
        total_areas: total_areas.unwrap_or(0) as usize,
        active_personnel: active_personnel.unwrap_or(0) as usize,
        equipment_utilization: equipment_utilization.unwrap_or(0.0),
    };

    match template.render() {
        Ok(html) => Ok(Html(html)),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn not_found_page() -> Result<Html<String>, axum::http::StatusCode> {
    let template = NotFoundTemplate {};
    
    match template.render() {
        Ok(html) => Ok(Html(html)),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn server_error_page() -> Result<Html<String>, axum::http::StatusCode> {
    let template = ServerErrorTemplate {};
    
    match template.render() {
        Ok(html) => Ok(Html(html)),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// HTMX Endpoints
async fn alerts_component(
    State(database): State<Database>,
) -> Result<Html<String>, axum::http::StatusCode> {
    // Generate alerts based on data in the database
    let mut alerts = Vec::new();
    
    // Check for overdue tasks
    let overdue_tasks = sqlx::query!(
        r#"
        SELECT t.id, t.name, s.name as site_name
        FROM task t
        JOIN site s ON t.site_id = s.id
        WHERE t.expected_period_end < CURRENT_DATE
        AND t.actual_period_end IS NULL
        LIMIT 5
        "#
    )
    .fetch_all(&database.pool)
    .await
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    
    for task in overdue_tasks {
        alerts.push(Alert {
            id: format!("task-{}", task.id),
            title: "Task Overdue".to_string(),
            message: format!("Task \"{}\" at site \"{}\" is overdue", task.name, task.site_name),
            severity: AlertSeverity::Warning,
            timestamp: Utc::now(),
            is_read: false,
            link: Some(format!("/tasks/{}", task.id)),
        });
    }
    
    // Check for material excesses
    let material_excesses = sqlx::query!(
        r#"
        SELECT e.task_id, e.material_id, m.name as material_name, t.name as task_name, s.name as site_name,
               e.actuial_amount, e.expected_amount
        FROM expenditure e
        JOIN material m ON e.material_id = m.id
        JOIN task t ON e.task_id = t.id
        JOIN site s ON t.site_id = s.id
        WHERE e.actuial_amount IS NOT NULL 
        AND e.actuial_amount > e.expected_amount * 1.1
        LIMIT 5
        "#
    )
    .fetch_all(&database.pool)
    .await
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    
    for excess in material_excesses {
        alerts.push(Alert {
            id: format!("material-{}-{}", excess.task_id, excess.material_id),
            title: "Material Excess".to_string(),
            message: format!(
                "Material \"{}\" usage exceeded estimate by {}% in task \"{}\" at site \"{}\"",
                excess.material_name,
                ((excess.actuial_amount.unwrap_or(0.0) / excess.expected_amount - 1.0) * 100.0).round(),
                excess.task_name,
                excess.site_name
            ),
            severity: AlertSeverity::Error,
            timestamp: Utc::now(),
            is_read: false,
            link: Some(format!("/tasks/{}", excess.task_id)),
        });
    }
    
    // Add equipment allocation alerts
    let equipment_low = sqlx::query!(
        r#"
        SELECT e.id, e.name, e.amount 
        FROM equipment e
        WHERE e.amount <= 5
        LIMIT 5
        "#
    )
    .fetch_all(&database.pool)
    .await
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    
    for equip in equipment_low {
        alerts.push(Alert {
            id: format!("equipment-{}", equip.id),
            title: "Low Equipment Stock".to_string(),
            message: format!("Equipment \"{}\" has only {} units remaining", equip.name, equip.amount),
            severity: AlertSeverity::Info,
            timestamp: Utc::now(),
            is_read: false,
            link: Some(format!("/equipment/{}", equip.id)),
        });
    }

    let template = AlertsTemplate { alerts };
    
    match template.render() {
        Ok(html) => Ok(Html(html)),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn stats_component(
    State(database): State<Database>,
) -> Result<Html<String>, axum::http::StatusCode> {
    let mut stats = Vec::new();
    
    // Count sites by status
    let active_sites = sqlx::query_scalar!(
        r#"
        SELECT COUNT(DISTINCT s.id)
        FROM site s
        JOIN task t ON s.id = t.site_id
        WHERE t.actual_period_end IS NULL
        "#
    )
    .fetch_one(&database.pool)
    .await
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?
    .unwrap_or(0);
    
    stats.push(Stat {
        id: "active-sites".to_string(),
        title: "Active Sites".to_string(),
        value: active_sites.to_string(),
        icon: "fa-building".to_string(),
        change_percentage: None,
        change_direction: None,
        link: Some("/sites".to_string()),
    });
    
    // Count tasks by status
    let completed_tasks = sqlx::query_scalar!(
        r#"
        SELECT COUNT(*)
        FROM task
        WHERE actual_period_end IS NOT NULL
        "#
    )
    .fetch_one(&database.pool)
    .await
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?
    .unwrap_or(0);
    
    let total_tasks = sqlx::query_scalar!(
        r#"
        SELECT COUNT(*)
        FROM task
        "#
    )
    .fetch_one(&database.pool)
    .await
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?
    .unwrap_or(0);
    
    let completion_percentage = if total_tasks > 0 {
        (completed_tasks as f64 / total_tasks as f64) * 100.0
    } else {
        0.0
    };
    
    stats.push(Stat {
        id: "completed-tasks".to_string(),
        title: "Tasks Completed".to_string(),
        value: format!("{}/{}", completed_tasks, total_tasks),
        icon: "fa-tasks".to_string(),
        change_percentage: Some(completion_percentage),
        change_direction: Some(ChangeDirection::Up),
        link: Some("/tasks".to_string()),
    });
    
    // Count active brigades
    let active_brigades = sqlx::query_scalar!(
        r#"
        SELECT COUNT(DISTINCT b.id)
        FROM brigade b
        JOIN task t ON t.brigade_id = b.id
        WHERE t.actual_period_end IS NULL
        "#
    )
    .fetch_one(&database.pool)
    .await
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?
    .unwrap_or(0);
    
    let total_brigades = sqlx::query_scalar!(
        r#"
        SELECT COUNT(*)
        FROM brigade
        "#
    )
    .fetch_one(&database.pool)
    .await
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?
    .unwrap_or(0);
    
    stats.push(Stat {
        id: "active-brigades".to_string(),
        title: "Active Brigades".to_string(),
        value: format!("{}/{}", active_brigades, total_brigades),
        icon: "fa-users".to_string(),
        change_percentage: None,
        change_direction: None,
        link: Some("/brigades".to_string()),
    });
    
    // Equipment utilization
    let equipment_utilization = sqlx::query_scalar!(
        r#"
        SELECT COALESCE(
            (SELECT SUM(ea.amount)::float FROM equipment_allocation ea WHERE ea.site_id IS NOT NULL) / 
            NULLIF((SELECT SUM(e.amount)::float FROM equipment e), 0),
            0.0
        )
        "#
    )
    .fetch_one(&database.pool)
    .await
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?
    .unwrap_or(0.0);
    
    stats.push(Stat {
        id: "equipment-utilization".to_string(),
        title: "Equipment Utilization".to_string(),
        value: format!("{}%", (equipment_utilization * 100.0).round()),
        icon: "fa-truck".to_string(),
        change_percentage: Some(equipment_utilization * 100.0),
        change_direction: Some(if equipment_utilization > 0.8 { 
            ChangeDirection::Up 
        } else if equipment_utilization < 0.3 { 
            ChangeDirection::Down 
        } else { 
            ChangeDirection::Neutral 
        }),
        link: Some("/equipment".to_string()),
    });
    
    let template = StatsTemplate { stats };
    
    match template.render() {
        Ok(html) => Ok(Html(html)),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn activity_component(
    State(database): State<Database>,
) -> Result<Html<String>, axum::http::StatusCode> {
    // Generate activity items based on recent database changes
    let mut activities = Vec::new();
    
    // Recent site creations
    let recent_sites = sqlx::query!(
        r#"
        SELECT s.id, s.name, s.description,
               tp.id as supervisor_id, 
               e.first_name || ' ' || e.last_name as supervisor_name
        FROM site s
        JOIN area a ON s.area_id = a.id
        JOIN technical_personnel tp ON a.supervisor_id = tp.id
        JOIN employee e ON tp.id = e.id
        ORDER BY s.id DESC
        LIMIT 5
        "#
    )
    .fetch_all(&database.pool)
    .await
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    
    for site in recent_sites {
        activities.push(ActivityItem {
            id: format!("site-{}", site.id),
            activity_type: ActivityType::SiteCreated,
            description: format!("New site \"{}\" was created", site.name),
            timestamp: Utc::now() - chrono::Duration::days(activities.len() as i64), // Just for demonstration
            user: Some(site.supervisor_name.unwrap_or_else(|| "Unknown".to_string())),
            link: Some(format!("/sites/{}", site.id)),
            entity_id: Some(site.id.to_string()),
            entity_type: Some("site".to_string()),
        });
    }
    
    // Recent completed tasks
    let completed_tasks = sqlx::query!(
        r#"
        SELECT t.id, t.name, t.actual_period_end,
               s.id as site_id, s.name as site_name,
               b.id as brigade_id,
               e.first_name || ' ' || e.last_name as brigadier_name
        FROM task t
        JOIN site s ON t.site_id = s.id
        JOIN brigade b ON t.brigade_id = b.id
        JOIN worker w ON b.brigadier_id = w.id
        JOIN employee e ON w.id = e.id
        WHERE t.actual_period_end IS NOT NULL
        ORDER BY t.actual_period_end DESC
        LIMIT 5
        "#
    )
    .fetch_all(&database.pool)
    .await
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    
    for task in completed_tasks {
        activities.push(ActivityItem {
            id: format!("task-{}", task.id),
            activity_type: ActivityType::TaskCompleted,
            description: format!("Task \"{}\" was completed at site \"{}\"", task.name, task.site_name),
            timestamp: task.actual_period_end
                .map(|d| chrono::DateTime::<Utc>::from_utc(
                    chrono::NaiveDateTime::new(d, chrono::NaiveTime::from_hms_opt(12, 0, 0).unwrap()),
                    Utc,
                ))
                .unwrap_or_else(Utc::now),
            user: Some(task.brigadier_name.unwrap_or_else(|| "Unknown".to_string())),
            link: Some(format!("/tasks/{}", task.id)),
            entity_id: Some(task.id.to_string()),
            entity_type: Some("task".to_string()),
        });
    }
    
    // Equipment allocations
    let allocations = sqlx::query!(
        r#"
        SELECT ea.equipment_id, ea.department_id, ea.site_id, ea.amount, ea.period_start,
               e.name as equipment_name,
               s.name as site_name,
               d.name as department_name
        FROM equipment_allocation ea
        JOIN equipment e ON ea.equipment_id = e.id
        JOIN department d ON ea.department_id = d.id
        LEFT JOIN site s ON ea.site_id = s.id
        ORDER BY ea.period_start DESC
        LIMIT 5
        "#
    )
    .fetch_all(&database.pool)
    .await
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    
    for allocation in allocations {
        let description = if let Some(site_id) = allocation.site_id {
            format!(
                "{} units of \"{}\" allocated to site \"{}\"",
                allocation.amount, allocation.equipment_name, allocation.site_name.unwrap_or_default()
            )
        } else {
            format!(
                "{} units of \"{}\" allocated to department \"{}\"",
                allocation.amount, allocation.equipment_name, allocation.department_name
            )
        };
        
        activities.push(ActivityItem {
            id: format!("allocation-{}-{}-{:?}", 
                allocation.equipment_id, allocation.department_id, allocation.site_id),
            activity_type: ActivityType::EquipmentAllocated,
            description,
            timestamp: allocation.period_start
                .map(|d| chrono::DateTime::<Utc>::from_utc(
                    chrono::NaiveDateTime::new(d, chrono::NaiveTime::from_hms_opt(12, 0, 0).unwrap()),
                    Utc,
                ))
                .unwrap_or_else(Utc::now),
            user: None,
            link: Some(format!("/equipment/{}", allocation.equipment_id)),
            entity_id: Some(allocation.equipment_id.to_string()),
            entity_type: Some("equipment".to_string()),
        });
    }
    
    // Sort activities by timestamp (newest first)
    activities.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
    
    // Limit to 10 most recent
    activities.truncate(10);
    
    let template = ActivityTemplate { activities };
    
    match template.render() {
        Ok(html) => Ok(Html(html)),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn global_search(
    Query(params): Query<SearchParams>,
    State(database): State<Database>,
) -> Result<Html<String>, axum::http::StatusCode> {
    let query = params.q.clone();
    let page = params.page.unwrap_or(1);
    let per_page = params.pagination.per_page.unwrap_or(10);
    let offset = (page - 1) * per_page;
    
    let mut results = Vec::new();
    let mut total_results = 0;
    
    if let Some(search_query) = &query {
        let search_pattern = format!("%{}%", search_query);
        
        // Search sites
        let sites = sqlx::query!(
            r#"
            SELECT s.id, s.name, s.description, 'name' as matching_field, 'Site' as entity_type
            FROM site s
            WHERE s.name ILIKE $1
            UNION
            SELECT s.id, s.name, s.description, 'description' as matching_field, 'Site' as entity_type
            FROM site s
            WHERE s.description ILIKE $1
            LIMIT $2 OFFSET $3
            "#,
            search_pattern,
            per_page as i64,
            offset as i64
        )
        .fetch_all(&database.pool)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
        
        for site in sites {
            results.push(SearchResult {
                id: site.id.to_string(),
                entity_type: EntityType::Site,
                title: site.name,
                description: site.description,
                matching_field: Some(site.matching_field),
                link: format!("/sites/{}", site.id),
            });
        }
        
        // Search departments
        let departments = sqlx::query!(
            r#"
            SELECT d.id, d.name, NULL as description, 'name' as matching_field, 'Department' as entity_type
            FROM department d
            WHERE d.name ILIKE $1
            LIMIT $2 OFFSET $3
            "#,
            search_pattern,
            per_page as i64,
            offset as i64
        )
        .fetch_all(&database.pool)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
        
        for department in departments {
            results.push(SearchResult {
                id: department.id.to_string(),
                entity_type: EntityType::Department,
                title: department.name,
                description: None,
                matching_field: Some(department.matching_field),
                link: format!("/departments/{}", department.id),
            });
        }
        
        // Search areas
        let areas = sqlx::query!(
            r#"
            SELECT a.id, a.name, NULL as description, 'name' as matching_field, 'Area' as entity_type
            FROM area a
            WHERE a.name ILIKE $1
            LIMIT $2 OFFSET $3
            "#,
            search_pattern,
            per_page as i64,
            offset as i64
        )
        .fetch_all(&database.pool)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
        
        for area in areas {
            results.push(SearchResult {
                id: area.id.to_string(),
                entity_type: EntityType::Area,
                title: area.name,
                description: None,
                matching_field: Some(area.matching_field),
                link: format!("/areas/{}", area.id),
            });
        }
        
        // Search personnel
        let personnel = sqlx::query!(
            r#"
            SELECT 
                e.id, 
                e.first_name || ' ' || e.last_name as title,
                e.phone_number as description,
                'name' as matching_field,
                'Personnel' as entity_type
            FROM employee e
            WHERE e.first_name ILIKE $1 OR e.last_name ILIKE $1
            LIMIT $2 OFFSET $3
            "#,
            search_pattern,
            per_page as i64,
            offset as i64
        )
        .fetch_all(&database.pool)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
        
        for person in personnel {
            results.push(SearchResult {
                id: person.id.to_string(),
                entity_type: EntityType::Personnel,
                title: person.title.unwrap_or_default(),
                description: Some(person.description.unwrap_or_default()),
                matching_field: Some(person.matching_field),
                link: format!("/personnel/{}", person.id),
            });
        }
        
        // Search equipment
        let equipment = sqlx::query!(
            r#"
            SELECT e.id, e.name as title, 'Available: ' || e.amount as description, 
                   'name' as matching_field, 'Equipment' as entity_type
            FROM equipment e
            WHERE e.name ILIKE $1
            LIMIT $2 OFFSET $3
            "#,
            search_pattern,
            per_page as i64,
            offset as i64
        )
        .fetch_all(&database.pool)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
        
        for equip in equipment {
            results.push(SearchResult {
                id: equip.id.to_string(),
                entity_type: EntityType::Equipment,
                title: equip.title.unwrap_or_default(),
                description: equip.description.map(|d| d.to_string()),
                matching_field: Some(equip.matching_field),
                link: format!("/equipment/{}", equip.id),
            });
        }
        
        // Search tasks
        let tasks = sqlx::query!(
            r#"
            SELECT 
                t.id, 
                t.name as title,
                t.description,
                'name' as matching_field,
                'Task' as entity_type
            FROM task t
            WHERE t.name ILIKE $1 OR t.description ILIKE $1
            LIMIT $2 OFFSET $3
            "#,
            search_pattern,
            per_page as i64,
            offset as i64
        )
        .fetch_all(&database.pool)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
        
        for task in tasks {
            results.push(SearchResult {
                id: task.id.to_string(),
                entity_type: EntityType::Task,
                title: task.title.unwrap_or_default(),
                description: task.description,
                matching_field: Some(task.matching_field),
                link: format!("/tasks/{}", task.id),
            });
        }
        
        // Search clients
        let clients = sqlx::query!(
            r#"
            SELECT 
                c.id, 
                c.name as title,
                c.address as description,
                'name' as matching_field,
                'Client' as entity_type
            FROM client c
            WHERE c.name ILIKE $1 OR c.contact_person_name ILIKE $1
            LIMIT $2 OFFSET $3
            "#,
            search_pattern,
            per_page as i64,
            offset as i64
        )
        .fetch_all(&database.pool)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
        
        for client in clients {
            results.push(SearchResult {
                id: client.id.to_string(),
                entity_type: EntityType::Client,
                title: client.title.unwrap_or_default(),
                description: client.description,
                matching_field: Some(client.matching_field),
                link: format!("/clients/{}", client.id),
            });
        }
        
        // Get total count for pagination
        total_results = sqlx::query_scalar!(
            r#"
            SELECT COUNT(*) FROM (
                SELECT s.id FROM site s WHERE s.name ILIKE $1 OR s.description ILIKE $1
                UNION ALL
                SELECT d.id FROM department d WHERE d.name ILIKE $1
                UNION ALL
                SELECT a.id FROM area a WHERE a.name ILIKE $1
                UNION ALL
                SELECT e.id FROM employee e WHERE e.first_name ILIKE $1 OR e.last_name ILIKE $1
                UNION ALL
                SELECT eq.id FROM equipment eq WHERE eq.name ILIKE $1
                UNION ALL
                SELECT t.id FROM task t WHERE t.name ILIKE $1 OR t.description ILIKE $1
                UNION ALL
                SELECT c.id FROM client c WHERE c.name ILIKE $1 OR c.contact_person_name ILIKE $1
            ) as combined_search
            "#,
            search_pattern
        )
        .fetch_one(&database.pool)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?
        .unwrap_or(0) as usize;
    }
    
    // Sort by entity type for better organization
    results.sort_by(|a, b| {
        let type_order = |et: &EntityType| -> usize {
            match et {
                EntityType::Site => 0,
                EntityType::Department => 1,
                EntityType::Area => 2,
                EntityType::Personnel => 3,
                EntityType::Equipment => 4,
                EntityType::Brigade => 5,
                EntityType::Task => 6,
                EntityType::Material => 7,
                EntityType::Client => 8,
            }
        };
        
        type_order(&a.entity_type).cmp(&type_order(&b.entity_type))
    });
    
    let total_pages = (total_results + per_page - 1) / per_page;
    
    let template = SearchResultsTemplate {
        results,
        query,
        total_results,
        current_page: page,
        total_pages,
    };
    
    match template.render() {
        Ok(html) => Ok(Html(html)),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
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
