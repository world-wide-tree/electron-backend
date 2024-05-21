use std::sync::Arc;

use axum::{routing::{get, post}, Router};

use crate::service::app_state::AppState;

static DEVICE_ROOT_PATH: &'static str = "/device";

pub fn device_router() -> Router<Arc<AppState>>{
    Router::new()
        .route(format!("{}", DEVICE_ROOT_PATH).as_str(), get("Device root"))
}