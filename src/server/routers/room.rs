use std::sync::Arc;

use axum::{routing::{delete, get, put, post}, Router};

use crate::{server::handlers::room::{delete_room_by_id_handler, get_room_by_id_handler, list_room_handler, post_room_handler, put_room_handler}, service::app_state::AppState};

static ROOM_ROOT_PATH: &'static str = "/room";

pub fn room_router() -> Router<Arc<AppState>>{
    Router::new()
        .route(format!("{}", ROOM_ROOT_PATH).as_str(), get(list_room_handler))
        .route(format!("{}", ROOM_ROOT_PATH).as_str(), post(post_room_handler))
        .route(format!("{}/:id", ROOM_ROOT_PATH).as_str(), get(get_room_by_id_handler))
        .route(format!("{}/:id", ROOM_ROOT_PATH).as_str(), delete(delete_room_by_id_handler))
        .route(format!("{}/:id", ROOM_ROOT_PATH).as_str(), put(put_room_handler))
}