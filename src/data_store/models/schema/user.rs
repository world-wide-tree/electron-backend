use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use crate::{data_store::controllers::user::USER_TABLE, models::user::UserModel};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSurrealSchema{
    id: Thing,
    user_name: String,
    password: String,
    first_name: String,
    last_name: String,
    phone_number: String,
}

impl From<UserModel> for UserSurrealSchema{
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