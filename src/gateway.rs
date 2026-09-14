//! Card payments, authorizations, captures, voids and refunds.
//!
//! See the methods on [`BepaidClient`].

use reqwest::Method;

use crate::client::BepaidClient;
use crate::error::BepaidError;
use crate::types::{
    AuthorizationEnvelope, AuthorizationRequest, AuthorizationResponse, CaptureEnvelope,
    CaptureRequest, CaptureResponse, ChargeRequest, PaymentRequest, PaymentResponse,
    PayoutEnvelope, PayoutRequest, PayoutResponse, RecipientTokenizationRequest, RefundEnvelope,
    RefundRequest, RefundResponse, TrackingIdStatus, Transaction, TransactionEnvelope,
    TransactionEnvelopeFull, VoidEnvelope, VoidRequest, VoidResponse,
};

#[derive(serde::Serialize)]
struct RequestEnvelope<T> {
    request: T,
}

impl BepaidClient {
    /// Create a payment. Returns tracking_id + uid.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use bepaid::BepaidClient;
    /// use bepaid::types::{CreditCardRaw, PaymentRequest};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), bepaid::BepaidError> {
    ///     let client = BepaidClient::new("shop_id", "secret_key");
    ///     let request = PaymentRequest {
    ///         amount: "700".to_owned(), // minor units
    ///         currency: "BYN".to_owned(),
    ///         test: true,
    ///         description: "Test payment".to_owned(),
    ///         tracking_id: "order-123".to_owned(),
    ///         language: None,
    ///         notification_url: None,
    ///         verification_url: None,
    ///         return_url: None,
    ///         billing_address: None,
    ///         credit_card: Some(CreditCardRaw {
    ///             number: "4242424242424242".to_owned(),
    ///             verification_value: "123".to_owned(),
    ///             holder: "John Smith".to_owned(),
    ///             exp_month: 10,
    ///             exp_year: 2030,
    ///             save_card: None,
    ///             token: None,
    ///         }),
    ///         customer: None,
    ///         additional_data: None,
    ///         encrypted_data: None,
    ///         fiscalization: None,
    ///     };
    ///     let payment = client.create_payment(request).await?;
    ///     println!("uid: {}", payment.uid);
    ///     Ok(())
    /// }
    /// ```
    pub async fn create_payment(
        &self,
        req: PaymentRequest,
    ) -> Result<PaymentResponse, BepaidError> {
        let envelope: TransactionEnvelope = self
            .request_json(
                Method::POST,
                &self.gateway("/transactions/payments"),
                Some(&RequestEnvelope { request: req }),
                Some("3"),
            )
            .await?;
        Ok(envelope.transaction)
    }

    /// Create an authorization (may require 3-D Secure redirect).
    ///
    /// # Example
    ///
    /// ```no_run
    /// use bepaid::BepaidClient;
    /// use bepaid::types::{AuthorizationRequest, CreditCardRaw};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), bepaid::BepaidError> {
    ///     let client = BepaidClient::new("shop_id", "secret_key");
    ///     let request = AuthorizationRequest {
    ///         amount: 104,
    ///         currency: "EUR".to_owned(),
    ///         description: "Test authorization".to_owned(),
    ///         payment_method_type: None,
    ///         tracking_id: "order-124".to_owned(),
    ///         test: Some(true),
    ///         credit_card: Some(CreditCardRaw {
    ///             number: "4242424242424242".to_owned(),
    ///             verification_value: "123".to_owned(),
    ///             holder: "John Smith".to_owned(),
    ///             exp_month: 10,
    ///             exp_year: 2030,
    ///             save_card: None,
    ///             token: None,
    ///         }),
    ///         customer: None,
    ///         billing_address: None,
    ///     };
    ///     let authorization = client.create_authorization(request).await?;
    ///     if let Some(url) = authorization.redirect_url {
    ///         // send the customer to url for 3-D Secure, then poll the transaction
    ///         println!("redirect: {url}");
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub async fn create_authorization(
        &self,
        req: AuthorizationRequest,
    ) -> Result<AuthorizationResponse, BepaidError> {
        let envelope: AuthorizationEnvelope = self
            .request_json(
                Method::POST,
                &self.gateway("/transactions/authorizations"),
                Some(&RequestEnvelope { request: req }),
                Some("3"),
            )
            .await?;
        Ok(envelope.transaction)
    }

    /// Capture previously authorized funds.
    pub async fn capture(&self, req: CaptureRequest) -> Result<CaptureResponse, BepaidError> {
        let envelope: CaptureEnvelope = self
            .request_json(
                Method::POST,
                &self.gateway("/transactions/captures"),
                Some(&RequestEnvelope { request: req }),
                Some("3"),
            )
            .await?;
        Ok(envelope.transaction)
    }

    /// Void a previously authorized transaction.
    pub async fn void(&self, req: VoidRequest) -> Result<VoidResponse, BepaidError> {
        let envelope: VoidEnvelope = self
            .request_json(
                Method::POST,
                &self.gateway("/transactions/voids"),
                Some(&RequestEnvelope { request: req }),
                Some("3"),
            )
            .await?;
        Ok(envelope.transaction)
    }

