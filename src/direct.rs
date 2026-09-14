//! Alternative payment methods (APM) and their confirmations.
//!
//! See the methods on [`BepaidClient`].

use reqwest::Method;

use crate::client::BepaidClient;
use crate::error::BepaidError;
use crate::types::{
    ApmConfirmEnvelope, ApmConfirmRequest, ApmConfirmResponse, ApmPaymentEnvelope,
    ApmPaymentRequest, ApmPaymentResponse, ApmPayoutEnvelope, ApmPayoutRequest, ApmPayoutResponse,
    ApmRefundEnvelope, ApmRefundRequest, ApmRefundResponse, BalanceRequest, BalanceResponse,
    CheckupRequest, CurrencyInfo, CurrencyQueryRequest, ProofEnvelope, ProofRequest, ProofResponse,
    SplitPaymentRequest, SplitPaymentResponse, Transaction, TransactionEnvelopeFull,
    TransactionListEnvelope,
};

#[derive(serde::Serialize)]
struct RequestEnvelope<T> {
    request: T,
}

impl BepaidClient {
    /// Pay by an alternative payment method (ERIP, Alfabank, MTS Money, ...).
    /// `payment_method` must carry the method-specific parameters (`{"type": "erip", ...}`).
    pub async fn create_apm_payment(
        &self,
        req: ApmPaymentRequest,
    ) -> Result<ApmPaymentResponse, BepaidError> {
        let envelope: ApmPaymentEnvelope = self
            .request_json(
                Method::POST,
                &self.api("/beyag/transactions/payments"),
                Some(&RequestEnvelope { request: req }),
                None,
            )
            .await?;
        Ok(envelope.transaction)
    }

    /// Full or partial refund of an APM transaction (allows `amount: None` for full).
    pub async fn apm_refund(
        &self,
        req: ApmRefundRequest,
    ) -> Result<ApmRefundResponse, BepaidError> {
        let envelope: ApmRefundEnvelope = self
            .request_json(
                Method::POST,
                &self.api("/beyag/transactions/refunds"),
                Some(&RequestEnvelope { request: req }),
                None,
            )
            .await?;
        Ok(envelope.transaction)
    }

    /// Full refund without reason via the alternative `/beyag/refunds` endpoint.
    pub async fn apm_full_refund(
        &self,
        parent_uid: &str,
        reason: &str,
        amount: Option<i64>,
    ) -> Result<ApmRefundResponse, BepaidError> {
        let req = ApmRefundRequest {
            parent_uid: parent_uid.to_owned(),
            reason: reason.to_owned(),
            amount,
            tracking_id: None,
            additional_data: None,
        };
        let envelope: ApmRefundEnvelope = self
            .request_json(
                Method::POST,
                &self.api("/beyag/refunds"),
                Some(&RequestEnvelope { request: req }),
                None,
            )
            .await?;
        Ok(envelope.transaction)
    }

    /// Confirm a payment from a third-party application.
    pub async fn confirm_apm_payment(
        &self,
        uid: &str,
        req: ApmConfirmRequest,
    ) -> Result<ApmConfirmResponse, BepaidError> {
        let envelope: ApmConfirmEnvelope = self
            .request_json(
                Method::POST,
                &self.api(&format!("/beyag/transactions/{uid}/confirm")),
                Some(&req),
                None,
            )
            .await?;
        Ok(envelope.response)
    }

    /// Get the current status of an APM transaction by uid.
    pub async fn get_apm_transaction(&self, uid: &str) -> Result<Transaction, BepaidError> {
        let envelope: TransactionEnvelopeFull = self
            .request_json(
                Method::GET,
                &self.api(&format!("/beyag/transactions/{uid}")),
                None::<&u8>,
                None,
            )
            .await?;
        Ok(envelope.transaction)
    }

    /// Get APM transactions by the merchant's tracking id.
    pub async fn get_apm_transactions_by_tracking_id(
        &self,
        tracking_id: &str,
    ) -> Result<Vec<Transaction>, BepaidError> {
        let envelope: TransactionListEnvelope = self
            .request_json(
                Method::GET,
                &self.api(&format!("/beyag/transactions/tracking_id/{tracking_id}")),
                None::<&u8>,
                None,
            )
            .await?;
        Ok(envelope.transactions)
    }

