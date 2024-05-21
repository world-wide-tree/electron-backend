use serde::{Deserialize, Serialize};

use crate::models::dto::user::{CreateUserDto, UpdateUserDto, UserSidninDto};



#[derive(Debug, Serialize)]
pub struct CreateUserSurreal{
    pub user_name: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub phone_number: String,
}

impl From<CreateUserDto> for CreateUserSurreal{
    fn from(value: CreateUserDto) -> Self {
        Self { 
            user_name: value.user_name, 
            password: value.password,
            first_name: value.first_name,
            last_name: value.last_name,
            phone_number: value.phone_number,
        }
    }
}
impl From<UserSidninDto> for CreateUserSurreal{
    fn from(value: UserSidninDto) -> Self {
        Self { 
            user_name: value.user_name, 
            password: value.password,
            first_name: value.first_name,
            last_name: value.last_name,
            phone_number: value.phone_number,
        }
    }
}
#[derive(Debug, Serialize)]
pub struct UpdateUserSurreal{
    pub user_name: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub phone_number: String,
}

impl From<UpdateUserDto> for UpdateUserSurreal{
    fn from(value: UpdateUserDto) -> Self {
        Self { 
            user_name: value.user_name, 
            password: value.password,
            first_name: value.first_name,
            last_name: value.last_name,
            phone_number: value.phone_number,
        }
    }
}