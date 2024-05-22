use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceModel{
    pub id: String,
    pub name: String,
    pub state: DeviceState,
    pub room: String, // id of room
    pub dtype: DeviceTypeModel, // id of device-type
}

#[derive(Debug, Serialize, Deserialize)]
pub enum DeviceState{
    ON,
    OFF
}

impl From<bool> for DeviceState{
    fn from(value: bool) -> Self {
        if value {
            Self::ON
        } else {
            Self::OFF
        }
    }
}
impl Into<bool> for DeviceState{
    fn into(self) -> bool {
        match self {
            Self::ON => true,
            Self::OFF => false
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceTypeModel{
    pub id: String,
    pub name: String
}

