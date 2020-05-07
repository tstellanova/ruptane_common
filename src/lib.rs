
use serde_derive::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VehicleUpdateRequest {
    pub vehicle_id: String,
    pub last_update: String,
    //TODO add a vehicle manifest
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VehicleUpdateResponse {
    pub resp_version: String,
    pub vehicle_id: String,
    pub message: String,
}


