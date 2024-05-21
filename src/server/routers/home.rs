use std::sync::Arc;

use axum::{routing::{get, post}, Router};

use crate::service::app_state::AppState;

static HOME_ROOT_PATH: &'static str = "/home";

pub fn home_router() -> Router<Arc<AppState>>{
    Router::new()
        .route(format!("{}",HOME_ROOT_PATH).as_str(), get("Home"))
}