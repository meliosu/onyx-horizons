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

// Additional types for templates
#[derive(Serialize, Deserialize, Clone)]
pub struct Department {
    pub id: i32,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Site {
    pub id: i32,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Brigade {
    pub id: i32,
    pub brigadier_name: String,
}

// Helper trait for computed values
pub trait ReportUtils {
    fn current_timestamp(&self) -> String {
        Utc::now().format("%Y-%m-%d %H:%M:%S").to_string()
    }
}

// Template types for Report pages
#[derive(Template)]
#[template(path = "reports/departments.html")]
pub struct DepartmentsReportsPageTemplate {
    pub departments: Vec<Department>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
}

#[derive(Template)]
#[template(path = "reports/sites.html")]
pub struct SitesReportsPageTemplate {
    pub sites: Vec<Site>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
}

#[derive(Template)]
#[template(path = "reports/brigades.html")]
pub struct BrigadesReportsPageTemplate {
    pub brigades: Vec<Brigade>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
}

// Template types for Report components
#[derive(Template)]
#[template(path = "reports/components/department_sites_report.html")]
pub struct DepartmentSitesReportTemplate {
    pub department_id: i32,
    pub department_name: String,
    pub items: Vec<DepartmentSiteReportItem>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub current_page: usize,
    pub total_pages: usize,
    // Add field for timestamp
    pub current_timestamp: String,
}

#[derive(Template)]
#[template(path = "reports/components/department_equipment_report.html")]
pub struct DepartmentEquipmentReportTemplate {
    pub department_id: i32,
    pub department_name: String,
    pub items: Vec<DepartmentEquipmentReportItem>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub current_page: usize,
    pub total_pages: usize,
    // Add field for timestamp
    pub current_timestamp: String,
}

impl DepartmentEquipmentReportTemplate {
    pub fn new(
        department_id: i32,
        department_name: String,
        items: Vec<DepartmentEquipmentReportItem>,
        start_date: Option<NaiveDate>,
        end_date: Option<NaiveDate>,
        current_page: usize,
        total_pages: usize,
    ) -> Self {
        Self {
            department_id,
            department_name,
            items,
            start_date,
            end_date,
            current_page,
            total_pages,
            current_timestamp: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        }
    }
}

#[derive(Template)]
#[template(path = "reports/components/department_tasks_report.html")]
pub struct DepartmentTasksReportTemplate {
    pub department_id: i32,
    pub department_name: String,
    pub items: Vec<DepartmentTaskReportItem>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub current_page: usize,
    pub total_pages: usize,
    // Add field for timestamp
    pub current_timestamp: String,
}

#[derive(Template)]
#[template(path = "reports/components/department_materials_report.html")]
pub struct DepartmentMaterialsReportTemplate {
    pub department_id: i32,
    pub department_name: String,
    pub items: Vec<DepartmentMaterialReportItem>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub current_page: usize,
    pub total_pages: usize,
    pub current_timestamp: String,
    pub total_materials_cost: f64,
    pub materials_with_excess_count: usize,
    pub sites_count_total: i32,
}

#[derive(Template)]
#[template(path = "reports/components/site_schedule_report.html")]
pub struct SiteScheduleReportTemplate {
    pub site_id: i32,
    pub site_name: String,
    pub items: Vec<SiteScheduleReportItem>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub current_page: usize,
    pub total_pages: usize,
    // Add field for timestamp
    pub current_timestamp: String,
}

impl SiteScheduleReportTemplate {
    // Add the task_position method to calculate vertical position
    pub fn task_position(&self, task_index: usize) -> String {
        format!("{}px", task_index * 32)
    }
}

#[derive(Template)]
#[template(path = "reports/components/site_materials_report.html")]
pub struct SiteMaterialsReportTemplate {
    pub site_id: i32,
    pub site_name: String,
    pub items: Vec<SiteMaterialReportItem>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub current_page: usize,
    pub total_pages: usize,
    // Add fields for computed values
    pub current_timestamp: String,
    pub total_materials_cost: f64,
    pub materials_with_excess_count: usize,
    pub materials_under_budget_count: usize,
}

#[derive(Template)]
#[template(path = "reports/components/site_equipment_report.html")]
pub struct SiteEquipmentReportTemplate {
    pub site_id: i32,
    pub site_name: String,
    pub items: Vec<SiteEquipmentReportItem>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub current_page: usize,
    pub total_pages: usize,
    // Add fields for computed values
    pub current_timestamp: String, 
    pub total_equipment_units: i32,
    pub unique_equipment_types: usize,
    pub average_days_allocated: i32,
}

#[derive(Template)]
#[template(path = "reports/components/site_brigades_report.html")]
pub struct SiteBrigadesReportTemplate {
    pub site_id: i32,
    pub site_name: String,
    pub items: Vec<SiteBrigadeReportItem>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub current_page: usize,
    pub total_pages: usize,
    // Add fields for computed values
    pub current_timestamp: String,
    pub total_brigades: usize,
    pub total_workers: i32,
    pub total_completed_tasks: i32,
}

#[derive(Template)]
#[template(path = "reports/components/site_construction_report.html")]
pub struct SiteConstructionReportTemplate {
    pub site_id: i32,
    pub site_name: String,
    pub item: SiteConstructionReportItem,
    // Add field for timestamp
    pub current_timestamp: String,
}

impl SiteConstructionReportTemplate {
    pub fn new(
        site_id: i32,
        site_name: String,
        item: SiteConstructionReportItem,
    ) -> Self {
        Self {
            site_id,
            site_name,
            item,
            current_timestamp: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        }
    }
    
    pub fn cost_efficiency(&self) -> String {
        if self.item.total_days > 0 {
            format!("${:.2} per day", self.item.total_materials_cost / self.item.total_days as f64)
        } else {
            "N/A".to_string()
        }
    }
    
    pub fn labor_efficiency(&self) -> String {
        if self.item.total_workers_involved > 0 {
            format!("{:.2}% per worker", 
                self.item.completed_percentage / self.item.total_workers_involved as f64)
        } else {
            "N/A".to_string()
        }
    }
    
    pub fn equipment_utilization(&self) -> String {
        if self.item.total_equipment_allocations > 0 {
            format!("{:.2} days per allocation", 
                self.item.total_days as f64 / self.item.total_equipment_allocations as f64)
        } else {
            "N/A".to_string()
        }
    }
}

#[derive(Template)]
#[template(path = "reports/components/brigade_tasks_report.html")]
pub struct BrigadeTasksReportTemplate {
    pub brigade_id: i32,
    pub brigade_name: String,
    pub items: Vec<BrigadeTaskReportItem>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub current_page: usize,
    pub total_pages: usize,
    // Add fields for computed values
    pub current_timestamp: String,
    pub total_tasks: usize,
    pub completed_tasks: usize,
    pub total_days_worked: i32,
}

#[derive(Template)]
#[template(path = "reports/components/brigade_sites_report.html")]
pub struct BrigadeSitesReportTemplate {
    pub brigade_id: i32,
    pub brigade_name: String,
    pub items: Vec<BrigadeSiteReportItem>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub current_page: usize,
    pub total_pages: usize,
    // Add fields for computed values
    pub current_timestamp: String,
    pub total_sites: usize,
    pub total_tasks: i32,
    pub total_days_worked: i32,
}

#[derive(Template)]
#[template(path = "reports/components/overdue_tasks_report.html")]
pub struct OverdueTasksReportTemplate {
    pub items: Vec<OverdueTaskReportItem>,
    pub current_page: usize,
    pub total_pages: usize,
    // Add fields for computed values
    pub current_timestamp: String,
    pub total_overdue_tasks: usize,
    pub average_days_overdue: i32,
    pub tasks_without_brigade: usize,
}

#[derive(Template)]
#[template(path = "reports/components/material_excesses_report.html")]
pub struct MaterialExcessesReportTemplate {
    pub items: Vec<MaterialExcessReportItem>,
    pub current_page: usize,
    pub total_pages: usize,
    // Add fields for computed values
    pub current_timestamp: String,
    pub total_materials_with_excess: usize,
    pub average_excess_percentage: f64,
    pub total_cost_impact: f64,
}

// Page Endpoints
async fn departments_reports_page(
    State(database): State<Database>,
) -> Html<String> {
    // Department reports page
    // Return rendered DepartmentsReportsPageTemplate
    Html(String::new())
}

async fn sites_reports_page(
    State(database): State<Database>,
) -> Html<String> {
    // Site reports page
    // Return rendered SitesReportsPageTemplate
    Html(String::new())
}

async fn brigades_reports_page(
    State(database): State<Database>,
) -> Html<String> {
    // Brigade reports page
    // Return rendered BrigadesReportsPageTemplate
    Html(String::new())
}

// HTMX Endpoints - Department Reports
async fn department_sites_report(
    State(database): State<Database>,
    Path(id): Path<i32>,
    Query(params): Query<DateRangeParams>,
) -> Html<String> {
    // Generate department sites report
    // Return rendered DepartmentSitesReportTemplate
    Html(String::new())
}

async fn department_equipment_report(
    State(database): State<Database>,
    Path(id): Path<i32>,
    Query(params): Query<DateRangeParams>,
) -> Html<String> {
    // Generate department equipment report
    // Return rendered DepartmentEquipmentReportTemplate
    Html(String::new())
}

async fn department_tasks_report(
    State(database): State<Database>,
    Path(id): Path<i32>,
    Query(params): Query<DateRangeParams>,
) -> Html<String> {
    // Generate department tasks report
    // Return rendered DepartmentTasksReportTemplate
    Html(String::new())
}

async fn department_materials_report(
    State(database): State<Database>,
    Path(id): Path<i32>,
    Query(params): Query<DateRangeParams>,
) -> Html<String> {
    // Generate department materials report
    // Return rendered DepartmentMaterialsReportTemplate
    Html(String::new())
}

// HTMX Endpoints - Site Reports
async fn site_schedule_report(
    State(database): State<Database>,
    Path(id): Path<i32>,
    Query(params): Query<DateRangeParams>,
) -> Html<String> {
    // Generate site schedule report
    // Return rendered SiteScheduleReportTemplate
    Html(String::new())
}

async fn site_materials_report(
    State(database): State<Database>,
    Path(id): Path<i32>,
    Query(params): Query<DateRangeParams>,
) -> Html<String> {
    // Generate site materials report
    // Return rendered SiteMaterialsReportTemplate
    Html(String::new())
}

async fn site_equipment_report(
    State(database): State<Database>,
    Path(id): Path<i32>,
    Query(params): Query<DateRangeParams>,
) -> Html<String> {
    // Generate site equipment report
    // Return rendered SiteEquipmentReportTemplate
    Html(String::new())
}

async fn site_brigades_report(
    State(database): State<Database>,
    Path(id): Path<i32>,
    Query(params): Query<DateRangeParams>,
) -> Html<String> {
    // Generate site brigades report
    // Return rendered SiteBrigadesReportTemplate
    Html(String::new())
}

async fn site_construction_report(
    State(database): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Generate site construction report
    // Return rendered SiteConstructionReportTemplate
    Html(String::new())
}

// HTMX Endpoints - Brigade Reports
async fn brigade_tasks_report(
    State(database): State<Database>,
    Path(id): Path<i32>,
    Query(params): Query<DateRangeParams>,
) -> Html<String> {
    // Generate brigade tasks report
    // Return rendered BrigadeTasksReportTemplate
    Html(String::new())
}

async fn brigade_sites_report(
    State(database): State<Database>,
    Path(id): Path<i32>,
    Query(params): Query<DateRangeParams>,
) -> Html<String> {
    // Generate brigade sites report
    // Return rendered BrigadeSitesReportTemplate
    Html(String::new())
}

// HTMX Endpoints - Specialized Reports
async fn overdue_tasks_report(
    State(database): State<Database>,
    Query(params): Query<PaginationParams>,
) -> Html<String> {
    // Generate overdue tasks report
    // Return rendered OverdueTasksReportTemplate
    Html(String::new())
}

async fn material_excesses_report(
    State(database): State<Database>,
    Query(params): Query<PaginationParams>,
) -> Html<String> {
    // Generate material excesses report
    // Return rendered MaterialExcessesReportTemplate
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
