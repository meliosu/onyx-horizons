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
use crate::sites::Site;
use crate::brigades::Brigade;

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
    pub site_id: Option<i32>,
    pub site_name: Option<String>,
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

// Template types for Task pages
#[derive(Template)]
#[template(path = "tasks/index.html")]
pub struct TasksPageTemplate {
    pub tasks: Vec<Task>,
    pub filter: Option<TaskFilter>,
    pub current_page: usize,
    pub total_pages: usize,
    pub sites: Vec<Site>,
    pub brigades: Vec<Brigade>,
}

#[derive(Template)]
#[template(path = "tasks/new.html")]
pub struct TaskNewPageTemplate {
    pub sites: Vec<Site>,
    pub brigades: Vec<Brigade>,
    pub site_id: Option<i32>,
}

#[derive(Template)]
#[template(path = "tasks/details.html")]
pub struct TaskDetailsPageTemplate {
    pub task: Task,
}

#[derive(Template)]
#[template(path = "tasks/edit.html")]
pub struct TaskEditPageTemplate {
    pub task: Task,
    pub sites: Vec<Site>,
    pub brigades: Vec<Brigade>,
}

// Template types for Material pages
#[derive(Template)]
#[template(path = "materials/index.html")]
pub struct MaterialsPageTemplate {
    pub materials: Vec<Material>,
    pub filter: Option<MaterialFilter>,
    pub current_page: usize,
    pub total_pages: usize,
}

#[derive(Template)]
#[template(path = "materials/new.html")]
pub struct MaterialNewPageTemplate {}

#[derive(Template)]
#[template(path = "materials/details.html")]
pub struct MaterialDetailsPageTemplate {
    pub material: Material,
}

#[derive(Template)]
#[template(path = "materials/edit.html")]
pub struct MaterialEditPageTemplate {
    pub material: Material,
}

// Template types for HTMX components - Tasks
#[derive(Template)]
#[template(path = "tasks/components/task_rows.html")]
pub struct TaskRowsTemplate {
    pub tasks: Vec<Task>,
}

#[derive(Template)]
#[template(path = "tasks/components/task_details.html")]
pub struct TaskDetailsTemplate {
    pub task: Task,
}

#[derive(Template)]
#[template(path = "tasks/components/task_materials.html")]
pub struct TaskMaterialsTemplate {
    pub task_id: i32,
    pub expenditures: Vec<Expenditure>,
    pub available_materials: Vec<Material>,
    pub current_page: usize,
    pub total_pages: usize,
}

#[derive(Template)]
#[template(path = "tasks/components/complete_task_form.html")]
pub struct CompleteTaskFormTemplate {
    pub task_id: i32,
}

// Template types for HTMX components - Materials
#[derive(Template)]
#[template(path = "materials/components/material_rows.html")]
pub struct MaterialRowsTemplate {
    pub materials: Vec<Material>,
}

#[derive(Template)]
#[template(path = "materials/components/material_details.html")]
pub struct MaterialDetailsTemplate {
    pub material: Material,
}

#[derive(Template)]
#[template(path = "materials/components/material_usage.html")]
pub struct MaterialUsageTemplate {
    pub material_id: i32,
    pub expenditures: Vec<Expenditure>,
    pub current_page: usize,
    pub total_pages: usize,
}

#[derive(Template)]
#[template(path = "components/success_notification.html")]
pub struct SuccessNotificationTemplate {
    pub message: String,
}

// Page Endpoints - Tasks
async fn tasks_page(
    State(database): State<Database>,
    Query(params): Query<TaskFilter>,
) -> Html<String> {
    // Tasks listing page
    // Return rendered TasksPageTemplate
    Html(String::new())
}

async fn task_new_page(
    State(database): State<Database>,
) -> Html<String> {
    // New task form
    // Return rendered TaskNewPageTemplate
    Html(String::new())
}

async fn task_details_page(
    State(database): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Task details page
    // Return rendered TaskDetailsPageTemplate
    Html(String::new())
}

async fn task_edit_page(
    State(database): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Edit task form
    // Return rendered TaskEditPageTemplate
    Html(String::new())
}

// Page Endpoints - Materials
async fn materials_page(
    State(database): State<Database>,
    Query(params): Query<MaterialFilter>,
) -> Html<String> {
    // Materials listing page
    // Return rendered MaterialsPageTemplate
    Html(String::new())
}

async fn material_new_page(
    State(database): State<Database>,
) -> Html<String> {
    // New material form
    // Return rendered MaterialNewPageTemplate
    Html(String::new())
}

async fn material_details_page(
    State(database): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Material details page
    // Return rendered MaterialDetailsPageTemplate
    Html(String::new())
}

