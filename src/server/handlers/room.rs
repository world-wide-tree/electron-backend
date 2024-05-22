use std::sync::Arc;

use axum::{extract::{Path, Query, State}, response::IntoResponse, Json};

use crate::{models::query_params::room::RoomQueryParams, server::models::room::{PostRoomSDto, PutRoomSDto}, service::app_state::AppState};

pub async fn get_room_by_id_handler(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse{
    "Get room by id"
}
pub async fn list_room_handler(
    Query(params): Query<RoomQueryParams>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse{
    "Get all rooms"
}
pub async fn post_room_handler(
    State(state): State<Arc<AppState>>,
    Json(body): Json<PostRoomSDto>
) -> impl IntoResponse{
    "Post room"
}
pub async fn delete_room_by_id_handler(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse{
    "Delete room by id"
}
pub async fn put_room_handler(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
    Json(body): Json<PutRoomSDto>
) -> impl IntoResponse{
    "Put room"
}
