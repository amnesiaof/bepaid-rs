use base64::{Engine, engine::general_purpose::STANDARD};
use reqwest::Client;

use crate::error::{ApiError, BepaidError};

/// Generic wrapper that serializes as `{"request": T}` — used by all POST
/// endpoints that follow the bePaid envelope convention.
#[derive(serde::Serialize)]
pub(crate) struct RequestEnvelope<T> {
    pub(crate) request: T,
}

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

    pub(crate) async fn request_async_json<T: serde::de::DeserializeOwned>(
        &self,
        url: &str,
    ) -> Result<T, BepaidError> {
        let invalid = || {
            BepaidError::InvalidRequest(
                "async polling URL must use the configured gateway origin without userinfo".into(),
            )
        };
        let destination = reqwest::Url::parse(url).map_err(|_| invalid())?;
        let gateway = reqwest::Url::parse(&self.gateway_url).map_err(|_| invalid())?;
        if !matches!(destination.scheme(), "http" | "https")
            || destination.origin() != gateway.origin()
            || !destination.username().is_empty()
            || destination.password().is_some()
            || url.split_once("://").is_none_or(|(_, rest)| {
                rest.split(['/', '?', '#'])
                    .next()
                    .is_some_and(|authority| authority.contains('@'))
            })
        {
            return Err(invalid());
        }
        let response = Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .build()?
            .get(destination)
            .header("Authorization", &self.auth)
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .header("X-API-Version", "3")
            .send()
            .await?;
        Ok(Self::check_response(response).await?.json().await?)
    }

    pub(crate) async fn request_json<T: serde::de::DeserializeOwned>(
        &self,
        method: reqwest::Method,
        url: &str,
        body: Option<&impl serde::Serialize>,
        api_version: Option<&str>,
    ) -> Result<T, BepaidError> {
        let resp = self
            .send_request(method, url, body, api_version, None)
            .await?;
        Ok(resp.json().await?)
    }

    /// Like [`BepaidClient::request_json`] with an optional idempotency header.
    pub(crate) async fn request_json_with_id<T: serde::de::DeserializeOwned>(
        &self,
        method: reqwest::Method,
        url: &str,
        body: Option<&impl serde::Serialize>,
        api_version: Option<&str>,
        request_id: Option<&str>,
    ) -> Result<T, BepaidError> {
        let resp = self
            .send_request(method, url, body, api_version, request_id)
            .await?;
        Ok(resp.json().await?)
    }

    /// Send a request and discard the body (for 2xx responses without a payload).
    pub(crate) async fn request_status(
        &self,
        method: reqwest::Method,
        url: &str,
        body: Option<&impl serde::Serialize>,
        api_version: Option<&str>,
    ) -> Result<(), BepaidError> {
        self.send_request(method, url, body, api_version, None)
            .await?;
        Ok(())
    }

    /// Sets the `RequestID` header when `request_id` is provided.
    pub(crate) async fn send_request(
        &self,
        method: reqwest::Method,
        url: &str,
        body: Option<&impl serde::Serialize>,
        api_version: Option<&str>,
        request_id: Option<&str>,
    ) -> Result<reqwest::Response, BepaidError> {
        let mut builder = self
            .http
            .request(method.clone(), url)
            .header("Authorization", &self.auth)
            .header("Content-Type", "application/json")
            .header("Accept", "application/json");
        if let Some(v) = api_version {
            builder = builder.header("X-API-Version", v);
        }
        if let Some(id) = request_id {
            builder = builder.header("RequestID", id);
        }
        if let Some(b) = body {
            builder = builder.json(b);
        }
        Self::check_response(builder.send().await?).await
    }

    async fn check_response(resp: reqwest::Response) -> Result<reqwest::Response, BepaidError> {
        let status = resp.status();
        if status.is_success() {
            Ok(resp)
        } else {
            let text = resp.text().await.unwrap_or_default();
            let body = serde_json::from_str::<serde_json::Value>(&text).ok();
            let details = body.as_ref().map(|v| {
                v.get("response")
                    .filter(|e| e.is_object())
                    .or_else(|| v.get("error").filter(|e| e.is_object()))
                    .unwrap_or(v)
            });
            let field = |name: &str| {
                details
                    .and_then(|v| v.get(name))
                    .or_else(|| body.as_ref().and_then(|v| v.get(name)))
                    .filter(|v| !v.is_null())
            };
            let message = field("message")
                .map(|m| {
                    m.as_str()
                        .map(String::from)
                        .unwrap_or_else(|| m.to_string())
                })
                .unwrap_or(text);
            let errors = field("errors")
                .or_else(|| field("message").filter(|m| m.is_object() || m.is_array()))
                .cloned();
            let error_code = field("error_code")
                .and_then(|v| v.as_str())
                .map(String::from);
            Err(BepaidError::Api(Box::new(ApiError {
                status: status.as_u16(),
                message,
                errors,
                error_code,
                code: field("code").and_then(|v| v.as_str()).map(String::from),
                friendly_message: field("friendly_message")
                    .and_then(|v| v.as_str())
                    .map(String::from),
            })))
        }
    }
}
