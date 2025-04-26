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
    // Department listing page
    // Return rendered DepartmentsPageTemplate
    
    let page = params.pagination.page.unwrap_or(1);
    let per_page = params.pagination.per_page.unwrap_or(10);
    
    // Mock data for departments
    let departments = vec![
        Department {
            id: 1,
            supervisor_id: Some(101),
            name: "North Construction Department".to_string(),
            supervisor_name: Some("John Smith".to_string()),
            areas_count: Some(3),
            sites_count: Some(8),
            personnel_count: Some(45),
        },
        Department {
            id: 2,
            supervisor_id: Some(102),
            name: "South Construction Department".to_string(),
            supervisor_name: Some("Sarah Johnson".to_string()),
            areas_count: Some(5),
            sites_count: Some(12),
            personnel_count: Some(78),
        },
        Department {
            id: 3,
            supervisor_id: None,
            name: "East Construction Department".to_string(),
            supervisor_name: None,
            areas_count: Some(2),
            sites_count: Some(4),
            personnel_count: Some(23),
        },
    ];
    
    let template = DepartmentsPageTemplate {
        departments,
        current_page: page,
        total_pages: 1, // Mock single page
        filter: Some(params),
    };
    
    match template.render() {
        Ok(html) => Html(html),
        Err(err) => {
            eprintln!("Template error: {}", err);
            Html("Error rendering departments page".to_string())
        }
    }
}

async fn department_new_page(
    State(database): State<Database>,
) -> Html<String> {
    // New department form
    // Return rendered DepartmentNewPageTemplate
    
    // Mock data for available supervisors
    let supervisors = vec![
        Supervisor {
            id: 101,
            name: "John Smith".to_string(),
            qualification: "Engineer".to_string(),
            position: Some("Construction Manager".to_string()),
        },
        Supervisor {
            id: 102,
            name: "Sarah Johnson".to_string(),
            qualification: "Engineer".to_string(),
            position: Some("Senior Project Manager".to_string()),
        },
        Supervisor {
            id: 103,
            name: "Michael Brown".to_string(),
            qualification: "Technologist".to_string(),
            position: Some("Process Supervisor".to_string()),
        },
        Supervisor {
            id: 104,
            name: "Emily Wilson".to_string(),
            qualification: "Engineer".to_string(),
            position: None,
        },
    ];
    
    let template = DepartmentNewPageTemplate { supervisors };
    
    match template.render() {
        Ok(html) => Html(html),
        Err(err) => {
            eprintln!("Template error: {}", err);
            Html("Error rendering new department page".to_string())
        }
    }
}

async fn department_details_page(
    State(database): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Department details page
    // Return rendered DepartmentDetailsPageTemplate
    
    // Mock data for the department
    let department = Department {
        id,
        supervisor_id: Some(101),
        name: format!("Department #{}", id),
        supervisor_name: Some("John Smith".to_string()),
        areas_count: Some(3),
        sites_count: Some(8),
        personnel_count: Some(45),
    };
    
    // Get the active tab from query params (would normally be part of the request)
    let active_tab = "areas".to_string(); // Default to areas tab
    
    let template = DepartmentDetailsPageTemplate { department, active_tab };
    
    match template.render() {
        Ok(html) => Html(html),
        Err(err) => {
            eprintln!("Template error: {}", err);
            Html(format!("Error rendering department details page for ID: {}", id))
        }
    }
}

async fn department_edit_page(
    State(database): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Edit department form
    // Return rendered DepartmentEditPageTemplate
    
    // Mock data for the department
    let department = Department {
        id,
        supervisor_id: Some(101),
        name: format!("Department #{}", id),
        supervisor_name: Some("John Smith".to_string()),
        areas_count: Some(3),
        sites_count: Some(8),
        personnel_count: Some(45),
    };
    
    // Mock data for available supervisors
    let supervisors = vec![
        Supervisor {
            id: 101,
            name: "John Smith".to_string(),
            qualification: "Engineer".to_string(),
            position: Some("Construction Manager".to_string()),
        },
        Supervisor {
            id: 102,
            name: "Sarah Johnson".to_string(),
            qualification: "Engineer".to_string(),
            position: Some("Senior Project Manager".to_string()),
        },
        Supervisor {
            id: 103,
            name: "Michael Brown".to_string(),
            qualification: "Technologist".to_string(),
            position: Some("Process Supervisor".to_string()),
        },
        Supervisor {
            id: 104,
            name: "Emily Wilson".to_string(),
            qualification: "Engineer".to_string(),
            position: None,
        },
    ];
    
    let template = DepartmentEditPageTemplate { department, supervisors };
    
    match template.render() {
        Ok(html) => Html(html),
        Err(err) => {
            eprintln!("Template error: {}", err);
            Html(format!("Error rendering department edit page for ID: {}", id))
        }
    }
}

