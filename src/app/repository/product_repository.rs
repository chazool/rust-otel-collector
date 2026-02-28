use crate::app::dto::product::ProductResponse;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::instrument;
use uuid::Uuid;

#[derive(Clone, Default)]
pub struct ProductRepository {
    store: Arc<RwLock<HashMap<Uuid, ProductResponse>>>,
}

impl ProductRepository {
    pub fn new() -> Self {
        Self {
            store: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    #[instrument(skip(self), name = "product_repo.create")]
    pub async fn create(&self, name: String, description: String) -> ProductResponse {
        let id = Uuid::new_v4();
        let product = ProductResponse {
            id,
            name,
            description,
        };
        self.store.write().await.insert(id, product.clone());
        product
    }

    #[instrument(skip(self), name = "product_repo.get_by_id", fields(product_id = %id))]
    pub async fn get_by_id(&self, id: Uuid) -> Option<ProductResponse> {
        self.store.read().await.get(&id).cloned()
    }

    #[instrument(skip(self), name = "product_repo.get_all")]
    pub async fn get_all(&self) -> Vec<ProductResponse> {
        self.store.read().await.values().cloned().collect()
    }

    #[instrument(skip(self), name = "product_repo.update", fields(product_id = %id))]
    pub async fn update(
        &self,
        id: Uuid,
        name: Option<String>,
        description: Option<String>,
    ) -> Option<ProductResponse> {
        let mut store = self.store.write().await;
        let product = store.get_mut(&id)?;
        if let Some(n) = name {
            product.name = n;
        }
        if let Some(d) = description {
            product.description = d;
        }
        Some(product.clone())
    }

    #[instrument(skip(self), name = "product_repo.delete", fields(product_id = %id))]
    pub async fn delete(&self, id: Uuid) -> bool {
        self.store.write().await.remove(&id).is_some()
    }
}
