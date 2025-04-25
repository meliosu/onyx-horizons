use axum::{
    extract::{Path, Query, State},
    response::Html,
    routing::{get, post, put, delete},
    Form, Router,
};
use serde::{Deserialize, Serialize};
use chrono::NaiveDate;
use askama::Template;

use crate::database::Database;
use crate::general::PaginationParams;
use crate::brigades::Brigade;
use crate::sites::Site;

// Task types
#[derive(Serialize, Deserialize)]
pub struct Task {
    pub id: i32,
    pub site_id: i32,
    pub brigade_id: Option<i32>,
    pub period_start: NaiveDate,
    pub expected_period_end: NaiveDate,
    pub actual_period_end: Option<NaiveDate>,
    pub name: String,
    pub description: Option<String>,
    // Joined fields for display
    pub site_name: Option<String>,
    pub brigade_name: Option<String>,
    pub completion_percentage: Option<i32>,
    pub is_completed: bool,
    pub is_overdue: bool,
    pub materials_count: Option<i32>,
}

// Material types
#[derive(Serialize, Deserialize)]
pub struct Material {
    pub id: i32,
    pub name: String,
    pub cost: f64,
    pub units: String,
    // Additional fields for UI display
    pub total_usage: Option<f64>,
    pub tasks_using_count: Option<i32>,
}

// Expenditure (task-material relationship)
#[derive(Serialize, Deserialize)]
pub struct Expenditure {
    pub task_id: i32,
    pub material_id: i32,
    pub expected_amount: f64,
    pub actual_amount: Option<f64>,
    // Joined fields for display
    pub material_name: Option<String>,
    pub material_cost: Option<f64>,
    pub material_units: Option<String>,
    pub excess_amount: Option<f64>,
    pub excess_percentage: Option<f64>,
}

// Create and update types
#[derive(Deserialize)]
pub struct TaskCreate {
    pub site_id: i32,
    pub brigade_id: Option<i32>,
    pub period_start: NaiveDate,
    pub expected_period_end: NaiveDate,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Deserialize)]
pub struct TaskUpdate {
    pub site_id: i32,
    pub brigade_id: Option<i32>,
    pub period_start: NaiveDate,
    pub expected_period_end: NaiveDate,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Deserialize)]
pub struct TaskCompleteUpdate {
    pub actual_period_end: NaiveDate,
}

#[derive(Deserialize)]
pub struct MaterialCreate {
    pub name: String,
    pub cost: f64,
    pub units: String,
}

#[derive(Deserialize)]
pub struct MaterialUpdate {
    pub name: String,
    pub cost: f64,
    pub units: String,
}

#[derive(Deserialize)]
pub struct ExpenditureCreate {
    pub material_id: i32,
    pub expected_amount: f64,
}

#[derive(Deserialize)]
pub struct ExpenditureUpdate {
    pub expected_amount: f64,
    pub actual_amount: Option<f64>,
}

// Filter types
#[derive(Deserialize)]
pub struct TaskFilter {
    pub site_id: Option<i32>,
    pub brigade_id: Option<i32>,
    pub completed: Option<bool>,
    pub overdue: Option<bool>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    #[serde(flatten)]
    pub pagination: PaginationParams,
}

#[derive(Deserialize)]
pub struct MaterialFilter {
    pub name: Option<String>,
    pub with_excesses: Option<bool>,
    #[serde(flatten)]
    pub pagination: PaginationParams,
}

// Template structs
#[derive(Template)]
#[template(path = "tasks/index.html")]
struct TasksTemplate {
    tasks: Vec<Task>,
    filters: TaskFilter,
    sites: Vec<Site>,
    brigades: Vec<Brigade>,
    total_pages: usize,
    current_page: usize,
}

#[derive(Template)]
#[template(path = "tasks/new.html")]
struct TaskNewTemplate {
    sites: Vec<Site>,
    brigades: Vec<Brigade>,
    materials: Vec<Material>,
}

#[derive(Template)]
#[template(path = "tasks/details.html")]
struct TaskDetailsTemplate {
    task: Task,
    site: Site,
    brigade: Option<Brigade>,
    materials: Vec<Expenditure>,
}

