use axum::{
    extract::{Path, Query, State},
    response::Html,
    routing::{get, post, put, delete},
    Form, Router,
};
use serde::{Deserialize, Serialize};

use crate::database::Database;
use crate::general::PaginationParams;
use crate::personnel::Worker;

// Brigade types
#[derive(Serialize, Deserialize)]
pub struct Brigade {
    pub id: i32,
    pub brigadier_id: i32,
    // Joined fields for display
    pub brigadier_name: Option<String>,
    pub workers_count: Option<i32>,
    pub active_tasks_count: Option<i32>,
    pub completed_tasks_count: Option<i32>,
    pub current_site_id: Option<i32>,
    pub current_site_name: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct BrigadeWorker {
    pub brigade_id: i32,
    pub worker_id: i32,
    pub worker: Worker,
    pub is_brigadier: bool,
}

// Create and update types
#[derive(Deserialize)]
pub struct BrigadeCreate {
    pub brigadier_id: i32,
    // Workers will be added separately via the workers endpoint
}

#[derive(Deserialize)]
pub struct BrigadeUpdate {
    pub brigadier_id: i32,
}

#[derive(Deserialize)]
pub struct WorkerAssignment {
    pub worker_id: i32,
}

// Filter types
#[derive(Deserialize)]
pub struct BrigadeFilter {
    pub brigadier_id: Option<i32>,
    pub has_active_tasks: Option<bool>,
    pub site_id: Option<i32>,
    #[serde(flatten)]
    pub pagination: PaginationParams,
}

// For UI display - task summary for a brigade
#[derive(Serialize, Deserialize)]
pub struct BrigadeTask {
    pub id: i32,
    pub site_id: i32,
    pub site_name: Option<String>,
    pub period_start: String,
    pub expected_period_end: String,
    pub actual_period_end: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub is_completed: bool,
    pub is_overdue: Option<bool>,
}

// For UI display - site summary for a brigade
#[derive(Serialize, Deserialize)]
pub struct BrigadeSite {
    pub id: i32,
    pub name: Option<String>,
    pub site_type: String,
    pub client_name: Option<String>,
    pub area_name: Option<String>,
    pub tasks_count: i32,
    pub active_tasks_count: i32,
}

// Brigadier selector for forms
#[derive(Serialize, Deserialize)]
pub struct BrigadierOption {
    pub id: i32,
    pub name: String,
    pub profession: String,
}

// Page Endpoints
async fn brigades_page(
    State(database): State<Database>,
    Query(params): Query<BrigadeFilter>,
) -> Html<String> {
    // Brigade listing page
    Html(String::new())
}

async fn brigade_new_page(
    State(database): State<Database>,
) -> Html<String> {
    // New brigade form
    Html(String::new())
}

async fn brigade_details_page(
    State(database): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Brigade details page
    Html(String::new())
}

async fn brigade_edit_page(
    State(database): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Edit brigade form
    Html(String::new())
}

// HTMX Endpoints
async fn fetch_brigades(
    State(database): State<Database>,
    Query(params): Query<BrigadeFilter>,
) -> Html<String> {
    // Fetch brigades with filters
    Html(String::new())
}

async fn create_brigade(
    State(database): State<Database>,
    Form(brigade): Form<BrigadeCreate>,
) -> Html<String> {
    // Create new brigade
    Html(String::new())
}

async fn fetch_brigade_details(
    State(database): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Fetch brigade details
    Html(String::new())
}

async fn update_brigade(
    State(database): State<Database>,
    Path(id): Path<i32>,
    Form(brigade): Form<BrigadeUpdate>,
) -> Html<String> {
    // Update brigade
    Html(String::new())
}

async fn delete_brigade(
    State(database): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Delete brigade
    Html(String::new())
}

async fn fetch_brigade_workers(
    State(database): State<Database>,
    Path(id): Path<i32>,
    Query(params): Query<PaginationParams>,
) -> Html<String> {
    // Fetch workers in brigade
    Html(String::new())
}

async fn add_worker_to_brigade(
    State(database): State<Database>,
    Path(id): Path<i32>,
    Form(assignment): Form<WorkerAssignment>,
) -> Html<String> {
    // Add worker to brigade
    Html(String::new())
}

async fn remove_worker_from_brigade(
    State(database): State<Database>,
    Path((brigade_id, worker_id)): Path<(i32, i32)>,
) -> Html<String> {
    // Remove worker from brigade
    Html(String::new())
}

async fn fetch_brigade_tasks(
    State(database): State<Database>,
    Path(id): Path<i32>,
    Query(params): Query<PaginationParams>,
) -> Html<String> {
    // Fetch tasks performed by brigade
    Html(String::new())
}

async fn fetch_brigade_sites(
    State(database): State<Database>,
    Path(id): Path<i32>,
    Query(params): Query<PaginationParams>,
) -> Html<String> {
    // Fetch sites where brigade worked
    Html(String::new())
}

async fn fetch_brigadiers(
    State(database): State<Database>,
) -> Html<String> {
    // Fetch available brigadiers
    Html(String::new())
}

async fn fetch_brigades_by_task(
    State(database): State<Database>,
    Path(task_id): Path<i32>,
    Query(params): Query<PaginationParams>,
) -> Html<String> {
    // Fetch brigades by task
    Html(String::new())
}

async fn fetch_brigades_by_site(
    State(database): State<Database>,
    Path(site_id): Path<i32>,
    Query(params): Query<PaginationParams>,
) -> Html<String> {
    // Fetch brigades by site
    Html(String::new())
}

pub fn router() -> Router<Database> {
    Router::new()
        // Page Endpoints
        .route("/brigades", get(brigades_page))
        .route("/brigades/new", get(brigade_new_page))
        .route("/brigades/{id}", get(brigade_details_page))
        .route("/brigades/{id}/edit", get(brigade_edit_page))
        // HTMX Endpoints - Brigades
        .route("/api/brigades", get(fetch_brigades).post(create_brigade))
        .route("/api/brigades/{id}", get(fetch_brigade_details).put(update_brigade).delete(delete_brigade))
        .route("/api/brigades/{id}/workers", get(fetch_brigade_workers).post(add_worker_to_brigade))
        .route("/api/brigades/{id}/workers/{worker_id}", delete(remove_worker_from_brigade))
        .route("/api/brigades/{id}/tasks", get(fetch_brigade_tasks))
        .route("/api/brigades/{id}/sites", get(fetch_brigade_sites))
        .route("/api/brigades/brigadiers", get(fetch_brigadiers))
        .route("/api/brigades/by-task/{task_id}", get(fetch_brigades_by_task))
        .route("/api/brigades/by-site/{site_id}", get(fetch_brigades_by_site))
}
