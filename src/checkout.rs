use reqwest::Method;

use crate::client::BepaidClient;
use crate::error::BepaidError;
use crate::types::{
    CheckoutEnvelope, CheckoutRequest, CheckoutResponse, CheckoutStatus, CheckoutStatusEnvelope,
};

#[derive(serde::Serialize)]
struct CheckoutEnvelopeReq<'a> {
    checkout: &'a CheckoutRequest,
}

#[derive(serde::Serialize)]
struct ApplePayValidateRequest<'a> {
    url: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    token: Option<&'a str>,
    context: &'static str,
}

impl BepaidClient {
    /// Create a hosted checkout (redirect customer to `redirect_url`).
    pub async fn create_checkout(
        &self,
        req: &CheckoutRequest,
    ) -> Result<CheckoutResponse, BepaidError> {
        let envelope: CheckoutEnvelope = self
            .request_json(
                Method::POST,
                &self.checkout("/ctp/api/checkouts"),
                Some(&CheckoutEnvelopeReq { checkout: req }),
                None,
            )
            .await?;
        Ok(envelope.checkout)
    }

    /// Get checkout / payment status by token.
    pub async fn get_checkout_status(&self, token: &str) -> Result<CheckoutStatus, BepaidError> {
        let envelope: CheckoutStatusEnvelope = self
            .request_json(
                Method::GET,
                &self.checkout(&format!("/ctp/api/checkouts/{token}")),
                None::<&u8>,
                None,
            )
            .await?;
        Ok(envelope.checkout)
    }

    /// Validate Apple Pay merchant session.
    pub async fn validate_apple_pay(
        &self,
        url: &str,
        token: Option<&str>,
    ) -> Result<serde_json::Value, BepaidError> {
        self.request_json(
            Method::POST,
            &self.checkout("/ctp/api/apple_pay/validate"),
            Some(&ApplePayValidateRequest {
                url,
                token,
                context: "merchant",
            }),
            None,
        )
        .await
    }
}
