use axum::{
    extract::{Path, Query, State},
    response::Html,
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};

use crate::database::Database;

// Generic selector option type
#[derive(Serialize)]
pub struct SelectOption {
    pub value: String,
    pub label: String,
    pub selected: bool,
    pub data_attributes: Option<Vec<(String, String)>>, // For additional data attributes
}

// Filter types for selectors
#[derive(Deserialize)]
pub struct SelectorFilter {
    pub search: Option<String>,  // For filtering options by search term
    pub selected_id: Option<i32>, // For pre-selecting an option
    pub exclude_ids: Option<Vec<i32>>, // For excluding certain IDs
}

// Validation types
#[derive(Serialize)]
pub struct ValidationMessage {
    pub field: String,
    pub is_valid: bool,
    pub message: Option<String>,
}

#[derive(Serialize)]
pub struct ValidationResponse {
    pub is_valid: bool,
    pub messages: Vec<ValidationMessage>,
}

// Form validation request 
#[derive(Deserialize)]
pub struct ValidationRequest {
    pub fields: std::collections::HashMap<String, String>,
}

// Selectors for departments
async fn department_selector(
    State(database): State<Database>,
    Query(filter): Query<SelectorFilter>,
) -> Html<String> {
    // Get department selector options
    Html(String::new())
}

// Selectors for areas
async fn area_selector(
    State(database): State<Database>,
    Query(filter): Query<SelectorFilter>,
) -> Html<String> {
    // Get area selector options
    Html(String::new())
}

// Selectors for areas filtered by department
async fn areas_by_department_selector(
    State(database): State<Database>,
    Path(department_id): Path<i32>,
    Query(filter): Query<SelectorFilter>,
) -> Html<String> {
    // Get areas filtered by department
    Html(String::new())
}

// Selectors for clients
async fn client_selector(
    State(database): State<Database>,
    Query(filter): Query<SelectorFilter>,
) -> Html<String> {
    // Get client selector options
    Html(String::new())
}

// Selectors for workers
async fn worker_selector(
    State(database): State<Database>,
    Query(filter): Query<SelectorFilter>,
) -> Html<String> {
    // Get worker selector options
    Html(String::new())
}

// Selectors for brigades
async fn brigade_selector(
    State(database): State<Database>,
    Query(filter): Query<SelectorFilter>,
) -> Html<String> {
    // Get brigade selector options
    Html(String::new())
}

// Selectors for sites
async fn site_selector(
    State(database): State<Database>,
    Query(filter): Query<SelectorFilter>,
) -> Html<String> {
    // Get site selector options
    Html(String::new())
}

// Selectors for materials
async fn material_selector(
    State(database): State<Database>,
    Query(filter): Query<SelectorFilter>,
) -> Html<String> {
    // Get material selector options
    Html(String::new())
}

// Form validation
async fn validate_form(
    Path(form_type): Path<String>,
    State(database): State<Database>,
    axum::Json(validation_request): axum::Json<ValidationRequest>,
) -> Html<String> {
    // Validate form fields based on form_type
    Html(String::new())
}

pub fn router() -> Router<Database> {
    Router::new()
        .route("/api/selectors/departments", get(department_selector))
        .route("/api/selectors/areas", get(area_selector))
        .route("/api/selectors/areas-by-department/{id}", get(areas_by_department_selector))
        .route("/api/selectors/clients", get(client_selector))
        .route("/api/selectors/workers", get(worker_selector))
        .route("/api/selectors/brigades", get(brigade_selector))
        .route("/api/selectors/sites", get(site_selector))
        .route("/api/selectors/materials", get(material_selector))
        .route("/api/validation/form/{form_type}", get(validate_form))
}