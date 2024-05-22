use crate::{data_store::{db_pool, models::{dto::home::{CreateHomeSurreal, UpdateHomeSurreal}, house::HomeSurreal, pagination::PaginationDb}}, models::{dto::home::{CreateHomeDto, UpdateHomeDto}, house::HomeModel, pagination::Pagination, query_params::home::HomeQueryParams}};

use super::device::DEVICE_TABLE;

pub static HOME_TABLE: &'static str = "Home";

pub async fn create_home_on_db(
    dto: CreateHomeDto
) -> HomeModel{
    let rst: HomeSurreal = db_pool()
        .create(HOME_TABLE)
        .content(CreateHomeSurreal::from(dto))
        .await
        .expect("Erre createing Home record on Db")
        .pop()
        .expect("Home Not found!")
    ;
    rst.into()
}

pub async fn update_home_on_db(
    id: String,
    dto: UpdateHomeDto
) -> HomeModel{
    let rst: HomeSurreal = db_pool()
        .update((HOME_TABLE, id.as_str()))
        .content(UpdateHomeSurreal::from(dto))
        .await
        .expect("Error updating Home record!")
        .expect("Error Home not found for update!")
    ;
    rst.into()
}

pub async fn get_home_by_id_on_db(
    id: String,
) -> HomeModel{
    let rst: HomeSurreal = db_pool()
        .select((HOME_TABLE, id.as_str()))
        .await
        .expect("Error of selecting home by id!")
        .expect("Home not found on db!")
    ;
    rst.into()
}

pub async fn list_home_on_db(
    params: HomeQueryParams
) -> Pagination<HomeModel>{
    let query0 = format!("LET $limit = {};",params.limit());
    let query1 = format!("LET $offset = {};",params.offset());
    let query2 = format!("LET $start = {};",params.start());
    let query3 = format!(
        "LET $rst = (SELECT * FROM {} START $start LIMIT $limit);",
        HOME_TABLE
    );
    let query4 = format!("LET $total = COUNT({});",HOME_TABLE);
    let query5 = 
"RETURN {  
    total: $total,
    limit: $limit,
    page: $offset,
    items: $rst 
};";
    let rst: Option<PaginationDb<HomeSurreal>> = db_pool()
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

pub async fn delete_home_by_id_on_db(
    id: String,
) -> (){
    let rst: HomeSurreal = db_pool()
        .delete((HOME_TABLE, id.as_str()))
        .await
        .expect("Error deleting home by id!")
        .expect("Home not found for delete!")
    ;
    ()
}