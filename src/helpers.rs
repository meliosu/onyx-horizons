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

// Template structs for selectors
#[derive(Template)]
#[template(path = "components/selectors/generic_selector.html")]
struct GenericSelectorTemplate {
    options: Vec<SelectOption>,
    name: String,
    id: String,
    placeholder: Option<String>,
    required: bool,
    disabled: bool,
}

#[derive(Template)]
#[template(path = "components/selectors/departments.html")]
struct DepartmentSelectorTemplate {
    options: Vec<SelectOption>,
    name: String,
    id: String,
    placeholder: Option<String>,
    required: bool,
    disabled: bool,
}

#[derive(Template)]
#[template(path = "components/selectors/areas.html")]
struct AreaSelectorTemplate {
    options: Vec<SelectOption>,
    name: String,
    id: String,
    placeholder: Option<String>,
    required: bool,
    disabled: bool,
    department_dependent: bool,
}

#[derive(Template)]
#[template(path = "components/selectors/clients.html")]
struct ClientSelectorTemplate {
    options: Vec<SelectOption>,
    name: String,
    id: String,
    placeholder: Option<String>,
    required: bool,
    disabled: bool,
}

#[derive(Template)]
#[template(path = "components/selectors/workers.html")]
struct WorkerSelectorTemplate {
    options: Vec<SelectOption>,
    name: String,
    id: String,
    placeholder: Option<String>,
    required: bool,
    disabled: bool,
    profession_filter: Option<String>,
}

#[derive(Template)]
#[template(path = "components/selectors/brigades.html")]
struct BrigadeSelectorTemplate {
    options: Vec<SelectOption>,
    name: String,
    id: String,
    placeholder: Option<String>,
    required: bool,
    disabled: bool,
}

#[derive(Template)]
#[template(path = "components/selectors/sites.html")]
struct SiteSelectorTemplate {
    options: Vec<SelectOption>,
    name: String,
    id: String,
    placeholder: Option<String>,
    required: bool,
    disabled: bool,
    area_dependent: bool,
}

#[derive(Template)]
#[template(path = "components/selectors/materials.html")]
struct MaterialSelectorTemplate {
    options: Vec<SelectOption>,
    name: String,
    id: String,
    placeholder: Option<String>,
    required: bool,
    disabled: bool,
}

#[derive(Template)]
#[template(path = "components/validation/field_validation.html")]
struct ValidationMessageTemplate {
    field: String,
    is_valid: bool,
    message: Option<String>,
}

// Department selector
async fn department_selector(
    State(database): State<Database>,
    Query(params): Query<SelectorParams>,
) -> Html<String> {
    // In a real implementation, fetch departments from the database
    let options = vec![
        SelectOption { 
            value: "1".to_string(), 
            label: "North Regional Department".to_string(), 
            selected: params.selected.as_ref().map_or(false, |s| s == "1") 
        },
        SelectOption { 
            value: "2".to_string(), 
            label: "South Regional Department".to_string(), 
            selected: params.selected.as_ref().map_or(false, |s| s == "2") 
        },
        SelectOption { 
            value: "3".to_string(), 
            label: "East Regional Department".to_string(), 
            selected: params.selected.as_ref().map_or(false, |s| s == "3") 
        },
    ];
    
    let template = DepartmentSelectorTemplate {
        options,
        name: "department_id".to_string(),
        id: "department-selector".to_string(),
        placeholder: Some("Select Department".to_string()),
        required: true,
        disabled: false,
    };
    
    Html(template.render().unwrap_or_else(|_| String::from("Error rendering department selector")))
}

// Area selector
async fn area_selector(
    State(database): State<Database>,
    Query(params): Query<SelectorParams>,
) -> Html<String> {
    // In a real implementation, fetch areas from the database
    let options = vec![
        SelectOption { 
            value: "1".to_string(), 
            label: "Downtown".to_string(), 
            selected: params.selected.as_ref().map_or(false, |s| s == "1") 
        },
        SelectOption { 
            value: "2".to_string(), 
            label: "Riverside".to_string(), 
            selected: params.selected.as_ref().map_or(false, |s| s == "2") 
        },
        SelectOption { 
            value: "3".to_string(), 
            label: "Industrial Zone".to_string(), 
            selected: params.selected.as_ref().map_or(false, |s| s == "3") 
        },
    ];
    
    let template = AreaSelectorTemplate {
        options,
        name: "area_id".to_string(),
        id: "area-selector".to_string(),
        placeholder: Some("Select Area".to_string()),
        required: true,
        disabled: false,
        department_dependent: true,
    };
    
    Html(template.render().unwrap_or_else(|_| String::from("Error rendering area selector")))
}

