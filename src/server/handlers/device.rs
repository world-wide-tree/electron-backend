use std::sync::Arc;

use axum::{extract::{Path, Query, State}, response::IntoResponse, Json};

use crate::{models::query_params::device::DeviceQueryParams, server::models::device::{PostDeviceSDto, PutDeviceSDto}, service::app_state::AppState};

pub async fn get_device_by_id_handler(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse{
    "Get device by id"
}
pub async fn list_device_handler(
    Query(params): Query<DeviceQueryParams>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse{
    "List devices"
}
pub async fn post_device_handler(
    State(state): State<Arc<AppState>>,
    Json(body): Json<PostDeviceSDto>
) -> impl IntoResponse{
    "Post device"
}
pub async fn delete_device_by_id_handler(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse{
    "Delete device by id"
}
pub async fn put_device_handler(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
    Json(body): Json<PutDeviceSDto>
) -> impl IntoResponse{
    "Put device"
}
