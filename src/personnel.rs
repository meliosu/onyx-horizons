use axum::{
    extract::{Path, Query, State},
    response::Html,
    routing::{get, post, put, delete},
    Form, Router,
};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use askama::Template;

use crate::database::Database;
use crate::general::PaginationParams;
use crate::departments::{Department, Area};

// Enums for personnel types
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Gender {
    Male,
    Female,
}

impl FromStr for Gender {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "male" => Ok(Gender::Male),
            "female" => Ok(Gender::Female),
            _ => Err(format!("Unknown gender: {}", s)),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(rename_all = "snake_case")]
pub enum EmployeeClass {
    Worker,
    TechnicalPersonnel,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(rename_all = "snake_case")]
pub enum WorkerProfession {
    Electrician,
    Plumber,
    Welder,
    Driver,
    Mason,
}

impl FromStr for WorkerProfession {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "electrician" => Ok(WorkerProfession::Electrician),
            "plumber" => Ok(WorkerProfession::Plumber),
            "welder" => Ok(WorkerProfession::Welder),
            "driver" => Ok(WorkerProfession::Driver),
            "mason" => Ok(WorkerProfession::Mason),
            _ => Err(format!("Unknown worker profession: {}", s)),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Qualification {
    Technician,
    Technologist,
    Engineer,
}

impl FromStr for Qualification {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "technician" => Ok(Qualification::Technician),
            "technologist" => Ok(Qualification::Technologist),
            "engineer" => Ok(Qualification::Engineer),
            _ => Err(format!("Unknown qualification: {}", s)),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Position {
    Master,
    Foreman,
    None,
}

impl FromStr for Position {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "master" => Ok(Position::Master),
            "foreman" => Ok(Position::Foreman),
            "none" => Ok(Position::None),
            "" => Ok(Position::None),
            _ => Err(format!("Unknown position: {}", s)),
        }
    }
}

// Base Employee type
#[derive(Serialize, Deserialize)]
pub struct Employee {
    pub id: i32,
    pub employee_class: EmployeeClass,
    pub first_name: String,
    pub last_name: String,
    pub middle_name: Option<String>,
    pub gender: Gender,
    pub photo: Option<String>,
    pub salary: i32,
    pub phone_number: String,
}

// Worker type
#[derive(Serialize, Deserialize)]
pub struct Worker {
    pub employee: Employee,
    pub profession: WorkerProfession,
    pub union_name: Option<String>,
    // Joined fields
    pub brigade_id: Option<i32>,
    pub brigade_name: Option<String>,
    pub is_brigadier: Option<bool>,
}

// Technical Personnel type
#[derive(Serialize, Deserialize)]
pub struct TechnicalPersonnel {
    pub employee: Employee,
    pub qualification: Qualification,
    pub position: Option<Position>,
    pub education_level: String,
    pub software_skills: Option<Vec<String>>,
    pub is_project_manager: bool,
    // Joined fields
    pub department_id: Option<i32>,
    pub department_name: Option<String>,
    pub area_id: Option<i32>,
    pub area_name: Option<String>,
}

// Profession-specific types
#[derive(Serialize, Deserialize)]
pub struct Electrician {
    pub worker_id: i32,
    pub voltage_specialization: String,
}

#[derive(Serialize, Deserialize)]
pub struct Plumber {
    pub worker_id: i32,
    pub pipe_specialization: String,
}

#[derive(Serialize, Deserialize)]
pub struct Welder {
    pub worker_id: i32,
    pub welding_machine: String,
}

#[derive(Serialize, Deserialize)]
pub struct Driver {
    pub worker_id: i32,
    pub vehicle_type: String,
    pub number_of_accidents: i32,
}

#[derive(Serialize, Deserialize)]
pub struct Mason {
    pub worker_id: i32,
    pub hq_restoration_skills: bool,
}

// Qualification-specific types
#[derive(Serialize, Deserialize)]
pub struct Technician {
    pub technical_personnel_id: i32,
    pub safety_training_level: String,
}

#[derive(Serialize, Deserialize)]
pub struct Technologist {
    pub technical_personnel_id: i32,
    pub management_tools: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Engineer {
    pub technical_personnel_id: i32,
    pub pe_license_id: i32,
}

// Combined detail types
#[derive(Serialize, Deserialize)]
pub struct WorkerDetails {
    pub worker: Worker,
    pub profession_details: ProfessionDetails,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "profession")]
pub enum ProfessionDetails {
    Electrician(Electrician),
    Plumber(Plumber),
    Welder(Welder),
    Driver(Driver),
    Mason(Mason),
}

