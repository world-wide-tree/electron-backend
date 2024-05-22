use std::sync::Arc;

use axum::{extract::{Path, Query, State}, response::IntoResponse, Json};

use crate::{models::query_params::home::HomeQueryParams, server::models::home::{PostHomeSDto, PutHomeSDto}, service::app_state::AppState};

pub async fn get_home_by_id_handler(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse{
    "Get home by id"
}
pub async fn list_home_handler(
    Query(params): Query<HomeQueryParams>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse{
    "List homes"
}
pub async fn post_home_handler(
    State(state): State<Arc<AppState>>,
    Json(body): Json<PostHomeSDto>,
) -> impl IntoResponse{
    "Post home"
}
pub async fn delete_home_by_id_handler(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse{
    "Delete home by id"
}
pub async fn put_home_handler(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
    Json(body): Json<PutHomeSDto>
) -> impl IntoResponse{
    "Put home"
}
