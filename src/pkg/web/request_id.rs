//! Request ID middleware: extract or generate ID, store in extensions, set response header, log.

use axum::{
    extract::{FromRequestParts, Request},
    http::{header::HeaderName, HeaderValue, request::Parts, StatusCode},
    middleware::Next,
    response::Response,
};
use tracing;
use uuid::Uuid;

pub const REQUEST_ID_HEADER: &str = "x-request-id";

/// Stored in request extensions; use as extractor so #[instrument] records it on the span.
#[derive(Clone, Copy, Debug)]
pub struct RequestId(pub Uuid);

impl std::fmt::Display for RequestId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<S> FromRequestParts<S> for RequestId
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<RequestId>()
            .copied()
            .ok_or((StatusCode::INTERNAL_SERVER_ERROR, "missing request_id"))
    }
}

/// Extract or generate request ID, insert into extensions, set response header, log start/complete.
pub async fn request_id_middleware(mut request: Request, next: Next) -> Response {
    let request_id = extract_or_generate_request_id(&request);
    let method = request.method().to_string();
    let path = request.uri().path().to_string();

    request.extensions_mut().insert(RequestId(request_id));

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
