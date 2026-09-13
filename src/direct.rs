use reqwest::Method;

use crate::client::BepaidClient;
use crate::error::BepaidError;
use crate::types::{
    ApmConfirmEnvelope, ApmConfirmRequest, ApmConfirmResponse, ApmPaymentEnvelope,
    ApmPaymentRequest, ApmPaymentResponse, ApmRefundEnvelope, ApmRefundRequest, ApmRefundResponse,
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
                false,
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
                false,
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
                false,
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
                false,
            )
            .await?;
        Ok(envelope.response)
    }
}
