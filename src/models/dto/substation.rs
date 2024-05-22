use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSubstationDto{
    pub code: String, // unique code of substation
    pub latitude: f64,
    pub longitude: f64,
    pub max_power: u64

}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateSubstationDto{
    pub code: String, // unique code of substation
    pub latitude: f64,
    pub longitude: f64,
    pub max_power: u64

}