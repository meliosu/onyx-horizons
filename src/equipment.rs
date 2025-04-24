use axum::{
    extract::{Path, Query, State},
    response::Html,
    routing::{get, post, put, delete},
    Form, Router,
};
use serde::{Deserialize, Serialize};
use chrono::NaiveDate;
use std::str::FromStr;

use crate::database::Database;
use crate::general::PaginationParams;

// Enums for fuel types
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(rename_all = "lowercase")]
pub enum FuelType {
    Diesel,
    Petrol,
    Electric,
    Gas,
    None,
}

impl FromStr for FuelType {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "diesel" => Ok(FuelType::Diesel),
            "petrol" => Ok(FuelType::Petrol),
            "electric" => Ok(FuelType::Electric),
            "gas" => Ok(FuelType::Gas),
            "none" | "" => Ok(FuelType::None),
            _ => Err(format!("Unknown fuel type: {}", s)),
        }
    }
}

// Equipment types
#[derive(Serialize, Deserialize)]
pub struct Equipment {
    pub id: i32,
    pub name: String,
    pub amount: i32,
    pub purchase_date: NaiveDate,
    pub purchase_cost: f64,
    pub fuel_type: Option<FuelType>,
    // Additional fields for UI display
    pub available_amount: Option<i32>,  // Amount not currently allocated
    pub allocations_count: Option<i32>, // Number of current allocations
}

#[derive(Serialize, Deserialize)]
pub struct Allocation {
    pub id: i32,  // Added to have a unique identifier for each allocation
    pub equipment_id: i32,
    pub department_id: i32,
    pub site_id: Option<i32>,
    pub amount: i32,
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
    // Joined fields for display
    pub equipment_name: Option<String>,
    pub department_name: Option<String>,
    pub site_name: Option<String>,
    pub is_active: Option<bool>, // Is the allocation currently active
}

// Create and update types
#[derive(Deserialize)]
pub struct EquipmentCreate {
    pub name: String,
    pub amount: i32,
    pub purchase_date: NaiveDate,
    pub purchase_cost: f64,
    pub fuel_type: Option<FuelType>,
}

#[derive(Deserialize)]
pub struct EquipmentUpdate {
    pub name: String,
    pub amount: i32,
    pub purchase_date: NaiveDate,
    pub purchase_cost: f64,
    pub fuel_type: Option<FuelType>,
}

#[derive(Deserialize)]
pub struct AllocationCreate {
    pub department_id: i32,
    pub site_id: Option<i32>,
    pub amount: i32,
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
}

#[derive(Deserialize)]
pub struct AllocationUpdate {
    pub department_id: i32,
    pub site_id: Option<i32>,
    pub amount: i32,
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
}

// Filter types
#[derive(Deserialize)]
pub struct EquipmentFilter {
    pub name: Option<String>,
    pub fuel_type: Option<FuelType>,
    pub available_only: Option<bool>,
    #[serde(flatten)]
    pub pagination: PaginationParams,
}

#[derive(Deserialize)]
pub struct AllocationFilter {
    pub department_id: Option<i32>,
    pub site_id: Option<i32>,
    pub active_only: Option<bool>,
    #[serde(flatten)]
    pub pagination: PaginationParams,
}

// Page Endpoints
async fn equipment_page(
    State(database): State<Database>,
    Query(params): Query<EquipmentFilter>,
) -> Html<String> {
    // Equipment listing page
    Html(String::new())
}

async fn equipment_new_page(
    State(database): State<Database>,
) -> Html<String> {
    // New equipment form
    Html(String::new())
}

async fn equipment_details_page(
    State(database): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Equipment details page
    Html(String::new())
}

async fn equipment_edit_page(
    State(database): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Edit equipment form
    Html(String::new())
}

// HTMX Endpoints for Equipment
async fn fetch_equipment(
    State(database): State<Database>,
    Query(params): Query<EquipmentFilter>,
) -> Html<String> {
    // Fetch equipment with filters
    Html(String::new())
}

async fn create_equipment(
    State(database): State<Database>,
    Form(equipment): Form<EquipmentCreate>,
) -> Html<String> {
    // Create new equipment
    Html(String::new())
}

async fn fetch_equipment_details(
    State(database): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Fetch equipment details
    Html(String::new())
}

async fn update_equipment(
    State(database): State<Database>,
    Path(id): Path<i32>,
    Form(equipment): Form<EquipmentUpdate>,
) -> Html<String> {
    // Update equipment
    Html(String::new())
}

async fn delete_equipment(
    State(database): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Delete equipment
    Html(String::new())
}

// HTMX Endpoints for Allocations
async fn fetch_equipment_allocations(
    State(database): State<Database>,
    Path(id): Path<i32>,
    Query(params): Query<AllocationFilter>,
) -> Html<String> {
    // Fetch current equipment allocations
    Html(String::new())
}

async fn fetch_allocation_history(
    State(database): State<Database>,
    Path(id): Path<i32>,
    Query(params): Query<PaginationParams>,
) -> Html<String> {
    // Fetch allocation history
    Html(String::new())
}

async fn create_allocation(
    State(database): State<Database>,
    Path(id): Path<i32>,
    Form(allocation): Form<AllocationCreate>,
) -> Html<String> {
    // Create new equipment allocation
    Html(String::new())
}

async fn update_allocation(
    State(database): State<Database>,
    Path(allocation_id): Path<i32>,
    Form(allocation): Form<AllocationUpdate>,
) -> Html<String> {
    // Update equipment allocation
    Html(String::new())
}

async fn delete_allocation(
    State(database): State<Database>,
    Path(allocation_id): Path<i32>,
) -> Html<String> {
    // Delete equipment allocation
    Html(String::new())
}

// Equipment by relationship
async fn fetch_equipment_by_department(
    State(database): State<Database>,
    Path(dept_id): Path<i32>,
    Query(params): Query<PaginationParams>,
) -> Html<String> {
    // Fetch equipment by department
    Html(String::new())
}

async fn fetch_equipment_by_site(
    State(database): State<Database>,
    Path(site_id): Path<i32>,
    Query(params): Query<PaginationParams>,
) -> Html<String> {
    // Fetch equipment by site
    Html(String::new())
}

// Utility endpoints
async fn fetch_fuel_types() -> Html<String> {
    // Fetch available fuel types
    Html(String::new())
}

pub fn router() -> Router<Database> {
    Router::new()
        // Page Endpoints
        .route("/equipment", get(equipment_page))
        .route("/equipment/new", get(equipment_new_page))
        .route("/equipment/:id", get(equipment_details_page))
        .route("/equipment/:id/edit", get(equipment_edit_page))
        // HTMX Endpoints - Equipment
        .route("/api/equipment", get(fetch_equipment).post(create_equipment))
        .route("/api/equipment/:id", get(fetch_equipment_details).put(update_equipment).delete(delete_equipment))
        // HTMX Endpoints - Allocations
        .route("/api/equipment/:id/allocations", get(fetch_equipment_allocations).post(create_allocation))
        .route("/api/equipment/:id/allocation-history", get(fetch_allocation_history))
        .route("/api/equipment/allocations/:allocation_id", put(update_allocation).delete(delete_allocation))
        // Equipment by relationship
        .route("/api/equipment/by-department/:dept_id", get(fetch_equipment_by_department))
        .route("/api/equipment/by-site/:site_id", get(fetch_equipment_by_site))
        // Utility endpoints
        .route("/api/equipment/fuel-types", get(fetch_fuel_types))
}
