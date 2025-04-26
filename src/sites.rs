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

// Enums for site types and risk levels
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(rename_all = "snake_case")]
pub enum SiteType {
    PowerPlant,
    Road,
    Housing,
    Bridge,
    Park,
}

impl FromStr for SiteType {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "power_plant" => Ok(SiteType::PowerPlant),
            "road" => Ok(SiteType::Road),
            "housing" => Ok(SiteType::Housing),
            "bridge" => Ok(SiteType::Bridge),
            "park" => Ok(SiteType::Park),
            _ => Err(format!("Unknown site type: {}", s)),
        }
    }
}

impl std::fmt::Display for SiteType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SiteType::PowerPlant => write!(f, "Power Plant"),
            SiteType::Road => write!(f, "Road"),
            SiteType::Housing => write!(f, "Housing"),
            SiteType::Bridge => write!(f, "Bridge"),
            SiteType::Park => write!(f, "Park"),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(rename_all = "lowercase")]
pub enum RiskLevel {
    High,
    Medium,
    Low,
}

impl FromStr for RiskLevel {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "high" => Ok(RiskLevel::High),
            "medium" => Ok(RiskLevel::Medium),
            "low" => Ok(RiskLevel::Low),
            _ => Err(format!("Unknown risk level: {}", s)),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(rename_all = "lowercase")]
pub enum EnergyEfficiency {
    High,
    Medium,
    Low,
}

impl std::fmt::Display for EnergyEfficiency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EnergyEfficiency::High => write!(f, "High"),
            EnergyEfficiency::Medium => write!(f, "Medium"),
            EnergyEfficiency::Low => write!(f, "Low"),
        }
    }
}

// Base Site types
#[derive(Serialize, Deserialize)]
pub struct Site {
    pub id: i32,
    pub name: String,
    pub area_id: i32,
    pub client_id: i32,
    pub site_type: SiteType,
    pub location: Location,
    pub risk_level: RiskLevel,
    pub description: Option<String>,
    // Joined fields
    pub area_name: Option<String>,
    pub client_name: Option<String>,
    pub department_id: Option<i32>,
    pub department_name: Option<String>,
    pub tasks_count: Option<i32>,
    pub equipment_count: Option<i32>,
    pub brigades_count: Option<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
}