#[derive(Serialize, Deserialize)]
pub struct TechnicalPersonnelDetails {
    pub technical_personnel: TechnicalPersonnel,
    pub qualification_details: QualificationDetails,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "qualification")]
pub enum QualificationDetails {
    Technician(Technician),
    Technologist(Technologist),
    Engineer(Engineer),
}

// Create and update types
#[derive(Deserialize)]
pub struct EmployeeCreate {
    pub first_name: String,
    pub last_name: String,
    pub middle_name: Option<String>,
    pub gender: Gender,
    pub photo: Option<String>,
    pub salary: i32,
    pub phone_number: String,
}

#[derive(Deserialize)]
pub struct WorkerCreate {
    pub employee: EmployeeCreate,
    pub profession: WorkerProfession,
    pub union_name: Option<String>,
    // Profession-specific fields handled separately
}

#[derive(Deserialize)]
pub struct TechnicalPersonnelCreate {
    pub employee: EmployeeCreate,
    pub qualification: Qualification,
    pub position: Option<Position>,
    pub education_level: String,
    pub software_skills: Option<Vec<String>>,
    pub is_project_manager: bool,
    // Qualification-specific fields handled separately
}

#[derive(Deserialize)]
pub struct ElectricianCreate {
    pub voltage_specialization: String,
}

#[derive(Deserialize)]
pub struct PlumberCreate {
    pub pipe_specialization: String,
}

#[derive(Deserialize)]
pub struct WelderCreate {
    pub welding_machine: String,
}

#[derive(Deserialize)]
pub struct DriverCreate {
    pub vehicle_type: String,
    pub number_of_accidents: i32,
}

#[derive(Deserialize)]
pub struct MasonCreate {
    pub hq_restoration_skills: bool,
}

#[derive(Deserialize)]
pub struct TechnicianCreate {
    pub safety_training_level: String,
}

#[derive(Deserialize)]
pub struct TechnologistCreate {
    pub management_tools: Vec<String>,
}

#[derive(Deserialize)]
pub struct EngineerCreate {
    pub pe_license_id: i32,
}

// Update types
#[derive(Deserialize)]
pub struct EmployeeUpdate {
    pub first_name: String,
    pub last_name: String,
    pub middle_name: Option<String>,
    pub gender: Gender,
    pub photo: Option<String>,
    pub salary: i32,
    pub phone_number: String,
}

#[derive(Deserialize)]
pub struct WorkerUpdate {
    pub employee: EmployeeUpdate,
    pub union_name: Option<String>,
    // Profession can't be changed after creation
}

#[derive(Deserialize)]
pub struct TechnicalPersonnelUpdate {
    pub employee: EmployeeUpdate,
    pub position: Option<Position>,
    pub education_level: String,
    pub software_skills: Option<Vec<String>>,
    pub is_project_manager: bool,
    // Qualification can't be changed after creation
}

// Filter types
#[derive(Deserialize)]
pub struct TechnicalPersonnelFilter {
    pub qualification: Option<Qualification>,
    pub position: Option<Position>,
    pub is_project_manager: Option<bool>,
    pub department_id: Option<i32>,
    pub area_id: Option<i32>,
    #[serde(flatten)]
    pub pagination: PaginationParams,
}

#[derive(Deserialize)]
pub struct WorkerFilter {
    pub profession: Option<WorkerProfession>,
    pub brigade_id: Option<i32>,
    pub is_brigadier: Option<bool>,
    #[serde(flatten)]
    pub pagination: PaginationParams,
}

#[derive(Deserialize)]
pub struct ProfessionTypeQuery {
    pub profession: WorkerProfession,
}

// Template structs
// Technical Personnel templates
#[derive(Template)]
#[template(path = "personnel/technical/index.html")]
struct TechnicalPersonnelTemplate {
    personnel: Vec<TechnicalPersonnel>,
    filters: TechnicalPersonnelFilter,
    departments: Vec<Department>,
    areas: Vec<Area>,
    total_pages: usize,
    current_page: usize,
}

#[derive(Template)]
#[template(path = "personnel/technical/new.html")]
struct TechnicalPersonnelNewTemplate {
    departments: Vec<Department>,
    areas: Vec<Area>,
}

#[derive(Template)]
#[template(path = "personnel/technical/details.html")]
struct TechnicalPersonnelDetailsTemplate {
    personnel_details: TechnicalPersonnelDetails,
    department: Option<Department>,
    area: Option<Area>,
    assigned_sites: Vec<Site>,
}

