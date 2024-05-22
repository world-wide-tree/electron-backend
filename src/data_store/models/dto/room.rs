use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use crate::{data_store::{controllers::home::HOME_TABLE, models::room::RoomTypeSurreal}, models::dto::room::{CreateRoomDto, CreateRoomTypeDto, UpdateRoomDto, UpdateRoomTypeDto}};

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateRoomSurreal{
    name: String,
    home: Thing, // id of house
    rtype: RoomTypeSurreal
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateRoomSurreal{
    name: String,
    home: Thing, // id of house
    rtype: RoomTypeSurreal
}
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateRoomTypeSurreal{
    name: String,
    image: String // Path of image
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateRoomTypeSurreal{
    name: String,
    image: String // Path of image
}


impl From<UpdateRoomDto> for UpdateRoomSurreal{
    fn from(value: UpdateRoomDto) -> Self {
        Self { 
            name: value.name, 
            home: Thing::from((HOME_TABLE,value.home.as_str())), 
            rtype: RoomTypeSurreal::from(value.rtype)
        }
    }
}
impl From<CreateRoomDto> for CreateRoomSurreal{
    fn from(value: CreateRoomDto) -> Self {
        Self { 
            name: value.name, 
            home: Thing::from((HOME_TABLE,value.home.as_str())), 
            rtype: RoomTypeSurreal::from(value.rtype) 
        }
    }
}
impl From<CreateRoomTypeDto> for CreateRoomTypeSurreal{
    fn from(value: CreateRoomTypeDto) -> Self {
        Self { 
            name: value.name, 
            image: value.image 
        }
    }
}
impl From<UpdateRoomTypeDto> for UpdateRoomTypeSurreal{
    fn from(value: UpdateRoomTypeDto) -> Self {
        Self { 
            name: value.name, 
            image: value.image 
        }
    }
}