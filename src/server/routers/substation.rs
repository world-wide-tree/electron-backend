use std::sync::Arc;

use axum::{routing::{get, post}, Router};

use crate::service::app_state::AppState;

static SUBSTATION_ROOT_PATH: &'static str = "/substation";

pub fn substation_router() -> Router<Arc<AppState>>{
    Router::new()
        .route(format!("{}",SUBSTATION_ROOT_PATH).as_str(), get("Substation"))
}