#[derive(Template)]
#[template(path = "tasks/edit.html")]
struct TaskEditTemplate {
    task: Task,
    sites: Vec<Site>,
    brigades: Vec<Brigade>,
}

#[derive(Template)]
#[template(path = "materials/index.html")]
struct MaterialsTemplate {
    materials: Vec<Material>,
    filters: MaterialFilter,
    total_pages: usize,
    current_page: usize,
}

#[derive(Template)]
#[template(path = "materials/new.html")]
struct MaterialNewTemplate {}

#[derive(Template)]
#[template(path = "materials/details.html")]
struct MaterialDetailsTemplate {
    material: Material,
    tasks: Vec<Expenditure>,
}

#[derive(Template)]
#[template(path = "materials/edit.html")]
struct MaterialEditTemplate {
    material: Material,
}

// HTMX Component templates
#[derive(Template)]
#[template(path = "components/tasks/table_rows.html")]
struct TaskRowsTemplate {
    tasks: Vec<Task>,
    current_page: usize,
    total_pages: usize,
}

#[derive(Template)]
#[template(path = "components/materials/table_rows.html")]
struct MaterialRowsTemplate {
    materials: Vec<Material>,
    current_page: usize,
    total_pages: usize,
}

#[derive(Template)]
#[template(path = "components/tasks/materials.html")]
struct TaskMaterialsTemplate {
    materials: Vec<Expenditure>,
    task_id: i32,
    available_materials: Vec<Material>,
    current_page: usize,
    total_pages: usize,
}

#[derive(Template)]
#[template(path = "components/materials/tasks.html")]
struct MaterialTasksTemplate {
    tasks: Vec<Expenditure>,
    material_id: i32,
    current_page: usize,
    total_pages: usize,
}

#[derive(Template)]
#[template(path = "components/tasks/expenditure_form.html")]
struct ExpenditureFormTemplate {
    task_id: i32,
    material_id: i32,
    expenditure: Option<Expenditure>,
}

// Page Endpoints - Tasks
async fn tasks_page(
    State(database): State<Database>,
    Query(params): Query<TaskFilter>,
) -> Html<String> {
    // Tasks listing page
    Html(String::new())
}

async fn task_new_page(
    State(database): State<Database>,
) -> Html<String> {
    // New task form
    Html(String::new())
}

async fn task_details_page(
    State(database): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Task details page
    Html(String::new())
}

async fn task_edit_page(
    State(database): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Edit task form
    Html(String::new())
}

// Page Endpoints - Materials
async fn materials_page(
    State(database): State<Database>,
    Query(params): Query<MaterialFilter>,
) -> Html<String> {
    // Materials listing page
    Html(String::new())
}

async fn material_new_page(
    State(database): State<Database>,
) -> Html<String> {
    // New material form
    Html(String::new())
}

async fn material_details_page(
    State(database): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Material details page
    Html(String::new())
}

async fn material_edit_page(
    State(database): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Edit material form
    Html(String::new())
}

// HTMX Endpoints - Tasks
async fn fetch_tasks(
    State(database): State<Database>,
    Query(params): Query<TaskFilter>,
) -> Html<String> {
    // Fetch tasks with filters
    Html(String::new())
}

async fn create_task(
    State(database): State<Database>,
    Form(task): Form<TaskCreate>,
) -> Html<String> {
    // Create new task
    Html(String::new())
}

async fn fetch_task_details(
    State(database): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Fetch task details
    Html(String::new())
}

async fn update_task(
    State(database): State<Database>,
    Path(id): Path<i32>,
    Form(task): Form<TaskUpdate>,
) -> Html<String> {
    // Update task
    Html(String::new())
}

async fn delete_task(
    State(database): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Delete task
    Html(String::new())
}

async fn complete_task(
    State(database): State<Database>,
    Path(id): Path<i32>,
    Form(task): Form<TaskCompleteUpdate>,
) -> Html<String> {
    // Mark task as completed
    Html(String::new())
}

