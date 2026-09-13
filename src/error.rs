use std::fmt;

#[derive(Debug)]
pub enum BepaidError {
    Api(ApiError),
    Http(reqwest::Error),
    Json(serde_json::Error),
}

#[derive(Debug)]
pub struct ApiError {
    pub status: u16,
    pub message: String,
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
