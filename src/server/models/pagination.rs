#[derive(Debug, )]
pub struct PaginationSDto<T>{
    pub count: usize,
    pub page_count: usize,
    pub limit: usize,
    pub results: Vec<T>
}