//! Request ID middleware: extract or generate a request ID, attach to tracing span and response header
//! so traces and structured logs can be correlated by the same ID.

use axum::{
    extract::Request,
    http::{header::HeaderName, HeaderValue},
    middleware::Next,
    response::Response,
};
use tracing::info_span;
use tracing_opentelemetry::OpenTelemetrySpanExt;
use uuid::Uuid;

pub const REQUEST_ID_HEADER: &str = "x-request-id";

/// Returns the request ID from the request (header or generated) and sets a span + response header.
pub async fn request_id_middleware(request: Request, next: Next) -> Response {
    let request_id = extract_or_generate_request_id(&request);
    let method = request.method().to_string();
    let path = request.uri().path().to_string();

    let span = info_span!(
        "request",
        request_id = %request_id,
        http.method = %method,
        http.route = %path,
    );
    // Export request_id and HTTP fields as OTEL span attributes for Jaeger
    span.set_attribute("request_id", request_id.to_string());
    span.set_attribute("http.method", method.clone());
    span.set_attribute("http.route", path.clone());
    let _guard = span.enter();

    tracing::info!(
        request_id = %request_id,
        method = %method,
        path = %path,
        "request started"
    );

    let mut response = next.run(request).await;

    if let Ok(value) = HeaderValue::try_from(request_id.to_string()) {
        response
            .headers_mut()
            .insert(HeaderName::from_static(REQUEST_ID_HEADER), value);
    }

    let status = response.status();
    tracing::info!(
        request_id = %request_id,
        status = %status,
        "request completed"
    );

    response
}

fn extract_or_generate_request_id<B>(request: &Request<B>) -> Uuid {
    request
        .headers()
        .get(REQUEST_ID_HEADER)
        .and_then(|v| v.to_str().ok())
        .and_then(|s| Uuid::parse_str(s).ok())
        .unwrap_or_else(Uuid::new_v4)
}
