use axum::{
    extract::{Path, Query, State},
    response::Html,
    routing::{get, post, put, delete},
    Form, Router,
};
use serde::{Deserialize, Serialize};

use crate::database::Database;
use crate::general::PaginationParams;

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

// Page Endpoints
async fn clients_page(
    State(database): State<Database>,
    Query(params): Query<ClientFilter>,
) -> Html<String> {
    // Client listing page
    Html(String::new())
}

async fn client_new_page(
    State(database): State<Database>,
) -> Html<String> {
    // New client form
    Html(String::new())
}

async fn client_details_page(
    State(database): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Client details page
    Html(String::new())
}

async fn client_edit_page(
    State(database): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Edit client form
    Html(String::new())
}

// HTMX Endpoints
async fn fetch_clients(
    State(database): State<Database>,
    Query(params): Query<ClientFilter>,
) -> Html<String> {
    // Fetch clients with filters
    Html(String::new())
}

async fn create_client(
    State(database): State<Database>,
    Form(client): Form<ClientCreate>,
) -> Html<String> {
    // Create new client
    Html(String::new())
}

async fn fetch_client_details(
    State(database): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Fetch client details
    Html(String::new())
}

async fn update_client(
    State(database): State<Database>,
    Path(id): Path<i32>,
    Form(client): Form<ClientUpdate>,
) -> Html<String> {
    // Update client
    Html(String::new())
}

async fn delete_client(
    State(database): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Delete client
    Html(String::new())
}

async fn fetch_client_sites(
    State(database): State<Database>,
    Path(id): Path<i32>,
    Query(params): Query<PaginationParams>,
) -> Html<String> {
    // Fetch sites for client
    Html(String::new())
}

async fn check_inn_unique(
    State(database): State<Database>,
    Path(inn): Path<String>,
) -> Html<String> {
    // Check if INN is unique
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
