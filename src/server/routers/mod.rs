use std::sync::Arc;

use axum::Router;
use axum::routing::get;

use crate::service::app_state::AppState;

use self::device::device_router;
use self::device_type::device_type_router;
use self::home::home_router;
use self::panel::panel_router;
use self::room::room_router;
use self::room_type::room_type_router;
use self::substation::substation_router;
use self::user::user_router;

pub mod user;
pub mod panel;
pub mod substation;
pub mod home;
pub mod room;
pub mod device;
pub mod room_type;
pub mod device_type;

static API_ROOT_PATH: &'static str = "/";

pub fn api_router(state: Arc<AppState>) -> Router{
    let router = Router::new()
        .route(API_ROOT_PATH, get("method_router"))
        .nest(API_ROOT_PATH, user_router())
        .nest(API_ROOT_PATH, device_router())
        .nest(API_ROOT_PATH, device_type_router())
        .nest(API_ROOT_PATH, room_router())
        .nest(API_ROOT_PATH, room_type_router())
        .nest(API_ROOT_PATH, substation_router())
        .nest(API_ROOT_PATH, home_router())
        .with_state(state)
    ;

    router
}