async fn material_edit_page(
    State(database): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Edit material form
    // Return rendered MaterialEditPageTemplate
    Html(String::new())
}

// HTMX Endpoints - Tasks
async fn fetch_tasks(
    State(database): State<Database>,
    Query(params): Query<TaskFilter>,
) -> Html<String> {
    // Fetch tasks with filters
    // Return rendered TaskRowsTemplate
    Html(String::new())
}

async fn create_task(
    State(database): State<Database>,
    Form(task): Form<TaskCreate>,
) -> Html<String> {
    // Create new task
    // Return rendered SuccessNotificationTemplate
    Html(String::new())
}

async fn fetch_task_details(
    State(database): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Fetch task details
    // Return rendered TaskDetailsTemplate
    Html(String::new())
}

async fn update_task(
    State(database): State<Database>,
    Path(id): Path<i32>,
    Form(task): Form<TaskUpdate>,
) -> Html<String> {
    // Update task
    // Return rendered TaskDetailsTemplate
    Html(String::new())
}

async fn delete_task(
    State(database): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Delete task
    // Return rendered SuccessNotificationTemplate
    Html(String::new())
}

async fn complete_task(
    State(database): State<Database>,
    Path(id): Path<i32>,
    Form(task): Form<TaskCompleteUpdate>,
) -> Html<String> {
    // Mark task as completed
    // Return rendered TaskDetailsTemplate
    Html(String::new())
}

async fn fetch_task_materials(
    State(database): State<Database>,
    Path(id): Path<i32>,
    Query(params): Query<PaginationParams>,
) -> Html<String> {
    // Fetch materials for task
    // Return rendered TaskMaterialsTemplate
    Html(String::new())
}

async fn add_material_to_task(
    State(database): State<Database>,
    Path(id): Path<i32>,
    Form(expenditure): Form<ExpenditureCreate>,
) -> Html<String> {
    // Add material to task
    // Return rendered SuccessNotificationTemplate
    Html(String::new())
}

async fn update_task_material(
    State(database): State<Database>,
    Path((task_id, material_id)): Path<(i32, i32)>,
    Form(expenditure): Form<ExpenditureUpdate>,
) -> Html<String> {
    // Update material usage for task
    // Return rendered SuccessNotificationTemplate
    Html(String::new())
}

async fn remove_material_from_task(
    State(database): State<Database>,
    Path((task_id, material_id)): Path<(i32, i32)>,
) -> Html<String> {
    // Remove material from task
    // Return rendered SuccessNotificationTemplate
    Html(String::new())
}

async fn fetch_tasks_by_site(
    State(database): State<Database>,
    Path(site_id): Path<i32>,
    Query(params): Query<PaginationParams>,
) -> Html<String> {
    // Fetch tasks by site
    // Return rendered TaskRowsTemplate
    Html(String::new())
}

async fn fetch_tasks_by_brigade(
    State(database): State<Database>,
    Path(brigade_id): Path<i32>,
    Query(params): Query<PaginationParams>,
) -> Html<String> {
    // Fetch tasks by brigade
    // Return rendered TaskRowsTemplate
    Html(String::new())
}

async fn fetch_overdue_tasks(
    State(database): State<Database>,
    Query(params): Query<PaginationParams>,
) -> Html<String> {
    // Fetch overdue tasks
    // Return rendered TaskRowsTemplate
    Html(String::new())
}

// HTMX Endpoints - Materials
async fn fetch_materials(
    State(database): State<Database>,
    Query(params): Query<MaterialFilter>,
) -> Html<String> {
    // Fetch materials with filters
    // Return rendered MaterialRowsTemplate
    Html(String::new())
}

async fn create_material(
    State(database): State<Database>,
    Form(material): Form<MaterialCreate>,
) -> Html<String> {
    // Create new material
    // Return rendered SuccessNotificationTemplate
    Html(String::new())
}

async fn fetch_material_details(
    State(database): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Fetch material details
    // Return rendered MaterialDetailsTemplate
    Html(String::new())
}

async fn update_material(
    State(database): State<Database>,
    Path(id): Path<i32>,
    Form(material): Form<MaterialUpdate>,
) -> Html<String> {
    // Update material
    // Return rendered MaterialDetailsTemplate
    Html(String::new())
}

async fn delete_material(
    State(database): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Delete material
    // Return rendered SuccessNotificationTemplate
    Html(String::new())
}

async fn fetch_materials_with_excesses(
    State(database): State<Database>,
    Query(params): Query<PaginationParams>,
) -> Html<String> {
    // Fetch materials with estimate excesses
    // Return rendered MaterialRowsTemplate
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
