use std::sync::Arc;

use axum::{extract::{Path, Query, State}, http::StatusCode, response::IntoResponse, Json};

use crate::{models::query_params::home::HomeQueryParams, server::models::home::{HomeSDto, PostHomeSDto, PutHomeSDto}, service::app_state::AppState};

pub async fn get_home_by_id_handler(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse{
    let rst = state.home().get_home_by_id(id).await;
    (StatusCode::OK, Json(HomeSDto::from(rst)))
}
pub async fn list_home_handler(
    Query(params): Query<HomeQueryParams>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse{
    let rst = state.home().get_all_homes(params).await;
    (StatusCode::OK, Json(rst))
}
pub async fn post_home_handler(
    State(state): State<Arc<AppState>>,
    Json(body): Json<PostHomeSDto>,
) -> impl IntoResponse{
    let rst = state.home().post_home(body.into()).await;
    (StatusCode::OK, Json(HomeSDto::from(rst)))
}
pub async fn delete_home_by_id_handler(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse{
    let rst = state.home().delete_home(id).await;
    (StatusCode::OK, Json(rst))
}
pub async fn put_home_handler(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
    Json(body): Json<PutHomeSDto>
) -> impl IntoResponse{
    let rst = state.home().put_home(id, body.into()).await;
    (StatusCode::OK, Json(HomeSDto::from(rst)))
}
