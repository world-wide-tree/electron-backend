use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use crate::{data_store::controllers::{home::HOME_TABLE, room::{ROOM_TABLE, ROOM_TYPE_TABLE}}, models::room::{RoomModel, RoomTypeModel}};


#[derive(Debug, Serialize, Deserialize)]
pub struct RoomSurreal{
    id: Thing,
    name: String,
    home: Thing, // id of house
    rtype: RoomTypeSurreal
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoomTypeSurreal{
    id: Thing,
    name: String,
    image: String // Path of image
}

impl Into<RoomModel> for RoomSurreal{
    fn into(self) -> RoomModel {
        RoomModel { 
            id: self.id.id.to_string(), 
            name: self.name, 
            home: self.home.id.to_string(), 
            rtype: self.rtype.into() 
        }
    }
}

impl From<RoomModel> for RoomSurreal{
    fn from(value: RoomModel) -> Self {
        Self { 
            id: Thing::from((ROOM_TABLE, value.id.as_str())), 
            name: value.name, 
            home: Thing::from((HOME_TABLE, value.home.as_str())), 
            rtype: RoomTypeSurreal::from(value.rtype)
        }
    }
}

impl Into<RoomTypeModel> for RoomTypeSurreal{
    fn into(self) -> RoomTypeModel {
        RoomTypeModel { 
            id: self.id.id.to_string(), 
            name: self.name, 
            image: self.image 
        }
    }
}

impl From<RoomTypeModel> for RoomTypeSurreal{
    fn from(value: RoomTypeModel) -> Self {
        Self { 
            id: Thing::from((ROOM_TYPE_TABLE, value.id.as_str())), 
            name: value.name, 
            image: value.image 
        }
    }
}