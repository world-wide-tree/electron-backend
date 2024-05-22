use crate::{data_store::controllers::home::{create_home_on_db, delete_home_by_id_on_db, get_home_by_id_on_db, list_home_on_db, update_home_on_db}, models::{dto::home::{CreateHomeDto, UpdateHomeDto}, house::HomeModel, pagination::Pagination, query_params::home::HomeQueryParams}};

pub struct HomeService{}

impl HomeService{
    pub fn new() -> Self{
        Self {  }
    }
    pub async fn post_home(&self, dto: CreateHomeDto) -> HomeModel{
        let rst = create_home_on_db(dto).await;
        rst
    }
    pub async fn get_home_by_id(&self, id: String) -> HomeModel{
        let rst = get_home_by_id_on_db(id).await;
        rst
    }
    pub async fn get_all_homes(&self, params: HomeQueryParams) -> Pagination<HomeModel>{
        let rst = list_home_on_db(params).await;
        rst
    }
    pub async fn put_home(&self, id: String, dto: UpdateHomeDto) -> HomeModel{
        let rst = update_home_on_db(id, dto).await;
        rst
    }
    pub async fn delete_home(&self, id: String) -> (){
        let rst = delete_home_by_id_on_db(id).await;
        rst
    }
}