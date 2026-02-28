use crate::app::dto::item::ItemResponse;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::instrument;
use uuid::Uuid;

#[derive(Clone, Default)]
pub struct ItemRepository {
    store: Arc<RwLock<HashMap<Uuid, ItemResponse>>>,
}

impl ItemRepository {
    pub fn new() -> Self {
        Self {
            store: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    #[instrument(skip(self), name = "item_repo.create", fields(product_id = %product_id))]
    pub async fn create(
        &self,
        name: String,
        description: String,
        price: f64,
        product_id: Uuid,
    ) -> ItemResponse {
        let id = Uuid::new_v4();
        let item = ItemResponse {
            id,
            name,
            description,
            price,
            product_id,
        };
        self.store.write().await.insert(id, item.clone());
        item
    }

    #[instrument(skip(self), name = "item_repo.get_by_id", fields(item_id = %id))]
    pub async fn get_by_id(&self, id: Uuid) -> Option<ItemResponse> {
        self.store.read().await.get(&id).cloned()
    }

    #[instrument(skip(self), name = "item_repo.get_all")]
    pub async fn get_all(&self) -> Vec<ItemResponse> {
        self.store.read().await.values().cloned().collect()
    }

    #[instrument(skip(self), name = "item_repo.get_by_product_id", fields(product_id = %product_id))]
    pub async fn get_by_product_id(&self, product_id: Uuid) -> Vec<ItemResponse> {
        self.store
            .read()
            .await
            .values()
            .filter(|i| i.product_id == product_id)
            .cloned()
            .collect()
    }

    #[instrument(skip(self), name = "item_repo.update", fields(item_id = %id))]
    pub async fn update(
        &self,
        id: Uuid,
        name: Option<String>,
        description: Option<String>,
        price: Option<f64>,
        product_id: Option<Uuid>,
    ) -> Option<ItemResponse> {
        let mut store = self.store.write().await;
        let item = store.get_mut(&id)?;
        if let Some(n) = name {
            item.name = n;
        }
        if let Some(d) = description {
            item.description = d;
        }
        if let Some(p) = price {
            item.price = p;
        }
        if let Some(pid) = product_id {
            item.product_id = pid;
        }
        Some(item.clone())
    }

    #[instrument(skip(self), name = "item_repo.delete", fields(item_id = %id))]
    pub async fn delete(&self, id: Uuid) -> bool {
        self.store.write().await.remove(&id).is_some()
    }

    #[instrument(skip(self), name = "item_repo.count_by_product_id", fields(product_id = %product_id))]
    pub async fn count_by_product_id(&self, product_id: Uuid) -> usize {
        self.store
            .read()
            .await
            .values()
            .filter(|i| i.product_id == product_id)
            .count()
    }
}
