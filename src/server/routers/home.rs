use std::sync::Arc;

use axum::{routing::{get, post, delete, put}, Router};

use crate::{server::handlers::home::{delete_home_by_id_handler, get_home_by_id_handler, list_home_handler, post_home_handler, put_home_handler}, service::app_state::AppState};

static HOME_ROOT_PATH: &'static str = "/home";

pub fn home_router() -> Router<Arc<AppState>>{
    Router::new()
    .route(format!("{}", HOME_ROOT_PATH).as_str(), get(list_home_handler))
    .route(format!("{}", HOME_ROOT_PATH).as_str(), post(post_home_handler))
    .route(format!("{}/:id", HOME_ROOT_PATH).as_str(), get(get_home_by_id_handler))
    .route(format!("{}/:id", HOME_ROOT_PATH).as_str(), delete(delete_home_by_id_handler))
    .route(format!("{}/:id", HOME_ROOT_PATH).as_str(), put(put_home_handler))
}