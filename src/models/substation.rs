use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SubstationModel{
    pub id: String,
    pub code: String, // unique code of substation
    pub latitude: f64,
    pub longitude: f64,
    pub max_power: u64

}