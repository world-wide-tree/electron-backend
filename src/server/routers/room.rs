use std::sync::Arc;

use axum::{routing::{get, post}, Router};

use crate::service::app_state::AppState;

static ROOM_ROOT_PATH: &'static str = "/room";

pub fn room_router() -> Router<Arc<AppState>>{
    Router::new()
        .route(format!("{}", ROOM_ROOT_PATH).as_str(), get("ROOM"))
}