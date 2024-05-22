use axum::{extract::{Path, Query}, response::IntoResponse, Json};

pub async fn get_device_type_by_id_handler(
    Path(id): Path<String>
) -> impl IntoResponse{
    "Get device-type by id"
}
pub async fn list_device_type_handler(
    Query(params): Query<String>
) -> impl IntoResponse{
    "List device-types"
}
pub async fn post_device_type_handler(
    Json(body): Json<String>
) -> impl IntoResponse{
    "Post device-type"
}
pub async fn delete_device_type_by_id_handler(
    Path(id): Path<String>
) -> impl IntoResponse{
    "Delete device-type by id"
}
pub async fn put_device_type_handler(
    Path(id): Path<String>,
    Json(body): Json<String>
) -> impl IntoResponse{
    "Put device-type"
}
