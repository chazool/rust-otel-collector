use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Clone)]
pub struct ItemResponse {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub price: f64,
    pub product_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct CreateItemRequest {
    pub name: String,
    pub description: String,
    pub price: f64,
    pub product_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct UpdateItemRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub price: Option<f64>,
    pub product_id: Option<Uuid>,
}
