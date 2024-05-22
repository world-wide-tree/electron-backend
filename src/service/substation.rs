use crate::{data_store::controllers::substation::{create_substation_on_db, get_substation_by_id_on_db, list_substation_on_db, update_substation_on_db}, models::{dto::substation::{CreateSubstationDto, UpdateSubstationDto}, pagination::Pagination, query_params::substation::SubstationQueryParams, substation::SubstationModel}};

pub struct SubstationService{}

impl SubstationService {
    pub fn new() -> Self{
        Self {  }
    }
    pub async fn post_substation(&self, dto: CreateSubstationDto) -> SubstationModel{
        let rst = create_substation_on_db(dto).await;
        rst
    }
    pub async fn put_substation(&self, id: String, dto: UpdateSubstationDto) -> SubstationModel{
        let rst = update_substation_on_db(id, dto).await;
        rst
    }
    pub async fn get_substation_by_id(&self, id: String) -> SubstationModel{
        let rst = get_substation_by_id_on_db(id).await;
        rst
    }
    pub async fn get_all_substation(&self, params: SubstationQueryParams) -> Pagination<SubstationModel>{
        let rst = list_substation_on_db(params).await;
        rst
    }
}