//! Alternative payment methods (APM) and their confirmations.
//!
//! See the methods on [`BepaidClient`].

use reqwest::Method;

use crate::client::{BepaidClient, RequestEnvelope};
use crate::error::BepaidError;
use crate::types::{
    ApmConfirmEnvelope, ApmConfirmRequest, ApmConfirmResponse, ApmPaymentEnvelope,
    ApmPaymentRequest, ApmPaymentResponse, ApmPayoutEnvelope, ApmPayoutRequest, ApmPayoutResponse,
    ApmRefundEnvelope, ApmRefundRequest, ApmRefundResponse, BalanceRequest, BalanceResponse,
    CheckServiceRequest, CheckServiceResponse, CheckupRequest, CurrencyInfo, CurrencyQueryRequest,
    ProofEnvelope, ProofRequest, ProofResponse, SplitPaymentRequest, SplitPaymentResponse,
    Transaction, TransactionEnvelopeFull, TransactionListEnvelope,
};

impl BepaidClient {
    #[allow(missing_docs)]
    pub async fn create_erip_payment(
        &self,
        req: ApmPaymentRequest,
    ) -> Result<ApmPaymentResponse, BepaidError> {
        let envelope: ApmPaymentEnvelope = self
            .request_json(
                Method::POST,
                &self.api("/beyag/payments"),
                Some(&RequestEnvelope { request: req }),
                None,
            )
            .await?;
        Ok(envelope.transaction)
    }

    #[allow(missing_docs)]
    pub async fn get_apm_refund(&self, uid: &str) -> Result<ApmRefundResponse, BepaidError> {
        let envelope: ApmRefundEnvelope = self
            .request_json(
                Method::GET,
                &self.api(&format!("/beyag/refunds/{uid}")),
                None::<&u8>,
                None,
            )
            .await?;
        Ok(envelope.transaction)
    }

    #[allow(missing_docs)]
    pub async fn get_erip_pay_list(
        &self,
        req: crate::types::EripPayListRequest,
    ) -> Result<serde_json::Value, BepaidError> {
        self.request_json(
            Method::POST,
            &self.api("/beyag/gateways/komplat/get_pay_list"),
            Some(&req),
            None,
        )
        .await
    }

    /// Pay by an alternative payment method (ERIP, Alfabank, MTS Money, ...).
    /// `payment_method` must carry the method-specific parameters (`{"type": "erip", ...}`).
    pub async fn create_apm_payment(
        &self,
        req: ApmPaymentRequest,
        request_id: Option<&str>,
    ) -> Result<ApmPaymentResponse, BepaidError> {
        let mut request = serde_json::to_value(req)?;
        request["method"] = request
            .as_object_mut()
            .unwrap()
            .remove("payment_method")
            .unwrap();
        let envelope: ApmPaymentEnvelope = self
            .request_json_with_id(
                Method::POST,
                &self.api("/beyag/transactions/payments"),
                Some(&RequestEnvelope { request }),
                None,
                request_id,
            )
            .await?;
        Ok(envelope.transaction)
    }

    /// Refund an APM transaction via `/beyag/transactions/refunds`.
    pub async fn apm_refund(
        &self,
        req: ApmRefundRequest,
        request_id: Option<&str>,
    ) -> Result<ApmRefundResponse, BepaidError> {
        let envelope: ApmRefundEnvelope = self
            .request_json_with_id(
                Method::POST,
                &self.api("/beyag/transactions/refunds"),
                Some(&RequestEnvelope { request: req }),
                None,
                request_id,
            )
            .await?;
        Ok(envelope.transaction)
    }

    /// Full or partial refund of an APM transaction via the alternative
    /// `/beyag/refunds` endpoint. `amount` is required, including for a full refund.
    pub async fn apm_full_refund(
        &self,
        parent_uid: &str,
        reason: &str,
        amount: Option<i64>,
        request_id: Option<&str>,
    ) -> Result<ApmRefundResponse, BepaidError> {
        let amount = amount.ok_or_else(|| {
            BepaidError::InvalidRequest("amount is required for /beyag/refunds".into())
        })?;
        let req = ApmRefundRequest {
            parent_uid: parent_uid.to_owned(),
            reason: reason.to_owned(),
            amount: Some(amount),
            tracking_id: None,
            additional_data: None,
        };
        let envelope: ApmRefundEnvelope = self
            .request_json_with_id(
                Method::POST,
                &self.api("/beyag/refunds"),
                Some(&RequestEnvelope { request: req }),
                None,
                request_id,
            )
            .await?;
        Ok(envelope.transaction)
    }

