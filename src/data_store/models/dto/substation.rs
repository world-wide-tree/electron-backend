use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateSubstationSurreal{
    code: String, // unique code of substation
    latitude: f64,
    longitude: f64,
    max_power: u64
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSubstationSurreal{
    code: String, // unique code of substation
    latitude: f64,
    longitude: f64,
    max_power: u64
}