use serde::{Deserialize, Serialize};

use crate::models::{device::{DeviceModel, DeviceState, DeviceTypeModel}, dto::device::{CreateDeviceDto, CreateDeviceTypeDto, UpdateDeviceDto, UpdateDeviceTypeDto}};

#[derive(Debug, Serialize, Deserialize)]
pub struct PostDeviceSDto{
    pub name: String,
    pub state: bool,
    pub room: String, // id of room
    pub dtype: DeviceTypeSDto, // id of device-type
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PutDeviceSDto{
    pub name: String,
    pub state: bool,
    pub room: String, // id of room
    pub dtype: DeviceTypeSDto, // id of device-type
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceSDto{
    pub id: String,
    pub name: String,
    pub state: bool,
    pub room: String, // id of room
    pub dtype: DeviceTypeSDto, // id of device-type
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PostDeviceTypeSDto{
    pub name: String,
    
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PutDeviceTypeSDto{
    pub name: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceTypeSDto{
    pub id: String,
    pub name: String
}

impl Into<DeviceTypeModel> for DeviceTypeSDto{
    fn into(self) -> DeviceTypeModel {
        DeviceTypeModel { 
            id: self.id, 
            name: self.name 
        }
    }
}

impl Into<CreateDeviceDto> for PostDeviceSDto{
    fn into(self) -> CreateDeviceDto {
        CreateDeviceDto { 
            name: self.name, 
            state: self.state.into(), 
            room: self.room, 
            dtype: self.dtype.into() 
        }
    }
}
impl Into<UpdateDeviceDto> for PutDeviceSDto{
    fn into(self) -> UpdateDeviceDto {
        UpdateDeviceDto { 
            name: self.name, 
            state: DeviceState::from(self.state), 
            room: self.room, 
            dtype: self.dtype.into()
        }
    }
}
impl From<DeviceModel> for DeviceSDto{
    fn from(value: DeviceModel) -> Self {
        Self { 
            id: value.id, 
            name: value.name, 
            state: value.state.into(), 
            room: value.room, 
            dtype: DeviceTypeSDto::from(value.dtype) 
        }
    }
}
impl Into<CreateDeviceTypeDto> for PostDeviceTypeSDto{
    fn into(self) -> CreateDeviceTypeDto {
        CreateDeviceTypeDto { 
            name: self.name 
        }
    }
}
impl Into<UpdateDeviceTypeDto> for PutDeviceTypeSDto{
    fn into(self) -> UpdateDeviceTypeDto {
        UpdateDeviceTypeDto { name: self.name }
    }
}
impl From<DeviceTypeModel> for DeviceTypeSDto{
    fn from(value: DeviceTypeModel) -> Self {
        Self { 
            id: value.id, 
            name: value.name 
        }
    }
}