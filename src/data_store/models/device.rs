use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use crate::{data_store::controllers::{device::{DEVICE_TABLE, DEVICE_TYPE_TABLE}, room::ROOM_TABLE}, models::device::{DeviceModel, DeviceTypeModel}};

#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceSurreal{
    pub id: Thing,
    pub name: String,
    pub state: bool,
    pub room: Thing,
    pub dtype: DeviceTypeSurreal
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceTypeSurreal{
    pub id: Thing,
    pub name: String
}

impl Into<DeviceModel> for DeviceSurreal{
    fn into(self) -> DeviceModel {
        DeviceModel { 
            id: self.id.id.to_string(), 
            name: self.name, 
            state: self.state.into(), 
            room: self.room.id.to_string(), 
            dtype: self.dtype.into()
        }
    }
}
impl From<DeviceModel> for DeviceSurreal{
    fn from(value: DeviceModel) -> Self {
        Self { 
            id: Thing::from((DEVICE_TABLE, value.id.as_str())), 
            name: value.name, 
            state: value.state.into(), 
            room: Thing::from((ROOM_TABLE, value.room.as_str())), 
            dtype: DeviceTypeSurreal::from(value.dtype) 
        }
    }
}
impl Into<DeviceTypeModel> for DeviceTypeSurreal{
    fn into(self) -> DeviceTypeModel {
        DeviceTypeModel { 
            id: self.id.to_string(), 
            name: self.name
        }
    }
}
impl From<DeviceTypeModel> for DeviceTypeSurreal{
    fn from(value: DeviceTypeModel) -> Self {
        Self { 
            id: Thing::from((DEVICE_TYPE_TABLE, value.id.as_str())), 
            name: value.name
        }
    }
}