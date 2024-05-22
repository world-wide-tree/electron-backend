use axum::{extract::{Path, Query}, response::IntoResponse, Json};

pub async fn list_substation_handler(
    Query(params): Query<String>
) -> impl IntoResponse{
    "List substations"
}
pub async fn post_substation_handler(
    Json(body): Json<String>
) -> impl IntoResponse{
    "Post substation"
}
pub async fn get_substation_by_id_handler(
    Path(id): Path<String>
) -> impl IntoResponse{
    "Get substation by id"
}
pub async fn delete_substation_by_id_handler(
    Path(id): Path<String>
) -> impl IntoResponse{
    "Delete substation by id"
}
pub async fn put_substation_handler(
    Path(id): Path<String>,
    Json(body): Json<String>
) -> impl IntoResponse{
    "Put substation"
}
