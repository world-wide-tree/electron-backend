use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use crate::{data_store::{controllers::room::ROOM_TABLE, models::device::DeviceTypeSurreal}, models::dto::device::{CreateDeviceDto, CreateDeviceTypeDto, UpdateDeviceDto, UpdateDeviceTypeDto}};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateDeviceSurreal{
    pub name: String,
    pub state: bool,
    pub room: Thing,
    pub dtype: DeviceTypeSurreal
}
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateDeviceSurreal{
    pub name: String,
    pub state: bool,
    pub room: Thing,
    pub dtype: DeviceTypeSurreal
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateDeviceTypeSurreal{
    pub name: String
}
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateDeviceTypeSurreal{
    pub name: String
}


impl From<UpdateDeviceDto> for UpdateDeviceSurreal{
    fn from(value: UpdateDeviceDto) -> Self {
        Self { 
            name: value.name, 
            state: value.state.into(), 
            room: Thing::from((ROOM_TABLE, value.room.as_str())), 
            dtype: DeviceTypeSurreal::from(value.dtype) 
        }
    }
}
impl From<CreateDeviceDto> for CreateDeviceSurreal{
    fn from(value: CreateDeviceDto) -> Self {
        Self { 
            name: value.name, 
            state: value.state.into(), 
            room: Thing::from((ROOM_TABLE, value.room.as_str())), 
            dtype: DeviceTypeSurreal::from(value.dtype) 
        }
    }
}

impl From<CreateDeviceTypeDto> for CreateDeviceTypeSurreal{
    fn from(value: CreateDeviceTypeDto) -> Self {
        Self { name: value.name }
    }
}

impl From<UpdateDeviceTypeDto> for UpdateDeviceTypeSurreal{
    fn from(value: UpdateDeviceTypeDto) -> Self {
        Self { name: value.name }
    }
}