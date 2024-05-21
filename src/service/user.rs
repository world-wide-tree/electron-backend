use crate::{data_store::controllers::user::{create_user_on_db, get_user_by_id_on_db, get_user_by_username_on_db, update_user_on_db}, models::{dto::user::{CreateUserDto, UpdateUserDto}, user::UserModel}, server::models::jwt::JWToken, UserLoginDto, UserSidninDto};

pub struct UserService{}


impl UserService{
    pub fn new() -> Self{
        Self{}
    }
    pub async fn get_user_by_id(&self, id: String) -> UserModel {
        let rst = get_user_by_id_on_db(id).await;
        rst
    }
    pub async fn update_user(&self, id: String, dto: UpdateUserDto) -> UserModel{
        let rst = update_user_on_db(id, dto).await;
        rst
    }
    pub async fn post_user(&self, dto: CreateUserDto) -> UserModel{
        let rst = create_user_on_db(dto).await;

        rst
    }
    
    pub async fn login(&self, dto: UserLoginDto) -> JWToken{
        let rst = get_user_by_username_on_db(dto.user_name).await;
        JWToken::new(format!("{}:{}", rst.id, rst.user_name))
    }
    pub async fn signin(&self, dto: UserSidninDto) -> JWToken{
        let rst = create_user_on_db(dto).await;
        JWToken::new(format!("{}:{}", rst.id, rst.user_name))
    }
}