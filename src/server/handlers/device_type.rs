use std::sync::Arc;

use axum::{extract::{Path, Query, State}, response::IntoResponse, Json};

use crate::{models::query_params::device::DeviceTypeQueryParams, server::models::device::{PostDeviceTypeSDto, PutDeviceSDto}, service::app_state::AppState};

pub async fn get_device_type_by_id_handler(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse{
    "Get device-type by id"
}
pub async fn list_device_type_handler(
    Query(params): Query<DeviceTypeQueryParams>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse{
    "List device-types"
}
pub async fn post_device_type_handler(
    State(state): State<Arc<AppState>>,
    Json(body): Json<PostDeviceTypeSDto>
) -> impl IntoResponse{
    "Post device-type"
}
pub async fn delete_device_type_by_id_handler(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse{
    "Delete device-type by id"
}
pub async fn put_device_type_handler(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
    Json(body): Json<PutDeviceSDto>
) -> impl IntoResponse{
    "Put device-type"
}
