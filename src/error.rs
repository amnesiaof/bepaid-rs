use std::fmt;

/// Error type returned by every client operation.
#[derive(Debug)]
pub enum BepaidError {
    /// The API responded with a non-2xx status.
    Api(ApiError),
    /// Transport-level failure (connection, timeout, TLS, …).
    Http(reqwest::Error),
    /// Failed to parse a request or response body as JSON.
    Json(serde_json::Error),
}

/// Structured error returned by the bePaid API.
#[derive(Debug)]
pub struct ApiError {
    /// HTTP status code of the failed response.
    pub status: u16,
    /// Human-readable error message from the API body.
    pub message: String,
    /// Per-field validation errors, when the API provides them.
    pub errors: Option<serde_json::Value>,
}

impl fmt::Display for BepaidError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Api(e) => write!(f, "API error {}: {}", e.status, e.message),
            Self::Http(e) => write!(f, "HTTP error: {e}"),
            Self::Json(e) => write!(f, "JSON error: {e}"),
        }
    }
}

impl std::error::Error for BepaidError {}

impl From<reqwest::Error> for BepaidError {
    fn from(e: reqwest::Error) -> Self {
        Self::Http(e)
    }
}

impl From<serde_json::Error> for BepaidError {
    fn from(e: serde_json::Error) -> Self {
        Self::Json(e)
    }
}
