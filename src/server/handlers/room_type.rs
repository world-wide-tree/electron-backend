use std::sync::Arc;

use axum::{extract::{Path, Query, State}, http::StatusCode, response::IntoResponse, Json};

use crate::{models::query_params::room::RoomTypeQueryParams, server::models::room::{PostRoomTypeSDto, PutRoomTypeSDto, RoomTypeSDto}, service::app_state::AppState};

pub async fn get_room_type_by_id_handler(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let rst = state.room().get_room_type_by_id(id).await;
    (StatusCode::OK, Json(RoomTypeSDto::from(rst)))
}
pub async fn list_room_type_handler(
    Query(params): Query<RoomTypeQueryParams>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let rst = state.room().get_all_room_types(params).await;
    (StatusCode::OK, Json(rst))
}
pub async fn post_room_type_handler(
    State(state): State<Arc<AppState>>,
    Json(body): Json<PostRoomTypeSDto>,
) -> impl IntoResponse {
    let rst = state.room().post_room_type(body.into()).await;
    (StatusCode::OK, Json(RoomTypeSDto::from(rst)))
}
pub async fn delete_room_type_handler(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let rst = state.room().delete_room_type_by_id(id).await;
    (StatusCode::OK, Json(rst))
}
pub async fn put_room_type_handler(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
    Json(body): Json<PutRoomTypeSDto>
) -> impl IntoResponse {
    let rst = state.room().put_room_type(id, body.into()).await;
    (StatusCode::OK, Json(RoomTypeSDto::from(rst)))
}