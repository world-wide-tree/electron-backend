use crate::{
    data_store::{
        db_pool, models::{
            dto::user::{CreateUserSurreal, UpdateUserSurreal}, 
            user::UserSurreal
        }
    }, 
    models::{
        dto::user::{CreateUserDto, UpdateUserDto}, 
        user::UserModel
    }
};

pub static USER_TABLE: &'static str = "users";

pub async fn create_user_on_db(
    dto: impl Into<CreateUserSurreal>
) -> UserModel {
    let rst: UserSurreal = db_pool()
        .create(USER_TABLE)
        .content(dto.into())
        .await
        .expect("Error creating records users!")
        .pop()
        .expect("User not created on DB!")
    ;
    rst.into()
}

pub async fn update_user_on_db(
    id: String,
    dto: UpdateUserDto
) -> UserModel{
    let rst: UserSurreal = db_pool()
        .update((USER_TABLE, id.as_str()))
        .content(UpdateUserSurreal::from(dto))
        .await
        .expect("Error updating user record!")
        .expect("Error user not found for update!")
    ;
    rst.into()
}

pub async fn get_user_by_id_on_db(
    id: String
) -> UserModel{
    let rst: UserSurreal = db_pool()
        .select((USER_TABLE, id.as_str()))
        .await
        .expect("Error of selecting user by id!")
        .expect("User Not found!")
    ;
    rst.into()
}

pub async fn get_user_by_username_on_db(
    username: String
) -> UserModel{
    let query = format!(
        "SELECT * FROM users WHERE user_name = \"{}\";",
        // USER_TABLE,
        username
    );
    let mut rst = db_pool()
        .query(query)
        //.select((USER_TABLE, id.as_str()))
        .await
        .expect("Error of selecting user by username!")
    ;
    let mut rst: Vec<UserSurreal> = rst.take(0)
        .expect("Error of selecting user by username!")
    ;
    rst.pop()
        .expect("User Not found!")
        .into()
}