use crate::app::service::healths as health_service;
use crate::pkg::web::RequestId;
use axum::{http::StatusCode, response::IntoResponse, Json};
use tracing::{debug, instrument};

/// Liveness probe; returns 200 if the service is running.
#[instrument(name = "handler.livez")]
pub async fn livez(request_id: RequestId) -> impl IntoResponse {
    debug!("livez start");
    let res = health_service::livez();
    debug!(message = "livez end", res = ?res);
    (StatusCode::OK, Json(res))
}

/// Readiness probe.
#[instrument(name = "handler.readyz")]
pub async fn readyz(request_id: RequestId) -> impl IntoResponse {
    debug!("readyz start");
    let res = health_service::readyz();
    debug!(message = "readyz end", res = ?res);
    (StatusCode::OK, Json(res))
}
