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
    
    // Mock data for departments (filtered if params are provided)
    let mut departments = vec![
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
    
    // Filter departments based on params
    if let Some(name) = &params.name {
        departments.retain(|dept| dept.name.to_lowercase().contains(&name.to_lowercase()));
    }
    
    if let Some(supervisor_id) = params.supervisor_id {
        departments.retain(|dept| dept.supervisor_id == Some(supervisor_id));
    }
    
    let template = DepartmentRowsTemplate { departments };
    
    match template.render() {
        Ok(html) => Html(html),
        Err(err) => {
            eprintln!("Template error: {}", err);
            Html("Error fetching departments".to_string())
        }
    }
}

async fn create_department(
    State(database): State<Database>,
    Form(department): Form<DepartmentCreate>,
) -> Html<String> {
    // Create new department
    // Return rendered SuccessNotificationTemplate
    
    // In a real implementation, this would create a new department in the database
    // For now, just return a success message
    let template = SuccessNotificationTemplate {
        message: format!("Department '{}' was created successfully.", department.name),
    };
    
    match template.render() {
        Ok(html) => Html(html),
        Err(err) => {
            eprintln!("Template error: {}", err);
            Html("Error creating department".to_string())
        }
    }
}

async fn fetch_department_details(
    State(database): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Fetch department details
    // Return rendered DepartmentDetailsTemplate
    
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
    
    let template = DepartmentDetailsTemplate { department };
    
    match template.render() {
        Ok(html) => Html(html),
        Err(err) => {
            eprintln!("Template error: {}", err);
            Html(format!("Error fetching department details for ID: {}", id))
        }
    }
}

async fn update_department(
    State(database): State<Database>,
    Path(id): Path<i32>,
    Form(department): Form<DepartmentUpdate>,
) -> Html<String> {
    // Update department
    // Return rendered DepartmentDetailsTemplate
    
    // In a real implementation, this would update the department in the database
    // Mock updated department data
    let updated_department = Department {
        id,
        supervisor_id: department.supervisor_id,
        name: department.name,
        supervisor_name: department.supervisor_id.map(|_| "New Supervisor Name".to_string()),
        areas_count: Some(3),
        sites_count: Some(8),
        personnel_count: Some(45),
    };
    
    let template = DepartmentDetailsTemplate { department: updated_department };
    
    match template.render() {
        Ok(html) => Html(html),
        Err(err) => {
            eprintln!("Template error: {}", err);
            Html(format!("Error updating department ID: {}", id))
        }
    }
}

async fn delete_department(
    State(database): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Delete department
    // Return rendered SuccessNotificationTemplate
    
    // In a real implementation, this would delete the department from the database
    let template = SuccessNotificationTemplate {
        message: format!("Department #{} was deleted successfully.", id),
    };
    
    match template.render() {
        Ok(html) => Html(html),
        Err(err) => {
            eprintln!("Template error: {}", err);
            Html(format!("Error deleting department ID: {}", id))
        }
    }
}

async fn fetch_department_areas(
    State(database): State<Database>,
    Path(id): Path<i32>,
    Query(params): Query<PaginationParams>,
) -> Html<String> {
    // Fetch areas for department
    // Return rendered DepartmentAreasTemplate
    
    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(10);
    
    // Mock data for areas in this department
    let areas = vec![
        Area {
            id: 1,
            department_id: id,
            supervisor_id: Some(201),
            name: "Downtown Construction Area".to_string(),
            department_name: Some(format!("Department #{}", id)),
            supervisor_name: Some("Robert Lee".to_string()),
            sites_count: Some(3),
            personnel_count: Some(15),
        },
        Area {
            id: 2,
            department_id: id,
            supervisor_id: Some(202),
            name: "Riverside Construction Area".to_string(),
            department_name: Some(format!("Department #{}", id)),
            supervisor_name: Some("Lisa Chen".to_string()),
            sites_count: Some(2),
            personnel_count: Some(12),
        },
        Area {
            id: 3,
            department_id: id,
            supervisor_id: None,
            name: "Industrial Park Construction Area".to_string(),
            department_name: Some(format!("Department #{}", id)),
            supervisor_name: None,
            sites_count: Some(4),
            personnel_count: Some(25),
        },
    ];
    
    let template = DepartmentAreasTemplate {
        areas,
        department_id: id,
        current_page: page,
        total_pages: 1, // Mock single page
    };
    
    match template.render() {
        Ok(html) => Html(html),
        Err(err) => {
            eprintln!("Template error: {}", err);
            Html(format!("Error fetching areas for department ID: {}", id))
        }
    }
}

