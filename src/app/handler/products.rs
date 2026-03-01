use crate::app::dto::product::{CreateProductRequest, UpdateProductRequest};
use crate::app::state::AppState;
use crate::app::service::product_service;
use crate::pkg::web::RequestId;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use tracing::instrument;
use uuid::Uuid;

#[instrument(skip(state), name = "handler.get_product", fields(product_id = %id))]
pub async fn get_product(
    request_id: RequestId,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match product_service::get_product(&state, id).await {
        Some(p) => (StatusCode::OK, Json(p)).into_response(),
        None => (StatusCode::NOT_FOUND, Json(serde_json::json!({"error": "Product not found"}))).into_response(),
    }
}

#[instrument(skip(state), name = "handler.list_products")]
pub async fn list_products(request_id: RequestId, State(state): State<AppState>) -> impl IntoResponse {
    let products = product_service::list_products(&state).await;
    (StatusCode::OK, Json(products))
}

#[instrument(skip(state, body), name = "handler.create_product")]
pub async fn create_product(
    request_id: RequestId,
    State(state): State<AppState>,
    Json(body): Json<CreateProductRequest>,
) -> impl IntoResponse {
    let product = product_service::create_product(&state, body).await;
    (StatusCode::CREATED, Json(product))
}

#[instrument(skip(state, body), name = "handler.update_product", fields(product_id = %id))]
pub async fn update_product(
    request_id: RequestId,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateProductRequest>,
) -> impl IntoResponse {
    match product_service::update_product(&state, id, body).await {
        Some(p) => (StatusCode::OK, Json(p)).into_response(),
        None => (StatusCode::NOT_FOUND, Json(serde_json::json!({"error": "Product not found"}))).into_response(),
    }
}

#[instrument(skip(state), name = "handler.delete_product", fields(product_id = %id))]
pub async fn delete_product(
    request_id: RequestId,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match product_service::delete_product(&state, id).await {
        true => StatusCode::NO_CONTENT.into_response(),
        false => (
            StatusCode::CONFLICT,
            Json(serde_json::json!({"error": "Cannot delete product: it has items. Delete items first."})),
        )
            .into_response(),
    }
}
