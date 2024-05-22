use axum::{extract::{Path, Query}, response::IntoResponse, Json};

pub async fn get_room_type_by_id_handler(
    Path(id): Path<String>
) -> impl IntoResponse {
    "Get room-type by id"
}
pub async fn list_room_type_handler(
    Query(params): Query<String>
) -> impl IntoResponse {
    "Get All room-type "
}
pub async fn post_room_type_handler(
    Json(body): Json<String>
) -> impl IntoResponse {
    "Post room-type"
}
pub async fn delete_room_type_handler(
    Path(id): Path<String>
) -> impl IntoResponse {
    "Delete toom-type by id"
}
pub async fn put_room_type_handler(
    Path(id): Path<String>,
    Json(body): Json<String>
) -> impl IntoResponse {
    "Put room-type"
}