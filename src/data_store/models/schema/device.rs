use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use crate::{data_store::{controllers::{device::{DEVICE_TABLE, DEVICE_TYPE_TABLE}, room::ROOM_TABLE}, models::device::{DeviceSurreal, DeviceTypeSurreal}}, models::device::{DeviceModel, DeviceTypeModel}};

#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceSurrealSchema{
    pub id: Thing,
    pub name: String,
    pub state: bool,
    pub room: Thing,
    pub dtype: Thing
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceTypeSurrealSchema{
    pub id: Thing,
    pub name: String
}

impl From<DeviceSurreal> for DeviceSurrealSchema{
    fn from(value: DeviceSurreal) -> Self {
        Self { 
            id: value.id, 
            name: value.name, 
            state: value.state, 
            room: value.room, 
            dtype: value.dtype.id
        }
    }
}

impl From<DeviceTypeSurreal> for DeviceTypeSurrealSchema{
    fn from(value: DeviceTypeSurreal) -> Self {
        Self { 
            id: value.id, 
            name: value.name 
        }
    }
}