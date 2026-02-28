use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct LivezResponse {
    /// Service liveness status
    pub status: bool,
}

#[derive(Debug, Serialize, Clone)]
pub struct ReadyzResponse {
    /// Service readiness status
    pub status: bool,
}
