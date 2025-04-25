use axum::{
    extract::{Path, Query, State},
    response::Html,
    routing::{get, post, put, delete},
    Form, Json, Router,
};
use serde::{Deserialize, Serialize};
use askama::Template;

use crate::database::Database;
use crate::general::{PaginationParams, SearchParams};

// Types for Department Management

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Department {
    pub id: i32,
    pub supervisor_id: Option<i32>,
    pub name: String,
    pub supervisor_name: Option<String>, // Joined from technical_personnel
    pub areas_count: Option<i32>,        // Count of related areas
    pub sites_count: Option<i32>,        // Count of related sites
    pub personnel_count: Option<i32>,    // Count of personnel
}

#[derive(Deserialize, Debug)]
pub struct DepartmentCreate {
    pub name: String,
    pub supervisor_id: Option<i32>,
}

#[derive(Deserialize, Debug)]
pub struct DepartmentUpdate {
    pub name: String,
    pub supervisor_id: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Deserialize, Debug)]
pub struct AreaCreate {
    pub name: String,
    pub department_id: i32,
    pub supervisor_id: Option<i32>,
}

#[derive(Deserialize, Debug)]
pub struct AreaUpdate {
    pub name: String,
    pub department_id: i32,
    pub supervisor_id: Option<i32>,
}

#[derive(Serialize, Debug, Clone)]
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

// Define Templates for Departments
#[derive(Template)]
#[template(path = "departments/index.html")]
struct DepartmentsTemplate {
    departments: Vec<Department>,
    filters: DepartmentFilter,
    total_pages: usize,
    current_page: usize,
}

#[derive(Template)]
#[template(path = "departments/new.html")]
struct DepartmentNewTemplate {
    supervisors: Vec<Supervisor>,
}

#[derive(Template)]
#[template(path = "departments/details.html")]
struct DepartmentDetailsTemplate {
    department: Department,
    supervisor: Option<Supervisor>,
}

#[derive(Template)]
#[template(path = "departments/edit.html")]
struct DepartmentEditTemplate {
    department: Department,
    supervisors: Vec<Supervisor>,
}

// Define Templates for Areas
#[derive(Template)]
#[template(path = "areas/index.html")]
struct AreasTemplate {
    areas: Vec<Area>,
    filters: AreaFilter,
    departments: Vec<Department>,
    total_pages: usize,
    current_page: usize,
}

#[derive(Template)]
#[template(path = "areas/new.html")]
struct AreaNewTemplate {
    departments: Vec<Department>,
    supervisors: Vec<Supervisor>,
}

#[derive(Template)]
#[template(path = "areas/details.html")]
struct AreaDetailsTemplate {
    area: Area,
    department: Department,
    supervisor: Option<Supervisor>,
}

#[derive(Template)]
#[template(path = "areas/edit.html")]
struct AreaEditTemplate {
    area: Area,
    departments: Vec<Department>,
    supervisors: Vec<Supervisor>,
}

// Define Templates for HTMX Components
#[derive(Template)]
#[template(path = "components/departments/table_rows.html")]
struct DepartmentRowsTemplate {
    departments: Vec<Department>,
    current_page: usize,
    total_pages: usize,
}

#[derive(Template)]
#[template(path = "components/departments/details.html")]
struct DepartmentDetailsComponentTemplate {
    department: Department,
}

#[derive(Template)]
#[template(path = "components/areas/table_rows.html")]
struct AreaRowsTemplate {
    areas: Vec<Area>,
    current_page: usize,
    total_pages: usize,
}

#[derive(Template)]
#[template(path = "components/areas/details.html")]
struct AreaDetailsComponentTemplate {
    area: Area,
}

#[derive(Template)]
#[template(path = "components/notification.html")]
struct NotificationTemplate {
    message: String,
    notification_type: String, // "success", "error", "warning"
}

#[derive(Template)]
#[template(path = "components/selectors/supervisors.html")]
struct SupervisorSelectorTemplate {
    supervisors: Vec<Supervisor>,
    selected_id: Option<i32>,
}

#[derive(Template)]
#[template(path = "components/equipment/table_rows.html")]
struct EquipmentRowsTemplate {
    equipment: Vec<Equipment>,
    current_page: usize,
    total_pages: usize,
}

#[derive(Template)]
#[template(path = "components/sites/table_rows.html")]
struct SiteRowsTemplate {
    sites: Vec<Site>,
    current_page: usize,
    total_pages: usize,
}

#[derive(Template)]
#[template(path = "components/personnel/table_rows.html")]
struct PersonnelRowsTemplate {
    personnel: Vec<Personnel>,
    current_page: usize,
    total_pages: usize,
}

