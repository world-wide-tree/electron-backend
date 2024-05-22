use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateHomeDto{
    pub name: String,
    pub address: String,
    pub latitude: f64,
    pub longtitude: f64,
    pub substation: String, // id of substation
    pub user: String // id of user
}
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateHomeDto{
    pub name: String,
    pub address: String,
    pub latitude: f64,
    pub longtitude: f64,
    pub substation: String, // id of substation
    pub user: String // id of user
}