#[derive(Template)]
#[template(path = "personnel/technical/edit.html")]
struct TechnicalPersonnelEditTemplate {
    personnel_details: TechnicalPersonnelDetails,
    departments: Vec<Department>,
    areas: Vec<Area>,
}

// Worker templates
#[derive(Template)]
#[template(path = "personnel/workers/index.html")]
struct WorkersTemplate {
    workers: Vec<Worker>,
    filters: WorkerFilter,
    brigades: Vec<Brigade>,
    total_pages: usize,
    current_page: usize,
}

#[derive(Template)]
#[template(path = "personnel/workers/new.html")]
struct WorkerNewTemplate {
    brigades: Vec<Brigade>,
}

#[derive(Template)]
#[template(path = "personnel/workers/details.html")]
struct WorkerDetailsTemplate {
    worker_details: WorkerDetails,
    brigade: Option<Brigade>,
    tasks_history: Vec<WorkerTask>,
}

#[derive(Template)]
#[template(path = "personnel/workers/edit.html")]
struct WorkerEditTemplate {
    worker_details: WorkerDetails,
    brigades: Vec<Brigade>,
}

// HTMX component templates
#[derive(Template)]
#[template(path = "components/personnel/technical/table_rows.html")]
struct TechnicalPersonnelRowsTemplate {
    personnel: Vec<TechnicalPersonnel>,
    current_page: usize,
    total_pages: usize,
}

#[derive(Template)]
#[template(path = "components/personnel/technical/details.html")]
struct TechnicalPersonnelDetailsComponentTemplate {
    personnel_details: TechnicalPersonnelDetails,
}

#[derive(Template)]
#[template(path = "components/personnel/workers/table_rows.html")]
struct WorkerRowsTemplate {
    workers: Vec<Worker>,
    current_page: usize,
    total_pages: usize,
}

#[derive(Template)]
#[template(path = "components/personnel/workers/details.html")]
struct WorkerDetailsComponentTemplate {
    worker_details: WorkerDetails,
}

#[derive(Template)]
#[template(path = "components/personnel/qualifications.html")]
struct QualificationsTemplate {
    selected: Option<Qualification>,
}

#[derive(Template)]
#[template(path = "components/personnel/positions.html")]
struct PositionsTemplate {
    selected: Option<Position>,
}

#[derive(Template)]
#[template(path = "components/personnel/professions.html")]
struct ProfessionsTemplate {
    selected: Option<WorkerProfession>,
}

#[derive(Template)]
#[template(path = "components/personnel/profession_fields.html")]
struct ProfessionFieldsTemplate {
    profession: WorkerProfession,
    details: Option<ProfessionDetails>,
}

