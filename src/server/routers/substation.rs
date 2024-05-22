use std::sync::Arc;

use axum::{routing::{get, post, delete, put}, Router};

use crate::{server::handlers::substation::{delete_substation_by_id_handler, get_substation_by_id_handler, list_substation_handler, post_substation_handler, put_substation_handler}, service::app_state::AppState};

static SUBSTATION_ROOT_PATH: &'static str = "/substation";

pub fn substation_router() -> Router<Arc<AppState>>{
    Router::new()
        .route(format!("{}", SUBSTATION_ROOT_PATH).as_str(), get(list_substation_handler))
        .route(format!("{}", SUBSTATION_ROOT_PATH).as_str(), post(post_substation_handler))
        .route(format!("{}/:id", SUBSTATION_ROOT_PATH).as_str(), get(get_substation_by_id_handler))
        .route(format!("{}/:id", SUBSTATION_ROOT_PATH).as_str(), delete(delete_substation_by_id_handler))
        .route(format!("{}/:id", SUBSTATION_ROOT_PATH).as_str(), put(put_substation_handler))
}