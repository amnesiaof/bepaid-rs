use reqwest::Method;

use crate::client::BepaidClient;
use crate::error::BepaidError;
use crate::types::{
    AuthorizationEnvelope, AuthorizationRequest, AuthorizationResponse, CaptureEnvelope,
    CaptureRequest, CaptureResponse, PaymentRequest, PaymentResponse, RefundEnvelope,
    RefundRequest, RefundResponse, Transaction, TransactionEnvelope, TransactionEnvelopeFull,
    VoidEnvelope, VoidRequest, VoidResponse,
};

#[derive(serde::Serialize)]
struct RequestEnvelope<T> {
    request: T,
}

impl BepaidClient {
    /// Create a payment. Returns tracking_id + uid.
    pub async fn create_payment(
        &self,
        req: PaymentRequest,
    ) -> Result<PaymentResponse, BepaidError> {
        let envelope: TransactionEnvelope = self
            .request_json(Method::POST, &self.gateway("/transactions/payments"), Some(&RequestEnvelope { request: req }), true)
            .await?;
        Ok(envelope.transaction)
    }

    /// Create an authorization (may require 3-D Secure redirect).
    pub async fn create_authorization(
        &self,
        req: AuthorizationRequest,
    ) -> Result<AuthorizationResponse, BepaidError> {
        let envelope: AuthorizationEnvelope = self
            .request_json(Method::POST, &self.gateway("/transactions/authorizations"), Some(&RequestEnvelope { request: req }), true)
            .await?;
        Ok(envelope.transaction)
    }

    /// Capture previously authorized funds.
    pub async fn capture(&self, req: CaptureRequest) -> Result<CaptureResponse, BepaidError> {
        let envelope: CaptureEnvelope = self
            .request_json(Method::POST, &self.gateway("/transactions/captures"), Some(&RequestEnvelope { request: req }), true)
            .await?;
        Ok(envelope.transaction)
    }

    /// Void a previously authorized transaction.
    pub async fn void(&self, req: VoidRequest) -> Result<VoidResponse, BepaidError> {
        let envelope: VoidEnvelope = self
            .request_json(Method::POST, &self.gateway("/transactions/voids"), Some(&RequestEnvelope { request: req }), true)
            .await?;
        Ok(envelope.transaction)
    }

    /// Full or partial refund of a payment.
    pub async fn refund(&self, req: RefundRequest) -> Result<RefundResponse, BepaidError> {
        let envelope: RefundEnvelope = self
            .request_json(Method::POST, &self.gateway("/transactions/refunds"), Some(&RequestEnvelope { request: req }), true)
            .await?;
        Ok(envelope.transaction)
    }

    /// Get transaction status by uid.
    pub async fn get_transaction(&self, uid: &str) -> Result<Transaction, BepaidError> {
        let envelope: TransactionEnvelopeFull = self
            .request_json(Method::GET, &self.gateway(&format!("/transactions/{uid}")), None::<&u8>, true)
            .await?;
        Ok(envelope.transaction)
    }
}