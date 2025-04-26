use axum::{
    extract::{Path, Query, State},
    response::Html,
    routing::{get, post, put, delete},
    Form, Router,
};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use askama::Template;

use crate::database::Database;
use crate::general::PaginationParams;

// Enums for site types and risk levels
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(rename_all = "snake_case")]
pub enum SiteType {
    PowerPlant,
    Road,
    Housing,
    Bridge,
    Park,
}

impl FromStr for SiteType {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "power_plant" => Ok(SiteType::PowerPlant),
            "road" => Ok(SiteType::Road),
            "housing" => Ok(SiteType::Housing),
            "bridge" => Ok(SiteType::Bridge),
            "park" => Ok(SiteType::Park),
            _ => Err(format!("Unknown site type: {}", s)),
        }
    }
}

impl std::fmt::Display for SiteType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SiteType::PowerPlant => write!(f, "Power Plant"),
            SiteType::Road => write!(f, "Road"),
            SiteType::Housing => write!(f, "Housing"),
            SiteType::Bridge => write!(f, "Bridge"),
            SiteType::Park => write!(f, "Park"),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(rename_all = "lowercase")]
pub enum RiskLevel {
    High,
    Medium,
    Low,
}

impl FromStr for RiskLevel {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "high" => Ok(RiskLevel::High),
            "medium" => Ok(RiskLevel::Medium),
            "low" => Ok(RiskLevel::Low),
            _ => Err(format!("Unknown risk level: {}", s)),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(rename_all = "lowercase")]
pub enum EnergyEfficiency {
    High,
    Medium,
    Low,
}

impl std::fmt::Display for EnergyEfficiency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EnergyEfficiency::High => write!(f, "High"),
            EnergyEfficiency::Medium => write!(f, "Medium"),
            EnergyEfficiency::Low => write!(f, "Low"),
        }
    }
}

// Base Site types
#[derive(Serialize, Deserialize)]
pub struct Site {
    pub id: i32,
    pub name: String,
    pub area_id: i32,
    pub client_id: i32,
    pub site_type: SiteType,
    pub location: Location,
    pub risk_level: RiskLevel,
    pub description: Option<String>,
    // Joined fields
    pub area_name: Option<String>,
    pub client_name: Option<String>,
    pub department_id: Option<i32>,
    pub department_name: Option<String>,
    pub tasks_count: Option<i32>,
    pub equipment_count: Option<i32>,
    pub brigades_count: Option<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
}

