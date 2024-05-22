use axum::{extract::{Path, Query}, response::IntoResponse, Json};

pub async fn get_room_by_id_handler(
    Path(id): Path<String>
) -> impl IntoResponse{
    "Get room by id"
}
pub async fn list_room_handler(
    Query(params): Query<String>
) -> impl IntoResponse{
    "Get all rooms"
}
pub async fn post_room_handler(
    Json(body): Json<String>
) -> impl IntoResponse{
    "Post room"
}
pub async fn delete_room_by_id_handler(
    Path(id): Path<String>
) -> impl IntoResponse{
    "Delete room by id"
}
pub async fn put_room_handler(
    Path(id): Path<String>,
    Json(body): Json<String>
) -> impl IntoResponse{
    "Put room"
}
