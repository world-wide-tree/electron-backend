pub struct PostDeviceSDto{
    pub name: String,
    pub state: bool,
    pub room: String, // id of room
    pub dtype: DeviceTypeSDto, // id of device-type
}
pub struct PutDeviceSDto{
    pub name: String,
    pub state: bool,
    pub room: String, // id of room
    pub dtype: DeviceTypeSDto, // id of device-type
}
pub struct DeviceSDto{
    pub id: String,
    pub name: String,
    pub state: bool,
    pub room: String, // id of room
    pub dtype: DeviceTypeSDto, // id of device-type
}
pub struct PostDeviceTypeSDto{
    pub name: String,
    
}
pub struct PutDeviceTypeSDto{
    pub name: String,
}
pub struct DeviceTypeSDto{
    pub id: String,
    pub name: String
}