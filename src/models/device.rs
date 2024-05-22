use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceModel{
    name: String,
    state: DeviceState,
    room: String, // id of room
    dtype: DeviceTypeModel, // id of device-type
}

#[derive(Debug, Serialize, Deserialize)]
pub enum DeviceState{
    ON,
    OFF
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceTypeModel{
    id: String,
    name: String
}

