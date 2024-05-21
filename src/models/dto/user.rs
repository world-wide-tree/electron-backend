use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserLoginDto{
    pub user_name: String,
    pub password: String
}
#[derive(Debug, Serialize, Deserialize)]
pub struct UserSidninDto{
    pub first_name: String,
    pub last_name: String,
    pub phone_number: String,

    pub user_name: String,
    pub password: String,
    pub password_confirm: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserDto{
    pub first_name: String,
    pub last_name: String,
    pub phone_number: String,
    pub user_name: String,
    pub password: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUserDto{
    pub first_name: String,
    pub last_name: String,
    pub phone_number: String,
    pub user_name: String,
    pub password: String,
}