// Additional models needed for templates
#[derive(Serialize, Deserialize, Clone)]
pub struct Brigade {
    pub id: i32,
    pub brigadier_id: i32,
    pub brigadier_name: String,
    pub workers_count: i32,
    pub current_site_id: Option<i32>,
    pub current_site_name: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Site {
    pub id: i32,
    pub description: String,
    pub site_type: String,
    pub status: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct WorkerTask {
    pub id: i32,
    pub name: String,
    pub site_id: i32,
    pub site_name: String,
    pub period_start: String,
    pub period_end: String,
    pub status: String,
}

// Page Endpoints - Technical Personnel
async fn technical_personnel_page(
    State(database): State<Database>,
    Query(params): Query<TechnicalPersonnelFilter>,
) -> Html<String> {
    // Technical personnel listing page
    Html(String::new())
}

async fn technical_personnel_new_page(
    State(database): State<Database>,
) -> Html<String> {
    // New technical personnel form
    Html(String::new())
}

async fn technical_personnel_details_page(
    State(database): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Technical personnel details page
    Html(String::new())
}

async fn technical_personnel_edit_page(
    State(database): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Edit technical personnel form
    Html(String::new())
}

// Page Endpoints - Workers
async fn workers_page(
    State(database): State<Database>,
    Query(params): Query<WorkerFilter>,
) -> Html<String> {
    // Workers listing page
    Html(String::new())
}

async fn worker_new_page(
    State(database): State<Database>,
) -> Html<String> {
    // New worker form
    Html(String::new())
}

async fn worker_details_page(
    State(database): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Worker details page
    Html(String::new())
}

async fn worker_edit_page(
    State(database): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Edit worker form
    Html(String::new())
}

// HTMX Endpoints - Technical Personnel
async fn fetch_technical_personnel(
    State(database): State<Database>,
    Query(params): Query<TechnicalPersonnelFilter>,
) -> Html<String> {
    // Fetch technical personnel with filters
    Html(String::new())
}

async fn create_technical_personnel(
    State(database): State<Database>,
    Form(personnel): Form<TechnicalPersonnelCreate>,
) -> Html<String> {
    // Create new technical personnel
    Html(String::new())
}

async fn fetch_technical_personnel_details(
    State(database): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Fetch technical personnel details
    Html(String::new())
}

async fn update_technical_personnel(
    State(database): State<Database>,
    Path(id): Path<i32>,
    Form(personnel): Form<TechnicalPersonnelUpdate>,
) -> Html<String> {
    // Update technical personnel
    Html(String::new())
}

async fn delete_technical_personnel(
    State(database): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Delete technical personnel
    Html(String::new())
}

async fn fetch_qualifications() -> Html<String> {
    // Fetch available qualifications
    Html(String::new())
}

async fn fetch_positions() -> Html<String> {
    // Fetch available positions
    Html(String::new())
}

// HTMX Endpoints - Workers
async fn fetch_workers(
    State(database): State<Database>,
    Query(params): Query<WorkerFilter>,
) -> Html<String> {
    // Fetch workers with filters
    Html(String::new())
}

async fn create_worker(
    State(database): State<Database>,
    Form(worker): Form<WorkerCreate>,
) -> Html<String> {
    // Create new worker
    Html(String::new())
}

async fn fetch_worker_details(
    State(database): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Fetch worker details
    Html(String::new())
}

async fn update_worker(
    State(database): State<Database>,
    Path(id): Path<i32>,
    Form(worker): Form<WorkerUpdate>,
) -> Html<String> {
    // Update worker
    Html(String::new())
}

async fn delete_worker(
    State(database): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Delete worker
    Html(String::new())
}

async fn fetch_professions() -> Html<String> {
    // Fetch available professions
    Html(String::new())
}

async fn fetch_profession_type_fields(
    Query(params): Query<ProfessionTypeQuery>,
) -> Html<String> {
    // Fetch form fields for selected profession
    Html(String::new())
}

// Personnel by relation
async fn fetch_personnel_by_department(
    State(database): State<Database>,
    Path(dept_id): Path<i32>,
    Query(params): Query<PaginationParams>,
) -> Html<String> {
    // Fetch personnel by department
    Html(String::new())
}

async fn fetch_personnel_by_area(
    State(database): State<Database>,
    Path(area_id): Path<i32>,
    Query(params): Query<PaginationParams>,
) -> Html<String> {
    // Fetch personnel by area
    Html(String::new())
}

async fn fetch_workers_by_brigade(
    State(database): State<Database>,
    Path(brigade_id): Path<i32>,
    Query(params): Query<PaginationParams>,
) -> Html<String> {
    // Fetch workers by brigade
    Html(String::new())
}

pub fn router() -> Router<Database> {
    Router::new()
        // Page Endpoints - Technical Personnel
        .route("/personnel/technical", get(technical_personnel_page))
        .route("/personnel/technical/new", get(technical_personnel_new_page))
        .route("/personnel/technical/{id}", get(technical_personnel_details_page))
        .route("/personnel/technical/{id}/edit", get(technical_personnel_edit_page))
        // Page Endpoints - Workers
        .route("/personnel/workers", get(workers_page))
        .route("/personnel/workers/new", get(worker_new_page))
        .route("/personnel/workers/{id}", get(worker_details_page))
        .route("/personnel/workers/{id}/edit", get(worker_edit_page))
        // HTMX Endpoints - Technical Personnel
        .route("/api/personnel/technical", get(fetch_technical_personnel).post(create_technical_personnel))
        .route("/api/personnel/technical/{id}", get(fetch_technical_personnel_details).put(update_technical_personnel).delete(delete_technical_personnel))
        .route("/api/personnel/technical/qualifications", get(fetch_qualifications))
        .route("/api/personnel/technical/positions", get(fetch_positions))
        // HTMX Endpoints - Workers
        .route("/api/personnel/workers", get(fetch_workers).post(create_worker))
        .route("/api/personnel/workers/{id}", get(fetch_worker_details).put(update_worker).delete(delete_worker))
        .route("/api/personnel/workers/professions", get(fetch_professions))
        .route("/api/personnel/workers/type-fields/{profession}", get(fetch_profession_type_fields))
        // Personnel by relation
        .route("/api/personnel/by-department/{dept_id}", get(fetch_personnel_by_department))
        .route("/api/personnel/by-area/{area_id}", get(fetch_personnel_by_area))
        .route("/api/personnel/by-brigade/{brigade_id}", get(fetch_workers_by_brigade))
}
