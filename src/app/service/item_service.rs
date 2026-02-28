use crate::app::dto::item::{CreateItemRequest, ItemResponse, UpdateItemRequest};
use crate::app::state::AppState;
use tracing::instrument;
use uuid::Uuid;

#[instrument(skip(state, req), name = "item_service.create_item")]
pub async fn create_item(state: &AppState, req: CreateItemRequest) -> Option<ItemResponse> {
    if state.product_repo.get_by_id(req.product_id).await.is_none() {
        return None;
    }
    Some(
        state
            .item_repo
            .create(req.name, req.description, req.price, req.product_id)
            .await,
    )
}

#[instrument(skip(state), name = "item_service.get_item", fields(item_id = %id))]
pub async fn get_item(state: &AppState, id: Uuid) -> Option<ItemResponse> {
    state.item_repo.get_by_id(id).await
}

#[instrument(skip(state), name = "item_service.list_items")]
pub async fn list_items(state: &AppState) -> Vec<ItemResponse> {
    state.item_repo.get_all().await
}

#[instrument(skip(state), name = "item_service.list_items_by_product", fields(product_id = %product_id))]
pub async fn list_items_by_product(state: &AppState, product_id: Uuid) -> Vec<ItemResponse> {
    state.item_repo.get_by_product_id(product_id).await
}

#[instrument(skip(state, req), name = "item_service.update_item", fields(item_id = %id))]
pub async fn update_item(
    state: &AppState,
    id: Uuid,
    req: UpdateItemRequest,
) -> Option<ItemResponse> {
    if let Some(pid) = req.product_id {
        if state.product_repo.get_by_id(pid).await.is_none() {
            return None;
        }
    }
    state
        .item_repo
        .update(id, req.name, req.description, req.price, req.product_id)
        .await
}

#[instrument(skip(state), name = "item_service.delete_item", fields(item_id = %id))]
pub async fn delete_item(state: &AppState, id: Uuid) -> bool {
    state.item_repo.delete(id).await
}