// Areas by department
async fn areas_by_department(
    State(database): State<Database>,
    Path(department_id): Path<i32>,
    Query(params): Query<SelectorParams>,
) -> Html<String> {
    // In a real implementation, fetch areas for the given department from the database
    let options = match department_id {
        1 => vec![
            SelectOption { 
                value: "1".to_string(), 
                label: "Downtown".to_string(), 
                selected: params.selected.as_ref().map_or(false, |s| s == "1") 
            },
            SelectOption { 
                value: "2".to_string(), 
                label: "Riverside".to_string(), 
                selected: params.selected.as_ref().map_or(false, |s| s == "2") 
            },
        ],
        2 => vec![
            SelectOption { 
                value: "3".to_string(), 
                label: "Industrial Zone".to_string(), 
                selected: params.selected.as_ref().map_or(false, |s| s == "3") 
            },
            SelectOption { 
                value: "4".to_string(), 
                label: "Suburban".to_string(), 
                selected: params.selected.as_ref().map_or(false, |s| s == "4") 
            },
        ],
        _ => vec![],
    };
    
    let template = AreaSelectorTemplate {
        options,
        name: "area_id".to_string(),
        id: "area-selector".to_string(),
        placeholder: Some("Select Area".to_string()),
        required: true,
        disabled: options.is_empty(),
        department_dependent: true,
    };
    
    Html(template.render().unwrap_or_else(|_| String::from("Error rendering areas by department")))
}

// Client selector
async fn client_selector(
    State(database): State<Database>,
    Query(params): Query<SelectorParams>,
) -> Html<String> {
    // In a real implementation, fetch clients from the database
    let options = vec![
        SelectOption { 
            value: "1".to_string(), 
            label: "City Administration".to_string(), 
            selected: params.selected.as_ref().map_or(false, |s| s == "1") 
        },
        SelectOption { 
            value: "2".to_string(), 
            label: "Riverstone Properties".to_string(), 
            selected: params.selected.as_ref().map_or(false, |s| s == "2") 
        },
        SelectOption { 
            value: "3".to_string(), 
            label: "Green Energy Corp".to_string(), 
            selected: params.selected.as_ref().map_or(false, |s| s == "3") 
        },
    ];
    
    let template = ClientSelectorTemplate {
        options,
        name: "client_id".to_string(),
        id: "client-selector".to_string(),
        placeholder: Some("Select Client".to_string()),
        required: true,
        disabled: false,
    };
    
    Html(template.render().unwrap_or_else(|_| String::from("Error rendering client selector")))
}

// Worker selector
async fn worker_selector(
    State(database): State<Database>,
    Query(params): Query<SelectorParams>,
) -> Html<String> {
    // In a real implementation, fetch workers from the database
    // Filter by profession if specified in params
    let profession_filter = params.search.clone();
    
    let options = vec![
        SelectOption { 
            value: "1".to_string(), 
            label: "John Smith - Electrician".to_string(), 
            selected: params.selected.as_ref().map_or(false, |s| s == "1") 
        },
        SelectOption { 
            value: "2".to_string(), 
            label: "Maria Rodriguez - Welder".to_string(), 
            selected: params.selected.as_ref().map_or(false, |s| s == "2") 
        },
        SelectOption { 
            value: "3".to_string(), 
            label: "David Lee - Plumber".to_string(), 
            selected: params.selected.as_ref().map_or(false, |s| s == "3") 
        },
    ];
    
    let template = WorkerSelectorTemplate {
        options,
        name: "worker_id".to_string(),
        id: "worker-selector".to_string(),
        placeholder: Some("Select Worker".to_string()),
        required: true,
        disabled: false,
        profession_filter,
    };
    
    Html(template.render().unwrap_or_else(|_| String::from("Error rendering worker selector")))
}

