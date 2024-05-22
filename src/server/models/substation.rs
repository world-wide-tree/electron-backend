use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct PostSubstationSDto{
    pub code: String, // unique code of substation
    pub latitude: u64,
    pub longitude: u64,
    pub max_power: i64
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PutSubstationSDto{
    pub code: String, // unique code of substation
    pub latitude: f64,
    pub longitude: f64,
    pub max_power: i64
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubstationSDto{
    pub id: String,
    pub code: String, // unique code of substation
    pub latitude: f64,
    pub longitude: f64,
    pub max_power: u64
}

