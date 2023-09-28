use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[derive(Debug)]

struct Coordinate {
    x: i32,
    y: i32,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]

struct Candidate {
    plate: String,
    confidence: f64,
    matches_template: i32,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]

struct BestPlate {
    plate: String,
    confidence: f64,
    matches_template: i32,
    plate_index: i32,
    region: String,
    region_confidence: i32,
    processing_time_ms: f64,
    requested_topn: i32,
    coordinates: Vec<Coordinate>,
    plate_crop_jpeg: String,
    vehicle_region: Coordinate,
    vehicle_detected: bool,
    candidates: Vec<Candidate>,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
struct VehiclePath {
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    t: i32,
    f: i32,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]

struct NamedConfidence {
    name: String,
    confidence: f64,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]

struct Vehicle {
    color: Vec<NamedConfidence>,
    make: Vec<NamedConfidence>,
    make_model: Vec<NamedConfidence>,
    body_type: Vec<NamedConfidence>,
    year: Vec<NamedConfidence>,
    orientation: Vec<NamedConfidence>,
    missing_plate: Vec<NamedConfidence>,
    is_vehicle: Vec<NamedConfidence>,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct MainStruct {
    data_type: String,
    version: i32,
    epoch_start: i64,
    epoch_end: i64,
    frame_start: i32,
    frame_end: i32,
    company_id: String,
    agent_uid: String,
    agent_version: String,
    agent_type: String,
    camera_id: i32,
    gps_latitude: f64,
    gps_longitude: f64,
    country: String,
    uuids: Vec<String>,
    vehicle_path: Vec<VehiclePath>,
    plate_indexes: Vec<i32>,
    candidates: Vec<Candidate>,
    best_plate: BestPlate,
    best_confidence: f64,
    best_plate_number: String,
    best_region: String,
    best_region_confidence: f64,
    matches_template: bool,
    plate_path: Vec<VehiclePath>,
    vehicle_crop_jpeg: String,
    overview_jpeg: String,
    best_uuid: String,
    best_uuid_epoch_ms: i64,
    best_image_width: i32,
    best_image_height: i32,
    travel_direction: f64,
    is_parked: bool,
    is_preview: bool,
    vehicle_signature: String,
    vehicle: Vehicle,
    web_server_config: serde_json::Value, // Using serde_json::Value as the fields are not defined
    direction_of_travel_id: i32,
    custom_data: String,
}
