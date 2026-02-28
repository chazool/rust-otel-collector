use axum::Router;
use tokio::net::TcpListener;
use tracing::debug;

use crate::pkg::config::app_config;

pub async fn init_web(app: Router) {
    debug!("init_web start");
    let config = app_config::get_config();
    let addr = format!("0.0.0.0:{}", config.port);
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
    debug!("init_web end");
}
