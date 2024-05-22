use serde::{Deserialize, Serialize};

use crate::models::device::{DeviceState, DeviceTypeModel};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateDeviceDto{
    pub name: String,
    pub state: DeviceState,
    pub room: String, // id of room
    pub dtype: DeviceTypeModel, // id of device-type
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateDeviceDto{
    pub name: String,
    pub state: DeviceState,
    pub room: String, // id of room
    pub dtype: DeviceTypeModel, // id of device-type
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateDeviceTypeDto{
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateDeviceTypeDto{
    pub name: String
}