async fn fetch_department_equipment(
    State(database): State<Database>,
    Path(id): Path<i32>,
    Query(params): Query<PaginationParams>,
) -> Html<String> {
    // Fetch equipment for department
    // Return rendered DepartmentEquipmentTemplate
    
    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(10);
    
    // Mock data for equipment in this department
    let equipment = vec![
        Equipment {
            id: 1,
            name: "Tower Crane XL-5000".to_string(),
            amount: 2,
            available_amount: Some(1),
            purchase_date: "2021-03-15".to_string(),
            purchase_cost: 250000.0,
            fuel_type: Some("Diesel".to_string()),
        },
        Equipment {
            id: 2,
            name: "Excavator CAT-320".to_string(),
            amount: 3,
            available_amount: Some(0),
            purchase_date: "2020-06-22".to_string(),
            purchase_cost: 180000.0,
            fuel_type: Some("Diesel".to_string()),
        },
        Equipment {
            id: 3,
            name: "Concrete Mixer B-2000".to_string(),
            amount: 5,
            available_amount: Some(2),
            purchase_date: "2022-01-10".to_string(),
            purchase_cost: 45000.0,
            fuel_type: None,
        },
    ];
    
    let template = DepartmentEquipmentTemplate {
        equipment,
        department_id: id,
        current_page: page,
        total_pages: 1, // Mock single page
    };
    
    match template.render() {
        Ok(html) => Html(html),
        Err(err) => {
            eprintln!("Template error: {}", err);
            Html(format!("Error fetching equipment for department ID: {}", id))
        }
    }
}

async fn fetch_department_sites(
    State(database): State<Database>,
    Path(id): Path<i32>,
    Query(params): Query<PaginationParams>,
) -> Html<String> {
    // Fetch sites for department
    // Return rendered DepartmentSitesTemplate
    
    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(10);
    
    // Mock data for sites in this department
    let sites = vec![
        Site {
            id: 1,
            name: "Downtown Tower".to_string(),
            area_id: 1,
            client_id: 101,
            site_type: SiteType::Housing,
            risk_level: RiskLevel::Medium,
            description: Some("25-floor commercial building in the city center".to_string()),
            area_name: Some("Downtown Construction Area".to_string()),
            client_name: Some("Metro Development Corp".to_string()),
            status: Some("in_progress".to_string()),
        },
        Site {
            id: 2,
            name: "Riverside Bridge".to_string(),
            area_id: 2,
            client_id: 102,
            site_type: SiteType::Bridge,
            risk_level: RiskLevel::High,
            description: Some("600m bridge across the river".to_string()),
            area_name: Some("Riverside Construction Area".to_string()),
            client_name: Some("City Administration".to_string()),
            status: Some("planned".to_string()),
        },
        Site {
            id: 3,
            name: "Green Valley Park".to_string(),
            area_id: 3,
            client_id: 103,
            site_type: SiteType::Park,
            risk_level: RiskLevel::Low,
            description: Some("Community park with recreational facilities".to_string()),
            area_name: Some("Industrial Park Construction Area".to_string()),
            client_name: Some("Parks & Recreation Department".to_string()),
            status: Some("completed".to_string()),
        },
    ];
    
    let template = DepartmentSitesTemplate {
        sites,
        department_id: id,
        current_page: page,
        total_pages: 1, // Mock single page
    };
    
    match template.render() {
        Ok(html) => Html(html),
        Err(err) => {
            eprintln!("Template error: {}", err);
            Html(format!("Error fetching sites for department ID: {}", id))
        }
    }
}

async fn fetch_department_personnel(
    State(database): State<Database>,
    Path(id): Path<i32>,
    Query(params): Query<PaginationParams>,
) -> Html<String> {
    // Fetch personnel for department
    // Return rendered DepartmentPersonnelTemplate
    
    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(10);
    
    // Mock data for personnel in this department
    let personnel = vec![
        TechnicalPersonnel {
            id: 101,
            first_name: "John".to_string(),
            last_name: "Smith".to_string(),
            qualification: Qualification::Engineer,
            position: Some("Construction Manager".to_string()),
            education_level: "Master's Degree".to_string(),
            is_project_manager: true,
            full_name: "John Smith".to_string(),
        },
        TechnicalPersonnel {
            id: 102,
            first_name: "Sarah".to_string(),
            last_name: "Johnson".to_string(),
            qualification: Qualification::Engineer,
            position: Some("Senior Project Manager".to_string()),
            education_level: "Master's Degree".to_string(),
            is_project_manager: true,
            full_name: "Sarah Johnson".to_string(),
        },
        TechnicalPersonnel {
            id: 103,
            first_name: "Michael".to_string(),
            last_name: "Brown".to_string(),
            qualification: Qualification::Technologist,
            position: Some("Process Supervisor".to_string()),
            education_level: "Bachelor's Degree".to_string(),
            is_project_manager: false,
            full_name: "Michael Brown".to_string(),
        },
        TechnicalPersonnel {
            id: 104,
            first_name: "Emily".to_string(),
            last_name: "Wilson".to_string(),
            qualification: Qualification::Technician,
            position: None,
            education_level: "Associate's Degree".to_string(),
            is_project_manager: false,
            full_name: "Emily Wilson".to_string(),
        },
    ];
    
    let template = DepartmentPersonnelTemplate {
        personnel,
        department_id: id,
        current_page: page,
        total_pages: 1, // Mock single page
    };
    
    match template.render() {
        Ok(html) => Html(html),
        Err(err) => {
            eprintln!("Template error: {}", err);
            Html(format!("Error fetching personnel for department ID: {}", id))
        }
    }
}

