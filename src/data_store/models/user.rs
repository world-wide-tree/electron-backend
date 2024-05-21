use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use crate::{data_store::controllers::user::USER_TABLE, models::user::UserModel};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSurreal{
    id: Thing,
    
    user_name: String,
    password: String,
    first_name: String,
    last_name: String,
    phone_number: String,
}

impl Into<UserModel> for UserSurreal{
    fn into(self) -> UserModel {
        UserModel { 
            id: self.id.id.to_string(), 
            user_name: self.user_name, 
            password: self.password,
            first_name: self.first_name,
            last_name: self.last_name,
            phone_number: self.phone_number
        }
    }
}

impl From<UserModel> for UserSurreal{
    fn from(value: UserModel) -> Self {
        Self { 
            id: Thing::from((USER_TABLE, value.id.as_str())), 
            user_name: value.user_name, 
            password: value.password,
            first_name: value.first_name,
            last_name: value.last_name,
            phone_number: value.phone_number
        }
    }
}