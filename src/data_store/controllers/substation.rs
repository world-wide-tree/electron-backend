use crate::{data_store::{db_pool, models::{dto::substation::{CreateSubstationSurreal, UpdateSubstationSurreal}, pagination::PaginationDb, substation::SubstationSurreal}}, models::{dto::substation::{CreateSubstationDto, UpdateSubstationDto}, pagination::Pagination, query_params::substation::SubstationQueryParams, substation::SubstationModel}};

pub static SUBSTATION_TABLE: &'static str = "Substation";

pub async fn create_substation_on_db(
    dto: CreateSubstationDto
) -> SubstationModel {
    let rst: SubstationSurreal = db_pool()
        .create(SUBSTATION_TABLE)
        .content(CreateSubstationSurreal::from(dto))
        .await
        .expect("Error createing Substation on Db!")
        .pop()
        .expect("Error Substation not created!")
    ;
    rst.into()
}

pub async fn update_substation_on_db(
    id: String,
    dto: UpdateSubstationDto
) -> SubstationModel{
    let rst: SubstationSurreal = db_pool()
        .update((SUBSTATION_TABLE, id.as_str()))
        .content(UpdateSubstationSurreal::from(dto))
        .await
        .expect("Error updateing Substation on db!")
        .expect("Error Substation not created!")
    ;
    rst.into()
}
pub async fn get_substation_by_id_on_db(
    id: String,
) -> SubstationModel{
    let rst: SubstationSurreal = db_pool()
        .select((SUBSTATION_TABLE, id.as_str()))
        .await
        .expect("Error selecting Substation on db!")
        .expect("Error Substation not found on db!")
    ;
    rst.into()
}
pub async fn delete_substation_by_id_on_db(
    id: String
) -> () {
    let rst: SubstationSurreal = db_pool()
        .delete((SUBSTATION_TABLE, id.as_str()))
        .await
        .expect("Error deleting Substation by id!")
        .expect("Substation not found for delete!")
    ;
    ()
}

pub async fn list_substation_on_db(
    params: SubstationQueryParams
) -> Pagination<SubstationModel>{
    let query0 = format!("LET $limit = {};",params.limit());
    let query1 = format!("LET $offset = {};",params.offset());
    let query2 = format!("LET $start = {};",params.start());
    let query3 = format!(
        "LET $rst = (SELECT * FROM {} START $start LIMIT $limit);",
        SUBSTATION_TABLE
    );
    let query4 = format!("LET $total = COUNT({});",SUBSTATION_TABLE);
    let query5 = 
"RETURN {  
    total: $total,
    limit: $limit,
    page: $offset,
    items: $rst 
};";
    let rst: Option<PaginationDb<SubstationSurreal>> = db_pool()
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
