use axum::{extract::{Path, Query}, response::IntoResponse, Json};

pub async fn get_device_by_id_handler(
    Path(id): Path<String>
) -> impl IntoResponse{
    "Get device by id"
}
pub async fn list_device_handler(
    Query(params): Query<String>
) -> impl IntoResponse{
    "List devices"
}
pub async fn post_device_handler(
    Json(body): Json<String>
) -> impl IntoResponse{
    "Post device"
}
pub async fn delete_device_by_id_handler(
    Path(id): Path<String>
) -> impl IntoResponse{
    "Delete device by id"
}
pub async fn put_device_handler(
    Path(id): Path<String>,
    Json(body): Json<String>
) -> impl IntoResponse{
    "Put device"
}
