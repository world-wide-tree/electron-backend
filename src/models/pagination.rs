use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Pagination<T>{
    pub limit: usize,       // Default 25
    pub total: usize,       // Total records on db
    pub count: usize,       // Total returned records from db
    pub page: usize,        // Current page index
    pub page_count: usize,  // Total page count
    pub items: Vec<T>,      // Returned items
}


impl<T> Pagination<T>{
    pub fn new(total: usize, limit: usize, page: usize, items: Vec<T>)-> Self{
        Self { 
            limit, 
            total, 
            count: items.len(), 
            page, 
            page_count: (total as f64 / limit as f64).ceil() as usize, 
            items
        }
    }
}


pub const DEFAULT_OFFSET: u32 = 0;
pub const DEFAULT_LIMIT: u32 = 25;