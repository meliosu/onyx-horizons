use axum::{
    extract::{Path, Query, State},
    response::Html,
    routing::get,
    Router,
};
use chrono::{NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use askama::Template;

use crate::database::Database;
use crate::general::PaginationParams;
use crate::departments::{Department, Area};
use crate::sites::Site;
use crate::brigades::Brigade;

// Common report types
#[derive(Deserialize)]
pub struct DateRangeParams {
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    #[serde(flatten)]
    pub pagination: PaginationParams,
}

// Department report types
#[derive(Serialize)]
pub struct DepartmentSiteReportItem {
    pub site_id: i32,
    pub site_name: Option<String>,
    pub site_type: String,
    pub area_name: String,
    pub client_name: String,
    pub tasks_count: i32,
    pub completed_tasks_count: i32,
    pub overdue_tasks_count: i32,
    pub materials_cost: f64,
    pub current_brigades_count: i32,
}

#[derive(Serialize)]
pub struct DepartmentEquipmentReportItem {
    pub equipment_id: i32,
    pub equipment_name: String,
    pub total_amount: i32,
    pub allocated_amount: i32,
    pub available_amount: i32,
    pub sites_count: i32,
    pub allocation_rate: f64,  // Percentage of time equipment is allocated
}

#[derive(Serialize)]
pub struct DepartmentTaskReportItem {
    pub task_id: i32,
    pub task_name: String,
    pub site_name: String,
    pub brigade_name: Option<String>,
    pub period_start: NaiveDate,
    pub expected_period_end: NaiveDate,
    pub actual_period_end: Option<NaiveDate>,
    pub is_completed: bool,
    pub is_overdue: bool,
    pub days_overdue: Option<i32>,
    pub materials_cost: f64,
}

#[derive(Serialize)]
pub struct DepartmentMaterialReportItem {
    pub material_id: i32,
    pub material_name: String,
    pub units: String,
    pub total_expected_amount: f64,
    pub total_actual_amount: Option<f64>,
    pub difference_amount: Option<f64>,
    pub difference_percentage: Option<f64>,
    pub total_cost: f64,
    pub sites_count: i32,
    pub tasks_count: i32,
}

// Site report types
#[derive(Serialize)]
pub struct SiteScheduleReportItem {
    pub task_id: i32,
    pub task_name: String,
    pub brigade_name: Option<String>,
    pub period_start: NaiveDate,
    pub expected_period_end: NaiveDate,
    pub actual_period_end: Option<NaiveDate>,
    pub is_completed: bool,
    pub is_on_schedule: bool,
    pub progress_percentage: Option<i32>,
}

#[derive(Serialize)]
pub struct SiteMaterialReportItem {
    pub material_id: i32,
    pub material_name: String,
    pub units: String,
    pub task_name: String,
    pub expected_amount: f64,
    pub actual_amount: Option<f64>,
    pub difference_amount: Option<f64>,
    pub difference_percentage: Option<f64>,
    pub cost_per_unit: f64,
    pub total_cost: f64,
}

#[derive(Serialize)]
pub struct SiteEquipmentReportItem {
    pub allocation_id: i32,
    pub equipment_id: i32,
    pub equipment_name: String,
    pub amount: i32,
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
    pub total_days: i32,
    pub fuel_type: Option<String>,
}

#[derive(Serialize)]
pub struct SiteBrigadeReportItem {
    pub brigade_id: i32,
    pub brigadier_name: String,
    pub workers_count: i32,
    pub tasks_count: i32,
    pub tasks_completed_count: i32,
    pub first_task_date: Option<NaiveDate>,
    pub last_task_date: Option<NaiveDate>,
    pub total_days_worked: i32,
}

#[derive(Serialize)]
pub struct SiteConstructionReportItem {
    pub start_date: Option<NaiveDate>,
    pub expected_end_date: Option<NaiveDate>,
    pub actual_end_date: Option<NaiveDate>,
    pub total_days: i32,
    pub completed_percentage: f64,
    pub total_brigades_involved: i32,
    pub total_workers_involved: i32,
    pub total_materials_cost: f64,
    pub total_equipment_allocations: i32,
}

// Brigade report types
#[derive(Serialize)]
pub struct BrigadeTaskReportItem {
    pub task_id: i32,
    pub task_name: String,
    pub site_name: String,
    pub site_type: String,
    pub period_start: NaiveDate,
    pub expected_period_end: NaiveDate,
    pub actual_period_end: Option<NaiveDate>,
    pub is_completed: bool,
    pub is_overdue: bool,
    pub days_worked: i32,
    pub materials_count: i32,
}

#[derive(Serialize)]
pub struct BrigadeSiteReportItem {
    pub site_id: i32,
    pub site_name: String,
    pub site_type: String,
    pub client_name: String,
    pub area_name: String,
    pub tasks_count: i32,
    pub first_task_date: Option<NaiveDate>,
    pub last_task_date: Option<NaiveDate>,
    pub total_days_worked: i32,
}

// Specialized report types
#[derive(Serialize)]
pub struct OverdueTaskReportItem {
    pub task_id: i32,
    pub task_name: String,
    pub site_name: String,
    pub site_type: String,
    pub area_name: String,
    pub department_name: String,
    pub brigade_name: Option<String>,
    pub expected_period_end: NaiveDate,
    pub days_overdue: i32,
    pub materials_allocated: bool,
    pub brigade_assigned: bool,
}

#[derive(Serialize)]
pub struct MaterialExcessReportItem {
    pub material_id: i32,
    pub material_name: String,
    pub units: String,
    pub site_name: String,
    pub task_name: String,
    pub expected_amount: f64,
    pub actual_amount: f64,
    pub difference_amount: f64,
    pub difference_percentage: f64,
    pub cost_impact: f64,
}

// Report parameter types
#[derive(Deserialize)]
pub struct ReportParams {
    pub report_type: String,
    #[serde(flatten)]
    pub date_range: DateRangeParams,
}

// Template structs for report pages
#[derive(Template)]
#[template(path = "reports/departments.html")]
struct DepartmentsReportsTemplate {
    departments: Vec<Department>,
    selected_department_id: Option<i32>,
    selected_department: Option<Department>,
    selected_report_type: Option<String>,
    start_date: Option<String>,
    end_date: Option<String>,
    report_data: Option<ReportData>,
}

#[derive(Template)]
#[template(path = "reports/sites.html")]
struct SitesReportsTemplate {
    sites: Vec<Site>,
    selected_site_id: Option<i32>,
    selected_site: Option<Site>,
    selected_report_type: Option<String>,
    start_date: Option<String>,
    end_date: Option<String>,
    report_data: Option<ReportData>,
}

#[derive(Template)]
#[template(path = "reports/brigades.html")]
struct BrigadesReportsTemplate {
    brigades: Vec<Brigade>,
    selected_brigade_id: Option<i32>,
    selected_brigade: Option<Brigade>,
    start_date: Option<String>,
    end_date: Option<String>,
    report_data: Option<ReportData>,
}

// Template structs for specific reports
#[derive(Template)]
#[template(path = "reports/components/department_sites_report.html")]
struct DepartmentSitesReportTemplate {
    department_id: i32,
    department_name: String,
    sites: Vec<DepartmentSiteReportItem>,
    start_date: Option<String>,
    end_date: Option<String>,
}

#[derive(Template)]
#[template(path = "reports/components/department_equipment_report.html")]
struct DepartmentEquipmentReportTemplate {
    department_id: i32,
    department_name: String,
    equipment: Vec<DepartmentEquipmentReportItem>,
    start_date: Option<String>,
    end_date: Option<String>,
}

#[derive(Template)]
#[template(path = "reports/components/department_tasks_report.html")]
struct DepartmentTasksReportTemplate {
    department_id: i32,
    department_name: String,
    tasks: Vec<DepartmentTaskReportItem>,
    start_date: Option<String>,
    end_date: Option<String>,
}

#[derive(Template)]
#[template(path = "reports/components/department_materials_report.html")]
struct DepartmentMaterialsReportTemplate {
    department_id: i32,
    department_name: String,
    materials: Vec<DepartmentMaterialReportItem>,
    start_date: Option<String>,
    end_date: Option<String>,
}

#[derive(Template)]
#[template(path = "reports/components/site_schedule_report.html")]
struct SiteScheduleReportTemplate {
    site_id: i32,
    site_name: String,
    schedule_items: Vec<SiteScheduleReportItem>,
    start_date: Option<String>,
    end_date: Option<String>,
}

#[derive(Template)]
#[template(path = "reports/components/site_materials_report.html")]
struct SiteMaterialsReportTemplate {
    site_id: i32,
    site_name: String,
    materials: Vec<SiteMaterialReportItem>,
    start_date: Option<String>,
    end_date: Option<String>,
}

#[derive(Template)]
#[template(path = "reports/components/site_equipment_report.html")]
struct SiteEquipmentReportTemplate {
    site_id: i32,
    site_name: String,
    equipment: Vec<SiteEquipmentReportItem>,
    start_date: Option<String>,
    end_date: Option<String>,
}

#[derive(Template)]
#[template(path = "reports/components/site_brigades_report.html")]
struct SiteBrigadesReportTemplate {
    site_id: i32,
    site_name: String,
    brigades: Vec<SiteBrigadeReportItem>,
    start_date: Option<String>,
    end_date: Option<String>,
}

#[derive(Template)]
#[template(path = "reports/components/site_construction_report.html")]
struct SiteConstructionReportTemplate {
    site_id: i32,
    site_name: String,
    report: SiteConstructionReportItem,
}

#[derive(Template)]
#[template(path = "reports/components/brigade_tasks_report.html")]
struct BrigadeTasksReportTemplate {
    brigade_id: i32,
    brigade_name: String,
    tasks: Vec<BrigadeTaskReportItem>,
    start_date: Option<String>,
    end_date: Option<String>,
}

#[derive(Template)]
#[template(path = "reports/components/brigade_sites_report.html")]
struct BrigadeSitesReportTemplate {
    brigade_id: i32,
    brigade_name: String,
    sites: Vec<BrigadeSiteReportItem>,
    start_date: Option<String>,
    end_date: Option<String>,
}

#[derive(Template)]
#[template(path = "reports/components/overdue_tasks_report.html")]
struct OverdueTasksReportTemplate {
    tasks: Vec<OverdueTaskReportItem>,
    total_count: usize,
    current_page: usize,
    total_pages: usize,
}

#[derive(Template)]
#[template(path = "reports/components/material_excesses_report.html")]
struct MaterialExcessesReportTemplate {
    materials: Vec<MaterialExcessReportItem>,
    total_cost_impact: f64,
    total_count: usize,
    current_page: usize,
    total_pages: usize,
}

// Union type for report data
#[derive(Serialize)]
#[serde(tag = "type")]
pub enum ReportData {
    DepartmentSites(Vec<DepartmentSiteReportItem>),
    DepartmentEquipment(Vec<DepartmentEquipmentReportItem>),
    DepartmentTasks(Vec<DepartmentTaskReportItem>),
    DepartmentMaterials(Vec<DepartmentMaterialReportItem>),
    SiteSchedule(Vec<SiteScheduleReportItem>),
    SiteMaterials(Vec<SiteMaterialReportItem>),
    SiteEquipment(Vec<SiteEquipmentReportItem>),
    SiteBrigades(Vec<SiteBrigadeReportItem>),
    SiteConstruction(SiteConstructionReportItem),
    BrigadeTasks(Vec<BrigadeTaskReportItem>),
    BrigadeSites(Vec<BrigadeSiteReportItem>),
    OverdueTasks(Vec<OverdueTaskReportItem>),
    MaterialExcesses(Vec<MaterialExcessReportItem>),
}

// Page Endpoints
async fn departments_reports_page(
    State(database): State<Database>,
    Query(params): Query<ReportParams>,
) -> Html<String> {
    // Department reports page
    Html(String::new())
}

async fn sites_reports_page(
    State(database): State<Database>,
    Query(params): Query<ReportParams>,
) -> Html<String> {
    // Site reports page
    Html(String::new())
}

async fn brigades_reports_page(
    State(database): State<Database>,
    Query(params): Query<DateRangeParams>,
) -> Html<String> {
    // Brigade reports page
    Html(String::new())
}

// HTMX Endpoints - Department Reports
async fn department_sites_report(
    State(database): State<Database>,
    Path(id): Path<i32>,
    Query(params): Query<DateRangeParams>,
) -> Html<String> {
    // Generate department sites report
    Html(String::new())
}

async fn department_equipment_report(
    State(database): State<Database>,
    Path(id): Path<i32>,
    Query(params): Query<DateRangeParams>,
) -> Html<String> {
    // Generate department equipment report
    Html(String::new())
}

async fn department_tasks_report(
    State(database): State<Database>,
    Path(id): Path<i32>,
    Query(params): Query<DateRangeParams>,
) -> Html<String> {
    // Generate department tasks report
    Html(String::new())
}

async fn department_materials_report(
    State(database): State<Database>,
    Path(id): Path<i32>,
    Query(params): Query<DateRangeParams>,
) -> Html<String> {
    // Generate department materials report
    Html(String::new())
}

// HTMX Endpoints - Site Reports
async fn site_schedule_report(
    State(database): State<Database>,
    Path(id): Path<i32>,
    Query(params): Query<DateRangeParams>,
) -> Html<String> {
    // Generate site schedule report
    Html(String::new())
}

async fn site_materials_report(
    State(database): State<Database>,
    Path(id): Path<i32>,
    Query(params): Query<DateRangeParams>,
) -> Html<String> {
    // Generate site materials report
    Html(String::new())
}

async fn site_equipment_report(
    State(database): State<Database>,
    Path(id): Path<i32>,
    Query(params): Query<DateRangeParams>,
) -> Html<String> {
    // Generate site equipment report
    Html(String::new())
}

async fn site_brigades_report(
    State(database): State<Database>,
    Path(id): Path<i32>,
    Query(params): Query<DateRangeParams>,
) -> Html<String> {
    // Generate site brigades report
    Html(String::new())
}

async fn site_construction_report(
    State(database): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Generate site construction report
    Html(String::new())
}

// HTMX Endpoints - Brigade Reports
async fn brigade_tasks_report(
    State(database): State<Database>,
    Path(id): Path<i32>,
    Query(params): Query<DateRangeParams>,
) -> Html<String> {
    // Generate brigade tasks report
    Html(String::new())
}

async fn brigade_sites_report(
    State(database): State<Database>,
    Path(id): Path<i32>,
    Query(params): Query<DateRangeParams>,
) -> Html<String> {
    // Generate brigade sites report
    Html(String::new())
}

// HTMX Endpoints - Specialized Reports
async fn overdue_tasks_report(
    State(database): State<Database>,
    Query(params): Query<PaginationParams>,
) -> Html<String> {
    // Generate overdue tasks report
    Html(String::new())
}

async fn material_excesses_report(
    State(database): State<Database>,
    Query(params): Query<PaginationParams>,
) -> Html<String> {
    // Generate material excesses report
    Html(String::new())
}

pub fn router() -> Router<Database> {
    Router::new()
        // Page Endpoints
        .route("/reports/departments", get(departments_reports_page))
        .route("/reports/sites", get(sites_reports_page))
        .route("/reports/brigades", get(brigades_reports_page))
        // HTMX Endpoints - Department Reports
        .route("/api/reports/departments/{id}/sites", get(department_sites_report))
        .route("/api/reports/departments/{id}/equipment", get(department_equipment_report))
        .route("/api/reports/departments/{id}/tasks", get(department_tasks_report))
        .route("/api/reports/departments/{id}/materials", get(department_materials_report))
        // HTMX Endpoints - Site Reports
        .route("/api/reports/sites/{id}/schedule", get(site_schedule_report))
        .route("/api/reports/sites/{id}/materials", get(site_materials_report))
        .route("/api/reports/sites/{id}/equipment", get(site_equipment_report))
        .route("/api/reports/sites/{id}/brigades", get(site_brigades_report))
        .route("/api/reports/sites/{id}/construction", get(site_construction_report))
        // HTMX Endpoints - Brigade Reports
        .route("/api/reports/brigades/{id}/tasks", get(brigade_tasks_report))
        .route("/api/reports/brigades/{id}/sites", get(brigade_sites_report))
        // HTMX Endpoints - Specialized Reports
        .route("/api/reports/tasks/overdue", get(overdue_tasks_report))
        .route("/api/reports/materials/excesses", get(material_excesses_report))
}
