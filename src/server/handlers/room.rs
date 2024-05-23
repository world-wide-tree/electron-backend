use std::sync::Arc;

use axum::{extract::{Path, Query, State}, http::StatusCode, response::IntoResponse, Json};

use crate::{models::query_params::room::RoomQueryParams, server::models::room::{PostRoomSDto, PutRoomSDto, RoomSDto}, service::app_state::AppState};

pub async fn get_room_by_id_handler(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse{
    let rst = state.room().get_room_by_id(id).await;
    (StatusCode::OK, Json(RoomSDto::from(rst)))
}
pub async fn list_room_handler(
    Query(params): Query<RoomQueryParams>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse{
    let rst = state.room().get_all_rooms(params).await;
    (StatusCode::OK, Json(rst))
}
pub async fn post_room_handler(
    State(state): State<Arc<AppState>>,
    Json(body): Json<PostRoomSDto>
) -> impl IntoResponse{
    let rst = state.room().post_room(body.into()).await;
    (StatusCode::OK, Json(RoomSDto::from(rst)))
}
pub async fn delete_room_by_id_handler(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse{
    let rst = state.room().delete_room_by_id(id).await;
    (StatusCode::OK, Json(rst))
}
pub async fn put_room_handler(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
    Json(body): Json<PutRoomSDto>
) -> impl IntoResponse{
    let rst = state.room().put_room(id, body.into()).await;
    (StatusCode::OK, Json(RoomSDto::from(rst)))
}
