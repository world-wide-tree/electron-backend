use serde::{Deserialize, Serialize};

use crate::models::pagination::Pagination;

#[derive(Deserialize, Serialize)]
pub struct PaginationDb<DbModel>{
    pub total: usize,
    pub limit: usize,
    pub page: usize,
    pub items: Vec<DbModel>
}

impl<Model, DbModel> Into<Pagination<Model>> 
for PaginationDb<DbModel>
where DbModel: Into<Model> {
    fn into(self) -> Pagination<Model> {
        let total = self.total;
        let limit = self.limit;
        let page = self.page;
        let items = self.items
            .into_iter()
            .map(DbModel::into)
            .collect::<Vec<Model>>()
        ;
        Pagination::new(total, limit, page, items)
    }
}