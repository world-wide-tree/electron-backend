use std::sync::Arc;

use axum::{extract::{Path, Query, State}, http::StatusCode, response::IntoResponse, Json};

use crate::{models::query_params::device::DeviceQueryParams, server::models::device::{PostDeviceSDto, PutDeviceSDto}, service::app_state::AppState};

pub async fn get_device_by_id_handler(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse{
    let rst = state.device().get_device_by_id(id).await;
    (StatusCode::OK, Json(rst))
}
pub async fn list_device_handler(
    Query(params): Query<DeviceQueryParams>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse{
    let rst = state.device().get_all_devices(params).await;
    (StatusCode::OK, Json(rst))
}
pub async fn post_device_handler(
    State(state): State<Arc<AppState>>,
    Json(body): Json<PostDeviceSDto>
) -> impl IntoResponse{
    let rst = state.device().post_device(body.into()).await;
    (StatusCode::OK, Json(rst))
}
pub async fn delete_device_by_id_handler(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse{
    let rst = state.device().delete_device_by_id(id).await;
    (StatusCode::OK, Json(rst))
}
pub async fn put_device_handler(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
    Json(body): Json<PutDeviceSDto>
) -> impl IntoResponse{
    let rst = state.device().put_device(id, body.into()).await;
    (StatusCode::OK, Json(rst))
}