// Site type-specific structs
#[derive(Serialize, Deserialize)]
pub struct PowerPlant {
    pub site_id: i32,
    pub energy_output: f64,
    pub energy_source: String,
    pub is_grid_connected: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Road {
    pub site_id: i32,
    pub length: f64,
    pub lanes: i32,
    pub surface: String,
}

#[derive(Serialize, Deserialize)]
pub struct Housing {
    pub site_id: i32,
    pub number_of_floors: i32,
    pub number_of_entrances: i32,
    pub housing_type: String,
    pub energy_efficiency: EnergyEfficiency,
}

#[derive(Serialize, Deserialize)]
pub struct Bridge {
    pub site_id: i32,
    pub length: f64,
    pub road_material: String,
    pub max_load: f64,
}

#[derive(Serialize, Deserialize)]
pub struct Park {
    pub site_id: i32,
    pub area: f64,
    pub has_playground: bool,
    pub has_lighting: bool,
}

// Combined site details structs
#[derive(Serialize, Deserialize)]
pub struct SiteDetails {
    pub site: Site,
    pub type_details: SiteTypeDetails,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SiteTypeDetails {
    PowerPlant(PowerPlant),
    Road(Road),
    Housing(Housing),
    Bridge(Bridge),
    Park(Park),
}

// Create and update structs
#[derive(Deserialize)]
pub struct SiteCreate {
    pub area_id: i32,
    pub client_id: i32,
    pub site_type: SiteType,
    pub latitude: f64,
    pub longitude: f64,
    pub risk_level: RiskLevel,
    pub description: Option<String>,
    // Type-specific fields will be handled separately based on site_type
}

#[derive(Deserialize)]
pub struct SiteUpdate {
    pub area_id: i32,
    pub client_id: i32,
    pub risk_level: RiskLevel,
    pub latitude: f64,
    pub longitude: f64,
    pub description: Option<String>,
    // Type cannot be changed after creation
}

#[derive(Deserialize)]
pub struct PowerPlantCreate {
    pub energy_output: f64,
    pub energy_source: String,
    pub is_grid_connected: bool,
}

#[derive(Deserialize)]
pub struct RoadCreate {
    pub length: f64,
    pub lanes: i32,
    pub surface: String,
}

#[derive(Deserialize)]
pub struct HousingCreate {
    pub number_of_floors: i32,
    pub number_of_entrances: i32,
    pub housing_type: String,
    pub energy_efficiency: EnergyEfficiency,
}

#[derive(Deserialize)]
pub struct BridgeCreate {
    pub length: f64,
    pub road_material: String,
    pub max_load: f64,
}

#[derive(Deserialize)]
pub struct ParkCreate {
    pub area: f64,
    pub has_playground: bool,
    pub has_lighting: bool,
}

// Task for site
#[derive(Serialize, Deserialize)]
pub struct Task {
    pub id: i32,
    pub site_id: i32,
    pub brigade_id: Option<i32>,
    pub period_start: String,
    pub expected_period_end: chrono::NaiveDate,
    pub actual_period_end: Option<chrono::NaiveDate>,
    pub name: String,
    pub description: Option<String>,
    // Joined fields
    pub brigade_name: Option<String>,
    pub materials_count: Option<i32>,
}

// Create task
#[derive(Deserialize)]
pub struct TaskCreate {
    pub brigade_id: Option<i32>,
    pub period_start: String,
    pub expected_period_end: String,
    pub name: String,
    pub description: Option<String>,
}

// Filter parameters
#[derive(Deserialize)]
pub struct SiteFilter {
    pub site_type: Option<SiteType>,
    pub area_id: Option<i32>,
    pub department_id: Option<i32>,
    pub client_id: Option<i32>,
    pub risk_level: Option<RiskLevel>,
    #[serde(flatten)]
    pub pagination: PaginationParams,
}

// Additional types needed for templates
#[derive(Serialize, Deserialize, Clone)]
pub struct Area {
    pub id: i32,
    pub name: String,
    pub department_id: i32,
    pub department_name: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Department {
    pub id: i32,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Client {
    pub id: i32,
    pub name: String,
    pub inn: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Brigade {
    pub id: i32,
    pub brigadier_name: String,
    pub workers_count: i32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MaterialUsage {
    pub task_id: i32,
    pub material_id: i32,
    pub material_name: String,
    pub expected_amount: f64,
    pub actual_amount: Option<f64>,
    pub units: String,
    pub cost_per_unit: f64,
    pub total_cost: f64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct EquipmentAllocation {
    pub equipment_id: i32,
    pub equipment_name: String,
    pub amount: i32,
    pub period_start: String,
    pub period_end: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Report {
    pub id: i32,
    pub title: String,
    pub date: String,
    pub report_type: String,
    pub status: String,
}

#[derive(Deserialize)]
pub struct SiteTypeQuery {
    pub site_type: SiteType,
}

// Template types for Site pages
#[derive(Template)]
#[template(path = "sites/index.html")]
pub struct SitesPageTemplate {
    pub sites: Vec<Site>,
    pub filter: Option<SiteFilter>,
    pub current_page: usize,
    pub total_pages: usize,
    pub areas: Vec<Area>,
    pub departments: Vec<Department>,
    pub clients: Vec<Client>,
}

#[derive(Template)]
#[template(path = "sites/new.html")]
pub struct SiteNewPageTemplate {
    pub areas: Vec<Area>,
    pub clients: Vec<Client>,
    pub area_id: Option<i32>,
    pub department_id: Option<i32>,
}

#[derive(Template)]
#[template(path = "sites/details.html")]
pub struct SiteDetailsPageTemplate {
    pub site_details: SiteDetails,
    pub active_tab: String, // schedule, materials, equipment, brigades, reports
}

#[derive(Template)]
#[template(path = "sites/edit.html")]
pub struct SiteEditPageTemplate {
    pub site_details: SiteDetails,
    pub areas: Vec<Area>,
    pub clients: Vec<Client>,
}

// Template types for HTMX components
#[derive(Template)]
#[template(path = "sites/components/site_rows.html")]
pub struct SiteRowsTemplate {
    pub sites: Vec<Site>,
}

#[derive(Template)]
#[template(path = "sites/components/site_details.html")]
pub struct SiteDetailsTemplate {
    pub site_details: SiteDetails,
}

#[derive(Template)]
#[template(path = "sites/components/schedule_tab.html")]
pub struct ScheduleTabTemplate {
    pub site_id: i32,
    pub tasks: Vec<Task>,
    pub brigades: Vec<Brigade>,
    pub current_page: usize,
    pub total_pages: usize,
}

#[derive(Template)]
#[template(path = "sites/components/materials_tab.html")]
pub struct MaterialsTabTemplate {
    pub site_id: i32,
    pub materials: Vec<MaterialUsage>,
    pub current_page: usize,
    pub total_pages: usize,
}

#[derive(Template)]
#[template(path = "sites/components/equipment_tab.html")]
pub struct EquipmentTabTemplate {
    pub site_id: i32,
    pub equipment_allocations: Vec<EquipmentAllocation>,
    pub current_page: usize,
    pub total_pages: usize,
}

#[derive(Template)]
#[template(path = "sites/components/brigades_tab.html")]
pub struct BrigadesTabTemplate {
    pub site_id: i32,
    pub brigades: Vec<Brigade>,
    pub current_page: usize,
    pub total_pages: usize,
}

#[derive(Template)]
#[template(path = "sites/components/reports_tab.html")]
pub struct ReportsTabTemplate {
    pub site_id: i32,
    pub reports: Vec<Report>,
    pub current_page: usize,
    pub total_pages: usize,
}

#[derive(Template)]
#[template(path = "sites/components/type_fields.html")]
pub struct TypeFieldsTemplate {
    pub site_type: SiteType,
    // Optional pre-filled values for edit mode
    pub power_plant: Option<PowerPlant>,
    pub road: Option<Road>,
    pub housing: Option<Housing>,
    pub bridge: Option<Bridge>,
    pub park: Option<Park>,
}

#[derive(Template)]
#[template(path = "sites/components/risk_level_selector.html")]
pub struct RiskLevelSelectorTemplate {
    pub selected_risk_level: Option<RiskLevel>,
}

#[derive(Template)]
#[template(path = "components/success_notification.html")]
pub struct SuccessNotificationTemplate {
    pub message: String,
}

// Page Endpoints
async fn sites_page(
    State(database): State<Database>,
    Query(params): Query<SiteFilter>,
) -> Html<String> {
    // Site listing page
    let page = params.pagination.page.unwrap_or(1);
    let per_page = params.pagination.per_page.unwrap_or(10);
    
    // Mock data for sites
    let sites = vec![
        Site {
            id: 1,
            name: "Downtown Tower".to_string(),
            area_id: 1,
            client_id: 101,
            site_type: SiteType::Housing,
            location: Location { latitude: 55.7558, longitude: 37.6173 },
            risk_level: RiskLevel::Medium,
            description: Some("25-floor commercial building in the city center".to_string()),
            area_name: Some("Downtown Construction Area".to_string()),
            client_name: Some("Metro Development Corp".to_string()),
            department_id: Some(1),
            department_name: Some("North Construction Department".to_string()),
            tasks_count: Some(12),
            equipment_count: Some(5),
            brigades_count: Some(3),
        },
        Site {
            id: 2,
            name: "Riverside Bridge".to_string(),
            area_id: 2,
            client_id: 102,
            site_type: SiteType::Bridge,
            location: Location { latitude: 55.7522, longitude: 37.6156 },
            risk_level: RiskLevel::High,
            description: Some("600m bridge across the river".to_string()),
            area_name: Some("Riverside Construction Area".to_string()),
            client_name: Some("City Administration".to_string()),
            department_id: Some(1),
            department_name: Some("North Construction Department".to_string()),
            tasks_count: Some(8),
            equipment_count: Some(7),
            brigades_count: Some(2),
        },
        Site {
            id: 3,
            name: "Green Valley Park".to_string(),
            area_id: 3,
            client_id: 103,
            site_type: SiteType::Park,
            location: Location { latitude: 55.7539, longitude: 37.6208 },
            risk_level: RiskLevel::Low,
            description: Some("Community park with recreational facilities".to_string()),
            area_name: Some("Industrial Park Construction Area".to_string()),
            client_name: Some("Parks & Recreation Department".to_string()),
            department_id: Some(2),
            department_name: Some("South Construction Department".to_string()),
            tasks_count: Some(5),
            equipment_count: Some(3),
            brigades_count: Some(1),
        },
        Site {
            id: 4,
            name: "Solar Power Plant".to_string(),
            area_id: 4,
            client_id: 104,
            site_type: SiteType::PowerPlant,
            location: Location { latitude: 55.7512, longitude: 37.6184 },
            risk_level: RiskLevel::Medium,
            description: Some("50MW solar power plant".to_string()),
            area_name: Some("Eastern Energy Zone".to_string()),
            client_name: Some("Energy Solutions Inc".to_string()),
            department_id: Some(3),
            department_name: Some("East Construction Department".to_string()),
            tasks_count: Some(15),
            equipment_count: Some(8),
            brigades_count: Some(4),
        },
        Site {
            id: 5,
            name: "Highway Bypass".to_string(),
            area_id: 5,
            client_id: 105,
            site_type: SiteType::Road,
            location: Location { latitude: 55.7545, longitude: 37.6220 },
            risk_level: RiskLevel::Medium,
            description: Some("15km highway bypass around the city".to_string()),
            area_name: Some("Northern Transport Hub".to_string()),
            client_name: Some("Transportation Department".to_string()),
            department_id: Some(1),
            department_name: Some("North Construction Department".to_string()),
            tasks_count: Some(20),
            equipment_count: Some(12),
            brigades_count: Some(5),
        },
    ];
    
    // Mock data for areas, departments, clients (for filters)
    let areas = vec![
        Area {
            id: 1,
            name: "Downtown Construction Area".to_string(),
            department_id: 1,
            department_name: Some("North Construction Department".to_string()),
        },
        Area {
            id: 2,
            name: "Riverside Construction Area".to_string(),
            department_id: 1,
            department_name: Some("North Construction Department".to_string()),
        },
        Area {
            id: 3,
            name: "Industrial Park Construction Area".to_string(),
            department_id: 2,
            department_name: Some("South Construction Department".to_string()),
        },
        Area {
            id: 4,
            name: "Eastern Energy Zone".to_string(),
            department_id: 3,
            department_name: Some("East Construction Department".to_string()),
        },
        Area {
            id: 5,
            name: "Northern Transport Hub".to_string(),
            department_id: 1,
            department_name: Some("North Construction Department".to_string()),
        },
    ];
    
    let departments = vec![
        Department {
            id: 1,
            name: "North Construction Department".to_string(),
        },
        Department {
            id: 2,
            name: "South Construction Department".to_string(),
        },
        Department {
            id: 3,
            name: "East Construction Department".to_string(),
        },
    ];
    
    let clients = vec![
        Client {
            id: 101,
            name: "Metro Development Corp".to_string(),
            inn: "1234567890".to_string(),
        },
        Client {
            id: 102,
            name: "City Administration".to_string(),
            inn: "0987654321".to_string(),
        },
        Client {
            id: 103,
            name: "Parks & Recreation Department".to_string(),
            inn: "5678901234".to_string(),
        },
        Client {
            id: 104,
            name: "Energy Solutions Inc".to_string(),
            inn: "6789012345".to_string(),
        },
        Client {
            id: 105,
            name: "Transportation Department".to_string(),
            inn: "7890123456".to_string(),
        },
    ];
    
    // Apply filters if specified
    let filtered_sites = sites.into_iter()
        .filter(|site| {
            // Filter by site type if specified
            if let Some(ref site_type) = params.site_type {
                if site.site_type != *site_type {
                    return false;
                }
            }
            
            // Filter by area if specified
            if let Some(area_id) = params.area_id {
                if site.area_id != area_id {
                    return false;
                }
            }
            
            // Filter by department if specified
            if let Some(department_id) = params.department_id {
                if site.department_id != Some(department_id) {
                    return false;
                }
            }
            
            // Filter by client if specified
            if let Some(client_id) = params.client_id {
                if site.client_id != client_id {
                    return false;
                }
            }
            
            // Filter by risk level if specified
            if let Some(ref risk_level) = params.risk_level {
                if site.risk_level != *risk_level {
                    return false;
                }
            }
            
            true
        })
        .collect();
    
    let template = SitesPageTemplate {
        sites: filtered_sites,
        filter: Some(params),
        current_page: page,
        total_pages: 1, // Mock single page
        areas,
        departments,
        clients,
    };
    
    match template.render() {
        Ok(html) => Html(html),
        Err(err) => {
            eprintln!("Template error: {}", err);
            Html("Error rendering sites page".to_string())
        }
    }
}

async fn site_new_page(
    State(database): State<Database>,
) -> Html<String> {
    // New site form
    // Mock data for available areas and clients
    let areas = vec![
        Area {
            id: 1,
            name: "Downtown Construction Area".to_string(),
            department_id: 1,
            department_name: Some("North Construction Department".to_string()),
        },
        Area {
            id: 2,
            name: "Riverside Construction Area".to_string(),
            department_id: 1,
            department_name: Some("North Construction Department".to_string()),
        },
        Area {
            id: 3,
            name: "Industrial Park Construction Area".to_string(),
            department_id: 2,
            department_name: Some("South Construction Department".to_string()),
        },
        Area {
            id: 4,
            name: "Eastern Energy Zone".to_string(),
            department_id: 3,
            department_name: Some("East Construction Department".to_string()),
        },
        Area {
            id: 5,
            name: "Northern Transport Hub".to_string(),
            department_id: 1,
            department_name: Some("North Construction Department".to_string()),
        },
    ];
    
    let clients = vec![
        Client {
            id: 101,
            name: "Metro Development Corp".to_string(),
            inn: "1234567890".to_string(),
        },
        Client {
            id: 102,
            name: "City Administration".to_string(),
            inn: "0987654321".to_string(),
        },
        Client {
            id: 103,
            name: "Parks & Recreation Department".to_string(),
            inn: "5678901234".to_string(),
        },
        Client {
            id: 104,
            name: "Energy Solutions Inc".to_string(),
            inn: "6789012345".to_string(),
        },
        Client {
            id: 105,
            name: "Transportation Department".to_string(),
            inn: "7890123456".to_string(),
        },
    ];
    
    // Check if we were given an area_id or department_id in the query
    let area_id = None;
    let department_id = None;
    
    let template = SiteNewPageTemplate {
        areas,
        clients,
        area_id,
        department_id,
    };
    
    match template.render() {
        Ok(html) => Html(html),
        Err(err) => {
            eprintln!("Template error: {}", err);
            Html("Error rendering new site form".to_string())
        }
    }
}

async fn site_details_page(
    State(database): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Site details page
    // Create mock site details based on ID
    let site_type = match id % 5 {
        0 => SiteType::PowerPlant,
        1 => SiteType::Housing,
        2 => SiteType::Bridge,
        3 => SiteType::Road,
        _ => SiteType::Park,
    };
    
    let site = Site {
        id,
        name: format!("Site #{}", id),
        area_id: 1 + (id % 5) as i32,
        client_id: 101 + (id % 5) as i32,
        site_type: site_type.clone(),
        location: Location { latitude: 55.75 + (id as f64 * 0.01), longitude: 37.61 + (id as f64 * 0.01) },
        risk_level: match id % 3 {
            0 => RiskLevel::Low,
            1 => RiskLevel::Medium,
            _ => RiskLevel::High,
        },
        description: Some(format!("Description for site #{}", id)),
        area_name: Some(format!("Area #{}", 1 + (id % 5))),
        client_name: Some(format!("Client #{}", 101 + (id % 5))),
        department_id: Some(1 + (id % 3) as i32),
        department_name: Some(format!("Department #{}", 1 + (id % 3))),
        tasks_count: Some(5 + (id % 20) as i32),
        equipment_count: Some(3 + (id % 10) as i32),
        brigades_count: Some(1 + (id % 5) as i32),
    };
    
    // Create type-specific details based on site type
    let type_details = match site_type {
        SiteType::PowerPlant => SiteTypeDetails::PowerPlant(PowerPlant {
            site_id: id,
            energy_output: 50.0 + (id as f64 * 10.0),
            energy_source: "Solar".to_string(),
            is_grid_connected: id % 2 == 0,
        }),
        SiteType::Road => SiteTypeDetails::Road(Road {
            site_id: id,
            length: 1000.0 + (id as f64 * 100.0),
            lanes: 4 + (id % 4) as i32,
            surface: "Asphalt".to_string(),
        }),
        SiteType::Housing => SiteTypeDetails::Housing(Housing {
            site_id: id,
            number_of_floors: 5 + (id % 20) as i32,
            number_of_entrances: 2 + (id % 6) as i32,
            housing_type: "Apartment".to_string(),
            energy_efficiency: match id % 3 {
                0 => EnergyEfficiency::Low,
                1 => EnergyEfficiency::Medium,
                _ => EnergyEfficiency::High,
            },
        }),
        SiteType::Bridge => SiteTypeDetails::Bridge(Bridge {
            site_id: id,
            length: 200.0 + (id as f64 * 50.0),
            road_material: "Concrete".to_string(),
            max_load: 20.0 + (id as f64 * 5.0),
        }),
        SiteType::Park => SiteTypeDetails::Park(Park {
            site_id: id,
            area: 1.0 + (id as f64 * 0.5),
            has_playground: id % 2 == 0,
            has_lighting: id % 3 == 0,
        }),
    };
    
    let site_details = SiteDetails {
        site,
        type_details,
    };
    
    // Get active tab from query (would normally be part of the request)
    // Default to schedule tab
    let active_tab = "schedule".to_string();
    
    let template = SiteDetailsPageTemplate {
        site_details,
        active_tab,
    };
    
    match template.render() {
        Ok(html) => Html(html),
        Err(err) => {
            eprintln!("Template error: {}", err);
            Html(format!("Error rendering site details page for ID: {}", id))
        }
    }
}

async fn site_edit_page(
    State(database): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Edit site form
    // Create mock site details based on ID (same as details page)
    let site_type = match id % 5 {
        0 => SiteType::PowerPlant,
        1 => SiteType::Housing,
        2 => SiteType::Bridge,
        3 => SiteType::Road,
        _ => SiteType::Park,
    };
    
    let site = Site {
        id,
        name: format!("Site #{}", id),
        area_id: 1 + (id % 5) as i32,
        client_id: 101 + (id % 5) as i32,
        site_type: site_type.clone(),
        location: Location { latitude: 55.75 + (id as f64 * 0.01), longitude: 37.61 + (id as f64 * 0.01) },
        risk_level: match id % 3 {
            0 => RiskLevel::Low,
            1 => RiskLevel::Medium,
            _ => RiskLevel::High,
        },
        description: Some(format!("Description for site #{}", id)),
        area_name: Some(format!("Area #{}", 1 + (id % 5))),
        client_name: Some(format!("Client #{}", 101 + (id % 5))),
        department_id: Some(1 + (id % 3) as i32),
        department_name: Some(format!("Department #{}", 1 + (id % 3))),
        tasks_count: Some(5 + (id % 20) as i32),
        equipment_count: Some(3 + (id % 10) as i32),
        brigades_count: Some(1 + (id % 5) as i32),
    };
    
    // Create type-specific details based on site type
    let type_details = match site_type {
        SiteType::PowerPlant => SiteTypeDetails::PowerPlant(PowerPlant {
            site_id: id,
            energy_output: 50.0 + (id as f64 * 10.0),
            energy_source: "Solar".to_string(),
            is_grid_connected: id % 2 == 0,
        }),
        SiteType::Road => SiteTypeDetails::Road(Road {
            site_id: id,
            length: 1000.0 + (id as f64 * 100.0),
            lanes: 4 + (id % 4) as i32,
            surface: "Asphalt".to_string(),
        }),
        SiteType::Housing => SiteTypeDetails::Housing(Housing {
            site_id: id,
            number_of_floors: 5 + (id % 20) as i32,
            number_of_entrances: 2 + (id % 6) as i32,
            housing_type: "Apartment".to_string(),
            energy_efficiency: match id % 3 {
                0 => EnergyEfficiency::Low,
                1 => EnergyEfficiency::Medium,
                _ => EnergyEfficiency::High,
            },
        }),
        SiteType::Bridge => SiteTypeDetails::Bridge(Bridge {
            site_id: id,
            length: 200.0 + (id as f64 * 50.0),
            road_material: "Concrete".to_string(),
            max_load: 20.0 + (id as f64 * 5.0),
        }),
        SiteType::Park => SiteTypeDetails::Park(Park {
            site_id: id,
            area: 1.0 + (id as f64 * 0.5),
            has_playground: id % 2 == 0,
            has_lighting: id % 3 == 0,
        }),
    };
    
    let site_details = SiteDetails {
        site,
        type_details,
    };
    
    // Mock data for areas and clients for select dropdowns
    let areas = vec![
        Area {
            id: 1,
            name: "Downtown Construction Area".to_string(),
            department_id: 1,
            department_name: Some("North Construction Department".to_string()),
        },
        Area {
            id: 2,
            name: "Riverside Construction Area".to_string(),
            department_id: 1,
            department_name: Some("North Construction Department".to_string()),
        },
        Area {
            id: 3,
            name: "Industrial Park Construction Area".to_string(),
            department_id: 2,
            department_name: Some("South Construction Department".to_string()),
        },
        Area {
            id: 4,
            name: "Eastern Energy Zone".to_string(),
            department_id: 3,
            department_name: Some("East Construction Department".to_string()),
        },
        Area {
            id: 5,
            name: "Northern Transport Hub".to_string(),
            department_id: 1,
            department_name: Some("North Construction Department".to_string()),
        },
    ];
    
    let clients = vec![
        Client {
            id: 101,
            name: "Metro Development Corp".to_string(),
            inn: "1234567890".to_string(),
        },
        Client {
            id: 102,
            name: "City Administration".to_string(),
            inn: "0987654321".to_string(),
        },
        Client {
            id: 103,
            name: "Parks & Recreation Department".to_string(),
            inn: "5678901234".to_string(),
        },
        Client {
            id: 104,
            name: "Energy Solutions Inc".to_string(),
            inn: "6789012345".to_string(),
        },
        Client {
            id: 105,
            name: "Transportation Department".to_string(),
            inn: "7890123456".to_string(),
        },
    ];
    
    let template = SiteEditPageTemplate {
        site_details,
        areas,
        clients,
    };
    
    match template.render() {
        Ok(html) => Html(html),
        Err(err) => {
            eprintln!("Template error: {}", err);
            Html(format!("Error rendering site edit page for ID: {}", id))
        }
    }
}

// HTMX Endpoints
async fn fetch_sites(
    State(database): State<Database>,
    Query(params): Query<SiteFilter>,
) -> Html<String> {
    // Fetch sites with filters
    // Mock data for sites
    let mut sites = vec![
        Site {
            id: 1,
            name: "Downtown Tower".to_string(),
            area_id: 1,
            client_id: 101,
            site_type: SiteType::Housing,
            location: Location { latitude: 55.7558, longitude: 37.6173 },
            risk_level: RiskLevel::Medium,
            description: Some("25-floor commercial building in the city center".to_string()),
            area_name: Some("Downtown Construction Area".to_string()),
            client_name: Some("Metro Development Corp".to_string()),
            department_id: Some(1),
            department_name: Some("North Construction Department".to_string()),
            tasks_count: Some(12),
            equipment_count: Some(5),
            brigades_count: Some(3),
        },
        Site {
            id: 2,
            name: "Riverside Bridge".to_string(),
            area_id: 2,
            client_id: 102,
            site_type: SiteType::Bridge,
            location: Location { latitude: 55.7522, longitude: 37.6156 },
            risk_level: RiskLevel::High,
            description: Some("600m bridge across the river".to_string()),
            area_name: Some("Riverside Construction Area".to_string()),
            client_name: Some("City Administration".to_string()),
            department_id: Some(1),
            department_name: Some("North Construction Department".to_string()),
            tasks_count: Some(8),
            equipment_count: Some(7),
            brigades_count: Some(2),
        },
        Site {
            id: 3,
            name: "Green Valley Park".to_string(),
            area_id: 3,
            client_id: 103,
            site_type: SiteType::Park,
            location: Location { latitude: 55.7539, longitude: 37.6208 },
            risk_level: RiskLevel::Low,
            description: Some("Community park with recreational facilities".to_string()),
            area_name: Some("Industrial Park Construction Area".to_string()),
            client_name: Some("Parks & Recreation Department".to_string()),
            department_id: Some(2),
            department_name: Some("South Construction Department".to_string()),
            tasks_count: Some(5),
            equipment_count: Some(3),
            brigades_count: Some(1),
        },
        Site {
            id: 4,
            name: "Solar Power Plant".to_string(),
            area_id: 4,
            client_id: 104,
            site_type: SiteType::PowerPlant,
            location: Location { latitude: 55.7512, longitude: 37.6184 },
            risk_level: RiskLevel::Medium,
            description: Some("50MW solar power plant".to_string()),
            area_name: Some("Eastern Energy Zone".to_string()),
            client_name: Some("Energy Solutions Inc".to_string()),
            department_id: Some(3),
            department_name: Some("East Construction Department".to_string()),
            tasks_count: Some(15),
            equipment_count: Some(8),
            brigades_count: Some(4),
        },
        Site {
            id: 5,
            name: "Highway Bypass".to_string(),
            area_id: 5,
            client_id: 105,
            site_type: SiteType::Road,
            location: Location { latitude: 55.7545, longitude: 37.6220 },
            risk_level: RiskLevel::Medium,
            description: Some("15km highway bypass around the city".to_string()),
            area_name: Some("Northern Transport Hub".to_string()),
            client_name: Some("Transportation Department".to_string()),
            department_id: Some(1),
            department_name: Some("North Construction Department".to_string()),
            tasks_count: Some(20),
            equipment_count: Some(12),
            brigades_count: Some(5),
        },
    ];
    
    // Apply filters if specified
    if let Some(ref site_type) = params.site_type {
        sites.retain(|site| site.site_type == *site_type);
    }
    
    if let Some(area_id) = params.area_id {
        sites.retain(|site| site.area_id == area_id);
    }
    
    if let Some(department_id) = params.department_id {
        sites.retain(|site| site.department_id == Some(department_id));
    }
    
    if let Some(client_id) = params.client_id {
        sites.retain(|site| site.client_id == client_id);
    }
    
    if let Some(ref risk_level) = params.risk_level {
        sites.retain(|site| site.risk_level == *risk_level);
    }
    
    let template = SiteRowsTemplate { sites };
    
    match template.render() {
        Ok(html) => Html(html),
        Err(err) => {
            eprintln!("Template error: {}", err);
            Html("Error fetching sites".to_string())
        }
    }
}

async fn create_site(
    State(database): State<Database>,
    Form(site): Form<SiteCreate>,
) -> Html<String> {
    // Create new site
    // In a real implementation, this would create a new site in the database
    // For now, just return a success message
    let template = SuccessNotificationTemplate {
        message: format!("Site was created successfully and assigned ID #42."),
    };
    
    match template.render() {
        Ok(html) => Html(html),
        Err(err) => {
            eprintln!("Template error: {}", err);
            Html("Error creating site".to_string())
        }
    }
}

async fn fetch_site_details(
    State(database): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Fetch site details
    // Create mock site details based on ID
    let site_type = match id % 5 {
        0 => SiteType::PowerPlant,
        1 => SiteType::Housing,
        2 => SiteType::Bridge,
        3 => SiteType::Road,
        _ => SiteType::Park,
    };
    
    let site = Site {
        id,
        name: format!("Site #{}", id),
        area_id: 1 + (id % 5) as i32,
        client_id: 101 + (id % 5) as i32,
        site_type: site_type.clone(),
        location: Location { latitude: 55.75 + (id as f64 * 0.01), longitude: 37.61 + (id as f64 * 0.01) },
        risk_level: match id % 3 {
            0 => RiskLevel::Low,
            1 => RiskLevel::Medium,
            _ => RiskLevel::High,
        },
        description: Some(format!("Description for site #{}", id)),
        area_name: Some(format!("Area #{}", 1 + (id % 5))),
        client_name: Some(format!("Client #{}", 101 + (id % 5))),
        department_id: Some(1 + (id % 3) as i32),
        department_name: Some(format!("Department #{}", 1 + (id % 3))),
        tasks_count: Some(5 + (id % 20) as i32),
        equipment_count: Some(3 + (id % 10) as i32),
        brigades_count: Some(1 + (id % 5) as i32),
    };
    
    // Create type-specific details based on site type
    let type_details = match site_type {
        SiteType::PowerPlant => SiteTypeDetails::PowerPlant(PowerPlant {
            site_id: id,
            energy_output: 50.0 + (id as f64 * 10.0),
            energy_source: "Solar".to_string(),
            is_grid_connected: id % 2 == 0,
        }),
        SiteType::Road => SiteTypeDetails::Road(Road {
            site_id: id,
            length: 1000.0 + (id as f64 * 100.0),
            lanes: 4 + (id % 4) as i32,
            surface: "Asphalt".to_string(),
        }),
        SiteType::Housing => SiteTypeDetails::Housing(Housing {
            site_id: id,
            number_of_floors: 5 + (id % 20) as i32,
            number_of_entrances: 2 + (id % 6) as i32,
            housing_type: "Apartment".to_string(),
            energy_efficiency: match id % 3 {
                0 => EnergyEfficiency::Low,
                1 => EnergyEfficiency::Medium,
                _ => EnergyEfficiency::High,
            },
        }),
        SiteType::Bridge => SiteTypeDetails::Bridge(Bridge {
            site_id: id,
            length: 200.0 + (id as f64 * 50.0),
            road_material: "Concrete".to_string(),
            max_load: 20.0 + (id as f64 * 5.0),
        }),
        SiteType::Park => SiteTypeDetails::Park(Park {
            site_id: id,
            area: 1.0 + (id as f64 * 0.5),
            has_playground: id % 2 == 0,
            has_lighting: id % 3 == 0,
        }),
    };
    
    let site_details = SiteDetails {
        site,
        type_details,
    };
    
    let template = SiteDetailsTemplate { site_details };
    
    match template.render() {
        Ok(html) => Html(html),
        Err(err) => {
            eprintln!("Template error: {}", err);
            Html(format!("Error fetching site details for ID: {}", id))
        }
    }
}

async fn update_site(
    State(database): State<Database>,
    Path(id): Path<i32>,
    Form(site_update): Form<SiteUpdate>,
) -> Html<String> {
    // Update site
    // In a real implementation, this would update the site in the database
    // For now, just return updated site details using the form data and mock data
    
    // Create mock site with updated fields
    let site = Site {
        id,
        name: format!("Updated Site #{}", id),
        area_id: site_update.area_id,
        client_id: site_update.client_id,
        // Site type can't be changed after creation, so we mock it based on ID
        site_type: match id % 5 {
            0 => SiteType::PowerPlant,
            1 => SiteType::Housing,
            2 => SiteType::Bridge,
            3 => SiteType::Road,
            _ => SiteType::Park,
        },
        location: Location {
            latitude: site_update.latitude,
            longitude: site_update.longitude,
        },
        risk_level: site_update.risk_level,
        description: site_update.description,
        area_name: Some("Updated Area Name".to_string()),
        client_name: Some("Updated Client Name".to_string()),
        department_id: Some(site_update.area_id / 2 + 1), // Mock calculation based on area
        department_name: Some("Updated Department Name".to_string()),
        tasks_count: Some(5),
        equipment_count: Some(3),
        brigades_count: Some(2),
    };
    
    // Create type-specific details based on site type
    let type_details = match site.site_type {
        SiteType::PowerPlant => SiteTypeDetails::PowerPlant(PowerPlant {
            site_id: id,
            energy_output: 50.0,
            energy_source: "Updated Source".to_string(),
            is_grid_connected: true,
        }),
        SiteType::Road => SiteTypeDetails::Road(Road {
            site_id: id,
            length: 1000.0,
            lanes: 4,
            surface: "Updated Surface".to_string(),
        }),
        SiteType::Housing => SiteTypeDetails::Housing(Housing {
            site_id: id,
            number_of_floors: 10,
            number_of_entrances: 2,
            housing_type: "Updated Type".to_string(),
            energy_efficiency: EnergyEfficiency::High,
        }),
        SiteType::Bridge => SiteTypeDetails::Bridge(Bridge {
            site_id: id,
            length: 200.0,
            road_material: "Updated Material".to_string(),
            max_load: 20.0,
        }),
        SiteType::Park => SiteTypeDetails::Park(Park {
            site_id: id,
            area: 2.5,
            has_playground: true,
            has_lighting: true,
        }),
    };
    
    let site_details = SiteDetails {
        site,
        type_details,
    };
    
    let template = SiteDetailsTemplate { site_details };
    
    match template.render() {
        Ok(html) => Html(html),
        Err(err) => {
            eprintln!("Template error: {}", err);
            Html(format!("Error updating site ID: {}", id))
        }
    }
}

async fn delete_site(
    State(database): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Delete site
    // In a real implementation, this would delete the site from the database
    // For now, just return a success message
    let template = SuccessNotificationTemplate {
        message: format!("Site #{} was deleted successfully.", id),
    };
    
    match template.render() {
        Ok(html) => Html(html),
        Err(err) => {
            eprintln!("Template error: {}", err);
            Html(format!("Error deleting site ID: {}", id))
        }
    }
}

async fn fetch_site_schedule(
    State(database): State<Database>,
    Path(id): Path<i32>,
    Query(params): Query<PaginationParams>,
) -> Html<String> {
    // Fetch construction schedule
    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(10);
    
    // Mock data for tasks at this site
    let tasks = vec![
        Task {
            id: id * 100 + 1,
            site_id: id,
            brigade_id: Some(1),
            period_start: "2023-05-01".to_string(),
            expected_period_end: chrono::NaiveDate::parse_from_str("2023-05-15", "%Y-%m-%d").unwrap(),
            actual_period_end: Some(chrono::NaiveDate::parse_from_str("2023-05-14", "%Y-%m-%d").unwrap()),
            name: "Foundation work".to_string(),
            description: Some("Preparation and pouring of concrete foundation".to_string()),
            brigade_name: Some("Foundation Brigade".to_string()),
            materials_count: Some(5),
        },
        Task {
            id: id * 100 + 2,
            site_id: id,
            brigade_id: Some(2),
            period_start: "2023-05-16".to_string(),
            expected_period_end: chrono::NaiveDate::parse_from_str("2023-06-05", "%Y-%m-%d").unwrap(),
            actual_period_end: Some(chrono::NaiveDate::parse_from_str("2023-06-08", "%Y-%m-%d").unwrap()),
            name: "Frame construction".to_string(),
            description: Some("Building the main structural frame".to_string()),
            brigade_name: Some("Structure Brigade".to_string()),
            materials_count: Some(8),
        },
        Task {
            id: id * 100 + 3,
            site_id: id,
            brigade_id: Some(3),
            period_start: "2023-06-09".to_string(),
            expected_period_end: chrono::NaiveDate::parse_from_str("2023-06-25", "%Y-%m-%d").unwrap(),
            actual_period_end: None,
            name: "Electrical installation".to_string(),
            description: Some("Installing all electrical wiring and fixtures".to_string()),
            brigade_name: Some("Electrical Brigade".to_string()),
            materials_count: Some(12),
        },
        Task {
            id: id * 100 + 4,
            site_id: id,
            brigade_id: Some(4),
            period_start: "2023-06-10".to_string(),
            expected_period_end: chrono::NaiveDate::parse_from_str("2023-06-30", "%Y-%m-%d").unwrap(),
            actual_period_end: None,
            name: "Plumbing installation".to_string(),
            description: Some("Installing all plumbing systems".to_string()),
            brigade_name: Some("Plumbing Brigade".to_string()),
            materials_count: Some(7),
        },
        Task {
            id: id * 100 + 5,
            site_id: id,
            brigade_id: None,
            period_start: "2023-07-01".to_string(),
            expected_period_end: chrono::NaiveDate::parse_from_str("2023-07-15", "%Y-%m-%d").unwrap(),
            actual_period_end: None,
            name: "Interior finishing".to_string(),
            description: Some("Painting, flooring, and interior finishing work".to_string()),
            brigade_name: None,
            materials_count: Some(15),
        },
    ];
    
    // Mock data for available brigades
    let brigades = vec![
        Brigade {
            id: 1,
            brigadier_name: "John Builder".to_string(),
            workers_count: 12,
        },
        Brigade {
            id: 2,
            brigadier_name: "Maria Constructa".to_string(),
            workers_count: 8,
        },
        Brigade {
            id: 3,
            brigadier_name: "Alex Electrician".to_string(),
            workers_count: 6,
        },
        Brigade {
            id: 4,
            brigadier_name: "Sam Plumber".to_string(),
            workers_count: 5,
        },
        Brigade {
            id: 5,
            brigadier_name: "Tina Finisher".to_string(),
            workers_count: 10,
        },
    ];
    
    let template = ScheduleTabTemplate {
        site_id: id,
        tasks,
        brigades,
        current_page: page,
        total_pages: 1, // Mock single page
    };
    
    match template.render() {
        Ok(html) => Html(html),
        Err(err) => {
            eprintln!("Template error: {}", err);
            Html(format!("Error fetching schedule for site ID: {}", id))
        }
    }
}

async fn fetch_site_materials(
    State(database): State<Database>,
    Path(id): Path<i32>,
    Query(params): Query<PaginationParams>,
) -> Html<String> {
    // Fetch materials usage
    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(10);
    
    // Mock data for materials at this site
    let materials = vec![
        MaterialUsage {
            task_id: id * 100 + 1,
            material_id: 1,
            material_name: "Concrete".to_string(),
            expected_amount: 45.0,
            actual_amount: Some(47.5),
            units: "cubic meters".to_string(),
            cost_per_unit: 120.0,
            total_cost: 5700.0,
        },
        MaterialUsage {
            task_id: id * 100 + 1,
            material_id: 2,
            material_name: "Rebar".to_string(),
            expected_amount: 2.5,
            actual_amount: Some(2.3),
            units: "tons".to_string(),
            cost_per_unit: 950.0,
            total_cost: 2185.0,
        },
        MaterialUsage {
            task_id: id * 100 + 2,
            material_id: 3,
            material_name: "Structural Steel".to_string(),
            expected_amount: 12.0,
            actual_amount: Some(12.5),
            units: "tons".to_string(),
            cost_per_unit: 1500.0,
            total_cost: 18750.0,
        },
        MaterialUsage {
            task_id: id * 100 + 2,
            material_id: 4,
            material_name: "Bolts and Fasteners".to_string(),
            expected_amount: 500.0,
            actual_amount: Some(520.0),
            units: "kg".to_string(),
            cost_per_unit: 15.0,
            total_cost: 7800.0,
        },
        MaterialUsage {
            task_id: id * 100 + 3,
            material_id: 5,
            material_name: "Electrical Cable".to_string(),
            expected_amount: 2000.0,
            actual_amount: None,
            units: "meters".to_string(),
            cost_per_unit: 5.0,
            total_cost: 10000.0,
        },
        MaterialUsage {
            task_id: id * 100 + 3,
            material_id: 6,
            material_name: "Electrical Panels".to_string(),
            expected_amount: 10.0,
            actual_amount: None,
            units: "units".to_string(),
            cost_per_unit: 350.0,
            total_cost: 3500.0,
        },
        MaterialUsage {
            task_id: id * 100 + 4,
            material_id: 7,
            material_name: "PVC Pipes".to_string(),
            expected_amount: 300.0,
            actual_amount: None,
            units: "meters".to_string(),
            cost_per_unit: 12.0,
            total_cost: 3600.0,
        },
    ];
    
    let template = MaterialsTabTemplate {
        site_id: id,
        materials,
        current_page: page,
        total_pages: 1, // Mock single page
    };
    
    match template.render() {
        Ok(html) => Html(html),
        Err(err) => {
            eprintln!("Template error: {}", err);
            Html(format!("Error fetching materials for site ID: {}", id))
        }
    }
}

async fn fetch_site_equipment(
    State(database): State<Database>,
    Path(id): Path<i32>,
    Query(params): Query<PaginationParams>,
) -> Html<String> {
    // Fetch allocated equipment
    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(10);
    
    // Mock data for equipment at this site
    let equipment_allocations = vec![
        EquipmentAllocation {
            equipment_id: 1,
            equipment_name: "Tower Crane XL-5000".to_string(),
            amount: 1,
            period_start: "2023-05-01".to_string(),
            period_end: "2023-08-31".to_string(),
        },
        EquipmentAllocation {
            equipment_id: 2,
            equipment_name: "Excavator CAT-320".to_string(),
            amount: 2,
            period_start: "2023-05-01".to_string(),
            period_end: "2023-05-15".to_string(),
        },
        EquipmentAllocation {
            equipment_id: 3,
            equipment_name: "Concrete Mixer B-2000".to_string(),
            amount: 2,
            period_start: "2023-05-10".to_string(),
            period_end: "2023-05-20".to_string(),
        },
        EquipmentAllocation {
            equipment_id: 4,
            equipment_name: "Dump Truck HD-10".to_string(),
            amount: 3,
            period_start: "2023-05-05".to_string(),
            period_end: "2023-06-15".to_string(),
        },
        EquipmentAllocation {
            equipment_id: 5,
            equipment_name: "Scaffold System S-2000".to_string(),
            amount: 10,
            period_start: "2023-06-01".to_string(),
            period_end: "2023-07-31".to_string(),
        },
    ];
    
    let template = EquipmentTabTemplate {
        site_id: id,
        equipment_allocations,
        current_page: page,
        total_pages: 1, // Mock single page
    };
    
    match template.render() {
        Ok(html) => Html(html),
        Err(err) => {
            eprintln!("Template error: {}", err);
            Html(format!("Error fetching equipment for site ID: {}", id))
        }
    }
}

async fn fetch_site_brigades(
    State(database): State<Database>,
    Path(id): Path<i32>,
    Query(params): Query<PaginationParams>,
) -> Html<String> {
    // Fetch assigned brigades
    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(10);
    
    // Mock data for brigades at this site
    let brigades = vec![
        Brigade {
            id: 1,
            brigadier_name: "John Builder".to_string(),
            workers_count: 12,
        },
        Brigade {
            id: 2,
            brigadier_name: "Maria Constructa".to_string(),
            workers_count: 8,
        },
        Brigade {
            id: 3,
            brigadier_name: "Alex Electrician".to_string(),
            workers_count: 6,
        },
        Brigade {
            id: 4,
            brigadier_name: "Sam Plumber".to_string(),
            workers_count: 5,
        },
    ];
    
    let template = BrigadesTabTemplate {
        site_id: id,
        brigades,
        current_page: page,
        total_pages: 1, // Mock single page
    };
    
    match template.render() {
        Ok(html) => Html(html),
        Err(err) => {
            eprintln!("Template error: {}", err);
            Html(format!("Error fetching brigades for site ID: {}", id))
        }
    }
}

async fn fetch_site_reports(
    State(database): State<Database>,
    Path(id): Path<i32>,
    Query(params): Query<PaginationParams>,
) -> Html<String> {
    // Fetch site reports
    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(10);
    
    // Mock data for reports for this site
    let reports = vec![
        Report {
            id: id * 100 + 1,
            title: "Foundation Completion Report".to_string(),
            date: "2023-05-15".to_string(),
            report_type: "construction".to_string(),
            status: "approved".to_string(),
        },
        Report {
            id: id * 100 + 2,
            title: "June Material Usage Report".to_string(),
            date: "2023-06-30".to_string(),
            report_type: "materials".to_string(),
            status: "generated".to_string(),
        },
        Report {
            id: id * 100 + 3,
            title: "Q2 Equipment Allocation Report".to_string(),
            date: "2023-06-30".to_string(),
            report_type: "equipment".to_string(),
            status: "draft".to_string(),
        },
        Report {
            id: id * 100 + 4,
            title: "First Phase Inspection Report".to_string(),
            date: "2023-07-01".to_string(),
            report_type: "inspection".to_string(),
            status: "approved".to_string(),
        },
        Report {
            id: id * 100 + 5,
            title: "Brigade Performance Report".to_string(),
            date: "2023-07-05".to_string(),
            report_type: "brigades".to_string(),
            status: "generated".to_string(),
        },
    ];
    
    let template = ReportsTabTemplate {
        site_id: id,
        reports,
        current_page: page,
        total_pages: 1, // Mock single page
    };
    
    match template.render() {
        Ok(html) => Html(html),
        Err(err) => {
            eprintln!("Template error: {}", err);
            Html(format!("Error fetching reports for site ID: {}", id))
        }
    }
}

async fn fetch_site_type_fields(
    State(database): State<Database>,
    Query(params): Query<SiteTypeQuery>,
) -> Html<String> {
    // Fetch form fields for selected site type
    let site_type = params.site_type;
    
    // No pre-filled values for new sites
    let power_plant = None;
    let road = None;
    let housing = None;
    let bridge = None;
    let park = None;
    
    let template = TypeFieldsTemplate {
        site_type,
        power_plant,
        road,
        housing,
        bridge,
        park,
    };
    
    match template.render() {
        Ok(html) => Html(html),
        Err(err) => {
            eprintln!("Template error: {}", err);
            Html("Error fetching site type fields".to_string())
        }
    }
}

async fn create_site_task(
    State(database): State<Database>,
    Path(id): Path<i32>,
    Form(task): Form<TaskCreate>,
) -> Html<String> {
    // Create new task for site
    // In a real implementation, this would create a new task in the database
    // For now, just return a success message
    let template = SuccessNotificationTemplate {
        message: format!("Task '{}' was created successfully for site #{}.", task.name, id),
    };
    
    match template.render() {
        Ok(html) => Html(html),
        Err(err) => {
            eprintln!("Template error: {}", err);
            Html(format!("Error creating task for site ID: {}", id))
        }
    }
}

async fn fetch_sites_by_department(
    State(database): State<Database>,
    Path(dept_id): Path<i32>,
    Query(params): Query<PaginationParams>,
) -> Html<String> {
    // Fetch sites by department
    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(10);
    
    // Mock data for sites in this department
    let sites = vec![
        Site {
            id: 1,
            name: "Downtown Tower".to_string(),
            area_id: 1,
            client_id: 101,
            site_type: SiteType::Housing,
            location: Location { latitude: 55.7558, longitude: 37.6173 },
            risk_level: RiskLevel::Medium,
            description: Some("25-floor commercial building in the city center".to_string()),
            area_name: Some("Downtown Construction Area".to_string()),
            client_name: Some("Metro Development Corp".to_string()),
            department_id: Some(dept_id),
            department_name: Some(format!("Department #{}", dept_id)),
            tasks_count: Some(12),
            equipment_count: Some(5),
            brigades_count: Some(3),
        },
        Site {
            id: 2,
            name: "Riverside Bridge".to_string(),
            area_id: 2,
            client_id: 102,
            site_type: SiteType::Bridge,
            location: Location { latitude: 55.7522, longitude: 37.6156 },
            risk_level: RiskLevel::High,
            description: Some("600m bridge across the river".to_string()),
            area_name: Some("Riverside Construction Area".to_string()),
            client_name: Some("City Administration".to_string()),
            department_id: Some(dept_id),
            department_name: Some(format!("Department #{}", dept_id)),
            tasks_count: Some(8),
            equipment_count: Some(7),
            brigades_count: Some(2),
        },
    ];
    
    let template = SiteRowsTemplate { sites };
    
    match template.render() {
        Ok(html) => Html(html),
        Err(err) => {
            eprintln!("Template error: {}", err);
            Html(format!("Error fetching sites for department ID: {}", dept_id))
        }
    }
}

async fn fetch_sites_by_area(
    State(database): State<Database>,
    Path(area_id): Path<i32>,
    Query(params): Query<PaginationParams>,
) -> Html<String> {
    // Fetch sites by area
    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(10);
    
    // Mock data for sites in this area
    let sites = vec![
        Site {
            id: 1,
            name: "Downtown Tower".to_string(),
            area_id: area_id,
            client_id: 101,
            site_type: SiteType::Housing,
            location: Location { latitude: 55.7558, longitude: 37.6173 },
            risk_level: RiskLevel::Medium,
            description: Some("25-floor commercial building in the city center".to_string()),
            area_name: Some(format!("Area #{}", area_id)),
            client_name: Some("Metro Development Corp".to_string()),
            department_id: Some(1),
            department_name: Some("North Construction Department".to_string()),
            tasks_count: Some(12),
            equipment_count: Some(5),
            brigades_count: Some(3),
        },
        Site {
            id: 2,
            name: "Riverside Bridge".to_string(),
            area_id: area_id,
            client_id: 102,
            site_type: SiteType::Bridge,
            location: Location { latitude: 55.7522, longitude: 37.6156 },
            risk_level: RiskLevel::High,
            description: Some("600m bridge across the river".to_string()),
            area_name: Some(format!("Area #{}", area_id)),
            client_name: Some("City Administration".to_string()),
            department_id: Some(1),
            department_name: Some("North Construction Department".to_string()),
            tasks_count: Some(8),
            equipment_count: Some(7),
            brigades_count: Some(2),
        },
    ];
    
    let template = SiteRowsTemplate { sites };
    
    match template.render() {
        Ok(html) => Html(html),
        Err(err) => {
            eprintln!("Template error: {}", err);
            Html(format!("Error fetching sites for area ID: {}", area_id))
        }
    }
}

async fn fetch_risk_levels() -> Html<String> {
    // Fetch available risk levels
    let selected_risk_level = None; // No pre-selected level
    
    let template = RiskLevelSelectorTemplate { selected_risk_level };
    
    match template.render() {
        Ok(html) => Html(html),
        Err(err) => {
            eprintln!("Template error: {}", err);
            Html("Error fetching risk levels".to_string())
        }
    }
}

pub fn router() -> Router<Database> {
    Router::new()
        // Page Endpoints
        .route("/sites", get(sites_page))
        .route("/sites/new", get(site_new_page))
        .route("/sites/{id}", get(site_details_page))
        .route("/sites/{id}/edit", get(site_edit_page))
        // HTMX Endpoints
        .route("/api/sites", get(fetch_sites).post(create_site))
        .route("/api/sites/{id}", get(fetch_site_details).put(update_site).delete(delete_site))
        .route("/api/sites/{id}/schedule", get(fetch_site_schedule))
        .route("/api/sites/{id}/materials", get(fetch_site_materials))
        .route("/api/sites/{id}/equipment", get(fetch_site_equipment))
        .route("/api/sites/{id}/brigades", get(fetch_site_brigades))
        .route("/api/sites/{id}/reports", get(fetch_site_reports))
        .route("/api/sites/type-fields", get(fetch_site_type_fields))
        .route("/api/sites/{id}/tasks", post(create_site_task))
        .route("/api/sites/by-department/{dept_id}", get(fetch_sites_by_department))
        .route("/api/sites/by-area/{area_id}", get(fetch_sites_by_area))
        .route("/api/sites/risk-levels", get(fetch_risk_levels))
}
