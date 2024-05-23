use std::sync::Arc;

use axum::{extract::{Path, Query, State}, http::StatusCode, response::IntoResponse, Json};

use crate::{models::query_params::substation::SubstationQueryParams, server::models::substation::{PostSubstationSDto, PutSubstationSDto, SubstationSDto}, service::app_state::AppState};

pub async fn list_substation_handler(
    Query(params): Query<SubstationQueryParams>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse{
    let rst = state.substation().get_all_substation(params).await;
    (StatusCode::OK, Json(rst))
}
pub async fn post_substation_handler(
    State(state): State<Arc<AppState>>,
    Json(body): Json<PostSubstationSDto>,
) -> impl IntoResponse{
    let rst = state.substation().post_substation(body.into()).await;
    (StatusCode::OK, Json(SubstationSDto::from(rst)))
}
pub async fn get_substation_by_id_handler(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse{
    let rst = state.substation().get_substation_by_id(id).await;
    (StatusCode::OK, Json(SubstationSDto::from(rst)))
}
pub async fn delete_substation_by_id_handler(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse{
    let rst = state.substation().delete_substation_by_id(id).await;
    (StatusCode::OK, Json(rst))
}
pub async fn put_substation_handler(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
    Json(body): Json<PutSubstationSDto>
) -> impl IntoResponse{
    let rst = state.substation().put_substation(id, body.into()).await;
    (StatusCode::OK, Json(SubstationSDto::from(rst)))
}
