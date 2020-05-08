
use serde_derive::{Serialize, Deserialize};
use std::collections::HashMap;


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

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(non_snake_case)]
pub struct Signature {
    pub keyid: String,
    pub method: String,
    pub sig: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(non_snake_case)]
pub struct FileInfo {
    pub hashes: HashMap<String, String>,
    pub length: usize,
}

pub struct SignedPayload<T> {
    pub signatures: Vec<Signature>,
    pub signed: T,
}


pub struct TimeAttestation {
    /// eg [ 1, 2 ]
    pub nonces: Vec<u32>,
    /// eg "2017-05-18T16:23:13Z"
    pub time: String,
}