// Area pages
async fn areas_page(
    State(database): State<Database>,
    Query(params): Query<AreaFilter>,
) -> Html<String> {
    // Area listing page
    // Return rendered AreasPageTemplate
    
    let page = params.pagination.page.unwrap_or(1);
    let per_page = params.pagination.per_page.unwrap_or(10);
    
    // Mock data for areas
    let areas = vec![
        Area {
            id: 1,
            department_id: 1,
            supervisor_id: Some(201),
            name: "Downtown Construction Area".to_string(),
            department_name: Some("North Construction Department".to_string()),
            supervisor_name: Some("Robert Lee".to_string()),
            sites_count: Some(3),
            personnel_count: Some(15),
        },
        Area {
            id: 2,
            department_id: 1,
            supervisor_id: Some(202),
            name: "Riverside Construction Area".to_string(),
            department_name: Some("North Construction Department".to_string()),
            supervisor_name: Some("Lisa Chen".to_string()),
            sites_count: Some(2),
            personnel_count: Some(12),
        },
        Area {
            id: 3,
            department_id: 2,
            supervisor_id: None,
            name: "Industrial Park Construction Area".to_string(),
            department_name: Some("South Construction Department".to_string()),
            supervisor_name: None,
            sites_count: Some(4),
            personnel_count: Some(25),
        },
    ];
    
    // Mock data for departments (for the filter dropdown)
    let departments = vec![
        Department {
            id: 1,
            supervisor_id: Some(101),
            name: "North Construction Department".to_string(),
            supervisor_name: Some("John Smith".to_string()),
            areas_count: None,
            sites_count: None,
            personnel_count: None,
        },
        Department {
            id: 2,
            supervisor_id: Some(102),
            name: "South Construction Department".to_string(),
            supervisor_name: Some("Sarah Johnson".to_string()),
            areas_count: None,
            sites_count: None,
            personnel_count: None,
        },
        Department {
            id: 3,
            supervisor_id: None,
            name: "East Construction Department".to_string(),
            supervisor_name: None,
            areas_count: None,
            sites_count: None,
            personnel_count: None,
        },
    ];
    
    let template = AreasPageTemplate {
        areas,
        departments,
        current_page: page,
        total_pages: 1, // Mock single page
        filter: Some(params),
    };
    
    match template.render() {
        Ok(html) => Html(html),
        Err(err) => {
            eprintln!("Template error: {}", err);
            Html("Error rendering areas page".to_string())
        }
    }
}

async fn area_new_page(
    State(database): State<Database>,
) -> Html<String> {
    // New area form
    // Return rendered AreaNewPageTemplate
    
    // Mock data for departments
    let departments = vec![
        Department {
            id: 1,
            supervisor_id: Some(101),
            name: "North Construction Department".to_string(),
            supervisor_name: Some("John Smith".to_string()),
            areas_count: None,
            sites_count: None,
            personnel_count: None,
        },
        Department {
            id: 2,
            supervisor_id: Some(102),
            name: "South Construction Department".to_string(),
            supervisor_name: Some("Sarah Johnson".to_string()),
            areas_count: None,
            sites_count: None,
            personnel_count: None,
        },
        Department {
            id: 3,
            supervisor_id: None,
            name: "East Construction Department".to_string(),
            supervisor_name: None,
            areas_count: None,
            sites_count: None,
            personnel_count: None,
        },
    ];
    
    // Mock data for available supervisors
    let supervisors = vec![
        Supervisor {
            id: 201,
            name: "Robert Lee".to_string(),
            qualification: "Engineer".to_string(),
            position: Some("Area Supervisor".to_string()),
        },
        Supervisor {
            id: 202,
            name: "Lisa Chen".to_string(),
            qualification: "Engineer".to_string(),
            position: Some("Area Manager".to_string()),
        },
        Supervisor {
            id: 203,
            name: "David Miller".to_string(),
            qualification: "Technologist".to_string(),
            position: Some("Process Manager".to_string()),
        },
    ];
    
    let template = AreaNewPageTemplate { departments, supervisors };
    
    match template.render() {
        Ok(html) => Html(html),
        Err(err) => {
            eprintln!("Template error: {}", err);
            Html("Error rendering new area page".to_string())
        }
    }
}

