//! Card payments, authorizations, captures, voids and refunds.
//!
//! See the methods on [`BepaidClient`].

use reqwest::Method;

use crate::client::{BepaidClient, RequestEnvelope};
use crate::error::BepaidError;
use crate::types::{
    AsyncAck, AsyncStatus, AuthorizationRequest, CaptureEnvelope, CaptureRequest, CaptureResponse,
    CardBalanceRequest, CardBalanceResponse, ChargeRequest, MasterpassDeleteCardRequest,
    MasterpassDeleteCardResponse, MasterpassGetCardRequest, MasterpassGetCardResponse,
    MasterpassGetCardsRequest, MasterpassGetCardsResponse, MasterpassGetSavedCardRequest,
    MasterpassLoginRequest, MasterpassLoginResponse, PaymentRequest, PayoutEnvelope, PayoutRequest,
    PayoutResponse, RecipientTokenizationRequest, RefundEnvelope, RefundRequest, RefundResponse,
    TokenizationRequest, TrackingIdStatus, Transaction, TransactionEnvelopeFull, VoidEnvelope,
    VoidRequest, VoidResponse,
};

impl BepaidClient {
    /// Log in to Masterpass (`POST /masterpass/login`, API v3).
    pub async fn masterpass_login(
        &self,
        req: MasterpassLoginRequest,
    ) -> Result<MasterpassLoginResponse, BepaidError> {
        self.request_json_with_id(
            Method::POST,
            &self.gateway("/masterpass/login"),
            Some(&req),
            Some("3"),
            None,
        )
        .await
    }

    /// List Masterpass cards (`POST /masterpass/get_cards`, API v3).
    pub async fn masterpass_get_cards(
        &self,
        req: MasterpassGetCardsRequest,
    ) -> Result<MasterpassGetCardsResponse, BepaidError> {
        self.request_json_with_id(
            Method::POST,
            &self.gateway("/masterpass/get_cards"),
            Some(&req),
            Some("3"),
            None,
        )
        .await
    }

    /// Retrieve a Masterpass card (`POST /masterpass/get_card`, API v3).
    pub async fn masterpass_get_card(
        &self,
        req: MasterpassGetCardRequest,
    ) -> Result<MasterpassGetCardResponse, BepaidError> {
        self.request_json_with_id(
            Method::POST,
            &self.gateway("/masterpass/get_card"),
            Some(&req),
            Some("3"),
            None,
        )
        .await
    }

    /// Retrieve a saved Masterpass card (`POST /masterpass/get_saved_card`, API v3).
    pub async fn masterpass_get_saved_card(
        &self,
        req: MasterpassGetSavedCardRequest,
    ) -> Result<MasterpassGetCardResponse, BepaidError> {
        self.request_json_with_id(
            Method::POST,
            &self.gateway("/masterpass/get_saved_card"),
            Some(&req),
            Some("3"),
            None,
        )
        .await
    }

    /// Delete a Masterpass card (`POST /masterpass/delete_card`, API v3).
    pub async fn masterpass_delete_card(
        &self,
        req: MasterpassDeleteCardRequest,
    ) -> Result<MasterpassDeleteCardResponse, BepaidError> {
        self.request_json_with_id(
            Method::POST,
            &self.gateway("/masterpass/delete_card"),
            Some(&req),
            Some("3"),
            None,
        )
        .await
    }

