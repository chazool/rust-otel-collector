use crate::app::service::healths as health_service;
use axum::{http::StatusCode, response::IntoResponse, Json};
use tracing::{debug, instrument};

/// Check service liveness.
///
/// Returns the liveness status of the service. This endpoint should always return 200 OK
/// if the service is running, regardless of its internal state.
#[instrument(name = "handler.livez")]
pub async fn livez() -> impl IntoResponse {
    debug!("livez start");
    let res = health_service::livez();
    debug!(message = "livez end", res = ?res);
    (StatusCode::OK, Json(res))
}

/// Check service readiness.
#[instrument(name = "handler.readyz")]
pub async fn readyz() -> impl IntoResponse {
    debug!("readyz start");
    let res = health_service::readyz();
    debug!(message = "readyz end", res = ?res);
    (StatusCode::OK, Json(res))
}