    /// Full or partial refund of a payment.
    pub async fn refund(&self, req: RefundRequest) -> Result<RefundResponse, BepaidError> {
        let envelope: RefundEnvelope = self
            .request_json(
                Method::POST,
                &self.gateway("/transactions/refunds"),
                Some(&RequestEnvelope { request: req }),
                Some("3"),
            )
            .await?;
        Ok(envelope.transaction)
    }

    /// Get transaction status by uid.
    pub async fn get_transaction(&self, uid: &str) -> Result<Transaction, BepaidError> {
        let envelope: TransactionEnvelopeFull = self
            .request_json(
                Method::GET,
                &self.gateway(&format!("/transactions/{uid}")),
                None::<&u8>,
                Some("3"),
            )
            .await?;
        Ok(envelope.transaction)
    }

    /// Get transaction status by the merchant's tracking id.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use bepaid::BepaidClient;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), bepaid::BepaidError> {
    ///     let client = BepaidClient::new("shop_id", "secret_key");
    ///     let status = client
    ///         .get_transaction_by_tracking_id("order-123")
    ///         .await?;
    ///     println!("status: {}", status.transaction_status.unwrap_or_default());
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_transaction_by_tracking_id(
        &self,
        tracking_id: &str,
    ) -> Result<TrackingIdStatus, BepaidError> {
        self.request_json(
            Method::GET,
            &self.gateway(&format!("/v2/transactions/tracking_id/{tracking_id}")),
            None::<&u8>,
            None,
        )
        .await
    }

    /// Create a card payout from the merchant balance.
    pub async fn create_payout(&self, req: PayoutRequest) -> Result<PayoutResponse, BepaidError> {
        let envelope: PayoutEnvelope = self
            .request_json(
                Method::POST,
                &self.gateway("/transactions/payouts"),
                Some(&RequestEnvelope { request: req }),
                Some("3"),
            )
            .await?;
        Ok(envelope.transaction)
    }

    /// Charge a previously tokenized card (oneclick / recurring).
    ///
    /// Takes a `credit_card.token` from a previous tokenization instead of card
    /// details.  When 3-D Secure applies, `redirect_url` in the response points
    /// to the customer verification page.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use bepaid::BepaidClient;
    /// use bepaid::types::{ChargeCreditCard, ChargeRequest};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), bepaid::BepaidError> {
    ///     let client = BepaidClient::new("shop_id", "secret_key");
    ///     let request = ChargeRequest {
    ///         amount: 700,
    ///         currency: "USD".to_owned(),
    ///         description: "Recurring charge".to_owned(),
    ///         tracking_id: Some("sub-123".to_owned()),
    ///         expired_at: None,
    ///         duplicate_check: None,
    ///         dynamic_billing_descriptor: None,
    ///         language: None,
    ///         notification_url: None,
    ///         verification_url: None,
    ///         return_url: None,
    ///         test: Some(true),
    ///         force_three_d_secure_verification: None,
    ///         credit_card: ChargeCreditCard {
    ///             number: None,
    ///             verification_value: None,
    ///             holder: None,
    ///             exp_month: None,
    ///             exp_year: None,
    ///             token: Some("13dded21-9b5e-4a18-91d2-9c24944c57d9".to_owned()),
    ///             skip_three_d_secure_verification: None,
    ///         },
    ///         customer: None,
    ///         additional_data: None,
    ///     };
    ///     let charge = client.charge_saved_card(request).await?;
    ///     println!("uid: {}", charge.uid);
    ///     Ok(())
    /// }
    /// ```
    pub async fn charge_saved_card(&self, req: ChargeRequest) -> Result<Transaction, BepaidError> {
        let envelope: TransactionEnvelopeFull = self
            .request_json(
                Method::POST,
                &self.gateway("/services/credit_cards/charges"),
                Some(&RequestEnvelope { request: req }),
                Some("3"),
            )
            .await?;
        Ok(envelope.transaction)
    }

    /// Tokenize a recipient's card for future payouts.
    ///
    /// POST `/transactions/recipient_tokenizations` (X-API-Version 3). No money
    /// moves: the returned card token is used in [`PayoutRequest`] requests.
    /// The response shape is not documented by bePaid, so raw JSON is returned.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use bepaid::BepaidClient;
    /// use bepaid::types::{PayoutCreditCard, RecipientTokenizationRequest};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), bepaid::BepaidError> {
    ///     let client = BepaidClient::new("shop_id", "secret_key");
    ///     let request = RecipientTokenizationRequest {
    ///         description: Some("Tokenize card".to_owned()),
    ///         tracking_id: None,
    ///         recipient_billing_address: None,
    ///         recipient_credit_card: PayoutCreditCard {
    ///             number: Some("4242424242424242".to_owned()),
    ///             holder: Some("John Smith".to_owned()),
    ///             exp_month: Some("10".to_owned()),
    ///             exp_year: Some("2030".to_owned()),
    ///         },
    ///         recipient: None,
    ///         additional_data: None,
    ///     };
    ///     let result: serde_json::Value = client.tokenize_recipient_card(request).await?;
    ///     println!("{result}");
    ///     Ok(())
    /// }
    /// ```
    pub async fn tokenize_recipient_card(
        &self,
        req: RecipientTokenizationRequest,
    ) -> Result<serde_json::Value, BepaidError> {
        self.request_json(
            Method::POST,
            &self.gateway("/transactions/recipient_tokenizations"),
            Some(&RequestEnvelope { request: req }),
            Some("3"),
        )
        .await
    }
}
