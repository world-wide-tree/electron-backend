use std::sync::Arc;

use axum::{routing::{get, post, delete, put}, Router};

use crate::{server::handlers::device::{delete_device_by_id_handler, get_device_by_id_handler, list_device_handler, post_device_handler, put_device_handler}, service::app_state::AppState};

static DEVICE_ROOT_PATH: &'static str = "/device";

pub fn device_router() -> Router<Arc<AppState>>{
    Router::new()
        .route(format!("{}", DEVICE_ROOT_PATH).as_str(), get(list_device_handler))
        .route(format!("{}", DEVICE_ROOT_PATH).as_str(), post(post_device_handler))
        .route(format!("{}/:id", DEVICE_ROOT_PATH).as_str(), get(get_device_by_id_handler))
        .route(format!("{}/:id", DEVICE_ROOT_PATH).as_str(), delete(delete_device_by_id_handler))
        .route(format!("{}/:id", DEVICE_ROOT_PATH).as_str(), put(put_device_handler))
}