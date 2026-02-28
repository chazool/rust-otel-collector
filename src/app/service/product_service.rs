use crate::app::dto::product::{CreateProductRequest, ProductResponse, UpdateProductRequest};
use crate::app::state::AppState;
use tracing::instrument;
use uuid::Uuid;

#[instrument(skip(state), name = "product_service.create_product")]
pub async fn create_product(state: &AppState, req: CreateProductRequest) -> ProductResponse {
    state
        .product_repo
        .create(req.name, req.description)
        .await
}

#[instrument(skip(state), name = "product_service.get_product", fields(product_id = %id))]
pub async fn get_product(state: &AppState, id: Uuid) -> Option<ProductResponse> {
    state.product_repo.get_by_id(id).await
}

#[instrument(skip(state), name = "product_service.list_products")]
pub async fn list_products(state: &AppState) -> Vec<ProductResponse> {
    state.product_repo.get_all().await
}

#[instrument(skip(state, req), name = "product_service.update_product", fields(product_id = %id))]
pub async fn update_product(
    state: &AppState,
    id: Uuid,
    req: UpdateProductRequest,
) -> Option<ProductResponse> {
    state
        .product_repo
        .update(id, req.name, req.description)
        .await
}

#[instrument(skip(state), name = "product_service.delete_product", fields(product_id = %id))]
pub async fn delete_product(state: &AppState, id: Uuid) -> bool {
    let count = state.item_repo.count_by_product_id(id).await;
    if count > 0 {
        return false;
    }
    state.product_repo.delete(id).await
}
