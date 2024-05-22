use axum::{extract::{Path, Query}, response::IntoResponse, Json};

pub async fn get_home_by_id_handler(
    Path(id): Path<String>
) -> impl IntoResponse{
    "Get home by id"
}
pub async fn list_home_handler(
    Query(params): Query<String>
) -> impl IntoResponse{
    "List homes"
}
pub async fn post_home_handler(
    Json(body): Json<String>
) -> impl IntoResponse{
    "Post home"
}
pub async fn delete_home_by_id_handler(
    Path(id): Path<String>
) -> impl IntoResponse{
    "Delete home by id"
}
pub async fn put_home_handler(
    Path(id): Path<String>,
    Json(body): Json<String>
) -> impl IntoResponse{
    "Put home"
}
