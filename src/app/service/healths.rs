use crate::app::dto::healths::{LivezResponse, ReadyzResponse};

pub fn livez() -> LivezResponse {
    LivezResponse { status: true }
}

pub fn readyz() -> ReadyzResponse {
    ReadyzResponse { status: true }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn livez_returns_ok() {
        let res = livez();
        assert!(res.status);
    }

    #[test]
    fn readyz_returns_ok() {
        let res = readyz();
        assert!(res.status);
    }
}
