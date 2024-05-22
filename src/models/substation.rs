use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Substation{
    code: String, // unique code of substation
    latitude: u64,
    longitude: u64,
    max_power: i64

}