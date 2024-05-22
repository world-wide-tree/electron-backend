use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateHomeSurreal{
    name: String,
    address: String,
    latitude: f64,
    longtitude: f64,
    substation: Thing, // id of substation
    user: Thing // id of user
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateHomeSurreal{
    name: String,
    address: String,
    latitude: f64,
    longtitude: f64,
    substation: Thing, // id of substation
    user: Thing // id of user
}