// Additional model types needed for related data
#[derive(Debug, Clone, Serialize)]
pub struct Equipment {
    pub id: i32,
    pub name: String,
    pub amount: i32,
    pub available_amount: i32,
    pub purchase_date: String,
    pub purchase_cost: f64,
    pub fuel_type: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Site {
    pub id: i32,
    pub description: String,
    pub type_: String,
    pub area_name: String,
    pub department_name: String,
    pub client_name: String,
    pub status: String,
    pub risk_level: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct Personnel {
    pub id: i32,
    pub name: String,
    pub qualification: String,
    pub position: Option<String>,
    pub education_level: String,
    pub is_project_manager: bool,
}

// Page Endpoints

// Department pages
async fn departments_page(
    State(database): State<Database>,
    Query(params): Query<DepartmentFilter>,
) -> Html<String> {
    // Department listing page
    Html(String::new())
}

async fn department_new_page(
    State(database): State<Database>,
) -> Html<String> {
    // New department form
    Html(String::new())
}

async fn department_details_page(
    State(database): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Department details page
    Html(String::new())
}

async fn department_edit_page(
    State(database): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Edit department form
    Html(String::new())
}

// Area pages
async fn areas_page(
    State(database): State<Database>,
    Query(params): Query<AreaFilter>,
) -> Html<String> {
    // Area listing page
    Html(String::new())
}

async fn area_new_page(
    State(database): State<Database>,
) -> Html<String> {
    // New area form
    Html(String::new())
}

async fn area_details_page(
    State(database): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Area details page
    Html(String::new())
}

async fn area_edit_page(
    State(database): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Edit area form
    Html(String::new())
}

// HTMX Endpoints

// Department API endpoints
async fn fetch_departments(
    State(database): State<Database>,
    Query(params): Query<DepartmentFilter>,
) -> Html<String> {
    // Fetch departments with filters
    Html(String::new())
}

async fn create_department(
    State(database): State<Database>,
    Form(department): Form<DepartmentCreate>,
) -> Html<String> {
    // Create new department
    Html(String::new())
}

async fn fetch_department_details(
    State(database): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Fetch department details
    Html(String::new())
}

async fn update_department(
    State(database): State<Database>,
    Path(id): Path<i32>,
    Form(department): Form<DepartmentUpdate>,
) -> Html<String> {
    // Update department
    Html(String::new())
}

async fn delete_department(
    State(database): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Delete department
    Html(String::new())
}

async fn fetch_department_areas(
    State(database): State<Database>,
    Path(id): Path<i32>,
    Query(params): Query<PaginationParams>,
) -> Html<String> {
    // Fetch areas for department
    Html(String::new())
}

async fn fetch_department_equipment(
    State(database): State<Database>,
    Path(id): Path<i32>,
    Query(params): Query<PaginationParams>,
) -> Html<String> {
    // Fetch equipment for department
    Html(String::new())
}

async fn fetch_department_sites(
    State(database): State<Database>,
    Path(id): Path<i32>,
    Query(params): Query<PaginationParams>,
) -> Html<String> {
    // Fetch sites for department
    Html(String::new())
}

async fn fetch_department_personnel(
    State(database): State<Database>,
    Path(id): Path<i32>,
    Query(params): Query<PaginationParams>,
) -> Html<String> {
    // Fetch personnel for department
    Html(String::new())
}

// Area API endpoints
async fn fetch_areas(
    State(database): State<Database>,
    Query(params): Query<AreaFilter>,
) -> Html<String> {
    // Fetch areas with filters
    Html(String::new())
}

async fn create_area(
    State(database): State<Database>,
    Form(area): Form<AreaCreate>,
) -> Html<String> {
    // Create new area
    Html(String::new())
}

async fn fetch_area_details(
    State(database): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Fetch area details
    Html(String::new())
}

async fn update_area(
    State(database): State<Database>,
    Path(id): Path<i32>,
    Form(area): Form<AreaUpdate>,
) -> Html<String> {
    // Update area
    Html(String::new())
}

async fn delete_area(
    State(database): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Delete area
    Html(String::new())
}

async fn fetch_area_sites(
    State(database): State<Database>,
    Path(id): Path<i32>,
    Query(params): Query<PaginationParams>,
) -> Html<String> {
    // Fetch sites for area
    Html(String::new())
}

async fn fetch_area_personnel(
    State(database): State<Database>,
    Path(id): Path<i32>,
    Query(params): Query<PaginationParams>,
) -> Html<String> {
    // Fetch personnel for area
    Html(String::new())
}

// Supervisor selectors
async fn fetch_department_supervisors(
    State(database): State<Database>,
) -> Html<String> {
    // Fetch available supervisors for departments
    Html(String::new())
}

async fn fetch_area_supervisors(
    State(database): State<Database>,
) -> Html<String> {
    // Fetch available supervisors for areas
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
