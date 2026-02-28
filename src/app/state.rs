use crate::app::repository::{ItemRepository, ProductRepository};

#[derive(Clone)]
pub struct AppState {
    pub product_repo: ProductRepository,
    pub item_repo: ItemRepository,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            product_repo: ProductRepository::new(),
            item_repo: ItemRepository::new(),
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
