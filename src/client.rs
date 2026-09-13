use base64::{Engine, engine::general_purpose::STANDARD};
use reqwest::Client;

use crate::error::{ApiError, BepaidError};

/// Default base URL for the Gateway API (card payments).
pub const DEFAULT_GATEWAY_URL: &str = "https://gateway.bepaid.by";
/// Default base URL for the Checkout API (hosted payment pages).
pub const DEFAULT_CHECKOUT_URL: &str = "https://checkout.bepaid.by";
/// Default base URL for the Direct/APM API.
pub const DEFAULT_API_URL: &str = "https://api.bepaid.by";
/// Default base URL for the Merchant API (reports, payout control).
pub const DEFAULT_MERCHANT_URL: &str = "https://merchant.bepaid.by";

/// Async client for the bePaid payment APIs.
///
/// Holds the `shop_id`/`secret_key` credentials (HTTP Basic auth) and the base
/// URLs. Every operation method lives on this type; see the module docs.
pub struct BepaidClient {
    pub(crate) http: Client,
    pub(crate) gateway_url: String,
    pub(crate) checkout_url: String,
    pub(crate) api_url: String,
    pub(crate) merchant_url: String,
    pub(crate) auth: String,
}

impl BepaidClient {
    /// Create a client with the default production base URLs.
    ///
    /// # Example
    ///
    /// ```
    /// use bepaid::BepaidClient;
    /// let client = BepaidClient::new("shop_id", "secret_key");
    /// ```
    pub fn new(shop_id: &str, secret_key: &str) -> Self {
        let credentials = format!("{shop_id}:{secret_key}");
        let auth = format!("Basic {}", STANDARD.encode(credentials));
        Self {
            http: Client::new(),
            gateway_url: DEFAULT_GATEWAY_URL.to_owned(),
            checkout_url: DEFAULT_CHECKOUT_URL.to_owned(),
            api_url: DEFAULT_API_URL.to_owned(),
            merchant_url: DEFAULT_MERCHANT_URL.to_owned(),
            auth,
        }
    }

    /// Create a client with custom base URLs (e.g. a gateway emulator or tests).
    pub fn with_urls(
        shop_id: &str,
        secret_key: &str,
        gateway_url: &str,
        checkout_url: &str,
        api_url: &str,
        merchant_url: &str,
    ) -> Self {
        let credentials = format!("{shop_id}:{secret_key}");
        let auth = format!("Basic {}", STANDARD.encode(credentials));
        Self {
            http: Client::new(),
            gateway_url: gateway_url.to_owned(),
            checkout_url: checkout_url.to_owned(),
            api_url: api_url.to_owned(),
            merchant_url: merchant_url.to_owned(),
            auth,
        }
    }

    pub(crate) fn gateway(&self, path: &str) -> String {
        format!("{}{path}", self.gateway_url)
    }

    pub(crate) fn checkout(&self, path: &str) -> String {
        format!("{}{path}", self.checkout_url)
    }

    pub(crate) fn api(&self, path: &str) -> String {
        format!("{}{path}", self.api_url)
    }

    pub(crate) fn merchant(&self, path: &str) -> String {
        format!("{}{path}", self.merchant_url)
    }

    pub(crate) async fn request_json<T: serde::de::DeserializeOwned>(
        &self,
        method: reqwest::Method,
        url: &str,
        body: Option<&impl serde::Serialize>,
        api_version: Option<&str>,
    ) -> Result<T, BepaidError> {
        let mut builder = self
            .http
            .request(method.clone(), url)
            .header("Authorization", &self.auth)
            .header("Content-Type", "application/json")
            .header("Accept", "application/json");
        if let Some(v) = api_version {
            builder = builder.header("X-API-Version", v);
        }
        if let Some(b) = body {
            builder = builder.json(b);
        }
        let resp = builder.send().await?;
        let status = resp.status();
        if status.is_success() {
            Ok(resp.json().await?)
        } else {
            let text = resp.text().await.unwrap_or_default();
            let message = serde_json::from_str::<serde_json::Value>(&text)
                .ok()
                .and_then(|v| {
                    v.get("response")
                        .or_else(|| v.get("error"))
                        .and_then(|e| e.get("message"))
                        .and_then(|m| m.as_str())
                        .map(String::from)
                })
                .unwrap_or(text.clone());
            let errors = serde_json::from_str::<serde_json::Value>(&text)
                .ok()
                .and_then(|v| {
                    v.get("response")
                        .or_else(|| v.get("error"))
                        .and_then(|e| e.get("errors"))
                        .cloned()
                });
            Err(BepaidError::Api(ApiError {
                status: status.as_u16(),
                message,
                errors,
            }))
        }
    }
}