async fn fetch_task_materials(
    State(database): State<Database>,
    Path(id): Path<i32>,
    Query(params): Query<PaginationParams>,
) -> Html<String> {
    // Fetch materials for task
    Html(String::new())
}

async fn add_material_to_task(
    State(database): State<Database>,
    Path(id): Path<i32>,
    Form(expenditure): Form<ExpenditureCreate>,
) -> Html<String> {
    // Add material to task
    Html(String::new())
}

async fn update_task_material(
    State(database): State<Database>,
    Path((task_id, material_id)): Path<(i32, i32)>,
    Form(expenditure): Form<ExpenditureUpdate>,
) -> Html<String> {
    // Update material usage for task
    Html(String::new())
}

async fn remove_material_from_task(
    State(database): State<Database>,
    Path((task_id, material_id)): Path<(i32, i32)>,
) -> Html<String> {
    // Remove material from task
    Html(String::new())
}

async fn fetch_tasks_by_site(
    State(database): State<Database>,
    Path(site_id): Path<i32>,
    Query(params): Query<PaginationParams>,
) -> Html<String> {
    // Fetch tasks by site
    Html(String::new())
}

async fn fetch_tasks_by_brigade(
    State(database): State<Database>,
    Path(brigade_id): Path<i32>,
    Query(params): Query<PaginationParams>,
) -> Html<String> {
    // Fetch tasks by brigade
    Html(String::new())
}

async fn fetch_overdue_tasks(
    State(database): State<Database>,
    Query(params): Query<PaginationParams>,
) -> Html<String> {
    // Fetch overdue tasks
    Html(String::new())
}

// HTMX Endpoints - Materials
async fn fetch_materials(
    State(database): State<Database>,
    Query(params): Query<MaterialFilter>,
) -> Html<String> {
    // Fetch materials with filters
    Html(String::new())
}

async fn create_material(
    State(database): State<Database>,
    Form(material): Form<MaterialCreate>,
) -> Html<String> {
    // Create new material
    Html(String::new())
}

async fn fetch_material_details(
    State(database): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Fetch material details
    Html(String::new())
}

async fn update_material(
    State(database): State<Database>,
    Path(id): Path<i32>,
    Form(material): Form<MaterialUpdate>,
) -> Html<String> {
    // Update material
    Html(String::new())
}

async fn delete_material(
    State(database): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Delete material
    Html(String::new())
}

async fn fetch_materials_with_excesses(
    State(database): State<Database>,
    Query(params): Query<PaginationParams>,
) -> Html<String> {
    // Fetch materials with estimate excesses
    Html(String::new())
}

pub fn router() -> Router<Database> {
    Router::new()
        // Page Endpoints - Tasks
        .route("/tasks", get(tasks_page))
        .route("/tasks/new", get(task_new_page))
        .route("/tasks/{id}", get(task_details_page))
        .route("/tasks/{id}/edit", get(task_edit_page))
        // Page Endpoints - Materials
        .route("/materials", get(materials_page))
        .route("/materials/new", get(material_new_page))
        .route("/materials/{id}", get(material_details_page))
        .route("/materials/{id}/edit", get(material_edit_page))
        // HTMX Endpoints - Tasks
        .route("/api/tasks", get(fetch_tasks).post(create_task))
        .route("/api/tasks/{id}", get(fetch_task_details).put(update_task).delete(delete_task))
        .route("/api/tasks/{id}/complete", put(complete_task))
        .route("/api/tasks/{id}/materials", get(fetch_task_materials).post(add_material_to_task))
        .route("/api/tasks/{id}/materials/{material_id}", put(update_task_material).delete(remove_material_from_task))
        .route("/api/tasks/by-site/{site_id}", get(fetch_tasks_by_site))
        .route("/api/tasks/by-brigade/{brigade_id}", get(fetch_tasks_by_brigade))
        .route("/api/tasks/overdue", get(fetch_overdue_tasks))
        // HTMX Endpoints - Materials
        .route("/api/materials", get(fetch_materials).post(create_material))
        .route("/api/materials/{id}", get(fetch_material_details).put(update_material).delete(delete_material))
        .route("/api/materials/with-excesses", get(fetch_materials_with_excesses))
}
