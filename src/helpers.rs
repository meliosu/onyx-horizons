use axum::{
    extract::{Path, Query, State},
    response::Html,
    routing::get,
    Form, Router,
};
use askama::Template;
use serde::{Deserialize, Serialize};

use crate::database::Database;

// Common selector types
#[derive(Serialize, Deserialize, Debug)]
pub struct SelectOption {
    pub value: String,
    pub label: String,
    pub selected: bool,
}

// Query parameters for selectors
#[derive(Deserialize)]
pub struct SelectorParams {
    pub selected: Option<String>,
    pub exclude_ids: Option<String>, // Comma-separated list of IDs to exclude
    pub search: Option<String>,
    pub limit: Option<usize>,
}

// Form validation request and response
#[derive(Deserialize)]
pub struct ValidationRequest {
    pub field: String,
    pub value: String,
    pub context: Option<String>, // Additional context like form ID or related entity
}

#[derive(Serialize)]
pub struct ValidationResponse {
    pub field: String,
    pub is_valid: bool,
    pub message: Option<String>,
}

// Department selector
async fn department_selector(
    State(database): State<Database>,
    Query(params): Query<SelectorParams>,
) -> Html<String> {
    Html::from(String::new())
}

// Area selector
async fn area_selector(
    State(database): State<Database>,
    Query(params): Query<SelectorParams>,
) -> Html<String> {
    Html::from(String::new())
}

// Areas by department
async fn areas_by_department(
    State(database): State<Database>,
    Path(department_id): Path<i32>,
    Query(params): Query<SelectorParams>,
) -> Html<String> {
    Html::from(String::new())
}

// Client selector
async fn client_selector(
    State(database): State<Database>,
    Query(params): Query<SelectorParams>,
) -> Html<String> {
    Html::from(String::new())
}

// Worker selector
async fn worker_selector(
    State(database): State<Database>,
    Query(params): Query<SelectorParams>,
) -> Html<String> {
    Html::from(String::new())
}

// Brigade selector
async fn brigade_selector(
    State(database): State<Database>,
    Query(params): Query<SelectorParams>,
) -> Html<String> {
    Html::from(String::new())
}

// Site selector
async fn site_selector(
    State(database): State<Database>,
    Query(params): Query<SelectorParams>,
) -> Html<String> {
    Html::from(String::new())
}

// Material selector
async fn material_selector(
    State(database): State<Database>,
    Query(params): Query<SelectorParams>,
) -> Html<String> {
    Html::from(String::new())
}

// Form validation
async fn validate_form_field(
    State(database): State<Database>,
    Path(form_type): Path<String>,
    Form(validation): Form<ValidationRequest>,
) -> Html<String> {
    Html::from(String::new())
}

pub fn router() -> Router<Database> {
    Router::new()
        // Selector Endpoints
        .route("/api/selectors/departments", get(department_selector))
        .route("/api/selectors/areas", get(area_selector))
        .route("/api/selectors/areas-by-department/:id", get(areas_by_department))
        .route("/api/selectors/clients", get(client_selector))
        .route("/api/selectors/workers", get(worker_selector))
        .route("/api/selectors/brigades", get(brigade_selector))
        .route("/api/selectors/sites", get(site_selector))
        .route("/api/selectors/materials", get(material_selector))
        // Validation Endpoints
        .route("/api/validation/form/:form_type", get(validate_form_field))
}
