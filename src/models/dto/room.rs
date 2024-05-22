use serde::{Deserialize, Serialize};

use crate::models::room::RoomTypeModel;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateRoomDto{
    pub name: String,
    pub home: String, // id of house
    pub rtype: RoomTypeModel
}
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateRoomDto{
    pub name: String,
    pub home: String, // id of house
    pub rtype: RoomTypeModel
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateRoomTypeDto{
    pub name: String,
    pub image: String // Path of image
}
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateRoomTypeDto{
    pub name: String,
    pub image: String // Path of image
}