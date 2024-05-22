use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use crate::{data_store::controllers::{home::HOME_TABLE, room::{ROOM_TABLE, ROOM_TYPE_TABLE}}, models::room::{RoomModel, RoomTypeModel}};


#[derive(Debug, Serialize, Deserialize)]
pub struct RoomSurrealSchema{
    id: Thing,
    name: String,
    home: Thing, // id of house
    rtype: Thing
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoomTypeSurrealSchema{
    id: Thing,
    name: String,
    image: String // Path of image
}


impl From<RoomModel> for RoomSurrealSchema{
    fn from(value: RoomModel) -> Self {
        Self { 
            id: Thing::from((ROOM_TABLE, value.id.as_str())), 
            name: value.name, 
            home: Thing::from((HOME_TABLE, value.hame.as_str())), 
            rtype: Thing::from((ROOM_TYPE_TABLE, value.rtype.id.as_str())), 
        }
    }
}


impl From<RoomTypeModel> for RoomTypeSurrealSchema{
    fn from(value: RoomTypeModel) -> Self {
        Self { 
            id: Thing::from((ROOM_TYPE_TABLE, value.id.as_str())), 
            name: value.name, 
            image: value.image 
        }
    }
}