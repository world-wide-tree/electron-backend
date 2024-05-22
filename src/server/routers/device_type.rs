use std::sync::Arc;

use axum::{routing::{get, post, delete, put}, Router};

use crate::{server::handlers::device_type::{delete_device_type_by_id_handler, get_device_type_by_id_handler, list_device_type_handler, post_device_type_handler, put_device_type_handler}, service::app_state::AppState};

static DEVICE_TYPE_ROOT_PATH: &'static str = "/device-type";

pub fn device_type_router() -> Router<Arc<AppState>>{
    Router::new()
        .route(format!("{}", DEVICE_TYPE_ROOT_PATH).as_str(), get(list_device_type_handler))
        .route(format!("{}", DEVICE_TYPE_ROOT_PATH).as_str(), post(post_device_type_handler))
        .route(format!("{}/:id", DEVICE_TYPE_ROOT_PATH).as_str(), get(get_device_type_by_id_handler))
        .route(format!("{}/:id", DEVICE_TYPE_ROOT_PATH).as_str(), delete(delete_device_type_by_id_handler))
        .route(format!("{}/:id", DEVICE_TYPE_ROOT_PATH).as_str(), put(put_device_type_handler))
}