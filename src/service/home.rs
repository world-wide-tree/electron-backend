use crate::models::{dto::home::CreateHomeDto, house::HomeModel};

pub struct HomeService{}

impl HomeService{
    pub fn new() -> Self{
        Self {  }
    }
    pub async fn post_home(&self, dto: CreateHomeDto) -> HomeModel{
        todo!()
    }
}