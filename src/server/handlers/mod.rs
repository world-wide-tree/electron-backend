use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::service::app_state::AppState;

use super::models::{jwt::JWToken, user::{UserLogin, UserSignin}};

pub async fn user_login_handler(
    State(state): State<Arc<AppState>>,
    Json(body): Json<UserLogin>,
) -> impl IntoResponse{
    let rst = state.user().login(body.into()).await;
    (StatusCode::OK, Json(rst))
}

pub async fn user_signin_handler(
    State(state): State<Arc<AppState>>,
    Json(body): Json<UserSignin>
) -> impl IntoResponse{
    let rst = state.user().signin(body.into()).await;
    (StatusCode::OK, Json(rst))
}
pub async fn user_me_handler() -> impl IntoResponse{
    "UserMe"
}