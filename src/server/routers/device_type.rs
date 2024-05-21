use std::sync::Arc;

use axum::{routing::{get, post}, Router};

use crate::service::app_state::AppState;

static DEVICE_TYPE_ROOT_PATH: &'static str = "/device-type";

pub fn device_type_router() -> Router<Arc<AppState>>{
    Router::new()
        .route(format!("{}", DEVICE_TYPE_ROOT_PATH).as_str(), get("DEVICE TYPE"))

}
