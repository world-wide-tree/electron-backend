use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct HouseModel{
    id: String,
    name: String,
    address: String,
    latitude: i64,
    longtitude: i64,
    substation: String, // id of substation
    user: String // id of user
}