    /// Payout an APM payment method (`POST /beyag/transactions/payouts`).
    pub async fn apm_payout(
        &self,
        req: ApmPayoutRequest,
    ) -> Result<ApmPayoutResponse, BepaidError> {
        let envelope: ApmPayoutEnvelope = self
            .request_json(
                Method::POST,
                &self.api("/beyag/transactions/payouts"),
                Some(&RequestEnvelope { request: req }),
                None,
            )
            .await?;
        Ok(envelope.transaction)
    }

    /// Submit proof of payment for an APM transaction.
    pub async fn apm_proof(
        &self,
        uid: &str,
        req: ProofRequest,
    ) -> Result<ProofResponse, BepaidError> {
        let envelope: ProofEnvelope = self
            .request_json(
                Method::POST,
                &self.api(&format!("/beyag/transactions/{uid}/proof")),
                Some(&RequestEnvelope { request: req }),
                None,
            )
            .await?;
        Ok(envelope.transaction)
    }

    /// Run a pre-authorization risk check on a card.
    pub async fn checkup(&self, req: CheckupRequest) -> Result<Transaction, BepaidError> {
        let envelope: TransactionEnvelopeFull = self
            .request_json(
                Method::POST,
                &self.gateway("/transactions/checkups"),
                Some(&RequestEnvelope { request: req }),
                Some("3"),
            )
            .await?;
        Ok(envelope.transaction)
    }

    /// Split a payment among several shops in one request. The `split` map in
    /// `additional_data` assigns an amount (minor units) per recipient shop id.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use std::collections::HashMap;
    ///
    /// use bepaid::BepaidClient;
    /// use bepaid::types::{SplitAdditionalData, SplitCreditCard, SplitPaymentRequest};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), bepaid::BepaidError> {
    ///     let client = BepaidClient::new("shop_id", "secret_key");
    ///     let mut split = HashMap::new();
    ///     split.insert("241".to_owned(), 40);
    ///     split.insert("242".to_owned(), 50);
    ///     let request = SplitPaymentRequest {
    ///         amount: 100,
    ///         currency: "USD".to_owned(),
    ///         description: "Test transaction".to_owned(),
    ///         tracking_id: "tracking_id_000".to_owned(),
    ///         billing_address: None,
    ///         credit_card: SplitCreditCard {
    ///             token: "credit-card-token".to_owned(),
    ///         },
    ///         customer: None,
    ///         additional_data: Some(SplitAdditionalData {
    ///             contract: None,
    ///             split,
    ///         }),
    ///     };
    ///     let response = client.create_split_payment(request).await?;
    ///     println!("{} split transactions", response.splits.len());
    ///     Ok(())
    /// }
    /// ```
    pub async fn create_split_payment(
        &self,
        req: SplitPaymentRequest,
    ) -> Result<SplitPaymentResponse, BepaidError> {
        self.request_json(
            Method::POST,
            &self.api("/splits/payment"),
            Some(&RequestEnvelope { request: req }),
            None,
        )
        .await
    }

    /// Query the balance of an APM gateway account.
    pub async fn get_balance(&self, req: BalanceRequest) -> Result<BalanceResponse, BepaidError> {
        self.request_json(Method::POST, &self.api("/beyag/balance"), Some(&req), None)
            .await
    }

    /// Query which currencies an APM gateway account supports.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use bepaid::BepaidClient;
    /// use bepaid::types::CurrencyQueryRequest;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), bepaid::BepaidError> {
    ///     let client = BepaidClient::new("shop_id", "secret_key");
    ///     let info = client
    ///         .get_currencies(CurrencyQueryRequest {
    ///             gateway_id: 1234,
    ///             account: Some("40701810842020395221".to_owned()),
    ///             country: Some("GB".to_owned()),
    ///         })
    ///         .await?;
    ///     println!("currency: {:?}", info.currency);
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_currencies(
        &self,
        req: CurrencyQueryRequest,
    ) -> Result<CurrencyInfo, BepaidError> {
        self.request_json(
            Method::POST,
            &self.api("/beyag/currencies"),
            Some(&req),
            None,
        )
        .await
    }
}
