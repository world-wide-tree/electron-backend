use std::sync::Arc;

use axum::{routing::{delete, get, post, put}, Router};

use crate::{server::handlers::room_type::{delete_room_type_handler, get_room_type_by_id_handler, list_room_type_handler, post_room_type_handler, put_room_type_handler}, service::app_state::AppState};

static ROOM_TYPE_ROOT_PATH: &'static str = "/room-type";

pub fn room_type_router() -> Router<Arc<AppState>>{
    Router::new()
        .route(format!("{}", ROOM_TYPE_ROOT_PATH).as_str(), get(get_room_type_by_id_handler))
        .route(format!("{}", ROOM_TYPE_ROOT_PATH).as_str(), post(post_room_type_handler))
        .route(format!("{}/:id", ROOM_TYPE_ROOT_PATH).as_str(), get(list_room_type_handler))
        .route(format!("{}/:id", ROOM_TYPE_ROOT_PATH).as_str(), delete(delete_room_type_handler))
        .route(format!("{}/:id", ROOM_TYPE_ROOT_PATH).as_str(), put(put_room_type_handler))
}