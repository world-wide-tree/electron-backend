use serde::{Deserialize, Serialize};

use crate::models::{dto::home::{CreateHomeDto, UpdateHomeDto}, house::HomeModel};

#[derive(Debug, Serialize, Deserialize)]
pub struct PostHomeSDto{
    pub name: String,
    pub address: String,
    pub latitude: f64,
    pub longtitude: f64,
    pub substation: String, // id of substation
    pub user: String // id of user
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PutHomeSDto{
    pub name: String,
    pub address: String,
    pub latitude: f64,
    pub longtitude: f64,
    pub substation: String, // id of substation
    pub user: String // id of user
}
#[derive(Debug, Serialize, Deserialize)]
pub struct HomeSDto{
    pub id: String,
    pub name: String,
    pub address: String,
    pub latitude: f64,
    pub longtitude: f64,
    pub substation: String, // id of substation
    pub user: String // id of user
}

impl Into<CreateHomeDto> for PostHomeSDto{
    fn into(self) -> CreateHomeDto {
        CreateHomeDto { 
            name: self.name, 
            address: self.address, 
            latitude: self.latitude, 
            longtitude: self.longtitude, 
            substation: self.substation, 
            user: self.user 
        }
    }
}

impl Into<UpdateHomeDto> for PutHomeSDto{
    fn into(self) -> UpdateHomeDto {
        UpdateHomeDto { 
            name: self.name, 
            address: self.address, 
            latitude: self.latitude, 
            longtitude: self.longtitude, 
            substation: self.substation, 
            user: self.user 
        }
    }
}

impl From<HomeModel> for HomeSDto{
    fn from(value: HomeModel) -> Self {
        Self { 
            id: value.id, 
            name: value.name, 
            address: value.address, 
            latitude: value.latitude, 
            longtitude: value.longtitude, 
            substation: value.substation, 
            user: value.user 
        }
    }
}