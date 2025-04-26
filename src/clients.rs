use axum::{
    extract::{Path, Query, State},
    response::Html,
    routing::{get, post, put, delete},
    Form, Router,
};
use serde::{Deserialize, Serialize};
use askama::Template;

use crate::database::Database;
use crate::general::PaginationParams;
use crate::sites::SiteType;

// Client types
#[derive(Serialize, Deserialize)]
pub struct Client {
    pub id: i32,
    pub name: String,
    pub inn: String,
    pub address: String,
    pub contact_person_email: String,
    pub contact_person_name: String,
    pub is_vip: bool,
    // Additional fields for UI display
    pub sites_count: Option<i32>, // Number of sites associated with this client
}

// Create and update types
#[derive(Deserialize)]
pub struct ClientCreate {
    pub name: String,
    pub inn: String,
    pub address: String,
    pub contact_person_email: String,
    pub contact_person_name: String,
    pub is_vip: bool,
}

#[derive(Deserialize)]
pub struct ClientUpdate {
    pub name: String,
    pub inn: String,
    pub address: String,
    pub contact_person_email: String,
    pub contact_person_name: String,
    pub is_vip: bool,
}

// Filter types
#[derive(Deserialize)]
pub struct ClientFilter {
    pub name: Option<String>,
    pub inn: Option<String>,
    pub is_vip: Option<bool>,
    #[serde(flatten)]
    pub pagination: PaginationParams,
}

// INN validation response
#[derive(Serialize, Deserialize)]
pub struct InnValidationResponse {
    pub is_unique: bool,
    pub message: Option<String>,
}

// Additional types for template data
#[derive(Serialize, Deserialize, Clone)]
pub struct Site {
    pub id: i32,
    pub name: String,
    pub site_type: SiteType,
    pub risk_level: String,
    pub area_name: Option<String>,
    pub department_name: Option<String>,
}

// Template types for Client pages
#[derive(Template)]
#[template(path = "clients/index.html")]
pub struct ClientsPageTemplate {
    pub clients: Vec<Client>,
    pub filter: Option<ClientFilter>,
    pub current_page: usize,
    pub total_pages: usize,
}

#[derive(Template)]
#[template(path = "clients/new.html")]
pub struct ClientNewPageTemplate {}

#[derive(Template)]
#[template(path = "clients/details.html")]
pub struct ClientDetailsPageTemplate {
    pub client: Client,
    pub active_tab: String, // sites
}

#[derive(Template)]
#[template(path = "clients/edit.html")]
pub struct ClientEditPageTemplate {
    pub client: Client,
}

// Template types for HTMX components
#[derive(Template)]
#[template(path = "clients/components/client_rows.html")]
pub struct ClientRowsTemplate {
    pub clients: Vec<Client>,
}

#[derive(Template)]
#[template(path = "clients/components/client_details.html")]
pub struct ClientDetailsTemplate {
    pub client: Client,
}

#[derive(Template)]
#[template(path = "clients/components/client_sites.html")]
pub struct ClientSitesTemplate {
    pub client_id: i32,
    pub sites: Vec<Site>,
    pub current_page: usize,
    pub total_pages: usize,
}

#[derive(Template)]
#[template(path = "clients/components/inn_validation.html")]
pub struct InnValidationTemplate {
    pub is_unique: bool,
    pub inn: String,
}

#[derive(Template)]
#[template(path = "components/success_notification.html")]
pub struct SuccessNotificationTemplate {
    pub message: String,
}

// Page Endpoints
async fn clients_page(
    State(database): State<Database>,
    Query(params): Query<ClientFilter>,
) -> Html<String> {
    // Client listing page
    // Return rendered ClientsPageTemplate
    Html(String::new())
}

async fn client_new_page(
    State(database): State<Database>,
) -> Html<String> {
    // New client form
    // Return rendered ClientNewPageTemplate
    Html(String::new())
}

async fn client_details_page(
    State(database): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Client details page
    // Return rendered ClientDetailsPageTemplate
    Html(String::new())
}

async fn client_edit_page(
    State(database): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Edit client form
    // Return rendered ClientEditPageTemplate
    Html(String::new())
}

// HTMX Endpoints
async fn fetch_clients(
    State(database): State<Database>,
    Query(params): Query<ClientFilter>,
) -> Html<String> {
    // Fetch clients with filters
    // Return rendered ClientRowsTemplate
    Html(String::new())
}

async fn create_client(
    State(database): State<Database>,
    Form(client): Form<ClientCreate>,
) -> Html<String> {
    // Create new client
    // Return rendered SuccessNotificationTemplate
    Html(String::new())
}

async fn fetch_client_details(
    State(database): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Fetch client details
    // Return rendered ClientDetailsTemplate
    Html(String::new())
}

async fn update_client(
    State(database): State<Database>,
    Path(id): Path<i32>,
    Form(client): Form<ClientUpdate>,
) -> Html<String> {
    // Update client
    // Return rendered ClientDetailsTemplate
    Html(String::new())
}

async fn delete_client(
    State(database): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Delete client
    // Return rendered SuccessNotificationTemplate
    Html(String::new())
}

async fn fetch_client_sites(
    State(database): State<Database>,
    Path(id): Path<i32>,
    Query(params): Query<PaginationParams>,
) -> Html<String> {
    // Fetch sites for client
    // Return rendered ClientSitesTemplate
    Html(String::new())
}

async fn check_inn_unique(
    State(database): State<Database>,
    Path(inn): Path<String>,
) -> Html<String> {
    // Check if INN is unique
    // Return rendered InnValidationTemplate
    Html(String::new())
}

pub fn router() -> Router<Database> {
    Router::new()
        // Page Endpoints
        .route("/clients", get(clients_page))
        .route("/clients/new", get(client_new_page))
        .route("/clients/{id}", get(client_details_page))
        .route("/clients/{id}/edit", get(client_edit_page))
        // HTMX Endpoints
        .route("/api/clients", get(fetch_clients).post(create_client))
        .route("/api/clients/{id}", get(fetch_client_details).put(update_client).delete(delete_client))
        .route("/api/clients/{id}/sites", get(fetch_client_sites))
        .route("/api/clients/check-inn/{inn}", get(check_inn_unique))
}
