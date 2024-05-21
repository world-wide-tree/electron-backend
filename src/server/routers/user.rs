use std::sync::Arc;

use axum::{routing::{get, post}, Router};

use crate::{server::handlers::{user_login_handler, user_signin_handler}, service::app_state::AppState};

static USER_ROOT_PATH: &'static str = "/user";

pub fn user_router() -> Router<Arc<AppState>>{
    Router::new()
        .route(format!("{}/login", USER_ROOT_PATH).as_str(), post(user_login_handler))
        .route(format!("{}/signin", USER_ROOT_PATH).as_str(), post(user_signin_handler))
        .route(format!("{}/me", USER_ROOT_PATH).as_str(), post(user_signin_handler))

}