async fn area_details_page(
    State(database): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Area details page
    // Return rendered AreaDetailsPageTemplate
    
    // Mock data for the area
    let area = Area {
        id,
        department_id: 1,
        supervisor_id: Some(201),
        name: format!("Area #{}", id),
        department_name: Some("North Construction Department".to_string()),
        supervisor_name: Some("Robert Lee".to_string()),
        sites_count: Some(3),
        personnel_count: Some(15),
    };
    
    // Mock data for the department
    let department = Department {
        id: 1,
        supervisor_id: Some(101),
        name: "North Construction Department".to_string(),
        supervisor_name: Some("John Smith".to_string()),
        areas_count: Some(3),
        sites_count: Some(8),
        personnel_count: Some(45),
    };
    
    // Get the active tab from query params (would normally be part of the request)
    let active_tab = "sites".to_string(); // Default to sites tab
    
    let template = AreaDetailsPageTemplate { area, department, active_tab };
    
    match template.render() {
        Ok(html) => Html(html),
        Err(err) => {
            eprintln!("Template error: {}", err);
            Html(format!("Error rendering area details page for ID: {}", id))
        }
    }
}

async fn area_edit_page(
    State(database): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Edit area form
    // Return rendered AreaEditPageTemplate
    
    // Mock data for the area
    let area = Area {
        id,
        department_id: 1,
        supervisor_id: Some(201),
        name: format!("Area #{}", id),
        department_name: Some("North Construction Department".to_string()),
        supervisor_name: Some("Robert Lee".to_string()),
        sites_count: Some(3),
        personnel_count: Some(15),
    };
    
    // Mock data for departments
    let departments = vec![
        Department {
            id: 1,
            supervisor_id: Some(101),
            name: "North Construction Department".to_string(),
            supervisor_name: Some("John Smith".to_string()),
            areas_count: None,
            sites_count: None,
            personnel_count: None,
        },
        Department {
            id: 2,
            supervisor_id: Some(102),
            name: "South Construction Department".to_string(),
            supervisor_name: Some("Sarah Johnson".to_string()),
            areas_count: None,
            sites_count: None,
            personnel_count: None,
        },
        Department {
            id: 3,
            supervisor_id: None,
            name: "East Construction Department".to_string(),
            supervisor_name: None,
            areas_count: None,
            sites_count: None,
            personnel_count: None,
        },
    ];
    
    // Mock data for available supervisors
    let supervisors = vec![
        Supervisor {
            id: 201,
            name: "Robert Lee".to_string(),
            qualification: "Engineer".to_string(),
            position: Some("Area Supervisor".to_string()),
        },
        Supervisor {
            id: 202,
            name: "Lisa Chen".to_string(),
            qualification: "Engineer".to_string(),
            position: Some("Area Manager".to_string()),
        },
        Supervisor {
            id: 203,
            name: "David Miller".to_string(),
            qualification: "Technologist".to_string(),
            position: Some("Process Manager".to_string()),
        },
    ];
    
    let template = AreaEditPageTemplate { area, departments, supervisors };
    
    match template.render() {
        Ok(html) => Html(html),
        Err(err) => {
            eprintln!("Template error: {}", err);
            Html(format!("Error rendering area edit page for ID: {}", id))
        }
    }
}

// HTMX Endpoints

