use axum::{
    extract::{Path, Query, State},
    response::Html,
    routing::{get, post, put, delete},
    Form, Json, Router,
};
use serde::{Deserialize, Serialize};
use askama::Template;

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
    Html(String::new())
}

async fn department_new_page(
    State(database): State<Database>,
) -> Html<String> {
    // New department form
    // Return rendered DepartmentNewPageTemplate
    Html(String::new())
}

async fn department_details_page(
    State(database): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Department details page
    // Return rendered DepartmentDetailsPageTemplate
    Html(String::new())
}

async fn department_edit_page(
    State(database): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Edit department form
    // Return rendered DepartmentEditPageTemplate
    Html(String::new())
}

// Area pages
async fn areas_page(
    State(database): State<Database>,
    Query(params): Query<AreaFilter>,
) -> Html<String> {
    // Area listing page
    // Return rendered AreasPageTemplate
    Html(String::new())
}

async fn area_new_page(
    State(database): State<Database>,
) -> Html<String> {
    // New area form
    // Return rendered AreaNewPageTemplate
    Html(String::new())
}

async fn area_details_page(
    State(database): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Area details page
    // Return rendered AreaDetailsPageTemplate
    Html(String::new())
}

async fn area_edit_page(
    State(database): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Edit area form
    // Return rendered AreaEditPageTemplate
    Html(String::new())
}

// HTMX Endpoints

// Department API endpoints
async fn fetch_departments(
    State(database): State<Database>,
    Query(params): Query<DepartmentFilter>,
) -> Html<String> {
    // Fetch departments with filters
    // Return rendered DepartmentRowsTemplate
    Html(String::new())
}

async fn create_department(
    State(database): State<Database>,
    Form(department): Form<DepartmentCreate>,
) -> Html<String> {
    // Create new department
    // Return rendered SuccessNotificationTemplate
    Html(String::new())
}

async fn fetch_department_details(
    State(database): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Fetch department details
    // Return rendered DepartmentDetailsTemplate
    Html(String::new())
}

async fn update_department(
    State(database): State<Database>,
    Path(id): Path<i32>,
    Form(department): Form<DepartmentUpdate>,
) -> Html<String> {
    // Update department
    // Return rendered DepartmentDetailsTemplate
    Html(String::new())
}

async fn delete_department(
    State(database): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Delete department
    // Return rendered SuccessNotificationTemplate
    Html(String::new())
}

async fn fetch_department_areas(
    State(database): State<Database>,
    Path(id): Path<i32>,
    Query(params): Query<PaginationParams>,
) -> Html<String> {
    // Fetch areas for department
    // Return rendered DepartmentAreasTemplate
    Html(String::new())
}

async fn fetch_department_equipment(
    State(database): State<Database>,
    Path(id): Path<i32>,
    Query(params): Query<PaginationParams>,
) -> Html<String> {
    // Fetch equipment for department
    // Return rendered DepartmentEquipmentTemplate
    Html(String::new())
}

async fn fetch_department_sites(
    State(database): State<Database>,
    Path(id): Path<i32>,
    Query(params): Query<PaginationParams>,
) -> Html<String> {
    // Fetch sites for department
    // Return rendered DepartmentSitesTemplate
    Html(String::new())
}

async fn fetch_department_personnel(
    State(database): State<Database>,
    Path(id): Path<i32>,
    Query(params): Query<PaginationParams>,
) -> Html<String> {
    // Fetch personnel for department
    // Return rendered DepartmentPersonnelTemplate
    Html(String::new())
}

// Area API endpoints
async fn fetch_areas(
    State(database): State<Database>,
    Query(params): Query<AreaFilter>,
) -> Html<String> {
    // Fetch areas with filters
    // Return rendered AreaRowsTemplate
    Html(String::new())
}

async fn create_area(
    State(database): State<Database>,
    Form(area): Form<AreaCreate>,
) -> Html<String> {
    // Create new area
    // Return rendered SuccessNotificationTemplate
    Html(String::new())
}

async fn fetch_area_details(
    State(database): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Fetch area details
    // Return rendered AreaDetailsTemplate
    Html(String::new())
}

async fn update_area(
    State(database): State<Database>,
    Path(id): Path<i32>,
    Form(area): Form<AreaUpdate>,
) -> Html<String> {
    // Update area
    // Return rendered AreaDetailsTemplate
    Html(String::new())
}

async fn delete_area(
    State(database): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Delete area
    // Return rendered SuccessNotificationTemplate
    Html(String::new())
}

async fn fetch_area_sites(
    State(database): State<Database>,
    Path(id): Path<i32>,
    Query(params): Query<PaginationParams>,
) -> Html<String> {
    // Fetch sites for area
    // Return rendered AreaSitesTemplate
    Html(String::new())
}

async fn fetch_area_personnel(
    State(database): State<Database>,
    Path(id): Path<i32>,
    Query(params): Query<PaginationParams>,
) -> Html<String> {
    // Fetch personnel for area
    // Return rendered AreaPersonnelTemplate
    Html(String::new())
}

// Supervisor selectors
async fn fetch_department_supervisors(
    State(database): State<Database>,
) -> Html<String> {
    // Fetch available supervisors for departments
    // Return rendered DepartmentSupervisorSelectorTemplate
    Html(String::new())
}

async fn fetch_area_supervisors(
    State(database): State<Database>,
) -> Html<String> {
    // Fetch available supervisors for areas
    // Return rendered AreaSupervisorSelectorTemplate
    Html(String::new())
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
