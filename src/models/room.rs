use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RoomModel{
    pub id: String,
    pub name: String,
    pub hame: String, // id of house
    pub rtype: RoomTypeModel
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoomTypeModel{
    pub id: String,
    pub name: String,
    pub image: String // Path of image
}