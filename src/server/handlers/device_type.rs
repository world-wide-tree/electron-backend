use std::sync::Arc;

use axum::{extract::{Path, Query, State}, http::StatusCode, response::IntoResponse, Json};

use crate::{models::query_params::device::DeviceTypeQueryParams, server::models::device::{PostDeviceTypeSDto, PutDeviceSDto, PutDeviceTypeSDto}, service::app_state::AppState};

pub async fn get_device_type_by_id_handler(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse{
    let rst = state.device().get_device_type_by_id(id).await;
    (StatusCode::OK, Json(rst))
}
pub async fn list_device_type_handler(
    Query(params): Query<DeviceTypeQueryParams>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse{
    let rst = state.device().get_all_deivece_types(params).await;
    (StatusCode::OK, Json(rst))
}
pub async fn post_device_type_handler(
    State(state): State<Arc<AppState>>,
    Json(body): Json<PostDeviceTypeSDto>
) -> impl IntoResponse{
    let rst = state.device().post_device_type(body.into()).await;
    (StatusCode::OK, Json(rst))
}
pub async fn delete_device_type_by_id_handler(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse{
    let rst = state.device().delete_device_type_by_id(id).await;
    (StatusCode::OK, Json(rst))
}
pub async fn put_device_type_handler(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
    Json(body): Json<PutDeviceTypeSDto>
) -> impl IntoResponse{
    let rst = state.device().put_device_type(id, body.into()).await;
    (StatusCode::OK, Json(rst))
}
