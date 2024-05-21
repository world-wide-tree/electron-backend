use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use crate::{data_store::controllers::user::USER_TABLE, models::user::UserModel};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSurreal{
    id: Thing,
    
    user_name: String,
    password: String,
}

impl Into<UserModel> for UserSurreal{
    fn into(self) -> UserModel {
        UserModel { 
            id: self.id.id.to_string(), 
            user_name: self.user_name, 
            password: self.password 
        }
    }
}

impl From<UserModel> for UserSurreal{
    fn from(value: UserModel) -> Self {
        Self { 
            id: Thing::from((USER_TABLE, value.id.as_str())), 
            user_name: value.user_name, 
            password: value.password
        }
    }
}