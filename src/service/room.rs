use crate::{data_store::controllers::{device::list_device_on_db, room::{create_room_on_db, create_room_type_on_db, delete_room_by_id_on_db, delete_room_type_by_id_on_db, get_room_by_id_on_db, get_room_type_by_id_on_db, list_room_on_db, list_room_type_on_db, update_room_on_db, update_room_type_on_db}}, models::{dto::room::{CreateRoomDto, CreateRoomTypeDto, UpdateRoomDto, UpdateRoomTypeDto}, pagination::Pagination, query_params::room::{RoomQueryParams, RoomTypeQueryParams}, room::{RoomModel, RoomTypeModel}}};

pub struct RoomService{}

impl RoomService{
    pub fn new() -> Self{
        Self {  }
    }
    pub async fn get_room_by_id(&self, id: String) -> RoomModel{
        let rst = get_room_by_id_on_db(id).await;
        rst
    }
    pub async fn get_all_rooms(&self, params: RoomQueryParams) -> Pagination<RoomModel>{
        let rst = list_room_on_db(params).await;
        rst
    }
    pub async fn post_room(&self, dto: CreateRoomDto) -> RoomModel{
        let rst = create_room_on_db(dto).await;
        rst
    }
    pub async fn put_room(&self, id: String, dto: UpdateRoomDto) -> RoomModel{
        let rst = update_room_on_db(id, dto).await;
        rst
    }
    pub async fn delete_room_by_id(&self, id: String) -> (){
        let rst = delete_room_by_id_on_db(id).await;
        rst
    }


    pub async fn get_room_type_by_id(&self, id: String) -> RoomTypeModel{
        let rst = get_room_type_by_id_on_db(id).await;
        rst
    }
    pub async fn get_all_room_types(&self, params: RoomTypeQueryParams) -> Pagination<RoomTypeModel>{
        let rst = list_room_type_on_db(params).await;
        rst
    }
    pub async fn post_room_type(&self, dto: CreateRoomTypeDto) -> RoomTypeModel{
        let rst = create_room_type_on_db(dto).await;
        rst
    }
    pub async fn put_room_type(&self, id: String, dto: UpdateRoomTypeDto) -> RoomTypeModel{
        let rst = update_room_type_on_db(id, dto).await;
        rst
    }
    pub async fn delete_room_type_by_id(&self, id: String) -> (){
        let rst = delete_room_type_by_id_on_db(id).await;
        rst
    }
}