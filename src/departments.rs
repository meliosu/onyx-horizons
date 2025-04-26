use axum::{
    extract::{Path, Query, State},
    response::Html,
    routing::{get, post, put, delete},
    Form, Json, Router,
};
use serde::{Deserialize, Serialize};
use askama::Template;
use chrono::Utc;
use rand::{thread_rng, Rng};

use crate::{database::Database, personnel::Qualification, sites::{RiskLevel, SiteType}};
use crate::general::{PaginationParams, SearchParams};

// Types for Department Management

#[derive(Serialize, Deserialize)]
pub struct Department {
    pub id: i32,
    pub supervisor_id: Option<i32>,
    pub name: String,
    pub supervisor_name: Option<String>, // Joined from technical_personnel
    pub areas_count: Option<i32>,        // Count of related areas
    pub sites_count: Option<i32>,        // Count of related sites
    pub personnel_count: Option<i32>,    // Count of personnel
}

#[derive(Deserialize)]
pub struct DepartmentCreate {
    pub name: String,
    pub supervisor_id: Option<i32>,
}

#[derive(Deserialize)]
pub struct DepartmentUpdate {
    pub name: String,
    pub supervisor_id: Option<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct Area {
    pub id: i32,
    pub department_id: i32,
    pub supervisor_id: Option<i32>,
    pub name: String,
    pub department_name: Option<String>, // Joined from department
    pub supervisor_name: Option<String>, // Joined from technical_personnel
    pub sites_count: Option<i32>,        // Count of related sites
    pub personnel_count: Option<i32>,    // Count of personnel
}

#[derive(Deserialize)]
pub struct AreaCreate {
    pub name: String,
    pub department_id: i32,
    pub supervisor_id: Option<i32>,
}

#[derive(Deserialize)]
pub struct AreaUpdate {
    pub name: String,
    pub department_id: i32,
    pub supervisor_id: Option<i32>,
}

#[derive(Serialize)]
pub struct Supervisor {
    pub id: i32,
    pub name: String,      // Concatenated first_name + last_name
    pub qualification: String,
    pub position: Option<String>,
}

#[derive(Deserialize)]
pub struct DepartmentFilter {
    pub name: Option<String>,
    pub supervisor_id: Option<i32>,
    #[serde(flatten)]
    pub pagination: PaginationParams,
}

#[derive(Deserialize)]
pub struct AreaFilter {
    pub name: Option<String>,
    pub department_id: Option<i32>,
    pub supervisor_id: Option<i32>,
    #[serde(flatten)]
    pub pagination: PaginationParams,
}

// Template types for Department pages
#[derive(Template)]
#[template(path = "departments/index.html")]
pub struct DepartmentsPageTemplate {
    pub departments: Vec<Department>,
    pub current_page: usize,
    pub total_pages: usize,
    pub filter: Option<DepartmentFilter>,
}

#[derive(Template)]
#[template(path = "departments/new.html")]
pub struct DepartmentNewPageTemplate {
    pub supervisors: Vec<Supervisor>,
}

#[derive(Template)]
#[template(path = "departments/details.html")]
pub struct DepartmentDetailsPageTemplate {
    pub department: Department,
    pub active_tab: String,
}

#[derive(Template)]
#[template(path = "departments/edit.html")]
pub struct DepartmentEditPageTemplate {
    pub department: Department,
    pub supervisors: Vec<Supervisor>,
}

// Template types for Area pages
#[derive(Template)]
#[template(path = "areas/index.html")]
pub struct AreasPageTemplate {
    pub areas: Vec<Area>,
    pub departments: Vec<Department>,
    pub current_page: usize,
    pub total_pages: usize,
    pub filter: Option<AreaFilter>,
}

#[derive(Template)]
#[template(path = "areas/new.html")]
pub struct AreaNewPageTemplate {
    pub departments: Vec<Department>,
    pub supervisors: Vec<Supervisor>,
}

#[derive(Template)]
#[template(path = "areas/details.html")]
pub struct AreaDetailsPageTemplate {
    pub area: Area,
    pub department: Department,
    pub active_tab: String,
}

#[derive(Template)]
#[template(path = "areas/edit.html")]
pub struct AreaEditPageTemplate {
    pub area: Area,
    pub departments: Vec<Department>,
    pub supervisors: Vec<Supervisor>,
}

// Template types for HTMX components - Department
#[derive(Template)]
#[template(path = "departments/components/department_rows.html")]
pub struct DepartmentRowsTemplate {
    pub departments: Vec<Department>,
}

#[derive(Template)]
#[template(path = "departments/components/department_details.html")]
pub struct DepartmentDetailsTemplate {
    pub department: Department,
}

#[derive(Template)]
#[template(path = "departments/components/department_areas.html")]
pub struct DepartmentAreasTemplate {
    pub areas: Vec<Area>,
    pub department_id: i32,
    pub current_page: usize,
    pub total_pages: usize,
}

#[derive(Template)]
#[template(path = "departments/components/department_equipment.html")]
pub struct DepartmentEquipmentTemplate {
    pub equipment: Vec<Equipment>,
    pub department_id: i32,
    pub current_page: usize,
    pub total_pages: usize,
}

#[derive(Template)]
#[template(path = "departments/components/department_sites.html")]
pub struct DepartmentSitesTemplate {
    pub sites: Vec<Site>,
    pub department_id: i32,
    pub current_page: usize,
    pub total_pages: usize,
}

#[derive(Template)]
#[template(path = "departments/components/department_personnel.html")]
pub struct DepartmentPersonnelTemplate {
    pub personnel: Vec<TechnicalPersonnel>,
    pub department_id: i32,
    pub current_page: usize,
    pub total_pages: usize,
}

// Template types for HTMX components - Area
#[derive(Template)]
#[template(path = "areas/components/area_rows.html")]
pub struct AreaRowsTemplate {
    pub areas: Vec<Area>,
}

#[derive(Template)]
#[template(path = "areas/components/area_details.html")]
pub struct AreaDetailsTemplate {
    pub area: Area,
    pub department: Department,
}

#[derive(Template)]
#[template(path = "areas/components/area_sites.html")]
pub struct AreaSitesTemplate {
    pub sites: Vec<Site>,
    pub area_id: i32,
    pub current_page: usize,
    pub total_pages: usize,
}

#[derive(Template)]
#[template(path = "areas/components/area_personnel.html")]
pub struct AreaPersonnelTemplate {
    pub personnel: Vec<TechnicalPersonnel>,
    pub area_id: i32,
    pub current_page: usize,
    pub total_pages: usize,
}

// Template types for supervisor selectors
#[derive(Template)]
#[template(path = "departments/components/supervisor_selector.html")]
pub struct DepartmentSupervisorSelectorTemplate {
    pub supervisors: Vec<Supervisor>,
}

#[derive(Template)]
#[template(path = "areas/components/supervisor_selector.html")]
pub struct AreaSupervisorSelectorTemplate {
    pub supervisors: Vec<Supervisor>,
}

// Additional data types needed for templates
#[derive(Serialize)]
pub struct Equipment {
    pub id: i32,
    pub name: String,
    pub amount: i32,
    pub available_amount: Option<i32>,
    pub purchase_date: String,
    pub purchase_cost: f64,
    pub fuel_type: Option<String>,
}

#[derive(Serialize)]
pub struct Site {
    pub id: i32,
    pub name: String,
    pub area_id: i32,
    pub client_id: i32,
    pub site_type: SiteType,
    pub risk_level: RiskLevel,
    pub description: Option<String>,
    pub area_name: Option<String>,
    pub client_name: Option<String>,
    pub status: Option<String>,
}

#[derive(Serialize)]
pub struct TechnicalPersonnel {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub qualification: Qualification,
    pub position: Option<String>,
    pub education_level: String,
    pub is_project_manager: bool,
    pub full_name: String,
}

// Page Endpoints

// Department pages
async fn departments_page(
    State(database): State<Database>,
    Query(params): Query<DepartmentFilter>,
) -> Html<String> {
    let page = params.pagination.page.unwrap_or(1);
    let per_page = params.pagination.per_page.unwrap_or(20);
    let offset = (page - 1) * per_page;
    
    // Base query for filtering departments
    let mut query = "
        SELECT 
            d.id, 
            d.name, 
            d.supervisor_id,
            tp.id as tech_personnel_id,
            e.first_name || ' ' || e.last_name as supervisor_name,
            COUNT(DISTINCT a.id) as areas_count,
            COUNT(DISTINCT s.id) as sites_count,
            COUNT(DISTINCT tp2.id) as personnel_count
        FROM 
            department d
        LEFT JOIN 
            technical_personnel tp ON d.supervisor_id = tp.id
        LEFT JOIN 
            employee e ON tp.id = e.id
        LEFT JOIN 
            area a ON a.department_id = d.id
        LEFT JOIN 
            site s ON s.area_id = a.id
        LEFT JOIN 
            technical_personnel tp2 ON tp2.id IN (
                SELECT supervisor_id FROM area WHERE department_id = d.id
                UNION
                SELECT d.supervisor_id WHERE d.supervisor_id IS NOT NULL
            )
    ".to_string();
    
    let mut conditions = Vec::new();
    
    if let Some(name) = &params.name {
        conditions.push(format!("d.name ILIKE '%{}%'", name.replace('\'', "''")));
    }
    
    if let Some(supervisor_id) = params.supervisor_id {
        conditions.push(format!("d.supervisor_id = {}", supervisor_id));
    }
    
    if !conditions.is_empty() {
        query.push_str(" WHERE ");
        query.push_str(&conditions.join(" AND "));
    }
    
    query.push_str(" GROUP BY d.id, d.name, d.supervisor_id, tp.id, e.first_name, e.last_name");
    
    let sort_by = params.pagination.sort_by.as_deref().unwrap_or("d.name");
    let sort_dir = params.pagination.sort_dir.as_deref().unwrap_or("asc");
    query.push_str(&format!(" ORDER BY {} {}", sort_by, sort_dir));
    
    query.push_str(&format!(" LIMIT {} OFFSET {}", per_page, offset));
    
    let departments_result = sqlx::query(&query)
        .fetch_all(&database.pool)
        .await;
    
    let departments = match departments_result {
        Ok(rows) => {
            rows.iter().map(|row| {
                Department {
                    id: row.get("id"),
                    supervisor_id: row.get("supervisor_id"),
                    name: row.get("name"),
                    supervisor_name: row.get("supervisor_name"),
                    areas_count: Some(row.get::<i64, _>("areas_count") as i32),
                    sites_count: Some(row.get::<i64, _>("sites_count") as i32),
                    personnel_count: Some(row.get::<i64, _>("personnel_count") as i32),
                }
            }).collect()
        },
        Err(_) => Vec::new(),
    };
    
    // Count total for pagination
    let count_query = "SELECT COUNT(*) FROM department";
    let total_count = sqlx::query_scalar::<_, i64>(count_query)
        .fetch_one(&database.pool)
        .await
        .unwrap_or(0) as usize;
    
    let total_pages = (total_count + per_page - 1) / per_page;
    
    let template = DepartmentsPageTemplate {
        departments,
        current_page: page,
        total_pages,
        filter: Some(params),
    };
    
    match template.render() {
        Ok(html) => Html(html),
        Err(_) => Html(String::from("<p>Error rendering departments page</p>")),
    }
}

async fn department_new_page(
    State(database): State<Database>,
) -> Html<String> {
    // Fetch available supervisors
    let supervisors_result = sqlx::query!(
        r#"
        SELECT 
            tp.id, 
            e.first_name || ' ' || e.last_name as name, 
            tp.qualification::text
        FROM 
            technical_personnel tp
        JOIN 
            employee e ON tp.id = e.id
        WHERE 
            tp.is_project_manager = true
        ORDER BY 
            name
        "#
    )
    .fetch_all(&database.pool)
    .await;
    
    let supervisors = match supervisors_result {
        Ok(rows) => {
            rows.iter().map(|row| {
                Supervisor {
                    id: row.id,
                    name: row.name.clone().unwrap_or_default(),
                    qualification: row.qualification.clone().unwrap_or_default(),
                    position: None,
                }
            }).collect()
        },
        Err(_) => Vec::new(),
    };
    
    let template = DepartmentNewPageTemplate { supervisors };
    
    match template.render() {
        Ok(html) => Html(html),
        Err(_) => Html(String::from("<p>Error rendering new department page</p>")),
    }
}

async fn department_details_page(
    State(database): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Fetch department details
    let department_result = sqlx::query!(
        r#"
        SELECT 
            d.id, 
            d.name, 
            d.supervisor_id,
            e.first_name || ' ' || e.last_name as supervisor_name,
            COUNT(DISTINCT a.id) as areas_count,
            COUNT(DISTINCT s.id) as sites_count,
            COUNT(DISTINCT tp2.id) as personnel_count
        FROM 
            department d
        LEFT JOIN 
            technical_personnel tp ON d.supervisor_id = tp.id
        LEFT JOIN 
            employee e ON tp.id = e.id
        LEFT JOIN 
            area a ON a.department_id = d.id
        LEFT JOIN 
            site s ON s.area_id = a.id
        LEFT JOIN 
            technical_personnel tp2 ON tp2.id IN (
                SELECT supervisor_id FROM area WHERE department_id = d.id
                UNION
                SELECT d.supervisor_id WHERE d.supervisor_id IS NOT NULL
            )
        WHERE 
            d.id = $1
        GROUP BY 
            d.id, d.name, d.supervisor_id, tp.id, e.first_name, e.last_name
        "#,
        id
    )
    .fetch_optional(&database.pool)
    .await;
    
    match department_result {
        Ok(Some(row)) => {
            let department = Department {
                id: row.id,
                supervisor_id: row.supervisor_id,
                name: row.name,
                supervisor_name: row.supervisor_name,
                areas_count: Some(row.areas_count.unwrap_or(0) as i32),
                sites_count: Some(row.sites_count.unwrap_or(0) as i32),
                personnel_count: Some(row.personnel_count.unwrap_or(0) as i32),
            };
            
            let template = DepartmentDetailsPageTemplate {
                department,
                active_tab: "areas".to_string(), // Default active tab
            };
            
            match template.render() {
                Ok(html) => Html(html),
                Err(_) => Html(String::from("<p>Error rendering department details page</p>")),
            }
        },
        Ok(None) => Html(String::from("<p>Department not found</p>")),
        Err(_) => Html(String::from("<p>Error fetching department details</p>")),
    }
}

async fn department_edit_page(
    State(database): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Fetch department details
    let department_result = sqlx::query!(
        r#"
        SELECT 
            d.id, 
            d.name, 
            d.supervisor_id,
            e.first_name || ' ' || e.last_name as supervisor_name
        FROM 
            department d
        LEFT JOIN 
            technical_personnel tp ON d.supervisor_id = tp.id
        LEFT JOIN 
            employee e ON tp.id = e.id
        WHERE 
            d.id = $1
        "#,
        id
    )
    .fetch_optional(&database.pool)
    .await;
    
    // Fetch available supervisors
    let supervisors_result = sqlx::query!(
        r#"
        SELECT 
            tp.id, 
            e.first_name || ' ' || e.last_name as name, 
            tp.qualification::text
        FROM 
            technical_personnel tp
        JOIN 
            employee e ON tp.id = e.id
        WHERE 
            tp.is_project_manager = true
        ORDER BY 
            name
        "#
    )
    .fetch_all(&database.pool)
    .await;
    
    match (department_result, supervisors_result) {
        (Ok(Some(dept_row)), Ok(sup_rows)) => {
            let department = Department {
                id: dept_row.id,
                supervisor_id: dept_row.supervisor_id,
                name: dept_row.name,
                supervisor_name: dept_row.supervisor_name,
                areas_count: None,
                sites_count: None,
                personnel_count: None,
            };
            
            let supervisors = sup_rows.iter().map(|row| {
                Supervisor {
                    id: row.id,
                    name: row.name.clone().unwrap_or_default(),
                    qualification: row.qualification.clone().unwrap_or_default(),
                    position: None,
                }
            }).collect();
            
            let template = DepartmentEditPageTemplate {
                department,
                supervisors,
            };
            
            match template.render() {
                Ok(html) => Html(html),
                Err(_) => Html(String::from("<p>Error rendering department edit page</p>")),
            }
        },
        (Ok(None), _) => Html(String::from("<p>Department not found</p>")),
        _ => Html(String::from("<p>Error fetching department data</p>")),
    }
}

// Area pages
async fn areas_page(
    State(database): State<Database>,
    Query(params): Query<AreaFilter>,
) -> Html<String> {
    let page = params.pagination.page.unwrap_or(1);
    let per_page = params.pagination.per_page.unwrap_or(20);
    let offset = (page - 1) * per_page;
    
    // Base query for filtering areas
    let mut query = "
        SELECT 
            a.id, 
            a.department_id,
            a.supervisor_id,
            a.name,
            d.name as department_name,
            e.first_name || ' ' || e.last_name as supervisor_name,
            COUNT(DISTINCT s.id) as sites_count,
            COUNT(DISTINCT tp.id) as personnel_count
        FROM 
            area a
        JOIN 
            department d ON a.department_id = d.id
        LEFT JOIN 
            technical_personnel tp ON a.supervisor_id = tp.id
        LEFT JOIN 
            employee e ON tp.id = e.id
        LEFT JOIN 
            site s ON s.area_id = a.id
    ".to_string();
    
    let mut conditions = Vec::new();
    let mut params_vec: Vec<&(dyn sqlx::Encode<sqlx::Postgres> + Sync)> = Vec::new();
    let mut param_index = 1;
    
    // Add filter conditions
    if let Some(name) = &params.name {
        conditions.push(format!("a.name ILIKE ${}", param_index));
        params_vec.push(&format!("%{}%", name));
        param_index += 1;
    }
    
    if let Some(department_id) = params.department_id {
        conditions.push(format!("a.department_id = ${}", param_index));
        params_vec.push(&department_id);
        param_index += 1;
    }
    
    if let Some(supervisor_id) = params.supervisor_id {
        conditions.push(format!("a.supervisor_id = ${}", param_index));
        params_vec.push(&supervisor_id);
        param_index += 1;
    }
    
    // Add WHERE clause if there are conditions
    if !conditions.is_empty() {
        query.push_str(" WHERE ");
        query.push_str(&conditions.join(" AND "));
    }
    
    // Add GROUP BY clause
    query.push_str(" GROUP BY a.id, a.name, a.department_id, a.supervisor_id, d.name, e.first_name, e.last_name");
    
    // Add ordering
    let sort_by = params.pagination.sort_by.as_deref().unwrap_or("a.name");
    let sort_dir = params.pagination.sort_dir.as_deref().unwrap_or("asc");
    query.push_str(&format!(" ORDER BY {} {}", sort_by, sort_dir));
    
    // Add pagination
    query.push_str(&format!(" LIMIT {} OFFSET {}", per_page, offset));
    
    // Execute query
    let areas_result = sqlx::query(&query)
        .fetch_all(&database.pool)
        .await;
    
    // Handle potential error
    let areas = match areas_result {
        Ok(rows) => {
            rows.iter().map(|row| {
                Area {
                    id: row.get("id"),
                    department_id: row.get("department_id"),
                    supervisor_id: row.get("supervisor_id"),
                    name: row.get("name"),
                    department_name: row.get("department_name"),
                    supervisor_name: row.get("supervisor_name"),
                    sites_count: Some(row.get::<i64, _>("sites_count") as i32),
                    personnel_count: Some(row.get::<i64, _>("personnel_count") as i32),
                }
            }).collect::<Vec<_>>()
        },
        Err(_) => Vec::new(), // Return empty list on error
    };
    
    // Fetch departments for filter dropdown
    let departments_result = sqlx::query!(
        "SELECT id, name FROM department ORDER BY name"
    )
    .fetch_all(&database.pool)
    .await;
    
    let departments = match departments_result {
        Ok(rows) => {
            rows.iter().map(|row| {
                Department {
                    id: row.id,
                    name: row.name.clone(),
                    supervisor_id: None,
                    supervisor_name: None,
                    areas_count: None,
                    sites_count: None,
                    personnel_count: None,
                }
            }).collect()
        },
        Err(_) => Vec::new(),
    };
    
    // Count total for pagination
    let count_query = "SELECT COUNT(*) FROM area";
    let total_count = sqlx::query_scalar::<_, i64>(count_query)
        .fetch_one(&database.pool)
        .await
        .unwrap_or(0) as usize;
    
    let total_pages = (total_count + per_page - 1) / per_page;
    
    // Render template
    let template = AreasPageTemplate {
        areas,
        departments,
        current_page: page,
        total_pages,
        filter: Some(params),
    };
    
    match template.render() {
        Ok(html) => Html(html),
        Err(_) => Html(String::from("<p>Error rendering areas page</p>")),
    }
}

async fn area_new_page(
    State(database): State<Database>,
) -> Html<String> {
    // Fetch departments for dropdown
    let departments_result = sqlx::query!(
        "SELECT id, name FROM department ORDER BY name"
    )
    .fetch_all(&database.pool)
    .await;
    
    // Fetch available supervisors
    let supervisors_result = sqlx::query!(
        r#"
        SELECT 
            tp.id, 
            e.first_name || ' ' || e.last_name as name, 
            tp.qualification::text
        FROM 
            technical_personnel tp
        JOIN 
            employee e ON tp.id = e.id
        WHERE 
            tp.is_project_manager = true
        ORDER BY 
            name
        "#
    )
    .fetch_all(&database.pool)
    .await;
    
    let departments = match departments_result {
        Ok(rows) => {
            rows.iter().map(|row| {
                Department {
                    id: row.id,
                    name: row.name.clone(),
                    supervisor_id: None,
                    supervisor_name: None,
                    areas_count: None,
                    sites_count: None,
                    personnel_count: None,
                }
            }).collect()
        },
        Err(_) => Vec::new(),
    };
    
    let supervisors = match supervisors_result {
        Ok(rows) => {
            rows.iter().map(|row| {
                Supervisor {
                    id: row.id,
                    name: row.name.clone().unwrap_or_default(),
                    qualification: row.qualification.clone().unwrap_or_default(),
                    position: None,
                }
            }).collect()
        },
        Err(_) => Vec::new(),
    };
    
    let template = AreaNewPageTemplate {
        departments,
        supervisors,
    };
    
    match template.render() {
        Ok(html) => Html(html),
        Err(_) => Html(String::from("<p>Error rendering new area page</p>")),
    }
}

async fn area_details_page(
    State(database): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Fetch area details
    let area_result = sqlx::query!(
        r#"
        SELECT 
            a.id, 
            a.name, 
            a.department_id,
            a.supervisor_id,
            d.name as department_name,
            e.first_name || ' ' || e.last_name as supervisor_name,
            COUNT(DISTINCT s.id) as sites_count,
            COUNT(DISTINCT tp2.id) as personnel_count
        FROM 
            area a
        JOIN 
            department d ON a.department_id = d.id
        LEFT JOIN 
            technical_personnel tp ON a.supervisor_id = tp.id
        LEFT JOIN 
            employee e ON tp.id = e.id
        LEFT JOIN 
            site s ON s.area_id = a.id
        LEFT JOIN 
            technical_personnel tp2 ON tp2.id = a.supervisor_id
        WHERE 
            a.id = $1
        GROUP BY 
            a.id, a.name, a.department_id, a.supervisor_id, d.name, e.first_name, e.last_name
        "#,
        id
    )
    .fetch_optional(&database.pool)
    .await;
    
    match area_result {
        Ok(Some(row)) => {
            let area = Area {
                id: row.id,
                department_id: row.department_id,
                supervisor_id: row.supervisor_id,
                name: row.name,
                department_name: Some(row.department_name),
                supervisor_name: row.supervisor_name,
                sites_count: Some(row.sites_count.unwrap_or(0) as i32),
                personnel_count: Some(row.personnel_count.unwrap_or(0) as i32),
            };
            
            // Fetch department basic info
            let department_result = sqlx::query!(
                "SELECT id, name, supervisor_id FROM department WHERE id = $1",
                row.department_id
            )
            .fetch_optional(&database.pool)
            .await;
            
            let department = match department_result {
                Ok(Some(dept)) => Department {
                    id: dept.id,
                    name: dept.name,
                    supervisor_id: dept.supervisor_id,
                    supervisor_name: None,
                    areas_count: None,
                    sites_count: None,
                    personnel_count: None,
                },
                _ => Department {
                    id: row.department_id,
                    name: row.department_name,
                    supervisor_id: None,
                    supervisor_name: None,
                    areas_count: None,
                    sites_count: None,
                    personnel_count: None,
                },
            };
            
            let template = AreaDetailsPageTemplate {
                area,
                department,
                active_tab: "sites".to_string(), // Default active tab
            };
            
            match template.render() {
                Ok(html) => Html(html),
                Err(_) => Html(String::from("<p>Error rendering area details page</p>")),
            }
        },
        Ok(None) => Html(String::from("<p>Area not found</p>")),
        Err(_) => Html(String::from("<p>Error fetching area details</p>")),
    }
}

async fn area_edit_page(
    State(database): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Fetch area details
    let area_result = sqlx::query!(
        r#"
        SELECT 
            a.id, 
            a.name, 
            a.department_id,
            a.supervisor_id,
            d.name as department_name,
            e.first_name || ' ' || e.last_name as supervisor_name
        FROM 
            area a
        JOIN 
            department d ON a.department_id = d.id
        LEFT JOIN 
            technical_personnel tp ON a.supervisor_id = tp.id
        LEFT JOIN 
            employee e ON tp.id = e.id
        WHERE 
            a.id = $1
        "#,
        id
    )
    .fetch_optional(&database.pool)
    .await;
    
    // Fetch departments for dropdown
    let departments_result = sqlx::query!(
        "SELECT id, name FROM department ORDER BY name"
    )
    .fetch_all(&database.pool)
    .await;
    
    // Fetch available supervisors
    let supervisors_result = sqlx::query!(
        r#"
        SELECT 
            tp.id, 
            e.first_name || ' ' || e.last_name as name, 
            tp.qualification::text
        FROM 
            technical_personnel tp
        JOIN 
            employee e ON tp.id = e.id
        WHERE 
            tp.is_project_manager = true
        ORDER BY 
            name
        "#
    )
    .fetch_all(&database.pool)
    .await;
    
    match (area_result, departments_result, supervisors_result) {
        (Ok(Some(area_row)), Ok(dept_rows), Ok(sup_rows)) => {
            let area = Area {
                id: area_row.id,
                department_id: area_row.department_id,
                supervisor_id: area_row.supervisor_id,
                name: area_row.name,
                department_name: Some(area_row.department_name),
                supervisor_name: area_row.supervisor_name,
                sites_count: None,
                personnel_count: None,
            };
            
            let departments = dept_rows.iter().map(|row| {
                Department {
                    id: row.id,
                    name: row.name.clone(),
                    supervisor_id: None,
                    supervisor_name: None,
                    areas_count: None,
                    sites_count: None,
                    personnel_count: None,
                }
            }).collect();
            
            let supervisors = sup_rows.iter().map(|row| {
                Supervisor {
                    id: row.id,
                    name: row.name.clone().unwrap_or_default(),
                    qualification: row.qualification.clone().unwrap_or_default(),
                    position: None,
                }
            }).collect();
            
            let template = AreaEditPageTemplate {
                area,
                departments,
                supervisors,
            };
            
            match template.render() {
                Ok(html) => Html(html),
                Err(_) => Html(String::from("<p>Error rendering area edit page</p>")),
            }
        },
        (Ok(None), _, _) => Html(String::from("<p>Area not found</p>")),
        _ => Html(String::from("<p>Error fetching area data</p>")),
    }
}

// HTMX Endpoints

// Department API endpoints
async fn fetch_departments(
    State(database): State<Database>,
    Query(params): Query<DepartmentFilter>,
) -> Html<String> {
    let page = params.pagination.page.unwrap_or(1);
    let per_page = params.pagination.per_page.unwrap_or(20);
    let offset = (page - 1) * per_page;
    
    // Build query similar to departments_page but for HTMX component
    let mut query = "
        SELECT 
            d.id, 
            d.name, 
            d.supervisor_id,
            e.first_name || ' ' || e.last_name as supervisor_name,
            COUNT(DISTINCT a.id) as areas_count,
            COUNT(DISTINCT s.id) as sites_count,
            COUNT(DISTINCT tp2.id) as personnel_count
        FROM 
            department d
        LEFT JOIN 
            technical_personnel tp ON d.supervisor_id = tp.id
        LEFT JOIN 
            employee e ON tp.id = e.id
        LEFT JOIN 
            area a ON a.department_id = d.id
        LEFT JOIN 
            site s ON s.area_id = a.id
        LEFT JOIN 
            technical_personnel tp2 ON tp2.id IN (
                SELECT supervisor_id FROM area WHERE department_id = d.id
                UNION
                SELECT d.supervisor_id WHERE d.supervisor_id IS NOT NULL
            )
    ".to_string();
    
    let mut conditions = Vec::new();
    
    if let Some(name) = &params.name {
        conditions.push(format!("d.name ILIKE '%{}%'", name.replace('\'', "''")));
    }
    
    if let Some(supervisor_id) = params.supervisor_id {
        conditions.push(format!("d.supervisor_id = {}", supervisor_id));
    }
    
    if !conditions.is_empty() {
        query.push_str(" WHERE ");
        query.push_str(&conditions.join(" AND "));
    }
    
    query.push_str(" GROUP BY d.id, d.name, d.supervisor_id, tp.id, e.first_name, e.last_name");
    
    let sort_by = params.pagination.sort_by.as_deref().unwrap_or("d.name");
    let sort_dir = params.pagination.sort_dir.as_deref().unwrap_or("asc");
    query.push_str(&format!(" ORDER BY {} {}", sort_by, sort_dir));
    
    query.push_str(&format!(" LIMIT {} OFFSET {}", per_page, offset));
    
    let departments_result = sqlx::query(&query)
        .fetch_all(&database.pool)
        .await;
    
    let departments = match departments_result {
        Ok(rows) => {
            rows.iter().map(|row| {
                Department {
                    id: row.get("id"),
                    supervisor_id: row.get("supervisor_id"),
                    name: row.get("name"),
                    supervisor_name: row.get("supervisor_name"),
                    areas_count: Some(row.get::<i64, _>("areas_count") as i32),
                    sites_count: Some(row.get::<i64, _>("sites_count") as i32),
                    personnel_count: Some(row.get::<i64, _>("personnel_count") as i32),
                }
            }).collect()
        },
        Err(_) => Vec::new(),
    };
    
    let template = DepartmentRowsTemplate { departments };
    
    match template.render() {
        Ok(html) => Html(html),
        Err(_) => Html(String::from("<p>Error rendering department rows</p>")),
    }
}

async fn create_department(
    State(database): State<Database>,
    Form(department): Form<DepartmentCreate>,
) -> Html<String> {
    // Insert new department
    let result = sqlx::query!(
        "INSERT INTO department (name, supervisor_id) VALUES ($1, $2) RETURNING id",
        department.name,
        department.supervisor_id
    )
    .fetch_one(&database.pool)
    .await;
    
    match result {
        Ok(_) => {
            // Return success notification
            let template = SuccessNotificationTemplate {
                message: format!("Department '{}' created successfully", department.name),
            };
            
            match template.render() {
                Ok(html) => Html(html),
                Err(_) => Html(String::from("<p>Department created but failed to render notification</p>")),
            }
        },
        Err(_) => Html(String::from("<p>Failed to create department</p>")),
    }
}

async fn fetch_department_details(
    State(database): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Fetch department details
    let department_result = sqlx::query!(
        r#"
        SELECT 
            d.id, 
            d.name, 
            d.supervisor_id,
            e.first_name || ' ' || e.last_name as supervisor_name
        FROM 
            department d
        LEFT JOIN 
            technical_personnel tp ON d.supervisor_id = tp.id
        LEFT JOIN 
            employee e ON tp.id = e.id
        WHERE 
            d.id = $1
        "#,
        id
    )
    .fetch_optional(&database.pool)
    .await;
    
    match department_result {
        Ok(Some(row)) => {
            let department = Department {
                id: row.id,
                supervisor_id: row.supervisor_id,
                name: row.name,
                supervisor_name: row.supervisor_name,
                areas_count: None,
                sites_count: None,
                personnel_count: None,
            };
            
            let template = DepartmentDetailsTemplate { department };
            
            match template.render() {
                Ok(html) => Html(html),
                Err(_) => Html(String::from("<p>Error rendering department details</p>")),
            }
        },
        Ok(None) => Html(String::from("<p>Department not found</p>")),
        Err(_) => Html(String::from("<p>Error fetching department details</p>")),
    }
}

async fn update_department(
    State(database): State<Database>,
    Path(id): Path<i32>,
    Form(department): Form<DepartmentUpdate>,
) -> Html<String> {
    // Update department
    let result = sqlx::query!(
        "UPDATE department SET name = $1, supervisor_id = $2 WHERE id = $3",
        department.name,
        department.supervisor_id,
        id
    )
    .execute(&database.pool)
    .await;
    
    match result {
        Ok(_) => {
            // Fetch updated department details
            let department_result = sqlx::query!(
                r#"
                SELECT 
                    d.id, 
                    d.name, 
                    d.supervisor_id,
                    e.first_name || ' ' || e.last_name as supervisor_name
                FROM 
                    department d
                LEFT JOIN 
                    technical_personnel tp ON d.supervisor_id = tp.id
                LEFT JOIN 
                    employee e ON tp.id = e.id
                WHERE 
                    d.id = $1
                "#,
                id
            )
            .fetch_optional(&database.pool)
            .await;
            
            match department_result {
                Ok(Some(row)) => {
                    let department = Department {
                        id: row.id,
                        supervisor_id: row.supervisor_id,
                        name: row.name,
                        supervisor_name: row.supervisor_name,
                        areas_count: None,
                        sites_count: None,
                        personnel_count: None,
                    };
                    
                    let template = DepartmentDetailsTemplate { department };
                    
                    match template.render() {
                        Ok(html) => Html(html),
                        Err(_) => Html(String::from("<p>Department updated but failed to render details</p>")),
                    }
                },
                _ => Html(String::from("<p>Department updated but failed to fetch details</p>")),
            }
        },
        Err(_) => Html(String::from("<p>Failed to update department</p>")),
    }
}

async fn delete_department(
    State(database): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // First, check if department has any areas
    let areas_count = sqlx::query_scalar!(
        "SELECT COUNT(*) FROM area WHERE department_id = $1",
        id
    )
    .fetch_one(&database.pool)
    .await;
    
    match areas_count {
        Ok(count) if count > 0 => {
            // Cannot delete department with areas
            Html(String::from("<p class=\"text-red-500\">Cannot delete department with associated areas</p>"))
        },
        Ok(_) => {
            // Delete department
            let result = sqlx::query!("DELETE FROM department WHERE id = $1", id)
                .execute(&database.pool)
                .await;
            
            match result {
                Ok(_) => {
                    let template = SuccessNotificationTemplate {
                        message: "Department deleted successfully".to_string(),
                    };
                    
                    match template.render() {
                        Ok(html) => Html(html),
                        Err(_) => Html(String::from("<p>Department deleted but failed to render notification</p>")),
                    }
                },
                Err(_) => Html(String::from("<p>Failed to delete department</p>")),
            }
        },
        Err(_) => Html(String::from("<p>Error checking if department can be deleted</p>")),
    }
}

async fn fetch_department_areas(
    State(database): State<Database>,
    Path(id): Path<i32>,
    Query(params): Query<PaginationParams>,
) -> Html<String> {
    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(10);
    let offset = (page - 1) * per_page;
    
    // Fetch areas for department
    let areas_result = sqlx::query!(
        r#"
        SELECT 
            a.id, 
            a.name, 
            a.department_id,
            a.supervisor_id,
            e.first_name || ' ' || e.last_name as supervisor_name,
            COUNT(DISTINCT s.id) as sites_count
        FROM 
            area a
        LEFT JOIN 
            technical_personnel tp ON a.supervisor_id = tp.id
        LEFT JOIN 
            employee e ON tp.id = e.id
        LEFT JOIN 
            site s ON s.area_id = a.id
        WHERE 
            a.department_id = $1
        GROUP BY 
            a.id, a.name, a.department_id, a.supervisor_id, e.first_name, e.last_name
        ORDER BY 
            a.name
        LIMIT $2 OFFSET $3
        "#,
        id,
        per_page as i64,
        offset as i64
    )
    .fetch_all(&database.pool)
    .await;
    
    // Count total for pagination
    let total_count_result = sqlx::query_scalar!(
        "SELECT COUNT(*) FROM area WHERE department_id = $1",
        id
    )
    .fetch_one(&database.pool)
    .await;
    
    match (areas_result, total_count_result) {
        (Ok(rows), Ok(total_count)) => {
            let areas = rows.iter().map(|row| {
                Area {
                    id: row.id,
                    department_id: row.department_id,
                    supervisor_id: row.supervisor_id,
                    name: row.name.clone(),
                    department_name: None, // Not needed for this component
                    supervisor_name: row.supervisor_name.clone(),
                    sites_count: Some(row.sites_count.unwrap_or(0) as i32),
                    personnel_count: None,
                }
            }).collect();
            
            let total_pages = (total_count as usize + per_page - 1) / per_page;
            
            let template = DepartmentAreasTemplate {
                areas,
                department_id: id,
                current_page: page,
                total_pages,
            };
            
            match template.render() {
                Ok(html) => Html(html),
                Err(_) => Html(String::from("<p>Error rendering department areas</p>")),
            }
        },
        _ => Html(String::from("<p>Error fetching department areas</p>")),
    }
}

async fn fetch_department_equipment(
    State(database): State<Database>,
    Path(id): Path<i32>,
    Query(params): Query<PaginationParams>,
) -> Html<String> {
    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(10);
    let offset = (page - 1) * per_page;
    
    // Fetch equipment for department
    let equipment_result = sqlx::query!(
        r#"
        SELECT 
            e.id, 
            e.name, 
            e.amount,
            COALESCE(e.amount - COALESCE(SUM(ea.amount), 0), e.amount) as available_amount,
            e.purchase_date,
            e.purchase_cost,
            e.fuel_type
        FROM 
            equipment e
        JOIN 
            equipment_allocation ea ON e.id = ea.equipment_id AND ea.department_id = $1
        LEFT JOIN 
            equipment_allocation ea2 ON e.id = ea2.equipment_id AND ea2.site_id IS NOT NULL
        GROUP BY 
            e.id, e.name, e.amount, e.purchase_date, e.purchase_cost, e.fuel_type
        ORDER BY 
            e.name
        LIMIT $2 OFFSET $3
        "#,
        id,
        per_page as i64,
        offset as i64
    )
    .fetch_all(&database.pool)
    .await;
    
    // Count total for pagination
    let total_count_result = sqlx::query_scalar!(
        r#"
        SELECT COUNT(DISTINCT e.id) 
        FROM equipment e
        JOIN equipment_allocation ea ON e.id = ea.equipment_id AND ea.department_id = $1
        "#,
        id
    )
    .fetch_one(&database.pool)
    .await;
    
    match (equipment_result, total_count_result) {
        (Ok(rows), Ok(total_count)) => {
            let equipment = rows.iter().map(|row| {
                Equipment {
                    id: row.id,
                    name: row.name.clone(),
                    amount: row.amount,
                    available_amount: Some(row.available_amount.unwrap_or(row.amount) as i32),
                    purchase_date: row.purchase_date.to_string(),
                    purchase_cost: row.purchase_cost as f64,
                    fuel_type: row.fuel_type.clone(),
                }
            }).collect();
            
            let total_pages = (total_count as usize + per_page - 1) / per_page;
            
            let template = DepartmentEquipmentTemplate {
                equipment,
                department_id: id,
                current_page: page,
                total_pages,
            };
            
            match template.render() {
                Ok(html) => Html(html),
                Err(_) => Html(String::from("<p>Error rendering department equipment</p>")),
            }
        },
        _ => Html(String::from("<p>Error fetching department equipment</p>")),
    }
}

async fn fetch_department_sites(
    State(database): State<Database>,
    Path(id): Path<i32>,
    Query(params): Query<PaginationParams>,
) -> Html<String> {
    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(10);
    let offset = (page - 1) * per_page;
    
    // Fetch sites for department
    let sites_result = sqlx::query!(
        r#"
        SELECT 
            s.id, 
            s.name, 
            s.area_id,
            s.client_id,
            s.type as site_type,
            s.risk_level,
            s.description,
            a.name as area_name,
            c.name as client_name,
            EXISTS (
                SELECT 1 FROM task t 
                WHERE t.site_id = s.id AND t.actual_period_end IS NULL
            ) as has_active_tasks
        FROM 
            site s
        JOIN 
            area a ON s.area_id = a.id
        JOIN 
            client c ON s.client_id = c.id
        WHERE 
            a.department_id = $1
        ORDER BY 
            s.name
        LIMIT $2 OFFSET $3
        "#,
        id,
        per_page as i64,
        offset as i64
    )
    .fetch_all(&database.pool)
    .await;
    
    // Count total for pagination
    let total_count_result = sqlx::query_scalar!(
        r#"
        SELECT COUNT(*) 
        FROM site s
        JOIN area a ON s.area_id = a.id
        WHERE a.department_id = $1
        "#,
        id
    )
    .fetch_one(&database.pool)
    .await;
    
    match (sites_result, total_count_result) {
        (Ok(rows), Ok(total_count)) => {
            let sites = rows.iter().map(|row| {
                Site {
                    id: row.id,
                    name: row.name.clone(),
                    area_id: row.area_id,
                    client_id: row.client_id,
                    site_type: row.site_type.clone().parse().unwrap_or(SiteType::Housing),
                    risk_level: row.risk_level.clone().parse().unwrap_or(RiskLevel::Medium),
                    description: row.description.clone(),
                    area_name: Some(row.area_name.clone()),
                    client_name: Some(row.client_name.clone()),
                    status: Some(if row.has_active_tasks.unwrap_or(false) { 
                        "Active".to_string() 
                    } else { 
                        "Completed".to_string() 
                    }),
                }
            }).collect();
            
            let total_pages = (total_count as usize + per_page - 1) / per_page;
            
            let template = DepartmentSitesTemplate {
                sites,
                department_id: id,
                current_page: page,
                total_pages,
            };
            
            match template.render() {
                Ok(html) => Html(html),
                Err(_) => Html(String::from("<p>Error rendering department sites</p>")),
            }
        },
        _ => Html(String::from("<p>Error fetching department sites</p>")),
    }
}

async fn fetch_department_personnel(
    State(database): State<Database>,
    Path(id): Path<i32>,
    Query(params): Query<PaginationParams>,
) -> Html<String> {
    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(10);
    let offset = (page - 1) * per_page;
    
    // Fetch technical personnel for department
    let personnel_result = sqlx::query!(
        r#"
        SELECT 
            tp.id, 
            e.first_name,
            e.last_name,
            tp.qualification,
            tp.position,
            tp.education_level,
            tp.is_project_manager
        FROM 
            technical_personnel tp
        JOIN 
            employee e ON tp.id = e.id
        WHERE 
            tp.id = (SELECT supervisor_id FROM department WHERE id = $1)
            OR
            tp.id IN (SELECT supervisor_id FROM area WHERE department_id = $1)
        ORDER BY 
            e.last_name, e.first_name
        LIMIT $2 OFFSET $3
        "#,
        id,
        per_page as i64,
        offset as i64
    )
    .fetch_all(&database.pool)
    .await;
    
    // Count total for pagination
    let total_count_result = sqlx::query_scalar!(
        r#"
        SELECT COUNT(*) 
        FROM technical_personnel tp
        WHERE 
            tp.id = (SELECT supervisor_id FROM department WHERE id = $1)
            OR
            tp.id IN (SELECT supervisor_id FROM area WHERE department_id = $1)
        "#,
        id
    )
    .fetch_one(&database.pool)
    .await;
    
    match (personnel_result, total_count_result) {
        (Ok(rows), Ok(total_count)) => {
            let personnel = rows.iter().map(|row| {
                TechnicalPersonnel {
                    id: row.id,
                    first_name: row.first_name.clone(),
                    last_name: row.last_name.clone(),
                    qualification: match row.qualification.as_ref().map(|s| s.as_str()) {
                        Some("technician") => Qualification::Technician,
                        Some("technologist") => Qualification::Technologist,
                        Some("engineer") => Qualification::Engineer,
                        _ => Qualification::Technician,
                    },
                    position: row.position.clone(),
                    education_level: row.education_level.clone(),
                    is_project_manager: row.is_project_manager,
                    full_name: format!("{} {}", row.first_name, row.last_name),
                }
            }).collect();
            
            let total_pages = (total_count as usize + per_page - 1) / per_page;
            
            let template = DepartmentPersonnelTemplate {
                personnel,
                department_id: id,
                current_page: page,
                total_pages,
            };
            
            match template.render() {
                Ok(html) => Html(html),
                Err(_) => Html(String::from("<p>Error rendering department personnel</p>")),
            }
        },
        _ => Html(String::from("<p>Error fetching department personnel</p>")),
    }
}

// Area API endpoints
async fn fetch_areas(
    State(database): State<Database>,
    Query(params): Query<AreaFilter>,
) -> Html<String> {
    let page = params.pagination.page.unwrap_or(1);
    let per_page = params.pagination.per_page.unwrap_or(20);
    let offset = (page - 1) * per_page;
    
    // Build query for HTMX component
    let mut query = "
        SELECT 
            a.id, 
            a.department_id,
            a.supervisor_id,
            a.name,
            d.name as department_name,
            e.first_name || ' ' || e.last_name as supervisor_name,
            COUNT(DISTINCT s.id) as sites_count,
            COUNT(DISTINCT tp.id) as personnel_count
        FROM 
            area a
        JOIN 
            department d ON a.department_id = d.id
        LEFT JOIN 
            technical_personnel tp ON a.supervisor_id = tp.id
        LEFT JOIN 
            employee e ON tp.id = e.id
        LEFT JOIN 
            site s ON s.area_id = a.id
    ".to_string();
    
    let mut conditions = Vec::new();
    
    if let Some(name) = &params.name {
        conditions.push(format!("a.name ILIKE '%{}%'", name.replace('\'', "''")));
    }
    
    if let Some(department_id) = params.department_id {
        conditions.push(format!("a.department_id = {}", department_id));
    }
    
    if let Some(supervisor_id) = params.supervisor_id {
        conditions.push(format!("a.supervisor_id = {}", supervisor_id));
    }
    
    if !conditions.is_empty() {
        query.push_str(" WHERE ");
        query.push_str(&conditions.join(" AND "));
    }
    
    query.push_str(" GROUP BY a.id, a.name, a.department_id, a.supervisor_id, d.name, e.first_name, e.last_name");
    
    let sort_by = params.pagination.sort_by.as_deref().unwrap_or("a.name");
    let sort_dir = params.pagination.sort_dir.as_deref().unwrap_or("asc");
    query.push_str(&format!(" ORDER BY {} {}", sort_by, sort_dir));
    
    query.push_str(&format!(" LIMIT {} OFFSET {}", per_page, offset));
    
    let areas_result = sqlx::query(&query)
        .fetch_all(&database.pool)
        .await;
    
    let areas = match areas_result {
        Ok(rows) => {
            rows.iter().map(|row| {
                Area {
                    id: row.get("id"),
                    department_id: row.get("department_id"),
                    supervisor_id: row.get("supervisor_id"),
                    name: row.get("name"),
                    department_name: row.get("department_name"),
                    supervisor_name: row.get("supervisor_name"),
                    sites_count: Some(row.get::<i64, _>("sites_count") as i32),
                    personnel_count: Some(row.get::<i64, _>("personnel_count") as i32),
                }
            }).collect()
        },
        Err(_) => Vec::new(),
    };
    
    let template = AreaRowsTemplate { areas };
    
    match template.render() {
        Ok(html) => Html(html),
        Err(_) => Html(String::from("<p>Error rendering area rows</p>")),
    }
}

async fn create_area(
    State(database): State<Database>,
    Form(area): Form<AreaCreate>,
) -> Html<String> {
    // Insert new area
    let result = sqlx::query!(
        "INSERT INTO area (name, department_id, supervisor_id) VALUES ($1, $2, $3) RETURNING id",
        area.name,
        area.department_id,
        area.supervisor_id
    )
    .fetch_one(&database.pool)
    .await;
    
    match result {
        Ok(_) => {
            // Return success notification
            let template = SuccessNotificationTemplate {
                message: format!("Area '{}' created successfully", area.name),
            };
            
            match template.render() {
                Ok(html) => Html(html),
                Err(_) => Html(String::from("<p>Area created but failed to render notification</p>")),
            }
        },
        Err(_) => Html(String::from("<p>Failed to create area</p>")),
    }
}

async fn fetch_area_details(
    State(database): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Fetch area details
    let area_result = sqlx::query!(
        r#"
        SELECT 
            a.id, 
            a.name, 
            a.department_id,
            a.supervisor_id,
            d.name as department_name,
            e.first_name || ' ' || e.last_name as supervisor_name
        FROM 
            area a
        JOIN 
            department d ON a.department_id = d.id
        LEFT JOIN 
            technical_personnel tp ON a.supervisor_id = tp.id
        LEFT JOIN 
            employee e ON tp.id = e.id
        WHERE 
            a.id = $1
        "#,
        id
    )
    .fetch_optional(&database.pool)
    .await;
    
    match area_result {
        Ok(Some(row)) => {
            let area = Area {
                id: row.id,
                department_id: row.department_id,
                supervisor_id: row.supervisor_id,
                name: row.name,
                department_name: Some(row.department_name),
                supervisor_name: row.supervisor_name,
                sites_count: None, // Not needed for this component
                personnel_count: None, // Not needed for this component
            };
            
            // Get department for reference
            let department = Department {
                id: row.department_id,
                name: row.department_name,
                supervisor_id: None,
                supervisor_name: None,
                areas_count: None,
                sites_count: None,
                personnel_count: None,
            };
            
            let template = AreaDetailsTemplate { area, department };
            
            match template.render() {
                Ok(html) => Html(html),
                Err(_) => Html(String::from("<p>Error rendering area details</p>")),
            }
        },
        Ok(None) => Html(String::from("<p>Area not found</p>")),
        Err(_) => Html(String::from("<p>Error fetching area details</p>")),
    }
}

async fn update_area(
    State(database): State<Database>,
    Path(id): Path<i32>,
    Form(area_update): Form<AreaUpdate>,
) -> Html<String> {
    // Update area
    let result = sqlx::query!(
        "UPDATE area SET name = $1, department_id = $2, supervisor_id = $3 WHERE id = $4",
        area_update.name,
        area_update.department_id,
        area_update.supervisor_id,
        id
    )
    .execute(&database.pool)
    .await;
    
    match result {
        Ok(_) => {
            // Fetch updated area details
            let area_result = sqlx::query!(
                r#"
                SELECT 
                    a.id, 
                    a.name, 
                    a.department_id,
                    a.supervisor_id,
                    d.name as department_name,
                    e.first_name || ' ' || e.last_name as supervisor_name
                FROM 
                    area a
                JOIN 
                    department d ON a.department_id = d.id
                LEFT JOIN 
                    technical_personnel tp ON a.supervisor_id = tp.id
                LEFT JOIN 
                    employee e ON tp.id = e.id
                WHERE 
                    a.id = $1
                "#,
                id
            )
            .fetch_optional(&database.pool)
            .await;
            
            match area_result {
                Ok(Some(row)) => {
                    let area = Area {
                        id: row.id,
                        department_id: row.department_id,
                        supervisor_id: row.supervisor_id,
                        name: row.name,
                        department_name: Some(row.department_name),
                        supervisor_name: row.supervisor_name,
                        sites_count: None,
                        personnel_count: None,
                    };
                    
                    // Get department for reference
                    let department = Department {
                        id: row.department_id,
                        name: row.department_name,
                        supervisor_id: None,
                        supervisor_name: None,
                        areas_count: None,
                        sites_count: None,
                        personnel_count: None,
                    };
                    
                    let template = AreaDetailsTemplate { area, department };
                    
                    match template.render() {
                        Ok(html) => Html(html),
                        Err(_) => Html(String::from("<p>Area updated but failed to render details</p>")),
                    }
                },
                _ => Html(String::from("<p>Area updated but failed to fetch details</p>")),
            }
        },
        Err(_) => Html(String::from("<p>Failed to update area</p>")),
    }
}

async fn delete_area(
    State(database): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // First, check if area has associated sites
    let sites_count = sqlx::query_scalar!(
        "SELECT COUNT(*) FROM site WHERE area_id = $1",
        id
    )
    .fetch_one(&database.pool)
    .await;
    
    match sites_count {
        Ok(count) if count > 0 => {
            // Cannot delete area with sites
            Html(String::from("<p class=\"text-red-500\">Cannot delete area with associated sites</p>"))
        },
        Ok(_) => {
            // Delete area
            let result = sqlx::query!("DELETE FROM area WHERE id = $1", id)
                .execute(&database.pool)
                .await;
            
            match result {
                Ok(_) => {
                    let template = SuccessNotificationTemplate {
                        message: "Area deleted successfully".to_string(),
                    };
                    
                    match template.render() {
                        Ok(html) => Html(html),
                        Err(_) => Html(String::from("<p>Area deleted but failed to render notification</p>")),
                    }
                },
                Err(_) => Html(String::from("<p>Failed to delete area</p>")),
            }
        },
        Err(_) => Html(String::from("<p>Error checking if area can be deleted</p>")),
    }
}

async fn fetch_area_sites(
    State(database): State<Database>,
    Path(id): Path<i32>,
    Query(params): Query<PaginationParams>,
) -> Html<String> {
    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(10);
    let offset = (page - 1) * per_page;
    
    // Fetch sites for area
    let sites_result = sqlx::query!(
        r#"
        SELECT 
            s.id, 
            s.name, 
            s.area_id,
            s.client_id,
            s.type as site_type,
            s.risk_level,
            s.description,
            c.name as client_name,
            EXISTS (
                SELECT 1 FROM task t 
                WHERE t.site_id = s.id AND t.actual_period_end IS NULL
            ) as has_active_tasks
        FROM 
            site s
        JOIN 
            client c ON s.client_id = c.id
        WHERE 
            s.area_id = $1
        ORDER BY 
            s.name
        LIMIT $2 OFFSET $3
        "#,
        id,
        per_page as i64,
        offset as i64
    )
    .fetch_all(&database.pool)
    .await;
    
    // Count total for pagination
    let total_count_result = sqlx::query_scalar!(
        "SELECT COUNT(*) FROM site WHERE area_id = $1",
        id
    )
    .fetch_one(&database.pool)
    .await;
    
    match (sites_result, total_count_result) {
        (Ok(rows), Ok(total_count)) => {
            let sites = rows.iter().map(|row| {
                Site {
                    id: row.id,
                    name: row.name.clone(),
                    area_id: row.area_id,
                    client_id: row.client_id,
                    site_type: row.site_type.clone().parse().unwrap_or(SiteType::Housing),
                    risk_level: row.risk_level.clone().parse().unwrap_or(RiskLevel::Medium),
                    description: row.description.clone(),
                    area_name: None, // Not needed for this view
                    client_name: Some(row.client_name.clone()),
                    status: Some(if row.has_active_tasks.unwrap_or(false) { 
                        "Active".to_string() 
                    } else { 
                        "Completed".to_string() 
                    }),
                }
            }).collect();
            
            let total_pages = (total_count as usize + per_page - 1) / per_page;
            
            let template = AreaSitesTemplate {
                sites,
                area_id: id,
                current_page: page,
                total_pages,
            };
            
            match template.render() {
                Ok(html) => Html(html),
                Err(_) => Html(String::from("<p>Error rendering area sites</p>")),
            }
        },
        _ => Html(String::from("<p>Error fetching area sites</p>")),
    }
}

async fn fetch_area_personnel(
    State(database): State<Database>,
    Path(id): Path<i32>,
    Query(params): Query<PaginationParams>,
) -> Html<String> {
    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(10);
    let offset = (page - 1) * per_page;
    
    // Fetch technical personnel for area
    let personnel_result = sqlx::query!(
        r#"
        SELECT 
            tp.id, 
            e.first_name,
            e.last_name,
            tp.qualification,
            tp.position,
            tp.education_level,
            tp.is_project_manager
        FROM 
            technical_personnel tp
        JOIN 
            employee e ON tp.id = e.id
        WHERE 
            tp.id = (SELECT supervisor_id FROM area WHERE id = $1)
        ORDER BY 
            e.last_name, e.first_name
        LIMIT $2 OFFSET $3
        "#,
        id,
        per_page as i64,
        offset as i64
    )
    .fetch_all(&database.pool)
    .await;
    
    // Count total for pagination
    let total_count_result = sqlx::query_scalar!(
        r#"
        SELECT COUNT(*) 
        FROM technical_personnel tp
        WHERE 
            tp.id = (SELECT supervisor_id FROM area WHERE id = $1)
        "#,
        id
    )
    .fetch_one(&database.pool)
    .await;
    
    match (personnel_result, total_count_result) {
        (Ok(rows), Ok(total_count)) => {
            let personnel = rows.iter().map(|row| {
                TechnicalPersonnel {
                    id: row.id,
                    first_name: row.first_name.clone(),
                    last_name: row.last_name.clone(),
                    qualification: match row.qualification.as_ref().map(|s| s.as_str()) {
                        Some("technician") => Qualification::Technician,
                        Some("technologist") => Qualification::Technologist,
                        Some("engineer") => Qualification::Engineer,
                        _ => Qualification::Technician,
                    },
                    position: row.position.clone(),
                    education_level: row.education_level.clone(),
                    is_project_manager: row.is_project_manager,
                    full_name: format!("{} {}", row.first_name, row.last_name),
                }
            }).collect();
            
            let total_pages = (total_count as usize + per_page - 1) / per_page;
            
            let template = AreaPersonnelTemplate {
                personnel,
                area_id: id,
                current_page: page,
                total_pages,
            };
            
            match template.render() {
                Ok(html) => Html(html),
                Err(_) => Html(String::from("<p>Error rendering area personnel</p>")),
            }
        },
        _ => Html(String::from("<p>Error fetching area personnel</p>")),
    }
}

async fn fetch_department_supervisors(
    State(database): State<Database>,
) -> Html<String> {
    // Fetch available supervisors for departments
    let supervisors_result = sqlx::query!(
        r#"
        SELECT 
            tp.id, 
            e.first_name || ' ' || e.last_name as name, 
            tp.qualification::text,
            tp.position::text
        FROM 
            technical_personnel tp
        JOIN 
            employee e ON tp.id = e.id
        WHERE 
            tp.is_project_manager = true
        ORDER BY 
            name
        "#
    )
    .fetch_all(&database.pool)
    .await;
    
    match supervisors_result {
        Ok(rows) => {
            let supervisors = rows.iter().map(|row| {
                Supervisor {
                    id: row.id,
                    name: row.name.clone().unwrap_or_default(),
                    qualification: row.qualification.clone().unwrap_or_default(),
                    position: row.position.clone(),
                }
            }).collect();
            
            let template = DepartmentSupervisorSelectorTemplate { supervisors };
            
            match template.render() {
                Ok(html) => Html(html),
                Err(_) => Html(String::from("<p>Error rendering supervisor selector</p>")),
            }
        },
        Err(_) => Html(String::from("<p>Error fetching supervisors</p>")),
    }
}

async fn fetch_area_supervisors(
    State(database): State<Database>,
) -> Html<String> {
    // Fetch available supervisors for areas
    let supervisors_result = sqlx::query!(
        r#"
        SELECT 
            tp.id, 
            e.first_name || ' ' || e.last_name as name, 
            tp.qualification::text,
            tp.position::text
        FROM 
            technical_personnel tp
        JOIN 
            employee e ON tp.id = e.id
        WHERE 
            tp.is_project_manager = true
        ORDER BY 
            name
        "#
    )
    .fetch_all(&database.pool)
    .await;
    
    match supervisors_result {
        Ok(rows) => {
            let supervisors = rows.iter().map(|row| {
                Supervisor {
                    id: row.id,
                    name: row.name.clone().unwrap_or_default(),
                    qualification: row.qualification.clone().unwrap_or_default(),
                    position: row.position.clone(),
                }
            }).collect();
            
            let template = AreaSupervisorSelectorTemplate { supervisors };
            
            match template.render() {
                Ok(html) => Html(html),
                Err(_) => Html(String::from("<p>Error rendering supervisor selector</p>")),
            }
        },
        Err(_) => Html(String::from("<p>Error fetching supervisors</p>")),
    }
}

// Add the missing SuccessNotificationTemplate
#[derive(Template)]
#[template(path = "general/components/success_notification.html")]
pub struct SuccessNotificationTemplate {
    pub message: String,
}

pub fn router() -> Router<Database> {
    Router::new()
        // Page Endpoints - Departments
        .route("/departments", get(departments_page))
        .route("/departments/new", get(department_new_page))
        .route("/departments/{id}", get(department_details_page))
        .route("/departments/{id}/edit", get(department_edit_page))
        // Page Endpoints - Areas
        .route("/areas", get(areas_page))
        .route("/areas/new", get(area_new_page))
        .route("/areas/{id}", get(area_details_page))
        .route("/areas/{id}/edit", get(area_edit_page))
        // HTMX Endpoints - Departments
        .route("/api/departments", get(fetch_departments).post(create_department))
        .route("/api/departments/{id}", get(fetch_department_details).put(update_department).delete(delete_department))
        .route("/api/departments/{id}/areas", get(fetch_department_areas))
        .route("/api/departments/{id}/equipment", get(fetch_department_equipment))
        .route("/api/departments/{id}/sites", get(fetch_department_sites))
        .route("/api/departments/{id}/personnel", get(fetch_department_personnel))
        .route("/api/departments/supervisors", get(fetch_department_supervisors))
        // HTMX Endpoints - Areas
        .route("/api/areas", get(fetch_areas).post(create_area))
        .route("/api/areas/{id}", get(fetch_area_details).put(update_area).delete(delete_area))
        .route("/api/areas/{id}/sites", get(fetch_area_sites))
        .route("/api/areas/{id}/personnel", get(fetch_area_personnel))
        .route("/api/areas/supervisors", get(fetch_area_supervisors))
}
