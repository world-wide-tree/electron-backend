use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use crate::data_store::models::room::RoomTypeSurreal;

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
