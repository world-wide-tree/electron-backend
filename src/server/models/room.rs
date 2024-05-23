use serde::{Deserialize, Serialize};

use crate::models::{dto::room::{CreateRoomDto, CreateRoomTypeDto, UpdateRoomDto, UpdateRoomTypeDto}, room::{RoomModel, RoomTypeModel}};


#[derive(Debug, Serialize, Deserialize)]
pub struct PostRoomSDto{
    name: String,
    home: String, // id of house
    rtype: RoomTypeSDto
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PutRoomSDto{
    name: String,
    home: String, // id of house
    rtype: RoomTypeSDto
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoomSDto{
    pub id: String,
    pub name: String,
    pub home: String, // id of house
    pub rtype: RoomTypeSDto
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
    pub name: String,
    pub image: String // Path of image
}


impl From<RoomTypeModel> for RoomTypeSDto{
    fn from(value: RoomTypeModel) -> Self {
        Self { 
            id: value.id, 
            name: value.name, 
            image: value.image 
        }
    }
}
impl From<RoomModel> for RoomSDto{
    fn from(value: RoomModel) -> Self {
        Self { 
            id: value.id, 
            name: value.name, 
            home: value.home, 
            rtype: RoomTypeSDto::from(value.rtype)
        }
    }
}

impl Into<CreateRoomDto> for PostRoomSDto{
    fn into(self) -> CreateRoomDto {
        CreateRoomDto { 
            name: self.name, 
            home: self.home, 
            rtype: self.rtype.into()
        }
    }
}
impl Into<RoomTypeModel> for RoomTypeSDto{
    fn into(self) -> RoomTypeModel {
        RoomTypeModel { 
            id: self.id, 
            name: self.name, 
            image: self.image 
        }
    }
}
impl Into<UpdateRoomDto> for PutRoomSDto{
    fn into(self) -> UpdateRoomDto {
        UpdateRoomDto { 
            name: self.name, 
            home: self.home, 
            rtype: self.rtype.into()
        }
    }
}


impl Into<CreateRoomTypeDto> for PostRoomTypeSDto{
    fn into(self) -> CreateRoomTypeDto {
        CreateRoomTypeDto { 
            name: self.name, 
            image: self.image 
        }
    }
}

impl Into<UpdateRoomTypeDto> for PutRoomTypeSDto{
    fn into(self) -> UpdateRoomTypeDto {
        UpdateRoomTypeDto { 
            name: self.name,
            image: self.image
        }
    }
}