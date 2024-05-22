use std::sync::Arc;

use axum::{extract::{Path, Query, State}, response::IntoResponse, Json};

use crate::{models::query_params::room::RoomTypeQueryParams, server::models::room::{PostRoomTypeSDto, PutRoomTypeSDto}, service::app_state::AppState};

pub async fn get_room_type_by_id_handler(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    "Get room-type by id"
}
pub async fn list_room_type_handler(
    Query(params): Query<RoomTypeQueryParams>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    "Get All room-type "
}
pub async fn post_room_type_handler(
    State(state): State<Arc<AppState>>,
    Json(body): Json<PostRoomTypeSDto>,
) -> impl IntoResponse {
    "Post room-type"
}
pub async fn delete_room_type_handler(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    "Delete toom-type by id"
}
pub async fn put_room_type_handler(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
    Json(body): Json<PutRoomTypeSDto>
) -> impl IntoResponse {
    "Put room-type"
}