use crate::{data_store::{db_pool, models::{dto::room::{CreateRoomSurreal, CreateRoomTypeSurreal, UpdateRoomSurreal, UpdateRoomTypeSurreal}, pagination::PaginationDb, room::{RoomSurreal, RoomTypeSurreal}}}, models::{dto::{device::UpdateDeviceDto, room::{CreateRoomDto, CreateRoomTypeDto, UpdateRoomDto, UpdateRoomTypeDto}}, pagination::Pagination, query_params::room::{RoomQueryParams, RoomTypeQueryParams}, room::{RoomModel, RoomTypeModel}}};

use super::home::HOME_TABLE;

pub static ROOM_TABLE: &'static str = "Room";
pub static ROOM_TYPE_TABLE: &'static str = "RoomType";

pub async fn create_room_on_db(
    dto: CreateRoomDto
) -> RoomModel{
    let rst: RoomSurreal = db_pool()
        .create(ROOM_TABLE)
        .content(CreateRoomSurreal::from(dto))
        .await
        .expect("Error of creating Room record on Db!")
        .pop()
        .expect("Home not created!")
    ;
    rst.into()
}

pub async fn get_room_by_id_on_db(
    id: String
) -> RoomModel {
    let rst: RoomSurreal = db_pool()
        .select((ROOM_TABLE,id.as_str()))
        .await
        .expect("Error of selecting Room record by id!")
        .expect("Error Room not found!")
    ;
    rst.into()
}

pub async fn list_room_on_db(
    params: RoomQueryParams
) -> Pagination<RoomModel>{
    let query0 = format!("LET $limit = {};",params.limit());
    let query1 = format!("LET $offset = {};",params.offset());
    let query2 = format!("LET $start = {};",params.start());
    let query3 = format!(
        "LET $rst = (SELECT * FROM {} START $start LIMIT $limit);",
        ROOM_TABLE
    );
    let query4 = format!("LET $total = COUNT({});",ROOM_TABLE);
    let query5 = 
"RETURN {  
    total: $total,
    limit: $limit,
    page: $offset,
    items: $rst 
};";
    let rst: Option<PaginationDb<RoomSurreal>> = db_pool()
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

pub async fn update_room_on_db(
    id: String,
    dto: UpdateRoomDto
) -> RoomModel {
    let rst: RoomSurreal = db_pool()
        .update((ROOM_TABLE, id.as_str()))
        .content(UpdateRoomSurreal::from(dto))
        .await
        .expect("Error of updating Room records!")
        .expect("Room not found for update!")
    ;
    rst.into()
}

pub async fn delete_room_by_id_on_db(
    id: String,
) -> (){
    let rst: RoomSurreal = db_pool()
        .delete((HOME_TABLE, id.as_str()))
        .await
        .expect("Error deleting room by id!")
        .expect("Room not found for deleted!")
    ;
    ()
}



pub async fn create_room_type_on_db(
    dto: CreateRoomTypeDto
) -> RoomTypeModel{
    let rst: RoomTypeSurreal = db_pool()
        .create(ROOM_TYPE_TABLE)
        .content(CreateRoomTypeSurreal::from(dto))
        .await
        .expect("Error of creating Room record on Db!")
        .pop()
        .expect("Home not created!")
    ;
    rst.into()
}

pub async fn get_room_type_by_id_on_db(
    id: String
) -> RoomTypeModel {
    let rst: RoomTypeSurreal = db_pool()
        .select((ROOM_TYPE_TABLE,id.as_str()))
        .await
        .expect("Error of selecting Room record by id!")
        .expect("Error Room not found!")
    ;
    rst.into()
}

pub async fn list_room_type_on_db(
    params: RoomTypeQueryParams
) -> Pagination<RoomTypeModel>{
    let query0 = format!("LET $limit = {};",params.limit());
    let query1 = format!("LET $offset = {};",params.offset());
    let query2 = format!("LET $start = {};",params.start());
    let query3 = format!(
        "LET $rst = (SELECT * FROM {} START $start LIMIT $limit);",
        ROOM_TYPE_TABLE
    );
    let query4 = format!("LET $total = COUNT({});",ROOM_TYPE_TABLE);
    let query5 = 
"RETURN {  
    total: $total,
    limit: $limit,
    page: $offset,
    items: $rst 
};";
    let rst: Option<PaginationDb<RoomTypeSurreal>> = db_pool()
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

pub async fn update_room_type_on_db(
    id: String,
    dto: UpdateRoomTypeDto
) -> RoomTypeModel {
    let rst: RoomTypeSurreal = db_pool()
        .update((ROOM_TYPE_TABLE, id.as_str()))
        .content(UpdateRoomTypeSurreal::from(dto))
        .await
        .expect("Error of updating Room records!")
        .expect("Room not found for update!")
    ;
    rst.into()
}

pub async fn delete_room_type_by_id_on_db(
    id: String,
) -> (){
    let rst: RoomTypeSurreal = db_pool()
        .delete((ROOM_TYPE_TABLE, id.as_str()))
        .await
        .expect("Error deleting room by id!")
        .expect("Room not found for deleted!")
    ;
    ()
}