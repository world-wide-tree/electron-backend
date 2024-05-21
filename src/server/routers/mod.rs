use std::sync::Arc;

use axum::Router;
use axum::routing::get;

use crate::service::app_state::AppState;

use self::user::user_router;

pub mod user;

static API_ROOT_PATH: &'static str = "/";

pub fn api_router(state: Arc<AppState>) -> Router{
    let router = Router::new()
        .route(API_ROOT_PATH, get("method_router"))
        .nest(API_ROOT_PATH, user_router())
        .with_state(state)
    ;

    router
}