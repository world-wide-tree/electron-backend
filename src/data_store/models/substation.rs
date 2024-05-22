use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use crate::{data_store::controllers::substation::SUBSTATION_TABLE, models::substation::SubstationModel};

#[derive(Debug, Serialize, Deserialize)]
pub struct SubstationSurreal{
    id: Thing,
    code: String, // unique code of substation
    latitude: f64,
    longitude: f64,
    max_power: u64
}

impl From<SubstationModel> for SubstationSurreal{
    fn from(value: SubstationModel) -> Self {
        Self { 
            id: Thing::from((SUBSTATION_TABLE, value.id.as_str())), 
            code: value.code, 
            latitude: value.latitude, 
            longitude: value.longitude, 
            max_power: value.max_power 
        }
    }
}
impl Into<SubstationModel> for SubstationSurreal{
    fn into(self) -> SubstationModel {
        SubstationModel { 
            id: self.id.id.to_string(), 
            code: self.code, 
            latitude: self.latitude, 
            longitude: self.longitude, 
            max_power: self.max_power 
        }
    }
}