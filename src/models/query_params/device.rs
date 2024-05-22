use serde::Deserialize;

use crate::models::pagination::{DEFAULT_LIMIT, DEFAULT_OFFSET};

#[derive(Clone, Debug, Deserialize)]
pub struct DeviceQueryParams{
    limit: Option<u32>,
    offset: Option<u32>,
}


impl DeviceQueryParams{
    pub fn limit(&self) -> u32{
        self.limit.unwrap_or(DEFAULT_LIMIT)
    }
    pub fn offset(&self) -> u32{
        self.offset.unwrap_or(DEFAULT_OFFSET)
    }
    pub fn start(&self) -> u32{
        self.offset() * self.limit()
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct DeviceTypeQueryParams{
    limit: Option<u32>,
    offset: Option<u32>,
}


impl DeviceTypeQueryParams{
    pub fn limit(&self) -> u32{
        self.limit.unwrap_or(DEFAULT_LIMIT)
    }
    pub fn offset(&self) -> u32{
        self.offset.unwrap_or(DEFAULT_OFFSET)
    }
    pub fn start(&self) -> u32{
        self.offset() * self.limit()
    }
}

