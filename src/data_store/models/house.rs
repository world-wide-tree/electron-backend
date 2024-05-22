use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use crate::{data_store::controllers::{home::HOME_TABLE, substation::SUBSTATION_TABLE, user::USER_TABLE}, models::house::HomeModel};

#[derive(Debug, Serialize, Deserialize)]
pub struct HomeSurreal{
    id: Thing,
    name: String,
    address: String,
    latitude: f64,
    longtitude: f64,
    substation: Thing, // id of substation
    user: Thing // id of user
}

impl Into<HomeModel> for HomeSurreal{
    fn into(self) -> HomeModel {
        HomeModel { 
            id: self.id.id.to_string(), 
            name: self.name, 
            address: self.address, 
            latitude: self.latitude, 
            longtitude: self.longtitude, 
            substation: self.substation.id.to_string(), 
            user: self.user.id.to_string() 
        }
    }
}

impl From<HomeModel> for HomeSurreal{
    fn from(value: HomeModel) -> Self {
        Self { 
            id: Thing::from((HOME_TABLE, value.id.as_str())), 
            name: value.name, 
            address: value.address, 
            latitude: value.latitude, 
            longtitude: value.longtitude, 
            substation: Thing::from((SUBSTATION_TABLE, value.substation.as_str())), 
            user: Thing::from((USER_TABLE, value.user.as_str()))
        }
    }
}