// Brigade selector
async fn brigade_selector(
    State(database): State<Database>,
    Query(params): Query<SelectorParams>,
) -> Html<String> {
    // In a real implementation, fetch brigades from the database
    let options = vec![
        SelectOption { 
            value: "1".to_string(), 
            label: "Brigade #1 - Smith (5 workers)".to_string(), 
            selected: params.selected.as_ref().map_or(false, |s| s == "1") 
        },
        SelectOption { 
            value: "2".to_string(), 
            label: "Brigade #2 - Rodriguez (8 workers)".to_string(), 
            selected: params.selected.as_ref().map_or(false, |s| s == "2") 
        },
        SelectOption { 
            value: "3".to_string(), 
            label: "Brigade #3 - Johnson (6 workers)".to_string(), 
            selected: params.selected.as_ref().map_or(false, |s| s == "3") 
        },
    ];
    
    let template = BrigadeSelectorTemplate {
        options,
        name: "brigade_id".to_string(),
        id: "brigade-selector".to_string(),
        placeholder: Some("Select Brigade".to_string()),
        required: true,
        disabled: false,
    };
    
    Html(template.render().unwrap_or_else(|_| String::from("Error rendering brigade selector")))
}

// Site selector
async fn site_selector(
    State(database): State<Database>,
    Query(params): Query<SelectorParams>,
) -> Html<String> {
    // In a real implementation, fetch sites from the database
    let options = vec![
        SelectOption { 
            value: "1".to_string(), 
            label: "Riverfront Housing".to_string(), 
            selected: params.selected.as_ref().map_or(false, |s| s == "1") 
        },
        SelectOption { 
            value: "2".to_string(), 
            label: "Downtown Bridge".to_string(), 
            selected: params.selected.as_ref().map_or(false, |s| s == "2") 
        },
        SelectOption { 
            value: "3".to_string(), 
            label: "Central Park Renovation".to_string(), 
            selected: params.selected.as_ref().map_or(false, |s| s == "3") 
        },
    ];
    
    let template = SiteSelectorTemplate {
        options,
        name: "site_id".to_string(),
        id: "site-selector".to_string(),
        placeholder: Some("Select Site".to_string()),
        required: true,
        disabled: false,
        area_dependent: true,
    };
    
    Html(template.render().unwrap_or_else(|_| String::from("Error rendering site selector")))
}

// Material selector
async fn material_selector(
    State(database): State<Database>,
    Query(params): Query<SelectorParams>,
) -> Html<String> {
    // In a real implementation, fetch materials from the database
    let options = vec![
        SelectOption { 
            value: "1".to_string(), 
            label: "Cement - $12/bag".to_string(), 
            selected: params.selected.as_ref().map_or(false, |s| s == "1") 
        },
        SelectOption { 
            value: "2".to_string(), 
            label: "Bricks - $0.75/piece".to_string(), 
            selected: params.selected.as_ref().map_or(false, |s| s == "2") 
        },
        SelectOption { 
            value: "3".to_string(), 
            label: "Steel Rebar - $8.50/meter".to_string(), 
            selected: params.selected.as_ref().map_or(false, |s| s == "3") 
        },
    ];
    
    let template = MaterialSelectorTemplate {
        options,
        name: "material_id".to_string(),
        id: "material-selector".to_string(),
        placeholder: Some("Select Material".to_string()),
        required: true,
        disabled: false,
    };
    
    Html(template.render().unwrap_or_else(|_| String::from("Error rendering material selector")))
}

// Form validation
async fn validate_form_field(
    State(database): State<Database>,
    Path(form_type): Path<String>,
    Form(validation): Form<ValidationRequest>,
) -> Html<String> {
    // In a real implementation, validate the field based on form type and context
    let validation_result = match (form_type.as_str(), validation.field.as_str()) {
        ("client", "inn") => {
            // Example: Validate INN format
            let is_valid = validation.value.len() == 10 && validation.value.chars().all(|c| c.is_digit(10));
            ValidationResponse {
                field: validation.field,
                is_valid,
                message: if is_valid { 
                    None 
                } else { 
                    Some("INN must be a 10-digit number".to_string()) 
                },
            }
        },
        ("equipment", "amount") => {
            // Example: Validate amount is a positive number
            let is_valid = validation.value.parse::<i32>().map_or(false, |n| n > 0);
            ValidationResponse {
                field: validation.field,
                is_valid,
                message: if is_valid { 
                    None 
                } else { 
                    Some("Amount must be a positive number".to_string()) 
                },
            }
        },
        // Add more validation cases as needed
        _ => ValidationResponse {
            field: validation.field,
            is_valid: true,
            message: None,
        },
    };
    
    let template = ValidationMessageTemplate {
        field: validation_result.field,
        is_valid: validation_result.is_valid,
        message: validation_result.message,
    };
    
    Html(template.render().unwrap_or_else(|_| String::from("Error rendering validation message")))
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