    /// Confirm a payment from a third-party application.
    pub async fn confirm_apm_payment(
        &self,
        uid: &str,
        req: ApmConfirmRequest,
        request_id: Option<&str>,
    ) -> Result<ApmConfirmResponse, BepaidError> {
        if [
            req.transaction_reference.is_some() || req.skip_duplicate_check.is_some(),
            req.confirm_type.is_some(),
            req.phone.is_some(),
        ]
        .into_iter()
        .filter(|present| *present)
        .count()
            > 1
        {
            return Err(BepaidError::InvalidRequest(
                "do not mix confirmation modes: transaction_reference/skip_duplicate_check, confirm_type or phone".into(),
            ));
        }
        if let Some(confirm_type) = req.confirm_type.as_deref()
            && !matches!(confirm_type, "confirm" | "cancel")
        {
            return Err(BepaidError::InvalidRequest(
                "confirm_type must be confirm or cancel".into(),
            ));
        }
        let url = self.api(&format!("/beyag/transactions/{uid}/confirm"));
        if req.confirm_type.is_some() {
            #[derive(serde::Deserialize)]
            struct Envelope {
                transaction: ApmConfirmResponse,
            }
            let envelope: Envelope = self
                .request_json_with_id(
                    Method::POST,
                    &url,
                    Some(&RequestEnvelope { request: req }),
                    None,
                    request_id,
                )
                .await?;
            Ok(envelope.transaction)
        } else if req.phone.is_some() {
            let envelope: ApmConfirmEnvelope = self
                .request_json_with_id(Method::POST, &url, Some(&req), None, request_id)
                .await?;
            Ok(envelope.response)
        } else {
            let envelope: ApmConfirmEnvelope = self
                .request_json_with_id(
                    Method::POST,
                    &url,
                    Some(&RequestEnvelope { request: req }),
                    None,
                    request_id,
                )
                .await?;
            Ok(envelope.response)
        }
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
        request_id: Option<&str>,
    ) -> Result<ApmPayoutResponse, BepaidError> {
        let envelope: ApmPayoutEnvelope = self
            .request_json_with_id(
                Method::POST,
                &self.api("/beyag/transactions/payouts"),
                Some(&RequestEnvelope { request: req }),
                None,
                request_id,
            )
            .await?;
        Ok(envelope.transaction)
    }

    /// Submit proof of payment for an APM transaction.
    pub async fn apm_proof(
        &self,
        uid: &str,
        req: ProofRequest,
        request_id: Option<&str>,
    ) -> Result<ProofResponse, BepaidError> {
        let envelope: ProofEnvelope = self
            .request_json_with_id(
                Method::POST,
                &self.api(&format!("/beyag/transactions/{uid}/proof")),
                Some(&RequestEnvelope { request: req }),
                None,
                request_id,
            )
            .await?;
        Ok(envelope.transaction)
    }

    /// Run a pre-authorization risk check on a card.
    pub async fn checkup(
        &self,
        req: CheckupRequest,
        request_id: Option<&str>,
    ) -> Result<Transaction, BepaidError> {
        let envelope: TransactionEnvelopeFull = self
            .request_json_with_id(
                Method::POST,
                &self.gateway("/transactions/checkups"),
                Some(&RequestEnvelope { request: req }),
                Some("3"),
                request_id,
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

    /// Check if a customer is an MTS Money participant (`API v3`).
    pub async fn check_mts_service(
        &self,
        req: CheckServiceRequest,
    ) -> Result<CheckServiceResponse, BepaidError> {
        self.request_json(
            Method::POST,
            &self.api("/beyag/gateways/mts_money_widget/check_service"),
            Some(&RequestEnvelope { request: req }),
            Some("3"),
        )
        .await
    }

    #[allow(missing_docs)]
    pub async fn check_mts_service_v2(
        &self,
        phone: &str,
        test: Option<bool>,
    ) -> Result<CheckServiceResponse, BepaidError> {
        let mut request = serde_json::json!({"customer": {"phone": phone}});
        if let Some(test) = test {
            request["test"] = serde_json::json!(test);
        }
        self.request_json(
            Method::POST,
            &self.api("/beyag/gateways/mts_money/check_service"),
            Some(&RequestEnvelope { request }),
            Some("2"),
        )
        .await
    }

    #[allow(missing_docs)]
    pub async fn test_qiwi_terminal_payment(
        &self,
        amount: i64,
        currency: &str,
        account: &str,
    ) -> Result<serde_json::Value, BepaidError> {
        let response = self
            .send_request(
                Method::POST,
                &self.api("/beyag/testing/payment"),
                Some(&serde_json::json!({"request": {
                    "amount": amount,
                    "currency": currency,
                    "method": {"type": "qiwi_terminal", "account": account},
                    "test": true
                }})),
                None,
                None,
            )
            .await?;
        let body = response.bytes().await?;
        if body.is_empty() {
            Ok(serde_json::json!({}))
        } else {
            Ok(serde_json::from_slice(&body)?)
        }
    }

    /// Get ERIP payment details by the payment uid (`GET /beyag/payments/:uid`).
    pub async fn get_erip_payment(&self, uid: &str) -> Result<Transaction, BepaidError> {
        let envelope: TransactionEnvelopeFull = self
            .request_json(
                Method::GET,
                &self.api(&format!("/beyag/payments/{uid}")),
                None::<&u8>,
                None,
            )
            .await?;
        Ok(envelope.transaction)
    }

    /// Get ERIP payment details by the merchant's order id
    /// (`GET /beyag/payments/?order_id=...`).
    pub async fn get_erip_payment_by_order_id(
        &self,
        order_id: &str,
    ) -> Result<Transaction, BepaidError> {
        let envelope: TransactionEnvelopeFull = self
            .request_json(
                Method::GET,
                &self.api(&format!("/beyag/payments/?order_id={order_id}")),
                None::<&u8>,
                None,
            )
            .await?;
        Ok(envelope.transaction)
    }

    /// Delete an ERIP payment requirement. Only `pending` or `permanent`
    /// requirements can be deleted; the payment moves to `deleted` status
    /// (`DELETE /beyag/payments/:uid`).
    pub async fn delete_erip_payment(&self, uid: &str) -> Result<Transaction, BepaidError> {
        let envelope: TransactionEnvelopeFull = self
            .request_json(
                Method::DELETE,
                &self.api(&format!("/beyag/payments/{uid}")),
                None::<&u8>,
                None,
            )
            .await?;
        Ok(envelope.transaction)
    }
}
