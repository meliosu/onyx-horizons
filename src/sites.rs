use axum::{
    extract::{Path, Query, State},
    response::Html,
    routing::{get, post, put, delete},
    Form, Router,
};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

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

// Base Site types
#[derive(Serialize, Deserialize)]
pub struct Site {
    pub id: i32,
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
    pub expected_period_end: String,
    pub actual_period_end: Option<String>,
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

// Page Endpoints
async fn sites_page(
    State(database): State<Database>,
    Query(params): Query<SiteFilter>,
) -> Html<String> {
    // Site listing page
    Html(String::new())
}

async fn site_new_page(
    State(database): State<Database>,
) -> Html<String> {
    // New site form
    Html(String::new())
}

async fn site_details_page(
    State(database): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Site details page
    Html(String::new())
}

async fn site_edit_page(
    State(database): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Edit site form
    Html(String::new())
}

// HTMX Endpoints
async fn fetch_sites(
    State(database): State<Database>,
    Query(params): Query<SiteFilter>,
) -> Html<String> {
    // Fetch sites with filters
    Html(String::new())
}

async fn create_site(
    State(database): State<Database>,
    Form(site): Form<SiteCreate>,
) -> Html<String> {
    // Create new site
    Html(String::new())
}

async fn fetch_site_details(
    State(database): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Fetch site details
    Html(String::new())
}

async fn update_site(
    State(database): State<Database>,
    Path(id): Path<i32>,
    Form(site): Form<SiteUpdate>,
) -> Html<String> {
    // Update site
    Html(String::new())
}

async fn delete_site(
    State(database): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Delete site
    Html(String::new())
}

async fn fetch_site_schedule(
    State(database): State<Database>,
    Path(id): Path<i32>,
    Query(params): Query<PaginationParams>,
) -> Html<String> {
    // Fetch construction schedule
    Html(String::new())
}

async fn fetch_site_materials(
    State(database): State<Database>,
    Path(id): Path<i32>,
    Query(params): Query<PaginationParams>,
) -> Html<String> {
    // Fetch materials usage
    Html(String::new())
}

async fn fetch_site_equipment(
    State(database): State<Database>,
    Path(id): Path<i32>,
    Query(params): Query<PaginationParams>,
) -> Html<String> {
    // Fetch allocated equipment
    Html(String::new())
}

async fn fetch_site_brigades(
    State(database): State<Database>,
    Path(id): Path<i32>,
    Query(params): Query<PaginationParams>,
) -> Html<String> {
    // Fetch assigned brigades
    Html(String::new())
}

async fn fetch_site_reports(
    State(database): State<Database>,
    Path(id): Path<i32>,
    Query(params): Query<PaginationParams>,
) -> Html<String> {
    // Fetch site reports
    Html(String::new())
}

async fn fetch_site_type_fields(
    State(database): State<Database>,
    Query(params): Query<SiteTypeQuery>,
) -> Html<String> {
    // Fetch form fields for selected site type
    Html(String::new())
}

#[derive(Deserialize)]
pub struct SiteTypeQuery {
    pub site_type: SiteType,
}

async fn create_site_task(
    State(database): State<Database>,
    Path(id): Path<i32>,
    Form(task): Form<TaskCreate>,
) -> Html<String> {
    // Create new task for site
    Html(String::new())
}

async fn fetch_sites_by_department(
    State(database): State<Database>,
    Path(dept_id): Path<i32>,
    Query(params): Query<PaginationParams>,
) -> Html<String> {
    // Fetch sites by department
    Html(String::new())
}

async fn fetch_sites_by_area(
    State(database): State<Database>,
    Path(area_id): Path<i32>,
    Query(params): Query<PaginationParams>,
) -> Html<String> {
    // Fetch sites by area
    Html(String::new())
}

async fn fetch_risk_levels() -> Html<String> {
    // Fetch available risk levels
    Html(String::new())
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
