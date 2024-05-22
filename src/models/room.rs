use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RoomModel{
    name: String,
    house: String, // id of house
    rtype: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoomTypeModel{
    id: String,
    name: String,
    image: String // Path of image
}