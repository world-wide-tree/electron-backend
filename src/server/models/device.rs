use serde::{Deserialize, Serialize};

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