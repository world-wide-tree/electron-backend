use serde::{Deserialize, Serialize};

use crate::models::dto::substation::{CreateSubstationDto, UpdateSubstationDto};

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

impl From<UpdateSubstationDto> for UpdateSubstationSurreal{
    fn from(value: UpdateSubstationDto) -> Self {
        Self { 
            code: value.code, 
            latitude: value.latitude, 
            longitude: value.longitude, 
            max_power: value.max_power 
        }
    }
}
impl From<CreateSubstationDto> for CreateSubstationSurreal{
    fn from(value: CreateSubstationDto) -> Self {
        Self { 
            code: value.code, 
            latitude: value.latitude, 
            longitude: value.longitude, 
            max_power: value.max_power 
        }
    }
}