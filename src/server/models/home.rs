use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PostHomeSDto{
    pub name: String,
    pub address: String,
    pub latitude: i64,
    pub longtitude: i64,
    pub substation: String, // id of substation
    pub user: String // id of user
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PutHomeSDto{
    pub name: String,
    pub address: String,
    pub latitude: i64,
    pub longtitude: i64,
    pub substation: String, // id of substation
    pub user: String // id of user
}
#[derive(Debug, Serialize, Deserialize)]
pub struct HomeSDto{
    pub id: String,
    pub name: String,
    pub address: String,
    pub latitude: i64,
    pub longtitude: i64,
    pub substation: String, // id of substation
    pub user: String // id of user
}