    /// Create a payment and return the full transaction.
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
    ///         duplicate_check: None,
    ///         expired_at: None,
    ///         dynamic_billing_descriptor: None,
    ///         billing_address: None,
    ///         credit_card: Some(CreditCardRaw {
    ///             number: Some("4242424242424242".to_owned()),
    ///             verification_value: Some("123".to_owned()),
    ///             holder: Some("John Smith".to_owned()),
    ///             exp_month: Some(10),
    ///             exp_year: Some(2030),
    ///             save_card: None,
    ///             token: None,
    ///             skip_three_d_secure_verification: None,
    ///             force_three_d_secure_verification: None,
    ///         }),
    ///         customer: None,
    ///         additional_data: None,
    ///         encrypted_data: None,
    ///         fiscalization: None,
    ///         custom_fields: None,
    ///     };
    ///     let payment = client.create_payment(request, None).await?;
    ///     println!("uid: {}", payment.uid);
    ///     Ok(())
    /// }
    /// ```
    pub async fn create_payment(
        &self,
        req: PaymentRequest,
        request_id: Option<&str>,
    ) -> Result<Transaction, BepaidError> {
        let envelope: TransactionEnvelopeFull = self
            .request_json_with_id(
                Method::POST,
                &self.gateway("/transactions/payments"),
                Some(&RequestEnvelope { request: req }),
                Some("3"),
                request_id,
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
    ///         duplicate_check: None,
    ///         language: None,
    ///         notification_url: None,
    ///         return_url: None,
    ///         expired_at: None,
    ///         dynamic_billing_descriptor: None,
    ///         credit_card: Some(CreditCardRaw {
    ///             number: Some("4242424242424242".to_owned()),
    ///             verification_value: Some("123".to_owned()),
    ///             holder: Some("John Smith".to_owned()),
    ///             exp_month: Some(10),
    ///             exp_year: Some(2030),
    ///             save_card: None,
    ///             token: None,
    ///             skip_three_d_secure_verification: None,
    ///             force_three_d_secure_verification: None,
    ///         }),
    ///         customer: None,
    ///         billing_address: None,
    ///         additional_data: None,
    ///         verification_url: None,
    ///         custom_fields: None,
    ///     };
    ///     let authorization = client.create_authorization(request, None).await?;
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
        request_id: Option<&str>,
    ) -> Result<Transaction, BepaidError> {
        let envelope: TransactionEnvelopeFull = self
            .request_json_with_id(
                Method::POST,
                &self.gateway("/transactions/authorizations"),
                Some(&RequestEnvelope { request: req }),
                Some("3"),
                request_id,
            )
            .await?;
        Ok(envelope.transaction)
    }

    /// Capture previously authorized funds.
    pub async fn capture(
        &self,
        req: CaptureRequest,
        request_id: Option<&str>,
    ) -> Result<CaptureResponse, BepaidError> {
        let envelope: CaptureEnvelope = self
            .request_json_with_id(
                Method::POST,
                &self.gateway("/transactions/captures"),
                Some(&RequestEnvelope { request: req }),
                Some("3"),
                request_id,
            )
            .await?;
        Ok(envelope.transaction)
    }

    /// Void a previously authorized transaction.
    pub async fn void(
        &self,
        req: VoidRequest,
        request_id: Option<&str>,
    ) -> Result<VoidResponse, BepaidError> {
        let envelope: VoidEnvelope = self
            .request_json_with_id(
                Method::POST,
                &self.gateway("/transactions/voids"),
                Some(&RequestEnvelope { request: req }),
                Some("3"),
                request_id,
            )
            .await?;
        Ok(envelope.transaction)
    }

    /// Full or partial refund of a payment.
    pub async fn refund(
        &self,
        req: RefundRequest,
        request_id: Option<&str>,
    ) -> Result<RefundResponse, BepaidError> {
        let envelope: RefundEnvelope = self
            .request_json_with_id(
                Method::POST,
                &self.gateway("/transactions/refunds"),
                Some(&RequestEnvelope { request: req }),
                Some("3"),
                request_id,
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
    pub async fn create_payout(
        &self,
        req: PayoutRequest,
        request_id: Option<&str>,
    ) -> Result<PayoutResponse, BepaidError> {
        let envelope: PayoutEnvelope = self
            .request_json_with_id(
                Method::POST,
                &self.gateway("/transactions/payouts"),
                Some(&RequestEnvelope { request: req }),
                Some("3"),
                request_id,
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
    ///     let charge = client.charge_saved_card(request, None).await?;
    ///     println!("uid: {}", charge.uid);
    ///     Ok(())
    /// }
    /// ```
    pub async fn charge_saved_card(
        &self,
        req: ChargeRequest,
        request_id: Option<&str>,
    ) -> Result<Transaction, BepaidError> {
        let envelope: TransactionEnvelopeFull = self
            .request_json_with_id(
                Method::POST,
                &self.gateway("/services/credit_cards/charges"),
                Some(&RequestEnvelope { request: req }),
                Some("3"),
                request_id,
            )
            .await?;
        Ok(envelope.transaction)
    }

    /// Tokenize a card as a 3-D Secure transaction.
    ///
    /// POST `/transactions/tokenizations` (X-API-Version 3). No money moves: the
    /// returned `credit_card.token` is used for later payments. Sending the
    /// customer through 3-D Secure may require `verification_url` and
    /// `return_url`; poll the transaction afterwards.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use bepaid::BepaidClient;
    /// use bepaid::types::{CreditCardRaw, TokenizationRequest};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), bepaid::BepaidError> {
    ///     let client = BepaidClient::new("shop_id", "secret_key");
    ///     let request = TokenizationRequest {
    ///         amount: 700,
    ///         currency: "BYN".to_owned(),
    ///         description: "Tokenize card".to_owned(),
    ///         tracking_id: None,
    ///         duplicate_check: None,
    ///         dynamic_billing_descriptor: None,
    ///         language: None,
    ///         notification_url: None,
    ///         verification_url: None,
    ///         return_url: None,
    ///         test: Some(true),
    ///         billing_address: None,
    ///         credit_card: Some(CreditCardRaw {
    ///             number: Some("4242424242424242".to_owned()),
    ///             verification_value: Some("123".to_owned()),
    ///             holder: Some("John Smith".to_owned()),
    ///             exp_month: Some(10),
    ///             exp_year: Some(2030),
    ///             save_card: None,
    ///             token: None,
    ///             skip_three_d_secure_verification: None,
    ///             force_three_d_secure_verification: None,
    ///         }),
    ///         three_d_secure: None,
    ///         travel: None,
    ///         customer: None,
    ///         additional_data: None,
    ///     };
    ///     let result = client.create_tokenization(request, None).await?;
    ///     if let Some(token) = result.credit_card.and_then(|c| c.token) {
    ///         println!("card token: {token}");
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub async fn create_tokenization(
        &self,
        req: TokenizationRequest,
        request_id: Option<&str>,
    ) -> Result<Transaction, BepaidError> {
        let envelope: TransactionEnvelopeFull = self
            .request_json_with_id(
                Method::POST,
                &self.gateway("/transactions/tokenizations"),
                Some(&RequestEnvelope { request: req }),
                Some("3"),
                request_id,
            )
            .await?;
        Ok(envelope.transaction)
    }

    /// Query the balance of a card gateway account (`POST /balance`, API v2).
    pub async fn get_card_balance(
        &self,
        req: CardBalanceRequest,
    ) -> Result<CardBalanceResponse, BepaidError> {
        self.request_json(
            Method::POST,
            &self.gateway("/balance"),
            Some(&RequestEnvelope { request: req }),
            Some("2"),
        )
        .await
    }

    /// Submit a payment for asynchronous processing
    /// (`POST /async/transactions/payments`, API v3).
    pub async fn create_payment_async(
        &self,
        req: PaymentRequest,
        request_id: Option<&str>,
    ) -> Result<AsyncAck, BepaidError> {
        self.request_json_with_id(
            Method::POST,
            &self.gateway("/async/transactions/payments"),
            Some(&RequestEnvelope { request: req }),
            Some("3"),
            request_id,
        )
        .await
    }

    /// Submit an authorization for asynchronous processing
    /// (`POST /async/transactions/authorizations`, API v3).
    pub async fn create_authorization_async(
        &self,
        req: AuthorizationRequest,
        request_id: Option<&str>,
    ) -> Result<AsyncAck, BepaidError> {
        self.request_json_with_id(
            Method::POST,
            &self.gateway("/async/transactions/authorizations"),
            Some(&RequestEnvelope { request: req }),
            Some("3"),
            request_id,
        )
        .await
    }

    /// Poll the status of an async processing request.
    ///
    /// Takes the absolute `status_url` returned in [`AsyncAck`].
    pub async fn get_async_status(&self, url: &str) -> Result<AsyncStatus, BepaidError> {
        self.request_async_json(url).await
    }

    /// Fetch the final transaction of a completed async request.
    ///
    /// Takes the absolute `response_url` returned in [`AsyncStatus`].
    pub async fn get_async_result(&self, url: &str) -> Result<Transaction, BepaidError> {
        let envelope: TransactionEnvelopeFull = self.request_async_json(url).await?;
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
