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

#[derive(serde::Serialize)]
struct ApplePayPaymentRequest<'a> {
    request: &'a str,
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
                Some("2"),
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

    /// Create a payment token for the payment page or widget.
    ///
    /// Response `redirect_url` sends the customer to the hosted payment
    /// widget; with `auto_pay` and a card token the payment starts itself.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use bepaid::BepaidClient;
    /// use bepaid::types::{CheckoutAdditionalData, CheckoutOrder, CheckoutRequest};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), bepaid::BepaidError> {
    ///     let client = BepaidClient::new("shop_id", "secret_key");
    ///     let request = CheckoutRequest {
    ///         test: Some(true),
    ///         transaction_type: "payment".to_owned(),
    ///         attempts: None,
    ///         iframe: None,
    ///         settings: None,
    ///         payment_method: None,
    ///         credit_card: None,
    ///         order: CheckoutOrder {
    ///             currency: "USD".to_owned(),
    ///             amount: 100,
    ///             description: Some("Widget order".to_owned()),
    ///             tracking_id: None,
    ///             expired_at: None,
    ///             additional_data: Some(CheckoutAdditionalData {
    ///                 contract: Some(vec!["recurring".to_owned()]), extra: None,
    ///             }),
    ///             custom_fields: None,
    ///         },
    ///         customer: None,
    ///         dynamic_billing_descriptor: None,
    ///         travel: None,
    ///     };
    ///     let token = client.create_payment_token(&request).await?;
    ///     println!("redirect: {}", token.redirect_url.unwrap_or_default());
    ///     Ok(())
    /// }
    /// ```
    pub async fn create_payment_token(
        &self,
        req: &CheckoutRequest,
    ) -> Result<CheckoutResponse, BepaidError> {
        let envelope: CheckoutEnvelope = self
            .request_json(
                Method::POST,
                &self.api("/payments/tokens"),
                Some(&CheckoutEnvelopeReq { checkout: req }),
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
            Some("2"),
        )
        .await
    }

    /// Submit an Apple Pay payment token after a successful session.
    ///
    /// The `token` is the base64-strict encoded `event.payment.token` object
    /// returned by Apple Pay.  Response shape is undocumented by bePaid so
    /// raw JSON is returned.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # async fn example(client: bepaid::BepaidClient) -> Result<(), bepaid::BepaidError> {
    /// let result: serde_json::Value = client
    ///     .apple_pay_payment("eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9...")
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn apple_pay_payment(&self, token: &str) -> Result<serde_json::Value, BepaidError> {
        self.request_json(
            Method::POST,
            &self.checkout("/apple_pay/payment"),
            Some(&ApplePayPaymentRequest { request: token }),
            None,
        )
        .await
    }
}
