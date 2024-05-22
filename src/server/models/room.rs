use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct PostRoomSDto{
    name: String,
    house: String, // id of house
    rtype: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PutRoomSDto{
    name: String,
    house: String, // id of house
    rtype: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoomSDto{
    pub id: String,
    name: String,
    house: String, // id of house
    rtype: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostRoomTypeSDto{
    name: String,
    image: String // Path of image
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PutRoomTypeSDto{
    name: String,
    image: String // Path of image
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoomTypeSDto{
    pub id: String,
    name: String,
    image: String // Path of image
}