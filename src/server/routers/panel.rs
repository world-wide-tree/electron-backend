use std::sync::Arc;

use axum::{routing::{get, post}, Router};

use crate::service::app_state::AppState;

static PANEL_ROOT_PATH: &'static str = "/panel";


pub fn panel_router() -> Router<Arc<AppState>>{
    Router::new()
        .route(format!("{}", PANEL_ROOT_PATH).as_str(), get("PANEL get"))
}