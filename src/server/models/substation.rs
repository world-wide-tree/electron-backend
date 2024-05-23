use serde::{Deserialize, Serialize};

use crate::models::{dto::{device::CreateDeviceDto, substation::{CreateSubstationDto, UpdateSubstationDto}}, substation::SubstationModel};


#[derive(Debug, Serialize, Deserialize)]
pub struct PostSubstationSDto{
    pub code: String, // unique code of substation
    pub latitude: f64,
    pub longitude: f64,
    pub max_power: u64
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PutSubstationSDto{
    pub code: String, // unique code of substation
    pub latitude: f64,
    pub longitude: f64,
    pub max_power: u64
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubstationSDto{
    pub id: String,
    pub code: String, // unique code of substation
    pub latitude: f64,
    pub longitude: f64,
    pub max_power: u64
}

impl Into<UpdateSubstationDto> for PutSubstationSDto{
    fn into(self) -> UpdateSubstationDto {
        UpdateSubstationDto { 
            code: self.code, 
            latitude: self.latitude, 
            longitude: self.longitude, 
            max_power: self.max_power 
        }
    }
}

impl From<SubstationModel> for SubstationSDto{
    fn from(value: SubstationModel) -> Self {
        Self { 
            id: value.id, 
            code: value.code, 
            latitude: value.latitude, 
            longitude: value.longitude, 
            max_power: value.max_power 
        }
    }
}

impl Into<CreateSubstationDto> for PostSubstationSDto{
    fn into(self) -> CreateSubstationDto {
        CreateSubstationDto { 
            code: self.code, 
            latitude: self.latitude, 
            longitude: self.longitude, 
            max_power: self.max_power 
        }
    }
}