// Site type-specific structs
#[derive(Serialize, Deserialize)]
pub struct PowerPlant {
    pub site_id: i32,
    pub energy_output: f64,
    pub energy_source: String,
    pub is_grid_connected: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Road {
    pub site_id: i32,
    pub length: f64,
    pub lanes: i32,
    pub surface: String,
}

#[derive(Serialize, Deserialize)]
pub struct Housing {
    pub site_id: i32,
    pub number_of_floors: i32,
    pub number_of_entrances: i32,
    pub housing_type: String,
    pub energy_efficiency: EnergyEfficiency,
}

#[derive(Serialize, Deserialize)]
pub struct Bridge {
    pub site_id: i32,
    pub length: f64,
    pub road_material: String,
    pub max_load: f64,
}

#[derive(Serialize, Deserialize)]
pub struct Park {
    pub site_id: i32,
    pub area: f64,
    pub has_playground: bool,
    pub has_lighting: bool,
}

// Combined site details structs
#[derive(Serialize, Deserialize)]
pub struct SiteDetails {
    pub site: Site,
    pub type_details: SiteTypeDetails,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SiteTypeDetails {
    PowerPlant(PowerPlant),
    Road(Road),
    Housing(Housing),
    Bridge(Bridge),
    Park(Park),
}

// Create and update structs
#[derive(Deserialize)]
pub struct SiteCreate {
    pub area_id: i32,
    pub client_id: i32,
    pub site_type: SiteType,
    pub latitude: f64,
    pub longitude: f64,
    pub risk_level: RiskLevel,
    pub description: Option<String>,
    // Type-specific fields will be handled separately based on site_type
}

#[derive(Deserialize)]
pub struct SiteUpdate {
    pub area_id: i32,
    pub client_id: i32,
    pub risk_level: RiskLevel,
    pub latitude: f64,
    pub longitude: f64,
    pub description: Option<String>,
    // Type cannot be changed after creation
}

#[derive(Deserialize)]
pub struct PowerPlantCreate {
    pub energy_output: f64,
    pub energy_source: String,
    pub is_grid_connected: bool,
}

#[derive(Deserialize)]
pub struct RoadCreate {
    pub length: f64,
    pub lanes: i32,
    pub surface: String,
}

#[derive(Deserialize)]
pub struct HousingCreate {
    pub number_of_floors: i32,
    pub number_of_entrances: i32,
    pub housing_type: String,
    pub energy_efficiency: EnergyEfficiency,
}

#[derive(Deserialize)]
pub struct BridgeCreate {
    pub length: f64,
    pub road_material: String,
    pub max_load: f64,
}

#[derive(Deserialize)]
pub struct ParkCreate {
    pub area: f64,
    pub has_playground: bool,
    pub has_lighting: bool,
}

// Task for site
#[derive(Serialize, Deserialize)]
pub struct Task {
    pub id: i32,
    pub site_id: i32,
    pub brigade_id: Option<i32>,
    pub period_start: String,
    pub expected_period_end: chrono::NaiveDate,
    pub actual_period_end: Option<chrono::NaiveDate>,
    pub name: String,
    pub description: Option<String>,
    // Joined fields
    pub brigade_name: Option<String>,
    pub materials_count: Option<i32>,
}

// Create task
#[derive(Deserialize)]
pub struct TaskCreate {
    pub brigade_id: Option<i32>,
    pub period_start: String,
    pub expected_period_end: String,
    pub name: String,
    pub description: Option<String>,
}

// Filter parameters
#[derive(Deserialize)]
pub struct SiteFilter {
    pub site_type: Option<SiteType>,
    pub area_id: Option<i32>,
    pub department_id: Option<i32>,
    pub client_id: Option<i32>,
    pub risk_level: Option<RiskLevel>,
    #[serde(flatten)]
    pub pagination: PaginationParams,
}

// Additional types needed for templates
#[derive(Serialize, Deserialize, Clone)]
pub struct Area {
    pub id: i32,
    pub name: String,
    pub department_id: i32,
    pub department_name: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Department {
    pub id: i32,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Client {
    pub id: i32,
    pub name: String,
    pub inn: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Brigade {
    pub id: i32,
    pub brigadier_name: String,
    pub workers_count: i32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MaterialUsage {
    pub task_id: i32,
    pub material_id: i32,
    pub material_name: String,
    pub expected_amount: f64,
    pub actual_amount: Option<f64>,
    pub units: String,
    pub cost_per_unit: f64,
    pub total_cost: f64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct EquipmentAllocation {
    pub equipment_id: i32,
    pub equipment_name: String,
    pub amount: i32,
    pub period_start: String,
    pub period_end: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Report {
    pub id: i32,
    pub title: String,
    pub date: String,
    pub report_type: String,
    pub status: String,
}

#[derive(Deserialize)]
pub struct SiteTypeQuery {
    pub site_type: SiteType,
}

// Template types for Site pages
#[derive(Template)]
#[template(path = "sites/index.html")]
pub struct SitesPageTemplate {
    pub sites: Vec<Site>,
    pub filter: Option<SiteFilter>,
    pub current_page: usize,
    pub total_pages: usize,
    pub areas: Vec<Area>,
    pub departments: Vec<Department>,
    pub clients: Vec<Client>,
}

#[derive(Template)]
#[template(path = "sites/new.html")]
pub struct SiteNewPageTemplate {
    pub areas: Vec<Area>,
    pub clients: Vec<Client>,
    pub area_id: Option<i32>,
    pub department_id: Option<i32>,
}

#[derive(Template)]
#[template(path = "sites/details.html")]
pub struct SiteDetailsPageTemplate {
    pub site_details: SiteDetails,
    pub active_tab: String, // schedule, materials, equipment, brigades, reports
}

#[derive(Template)]
#[template(path = "sites/edit.html")]
pub struct SiteEditPageTemplate {
    pub site_details: SiteDetails,
    pub areas: Vec<Area>,
    pub clients: Vec<Client>,
}

// Template types for HTMX components
#[derive(Template)]
#[template(path = "sites/components/site_rows.html")]
pub struct SiteRowsTemplate {
    pub sites: Vec<Site>,
}

#[derive(Template)]
#[template(path = "sites/components/site_details.html")]
pub struct SiteDetailsTemplate {
    pub site_details: SiteDetails,
}

#[derive(Template)]
#[template(path = "sites/components/schedule_tab.html")]
pub struct ScheduleTabTemplate {
    pub site_id: i32,
    pub tasks: Vec<Task>,
    pub brigades: Vec<Brigade>,
    pub current_page: usize,
    pub total_pages: usize,
}

#[derive(Template)]
#[template(path = "sites/components/materials_tab.html")]
pub struct MaterialsTabTemplate {
    pub site_id: i32,
    pub materials: Vec<MaterialUsage>,
    pub current_page: usize,
    pub total_pages: usize,
}

#[derive(Template)]
#[template(path = "sites/components/equipment_tab.html")]
pub struct EquipmentTabTemplate {
    pub site_id: i32,
    pub equipment_allocations: Vec<EquipmentAllocation>,
    pub current_page: usize,
    pub total_pages: usize,
}

#[derive(Template)]
#[template(path = "sites/components/brigades_tab.html")]
pub struct BrigadesTabTemplate {
    pub site_id: i32,
    pub brigades: Vec<Brigade>,
    pub current_page: usize,
    pub total_pages: usize,
}

#[derive(Template)]
#[template(path = "sites/components/reports_tab.html")]
pub struct ReportsTabTemplate {
    pub site_id: i32,
    pub reports: Vec<Report>,
    pub current_page: usize,
    pub total_pages: usize,
}

#[derive(Template)]
#[template(path = "sites/components/type_fields.html")]
pub struct TypeFieldsTemplate {
    pub site_type: SiteType,
    // Optional pre-filled values for edit mode
    pub power_plant: Option<PowerPlant>,
    pub road: Option<Road>,
    pub housing: Option<Housing>,
    pub bridge: Option<Bridge>,
    pub park: Option<Park>,
}

#[derive(Template)]
#[template(path = "sites/components/risk_level_selector.html")]
pub struct RiskLevelSelectorTemplate {
    pub selected_risk_level: Option<RiskLevel>,
}

#[derive(Template)]
#[template(path = "components/success_notification.html")]
pub struct SuccessNotificationTemplate {
    pub message: String,
}

// Page Endpoints
async fn sites_page(
    State(database): State<Database>,
    Query(params): Query<SiteFilter>,
) -> Html<String> {
    // Site listing page
    Html::from(String::new())
}

async fn site_new_page(
    State(database): State<Database>,
) -> Html<String> {
    // New site form
    Html::from(String::new())
}

async fn site_details_page(
    State(database): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Site details page
    Html::from(String::new())
}

async fn site_edit_page(
    State(database): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Edit site form
    Html::from(String::new())
}

// HTMX Endpoints
async fn fetch_sites(
    State(database): State<Database>,
    Query(params): Query<SiteFilter>,
) -> Html<String> {
    // Fetch sites with filters
    Html::from(String::new())
}

async fn create_site(
    State(database): State<Database>,
    Form(site): Form<SiteCreate>,
) -> Html<String> {
    // Create new site
    Html::from(String::new())
}

async fn fetch_site_details(
    State(database): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Fetch site details
    Html::from(String::new())
}

async fn update_site(
    State(database): State<Database>,
    Path(id): Path<i32>,
    Form(site_update): Form<SiteUpdate>,
) -> Html<String> {
    // Update site
    Html::from(String::new())
}

async fn delete_site(
    State(database): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Delete site
    // Return rendered SuccessNotificationTemplate
    Html::from(String::new())
}

async fn fetch_site_schedule(
    State(database): State<Database>,
    Path(id): Path<i32>,
    Query(params): Query<PaginationParams>,
) -> Html<String> {
    // Fetch construction schedule
    Html::from(String::new())
}

async fn fetch_site_materials(
    State(database): State<Database>,
    Path(id): Path<i32>,
    Query(params): Query<PaginationParams>,
) -> Html<String> {
    // Fetch materials usage
    Html::from(String::new())
}

async fn fetch_site_equipment(
    State(database): State<Database>,
    Path(id): Path<i32>,
    Query(params): Query<PaginationParams>,
) -> Html<String> {
    // Fetch allocated equipment
    Html::from(String::new())
}

async fn fetch_site_brigades(
    State(database): State<Database>,
    Path(id): Path<i32>,
    Query(params): Query<PaginationParams>,
) -> Html<String> {
    // Fetch assigned brigades
    Html::from(String::new())
}

async fn fetch_site_reports(
    State(database): State<Database>,
    Path(id): Path<i32>,
    Query(params): Query<PaginationParams>,
) -> Html<String> {
    // Fetch site reports
    Html::from(String::new())
}

async fn fetch_site_type_fields(
    State(database): State<Database>,
    Query(params): Query<SiteTypeQuery>,
) -> Html<String> {
    // Fetch form fields for selected site type
    Html::from(String::new())
}

async fn create_site_task(
    State(database): State<Database>,
    Path(id): Path<i32>,
    Form(task): Form<TaskCreate>,
) -> Html<String> {
    // Create new task for site
    Html::from(String::new())
}

async fn fetch_sites_by_department(
    State(database): State<Database>,
    Path(dept_id): Path<i32>,
    Query(params): Query<PaginationParams>,
) -> Html<String> {
    // Fetch sites by department
    Html::from(String::new())
}

async fn fetch_sites_by_area(
    State(database): State<Database>,
    Path(area_id): Path<i32>,
    Query(params): Query<PaginationParams>,
) -> Html<String> {
    // Fetch sites by area
    Html::from(String::new())
}

async fn fetch_risk_levels() -> Html<String> {
    // Fetch available risk levels
    Html::from(String::new())
}

pub fn router() -> Router<Database> {
    Router::new()
        // Page Endpoints
        .route("/sites", get(sites_page))
        .route("/sites/new", get(site_new_page))
        .route("/sites/{id}", get(site_details_page))
        .route("/sites/{id}/edit", get(site_edit_page))
        // HTMX Endpoints
        .route("/api/sites", get(fetch_sites).post(create_site))
        .route("/api/sites/{id}", get(fetch_site_details).put(update_site).delete(delete_site))
        .route("/api/sites/{id}/schedule", get(fetch_site_schedule))
        .route("/api/sites/{id}/materials", get(fetch_site_materials))
        .route("/api/sites/{id}/equipment", get(fetch_site_equipment))
        .route("/api/sites/{id}/brigades", get(fetch_site_brigades))
        .route("/api/sites/{id}/reports", get(fetch_site_reports))
        .route("/api/sites/type-fields", get(fetch_site_type_fields))
        .route("/api/sites/{id}/tasks", post(create_site_task))
        .route("/api/sites/by-department/{dept_id}", get(fetch_sites_by_department))
        .route("/api/sites/by-area/{area_id}", get(fetch_sites_by_area))
        .route("/api/sites/risk-levels", get(fetch_risk_levels))
}
