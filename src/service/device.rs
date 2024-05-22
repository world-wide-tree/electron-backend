use crate::{data_store::controllers::device::{create_device_on_db, create_device_type_on_db, delete_device_by_id_on_db, delete_device_type_by_id_on_db, get_device_by_id_on_db, get_device_type_by_id_on_db, list_device_on_db, list_device_type_on_db, update_device_on_db, update_device_type_on_db}, models::{device::{DeviceModel, DeviceTypeModel}, dto::device::{CreateDeviceDto, CreateDeviceTypeDto, UpdateDeviceDto, UpdateDeviceTypeDto}, pagination::Pagination, query_params::device::{DeviceQueryParams, DeviceTypeQueryParams}}};

pub struct DeviceService{}

impl DeviceService{
    pub fn new() -> Self{
        Self {  }
    }
    pub async fn post_device(&self,dto: CreateDeviceDto) -> DeviceModel{
        let rst = create_device_on_db(dto).await;
        rst
    }
    pub async fn get_device_by_id(&self, id: String) -> DeviceModel{
        let rst = get_device_by_id_on_db(id).await;
        rst
    }
    pub async fn get_all_devices(&self, params: DeviceQueryParams) -> Pagination<DeviceModel>{
        let rst = list_device_on_db(params).await;
        rst
    }
    pub async fn put_device(&self, id: String, dto: UpdateDeviceDto) -> DeviceModel {
        let rst = update_device_on_db(id, dto).await;
        rst
    }
    pub async fn delete_device_by_id(&self, id: String) -> () {
        let rst = delete_device_by_id_on_db(id).await;
        rst
    }

    pub async fn post_device_type(&self, dto: CreateDeviceTypeDto) -> DeviceTypeModel{
        let rst = create_device_type_on_db(dto).await;
        rst
    }
    pub async fn get_device_type_by_id(&self, id: String) -> DeviceTypeModel{
        let rst = get_device_type_by_id_on_db(id).await;
        rst
    }
    pub async fn get_all_deivece_types(&self, params: DeviceTypeQueryParams) -> Pagination<DeviceTypeModel>{
        let rst = list_device_type_on_db(params).await;
        rst
    }
    pub async fn put_device_type(&self, id: String, dto: UpdateDeviceTypeDto) -> DeviceTypeModel{
        let rst = update_device_type_on_db(id, dto).await;
        rst
    }
    pub async fn delete_device_type_by_id(&self, id: String) -> (){
        let rst = delete_device_type_by_id_on_db(id).await;
        rst
    }
}