// Department API endpoints
async fn fetch_departments(
    State(database): State<Database>,
    Query(params): Query<DepartmentFilter>,
) -> Html<String> {
    // Fetch departments with filters
    // Return rendered DepartmentRowsTemplate
    Html::from(String::new())
}

async fn create_department(
    State(database): State<Database>,
    Form(department): Form<DepartmentCreate>,
) -> Html<String> {
    // Create new department
    // Return rendered SuccessNotificationTemplate
    Html::from(String::new())
}

async fn fetch_department_details(
    State(database): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Fetch department details
    // Return rendered DepartmentDetailsTemplate
    Html::from(String::new())
}

async fn update_department(
    State(database): State<Database>,
    Path(id): Path<i32>,
    Form(department): Form<DepartmentUpdate>,
) -> Html<String> {
    // Update department
    // Return rendered DepartmentDetailsTemplate
    Html::from(String::new())
}

async fn delete_department(
    State(database): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Delete department
    // Return rendered SuccessNotificationTemplate
    Html::from(String::new())
}

async fn fetch_department_areas(
    State(database): State<Database>,
    Path(id): Path<i32>,
    Query(params): Query<PaginationParams>,
) -> Html<String> {
    // Fetch areas for department
    // Return rendered DepartmentAreasTemplate
    Html::from(String::new())
}

async fn fetch_department_equipment(
    State(database): State<Database>,
    Path(id): Path<i32>,
    Query(params): Query<PaginationParams>,
) -> Html<String> {
    // Fetch equipment for department
    // Return rendered DepartmentEquipmentTemplate
    Html::from(String::new())
}

async fn fetch_department_sites(
    State(database): State<Database>,
    Path(id): Path<i32>,
    Query(params): Query<PaginationParams>,
) -> Html<String> {
    // Fetch sites for department
    // Return rendered DepartmentSitesTemplate
    Html::from(String::new())
}

async fn fetch_department_personnel(
    State(database): State<Database>,
    Path(id): Path<i32>,
    Query(params): Query<PaginationParams>,
) -> Html<String> {
    // Fetch personnel for department
    // Return rendered DepartmentPersonnelTemplate
    Html::from(String::new())
}

// Area API endpoints
async fn fetch_areas(
    State(database): State<Database>,
    Query(params): Query<AreaFilter>,
) -> Html<String> {
    // Fetch areas with filters
    // Return rendered AreaRowsTemplate
    Html::from(String::new())
}

async fn create_area(
    State(database): State<Database>,
    Form(area): Form<AreaCreate>,
) -> Html<String> {
    // Create new area
    // Return rendered SuccessNotificationTemplate
    Html::from(String::new())
}

async fn fetch_area_details(
    State(database): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Fetch area details
    // Return rendered AreaDetailsTemplate
    Html::from(String::new())
}

async fn update_area(
    State(database): State<Database>,
    Path(id): Path<i32>,
    Form(area_update): Form<AreaUpdate>,
) -> Html<String> {
    // Update area
    // Return rendered AreaDetailsTemplate
    Html::from(String::new())
}

async fn delete_area(
    State(database): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Delete area
    // Return rendered SuccessNotificationTemplate
    Html::from(String::new())
}

async fn fetch_area_sites(
    State(database): State<Database>,
    Path(id): Path<i32>,
    Query(params): Query<PaginationParams>,
) -> Html<String> {
    // Fetch sites for area
    // Return rendered AreaSitesTemplate
    Html::from(String::new())
}

async fn fetch_area_personnel(
    State(database): State<Database>,
    Path(id): Path<i32>,
    Query(params): Query<PaginationParams>,
) -> Html<String> {
    // Fetch personnel for area
    // Return rendered AreaPersonnelTemplate
    Html::from(String::new())
}

// Supervisor selectors
async fn fetch_department_supervisors(
    State(database): State<Database>,
) -> Html<String> {
    // Fetch available supervisors for departments
    // Return rendered DepartmentSupervisorSelectorTemplate
    Html::from(String::new())
}

async fn fetch_area_supervisors(
    State(database): State<Database>,
) -> Html<String> {
    // Fetch available supervisors for areas
    // Return rendered AreaSupervisorSelectorTemplate
    Html::from(String::new())
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