// Area API endpoints
async fn fetch_areas(
    State(database): State<Database>,
    Query(params): Query<AreaFilter>,
) -> Html<String> {
    // Fetch areas with filters
    // Return rendered AreaRowsTemplate
    
    // Mock data for areas (filtered if params are provided)
    let mut areas = vec![
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
    
    // Filter areas based on params
    if let Some(name) = &params.name {
        areas.retain(|area| area.name.to_lowercase().contains(&name.to_lowercase()));
    }
    
    if let Some(department_id) = params.department_id {
        areas.retain(|area| area.department_id == department_id);
    }
    
    if let Some(supervisor_id) = params.supervisor_id {
        areas.retain(|area| area.supervisor_id == Some(supervisor_id));
    }
    
    let template = AreaRowsTemplate { areas };
    
    match template.render() {
        Ok(html) => Html(html),
        Err(err) => {
            eprintln!("Template error: {}", err);
            Html("Error fetching areas".to_string())
        }
    }
}

async fn create_area(
    State(database): State<Database>,
    Form(area): Form<AreaCreate>,
) -> Html<String> {
    // Create new area
    // Return rendered SuccessNotificationTemplate
    
    // In a real implementation, this would create a new area in the database
    // For now, just return a success message
    let template = SuccessNotificationTemplate {
        message: format!("Area '{}' was created successfully.", area.name),
    };
    
    match template.render() {
        Ok(html) => Html(html),
        Err(err) => {
            eprintln!("Template error: {}", err);
            Html("Error creating area".to_string())
        }
    }
}

async fn fetch_area_details(
    State(database): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Fetch area details
    // Return rendered AreaDetailsTemplate
    
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
    
    let template = AreaDetailsTemplate { area, department };
    
    match template.render() {
        Ok(html) => Html(html),
        Err(err) => {
            eprintln!("Template error: {}", err);
            Html(format!("Error fetching area details for ID: {}", id))
        }
    }
}

async fn update_area(
    State(database): State<Database>,
    Path(id): Path<i32>,
    Form(area_update): Form<AreaUpdate>,
) -> Html<String> {
    // Update area
    // Return rendered AreaDetailsTemplate
    
    // In a real implementation, this would update the area in the database
    // Mock updated area data
    let area = Area {
        id,
        department_id: area_update.department_id,
        supervisor_id: area_update.supervisor_id,
        name: area_update.name,
        department_name: Some("Department Name".to_string()), // This would come from the DB in a real impl
        supervisor_name: area_update.supervisor_id.map(|_| "Supervisor Name".to_string()),
        sites_count: Some(3),
        personnel_count: Some(15),
    };
    
    // Mock data for the department
    let department = Department {
        id: area_update.department_id,
        supervisor_id: Some(101),
        name: "Department Name".to_string(),
        supervisor_name: Some("Department Supervisor".to_string()),
        areas_count: Some(3),
        sites_count: Some(8),
        personnel_count: Some(45),
    };
    
    let template = AreaDetailsTemplate { area, department };
    
    match template.render() {
        Ok(html) => Html(html),
        Err(err) => {
            eprintln!("Template error: {}", err);
            Html(format!("Error updating area ID: {}", id))
        }
    }
}

async fn delete_area(
    State(database): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Delete area
    // Return rendered SuccessNotificationTemplate
    
    // In a real implementation, this would delete the area from the database
    let template = SuccessNotificationTemplate {
        message: format!("Area #{} was deleted successfully.", id),
    };
    
    match template.render() {
        Ok(html) => Html(html),
        Err(err) => {
            eprintln!("Template error: {}", err);
            Html(format!("Error deleting area ID: {}", id))
        }
    }
}

async fn fetch_area_sites(
    State(database): State<Database>,
    Path(id): Path<i32>,
    Query(params): Query<PaginationParams>,
) -> Html<String> {
    // Fetch sites for area
    // Return rendered AreaSitesTemplate
    
    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(10);
    
    // Mock data for sites in this area
    let sites = vec![
        Site {
            id: 1,
            name: "Downtown Tower".to_string(),
            area_id: id,
            client_id: 101,
            site_type: SiteType::Housing,
            risk_level: RiskLevel::Medium,
            description: Some("25-floor commercial building in the city center".to_string()),
            area_name: Some(format!("Area #{}", id)),
            client_name: Some("Metro Development Corp".to_string()),
            status: Some("in_progress".to_string()),
        },
        Site {
            id: 2,
            name: "Riverside Bridge".to_string(),
            area_id: id,
            client_id: 102,
            site_type: SiteType::Bridge,
            risk_level: RiskLevel::High,
            description: Some("600m bridge across the river".to_string()),
            area_name: Some(format!("Area #{}", id)),
            client_name: Some("City Administration".to_string()),
            status: Some("planned".to_string()),
        },
        Site {
            id: 3,
            name: "Green Valley Park".to_string(),
            area_id: id,
            client_id: 103,
            site_type: SiteType::Park,
            risk_level: RiskLevel::Low,
            description: Some("Community park with recreational facilities".to_string()),
            area_name: Some(format!("Area #{}", id)),
            client_name: Some("Parks & Recreation Department".to_string()),
            status: Some("completed".to_string()),
        },
    ];
    
    let template = AreaSitesTemplate {
        sites,
        area_id: id,
        current_page: page,
        total_pages: 1, // Mock single page
    };
    
    match template.render() {
        Ok(html) => Html(html),
        Err(err) => {
            eprintln!("Template error: {}", err);
            Html(format!("Error fetching sites for area ID: {}", id))
        }
    }
}

async fn fetch_area_personnel(
    State(database): State<Database>,
    Path(id): Path<i32>,
    Query(params): Query<PaginationParams>,
) -> Html<String> {
    // Fetch personnel for area
    // Return rendered AreaPersonnelTemplate
    
    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(10);
    
    // Mock data for personnel in this area
    let personnel = vec![
        TechnicalPersonnel {
            id: 201,
            first_name: "Robert".to_string(),
            last_name: "Lee".to_string(),
            qualification: Qualification::Engineer,
            position: Some("Area Supervisor".to_string()),
            education_level: "Master's Degree".to_string(),
            is_project_manager: true,
            full_name: "Robert Lee".to_string(),
        },
        TechnicalPersonnel {
            id: 202,
            first_name: "Lisa".to_string(),
            last_name: "Chen".to_string(),
            qualification: Qualification::Engineer,
            position: Some("Project Manager".to_string()),
            education_level: "Master's Degree".to_string(),
            is_project_manager: true,
            full_name: "Lisa Chen".to_string(),
        },
        TechnicalPersonnel {
            id: 203,
            first_name: "David".to_string(),
            last_name: "Miller".to_string(),
            qualification: Qualification::Technologist,
            position: Some("Foreman".to_string()),
            education_level: "Bachelor's Degree".to_string(),
            is_project_manager: false,
            full_name: "David Miller".to_string(),
        },
        TechnicalPersonnel {
            id: 204,
            first_name: "Jennifer".to_string(),
            last_name: "Taylor".to_string(),
            qualification: Qualification::Technician,
            position: Some("Assistant Supervisor".to_string()),
            education_level: "Bachelor's Degree".to_string(),
            is_project_manager: false,
            full_name: "Jennifer Taylor".to_string(),
        },
    ];
    
    let template = AreaPersonnelTemplate {
        personnel,
        area_id: id,
        current_page: page,
        total_pages: 1, // Mock single page
    };
    
    match template.render() {
        Ok(html) => Html(html),
        Err(err) => {
            eprintln!("Template error: {}", err);
            Html(format!("Error fetching personnel for area ID: {}", id))
        }
    }
}

// Supervisor selectors
async fn fetch_department_supervisors(
    State(database): State<Database>,
) -> Html<String> {
    // Fetch available supervisors for departments
    // Return rendered DepartmentSupervisorSelectorTemplate
    
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
    
    let template = DepartmentSupervisorSelectorTemplate { supervisors };
    
    match template.render() {
        Ok(html) => Html(html),
        Err(err) => {
            eprintln!("Template error: {}", err);
            Html("Error fetching department supervisors".to_string())
        }
    }
}

async fn fetch_area_supervisors(
    State(database): State<Database>,
) -> Html<String> {
    // Fetch available supervisors for areas
    // Return rendered AreaSupervisorSelectorTemplate
    
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
    
    let template = AreaSupervisorSelectorTemplate { supervisors };
    
    match template.render() {
        Ok(html) => Html(html),
        Err(err) => {
            eprintln!("Template error: {}", err);
            Html("Error fetching area supervisors".to_string())
        }
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
