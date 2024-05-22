use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use crate::{data_store::controllers::{substation::SUBSTATION_TABLE, user::USER_TABLE}, models::dto::home::{CreateHomeDto, UpdateHomeDto}};

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateHomeSurreal{
    name: String,
    address: String,
    latitude: f64,
    longtitude: f64,
    substation: Thing, // id of substation
    user: Thing // id of user
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateHomeSurreal{
    name: String,
    address: String,
    latitude: f64,
    longtitude: f64,
    substation: Thing, // id of substation
    user: Thing // id of user
}

impl From<CreateHomeDto> for CreateHomeSurreal{
    fn from(value: CreateHomeDto) -> Self {
        Self { 
            name: value.name, 
            address: value.address, 
            latitude: value.latitude, 
            longtitude: value.longtitude, 
            substation: Thing::from((SUBSTATION_TABLE, value.substation.as_str())), 
            user: Thing::from((USER_TABLE, value.user.as_str()))
        }
    }
}
impl From<UpdateHomeDto> for UpdateHomeSurreal{
    fn from(value: UpdateHomeDto) -> Self {
        Self { 
            name: value.name, 
            address: value.address, 
            latitude: value.latitude, 
            longtitude: value.longtitude, 
            substation: Thing::from((SUBSTATION_TABLE, value.substation.as_str())), 
            user: Thing::from((USER_TABLE, value.user.as_str()))
        }
    }
}