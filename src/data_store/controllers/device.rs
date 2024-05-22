use crate::{data_store::{db_pool, models::{device::{DeviceSurreal, DeviceTypeSurreal}, dto::device::{CreateDeviceTypeSurreal, UpdateDeviceSurreal, UpdateDeviceTypeSurreal}, pagination::PaginationDb}}, models::{device::{DeviceModel, DeviceTypeModel}, dto::device::{CreateDeviceDto, CreateDeviceTypeDto, UpdateDeviceDto, UpdateDeviceTypeDto}, pagination::Pagination, query_params::device::{DeviceQueryParams, DeviceTypeQueryParams}}};

use super::user::USER_TABLE;

pub static DEVICE_TABLE: &'static str = "Device";
pub static DEVICE_TYPE_TABLE: &'static str = "DeviceType";

pub async fn create_device_on_db(
    dto: CreateDeviceDto
) -> DeviceModel{
    let rst: DeviceSurreal = db_pool()
        .create(DEVICE_TABLE)
        .content(dto)
        .await
        .expect("Error creating records Device!")
        .pop()
        .expect("Device not created on DB!")
    ;
    rst.into()
}
pub async fn update_device_on_db(
    id: String,
    dto: UpdateDeviceDto
) -> DeviceModel{
    let rst: DeviceSurreal = db_pool()
        .update((DEVICE_TABLE, id.as_str()))
        .content(UpdateDeviceSurreal::from(dto))
        .await
        .expect("Error updating Device record!")
        .expect("Error device not found for update!")
    ;
    rst.into()
}
pub async fn get_device_by_id_on_db(
    id: String
) -> DeviceModel{
    let rst: DeviceSurreal = db_pool()
        .select((DEVICE_TABLE, id.as_str()))
        .await
        .expect("Error of selecting user by id!")
        .expect("Device Not found!")
    ;
    rst.into()
}
pub async fn list_device_on_db(
    params: DeviceQueryParams
) -> Pagination<DeviceModel>{
    let query0 = format!("LET $limit = {};",params.limit());
    let query1 = format!("LET $offset = {};",params.offset());
    let query2 = format!("LET $start = {};",params.start());
    let query3 = format!(
        "LET $rst = (SELECT * FROM {} START $start LIMIT $limit);",
        DEVICE_TABLE
    );
    let query4 = format!("LET $total = COUNT({});",DEVICE_TABLE);
    let query5 = 
"RETURN {  
    total: $total,
    limit: $limit,
    page: $offset,
    items: $rst 
};";
    let rst: Option<PaginationDb<DeviceModel>> = db_pool()
        .query(query0)
        .query(query1)
        .query(query2)
        .query(query3)
        .query(query4)
        .query(query5)
        .await
        .expect("Error selecting Devices!")
        .take(5)
        .expect("Error selecting Devices!")
    ;
    rst.map(PaginationDb::into).expect("Not Result returner when selected Device")
}

pub async fn delete_device_by_id(
    id: String
) -> (){
    let rst: DeviceModel = db_pool()
        .delete((DEVICE_TABLE, id.as_str()))
        .await
        .expect("Error deleting device by id!")
        .expect("Device not found for delete!")
    ;
    ()
}

pub async fn get_device_type_by_id_on_db(
    id: String
) -> DeviceTypeModel {
    let rst: DeviceTypeSurreal = db_pool()
        .select((DEVICE_TYPE_TABLE, id.as_str()))
        .await
        .expect("Error selecting device-type vy id!")
        .expect("Error DeviceType Not found!")
    ;
    rst.into()
}
pub async fn list_device_type_on_db(
    params:DeviceTypeQueryParams
) -> Pagination<DeviceTypeModel>{
    let query0 = format!("LET $limit = {};",params.limit());
    let query1 = format!("LET $offset = {};",params.offset());
    let query2 = format!("LET $start = {};",params.start());
    let query3 = format!(
        "LET $rst = (SELECT * FROM {} START $start LIMIT $limit);",
        DEVICE_TYPE_TABLE
    );
    let query4 = format!("LET $total = COUNT({});",DEVICE_TYPE_TABLE);
    let query5 = 
"RETURN {  
    total: $total,
    limit: $limit,
    page: $offset,
    items: $rst 
};";
    let rst: Option<PaginationDb<DeviceTypeSurreal>> = db_pool()
        .query(query0)
        .query(query1)
        .query(query2)
        .query(query3)
        .query(query4)
        .query(query5)
        .await
        .expect("Error selecting DevicesType!")
        .take(5)
        .expect("Error selecting DevicesType!")
    ;
    rst.map(|a| a.into()).expect("Not Result returned when selected DeviceType")
}

pub async fn update_device_type_on_db(
    id: String,
    dto: UpdateDeviceTypeDto
) -> DeviceTypeModel{
    let rst: DeviceTypeSurreal = db_pool()
        .update((DEVICE_TYPE_TABLE, id.as_str()))
        .content(UpdateDeviceTypeSurreal::from(dto))
        .await
        .expect("Error updating DeviceType on db!")
        .expect("Device not found for update!")
    ;
    rst.into()
}

pub async fn create_device_type_on_db(
    dto: CreateDeviceTypeDto
) -> DeviceTypeModel{
    let rst: DeviceTypeSurreal = db_pool()
        .create(DEVICE_TYPE_TABLE)
        .content(CreateDeviceTypeSurreal::from(dto))
        .await
        .expect("Error creating device-type record!")
        .pop()
        .expect("Device Not created on DB!")
    ;
    rst.into()
}
pub async fn delete_device_type_by_id_on_db(
    id: String,
) -> (){
    let rst: DeviceTypeSurreal = db_pool()
        .delete((DEVICE_TYPE_TABLE, id.as_str()))
        .await
        .expect("Error deleting device-type records!")
        .expect("Device not found for delete on db!")
    ;
    ()
}