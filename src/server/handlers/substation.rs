use std::sync::Arc;

use axum::{extract::{Path, Query, State}, response::IntoResponse, Json};

use crate::{models::query_params::substation::SubstationQueryParams, server::models::substation::{PostSubstationSDto, PutSubstationSDto}, service::app_state::AppState};

pub async fn list_substation_handler(
    Query(params): Query<SubstationQueryParams>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse{
    "List substations"
}
pub async fn post_substation_handler(
    State(state): State<Arc<AppState>>,
    Json(body): Json<PostSubstationSDto>,
) -> impl IntoResponse{
    "Post substation"
}
pub async fn get_substation_by_id_handler(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse{
    "Get substation by id"
}
pub async fn delete_substation_by_id_handler(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse{
    "Delete substation by id"
}
pub async fn put_substation_handler(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
    Json(body): Json<PutSubstationSDto>
) -> impl IntoResponse{
    "Put substation"
}
