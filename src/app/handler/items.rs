use crate::app::dto::item::{CreateItemRequest, UpdateItemRequest};
use crate::app::state::AppState;
use crate::app::service::item_service;
use crate::pkg::web::RequestId;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use tracing::instrument;
use uuid::Uuid;

#[derive(Debug, serde::Deserialize)]
pub struct ListItemsQuery {
    pub product_id: Option<Uuid>,
}

#[instrument(skip(state), name = "handler.get_item", fields(item_id = %id))]
pub async fn get_item(
    request_id: RequestId,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match item_service::get_item(&state, id).await {
        Some(i) => (StatusCode::OK, Json(i)).into_response(),
        None => (StatusCode::NOT_FOUND, Json(serde_json::json!({"error": "Item not found"}))).into_response(),
    }
}

#[instrument(skip(state), name = "handler.list_items_by_product", fields(product_id = %product_id))]
pub async fn list_items_by_product(
    request_id: RequestId,
    State(state): State<AppState>,
    Path(product_id): Path<Uuid>,
) -> impl IntoResponse {
    let items = item_service::list_items_by_product(&state, product_id).await;
    (StatusCode::OK, Json(items))
}

#[instrument(skip(state), name = "handler.list_items")]
pub async fn list_items(
    request_id: RequestId,
    State(state): State<AppState>,
    Query(query): Query<ListItemsQuery>,
) -> impl IntoResponse {
    let items = match query.product_id {
        Some(pid) => item_service::list_items_by_product(&state, pid).await,
        None => item_service::list_items(&state).await,
    };
    (StatusCode::OK, Json(items))
}

#[instrument(skip(state, body), name = "handler.create_item")]
pub async fn create_item(
    request_id: RequestId,
    State(state): State<AppState>,
    Json(body): Json<CreateItemRequest>,
) -> impl IntoResponse {
    match item_service::create_item(&state, body).await {
        Some(item) => (StatusCode::CREATED, Json(item)).into_response(),
        None => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": "Product not found for product_id"})),
        )
            .into_response(),
    }
}

#[instrument(skip(state, body), name = "handler.update_item", fields(item_id = %id))]
pub async fn update_item(
    request_id: RequestId,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateItemRequest>,
) -> impl IntoResponse {
    match item_service::update_item(&state, id, body).await {
        Some(i) => (StatusCode::OK, Json(i)).into_response(),
        None => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({"error": "Item not found or product_id invalid"})),
        )
            .into_response(),
    }
}

#[instrument(skip(state), name = "handler.delete_item", fields(item_id = %id))]
pub async fn delete_item(
    request_id: RequestId,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    if item_service::delete_item(&state, id).await {
        StatusCode::NO_CONTENT.into_response()
    } else {
        (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({"error": "Item not found"})),
        )
            .into_response()
    }
}
