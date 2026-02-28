use crate::app::handler::{healths, items, products};
use crate::app::state::AppState;
use crate::pkg::web::request_id_middleware;
use axum::{middleware, routing::get, Router};
use tracing::debug;

pub fn get_routes() -> Router {
    debug!("get_routes start");

    let state = AppState::new();

    let routes = Router::new()
        .route("/livez", get(healths::livez))
        .route("/readyz", get(healths::readyz))
        .route("/products", get(products::list_products).post(products::create_product))
        .route(
            "/products/{id}",
            get(products::get_product)
                .put(products::update_product)
                .delete(products::delete_product),
        )
        .route("/products/{product_id}/items", get(items::list_items_by_product))
        .route("/items", get(items::list_items).post(items::create_item))
        .route(
            "/items/{id}",
            get(items::get_item)
                .put(items::update_item)
                .delete(items::delete_item),
        )
        .with_state(state)
        .layer(middleware::from_fn(request_id_middleware));

    debug!("get_routes end");
    Router::new().nest("/api/v1", routes)
}
