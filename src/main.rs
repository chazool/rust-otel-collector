use rust_otel_collector::{app::route, pkg::web};
use rust_otel_collector::pkg::tracing::tracing as custom_tracing;

#[tokio::main]
async fn main() {
    tracing::info!("rust-otel-collector service starting...");

    rust_otel_collector::pkg::config::app_config::init_app_config();
    let _guard = custom_tracing::init_tracing_subscriber();

    let app = route::get_routes();
    web::init_web(app).await;

    tracing::info!("rust-otel-collector service stopping...");
}
