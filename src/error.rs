use std::fmt;

/// Error type returned by every client operation.
#[derive(Debug)]
pub enum BepaidError {
    #[allow(missing_docs)]
    InvalidRequest(String),
    /// The API responded with a non-2xx status.
    Api(Box<ApiError>),
    /// Transport-level failure (connection, timeout, TLS, …).
    Http(reqwest::Error),
    /// Failed to parse a request or response body as JSON.
    Json(serde_json::Error),
    /// Failed to base64-decode a webhook signature.
    Base64(base64::DecodeError),
    /// Failed to parse the RSA public key for webhook signature verification.
    RsaKey(rsa::pkcs8::spki::Error),
    /// RSA signature parsing/verification failed for a webhook notification.
    RsaSignature(rsa::signature::Error),
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
    #[allow(missing_docs)]
    pub error_code: Option<String>,
    /// Transaction result code.
    pub code: Option<String>,
    /// Customer-facing error message.
    pub friendly_message: Option<String>,
}

impl fmt::Display for BepaidError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidRequest(message) => write!(f, "Invalid request: {message}"),
            Self::Api(e) => write!(f, "API error {}: {}", e.status, e.message),
            Self::Http(e) => write!(f, "HTTP error: {e}"),
            Self::Json(e) => write!(f, "JSON error: {e}"),
            Self::Base64(e) => write!(f, "Base64 error: {e}"),
            Self::RsaKey(e) => write!(f, "RSA key error: {e}"),
            Self::RsaSignature(e) => write!(f, "RSA signature error: {e}"),
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
