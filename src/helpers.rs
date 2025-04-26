use axum::{
    extract::{Path, Query, State},
    response::Html,
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use askama::Template;

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

// Template types for selectors
#[derive(Template)]
#[template(path = "helpers/selectors/department_selector.html")]
pub struct DepartmentSelectorTemplate {
    pub options: Vec<SelectOption>,
    pub name: String,
    pub id: String,
    pub label: Option<String>,
    pub placeholder: Option<String>,
    pub required: bool,
    pub selected_id: Option<i32>,
    pub search_term: Option<String>,
}

#[derive(Template)]
#[template(path = "helpers/selectors/area_selector.html")]
pub struct AreaSelectorTemplate {
    pub options: Vec<SelectOption>,
    pub name: String,
    pub id: String,
    pub label: Option<String>,
    pub placeholder: Option<String>,
    pub required: bool,
    pub selected_id: Option<i32>,
    pub search_term: Option<String>,
}

#[derive(Template)]
#[template(path = "helpers/selectors/areas_by_department_selector.html")]
pub struct AreasByDepartmentSelectorTemplate {
    pub options: Vec<SelectOption>,
    pub name: String,
    pub id: String,
    pub label: Option<String>,
    pub placeholder: Option<String>,
    pub required: bool,
    pub selected_id: Option<i32>,
    pub department_id: i32,
    pub search_term: Option<String>,
}

#[derive(Template)]
#[template(path = "helpers/selectors/client_selector.html")]
pub struct ClientSelectorTemplate {
    pub options: Vec<SelectOption>,
    pub name: String,
    pub id: String,
    pub label: Option<String>,
    pub placeholder: Option<String>,
    pub required: bool,
    pub selected_id: Option<i32>,
    pub search_term: Option<String>,
}

#[derive(Template)]
#[template(path = "helpers/selectors/worker_selector.html")]
pub struct WorkerSelectorTemplate {
    pub options: Vec<SelectOption>,
    pub name: String,
    pub id: String,
    pub label: Option<String>,
    pub placeholder: Option<String>,
    pub required: bool,
    pub selected_id: Option<i32>,
    pub search_term: Option<String>,
    pub show_profession: bool,
}

#[derive(Template)]
#[template(path = "helpers/selectors/brigade_selector.html")]
pub struct BrigadeSelectorTemplate {
    pub options: Vec<SelectOption>,
    pub name: String,
    pub id: String,
    pub label: Option<String>,
    pub placeholder: Option<String>,
    pub required: bool,
    pub selected_id: Option<i32>,
    pub search_term: Option<String>,
}

#[derive(Template)]
#[template(path = "helpers/selectors/site_selector.html")]
pub struct SiteSelectorTemplate {
    pub options: Vec<SelectOption>,
    pub name: String,
    pub id: String,
    pub label: Option<String>,
    pub placeholder: Option<String>,
    pub required: bool,
    pub selected_id: Option<i32>,
    pub search_term: Option<String>,
}

#[derive(Template)]
#[template(path = "helpers/selectors/material_selector.html")]
pub struct MaterialSelectorTemplate {
    pub options: Vec<SelectOption>,
    pub name: String,
    pub id: String,
    pub label: Option<String>,
    pub placeholder: Option<String>,
    pub required: bool,
    pub selected_id: Option<i32>,
    pub search_term: Option<String>,
    pub show_units: bool,
}

// Template types for validation
#[derive(Template)]
#[template(path = "helpers/validation/validation_message.html")]
pub struct ValidationMessageTemplate {
    pub field: String,
    pub is_valid: bool,
    pub message: Option<String>,
}

#[derive(Template)]
#[template(path = "helpers/validation/form_validation.html")]
pub struct FormValidationTemplate {
    pub is_valid: bool,
    pub messages: Vec<ValidationMessage>,
    pub form_type: String,
}

// Selectors for departments
async fn department_selector(
    State(database): State<Database>,
    Query(filter): Query<SelectorFilter>,
) -> Html<String> {
    // Should return rendered DepartmentSelectorTemplate
    // Get department selector options
    Html(String::new())
}

// Selectors for areas
async fn area_selector(
    State(database): State<Database>,
    Query(filter): Query<SelectorFilter>,
) -> Html<String> {
    // Should return rendered AreaSelectorTemplate
    // Get area selector options
    Html(String::new())
}

// Selectors for areas filtered by department
async fn areas_by_department_selector(
    State(database): State<Database>,
    Path(department_id): Path<i32>,
    Query(filter): Query<SelectorFilter>,
) -> Html<String> {
    // Should return rendered AreasByDepartmentSelectorTemplate
    // Get areas filtered by department
    Html(String::new())
}

// Selectors for clients
async fn client_selector(
    State(database): State<Database>,
    Query(filter): Query<SelectorFilter>,
) -> Html<String> {
    // Should return rendered ClientSelectorTemplate
    // Get client selector options
    Html(String::new())
}

// Selectors for workers
async fn worker_selector(
    State(database): State<Database>,
    Query(filter): Query<SelectorFilter>,
) -> Html<String> {
    // Should return rendered WorkerSelectorTemplate
    // Get worker selector options
    Html(String::new())
}

// Selectors for brigades
async fn brigade_selector(
    State(database): State<Database>,
    Query(filter): Query<SelectorFilter>,
) -> Html<String> {
    // Should return rendered BrigadeSelectorTemplate
    // Get brigade selector options
    Html(String::new())
}

// Selectors for sites
async fn site_selector(
    State(database): State<Database>,
    Query(filter): Query<SelectorFilter>,
) -> Html<String> {
    // Should return rendered SiteSelectorTemplate
    // Get site selector options
    Html(String::new())
}

// Selectors for materials
async fn material_selector(
    State(database): State<Database>,
    Query(filter): Query<SelectorFilter>,
) -> Html<String> {
    // Should return rendered MaterialSelectorTemplate
    // Get material selector options
    Html(String::new())
}

// Form validation
async fn validate_form(
    Path(form_type): Path<String>,
    State(database): State<Database>,
    axum::Json(validation_request): axum::Json<ValidationRequest>,
) -> Html<String> {
    // Should return rendered FormValidationTemplate
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