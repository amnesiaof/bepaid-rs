use reqwest::Method;

use crate::client::BepaidClient;
use crate::error::BepaidError;
use crate::types::{P2pEnvelope, P2pRequest, P2pResponse};

#[derive(serde::Serialize)]
struct RequestEnvelope<T> {
    request: T,
}

impl BepaidClient {
    /// Transfer money between two cards.
    /// Response may carry `redirect_url` when 3-D Secure verification is required.
    pub async fn create_p2p(&self, req: P2pRequest) -> Result<P2pResponse, BepaidError> {
        let envelope: P2pEnvelope = self
            .request_json(
                Method::POST,
                &self.gateway("/transactions/p2ps"),
                Some(&RequestEnvelope { request: req }),
                true,
            )
            .await?;
        Ok(envelope.transaction)
    }
}
