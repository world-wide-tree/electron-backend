use serde::Deserialize;

use crate::models::dto::user::{UserLoginDto, UserSidninDto};

#[derive(Debug, Deserialize)]
pub struct UserLogin{
    pub user_name: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct UserSignin{
    pub first_name: String,
    pub last_name: String,
    pub phone_number: String,
    pub user_name: String,
    pub password: String,
    pub password_confirm: String,
}

impl Into<UserLoginDto> for UserLogin{
    fn into(self) -> UserLoginDto {
        UserLoginDto { 
            user_name: self.user_name, 
            password: self.password 
        }
    }
}

impl Into<UserSidninDto> for UserSignin{
    fn into(self) -> UserSidninDto {
        UserSidninDto { 
            first_name: self.first_name, 
            last_name: self.last_name, 
            phone_number: self.phone_number, 
            user_name: self.user_name, 
            password: self.password, 
            password_confirm: self.password_confirm
        }
    }
}