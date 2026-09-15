//! Peer-to-peer card transfers.
//!
//! See the methods on [`BepaidClient`].

use reqwest::Method;

use crate::client::{BepaidClient, RequestEnvelope};
use crate::error::BepaidError;
use crate::types::{P2pEnvelope, P2pRequest, P2pResponse, VerifyP2pResponse};

impl BepaidClient {
    /// Transfer money between two cards.
    /// Response may carry `redirect_url` when 3-D Secure verification is required.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use bepaid::BepaidClient;
    /// use bepaid::types::{P2pCard, P2pRequest};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), bepaid::BepaidError> {
    ///     let client = BepaidClient::new("shop_id", "secret_key");
    ///     let request = P2pRequest {
    ///         amount: 1000,
    ///         currency: "BYN".to_owned(),
    ///         credit_card: P2pCard {
    ///             number: Some("4242424242424242".to_owned()),
    ///             holder: None,
    ///             verification_value: None,
    ///             exp_month: None,
    ///             exp_year: None,
    ///             token: None,
    ///         },
    ///         recipient_card: P2pCard {
    ///             token: Some("token".to_owned()),
    ///             number: None,
    ///             holder: None,
    ///             verification_value: None,
    ///             exp_month: None,
    ///             exp_year: None,
    ///         },
    ///         test: Some(true),
    ///         tracking_id: Some("order-125".to_owned()),
    ///         additional_data: None,
    ///     };
    ///     let p2p = client.create_p2p(request).await?;
    ///     println!("uid: {}", p2p.uid.unwrap_or_default());
    ///     Ok(())
    /// }
    /// ```
    pub async fn create_p2p(&self, req: P2pRequest) -> Result<P2pResponse, BepaidError> {
        let envelope: P2pEnvelope = self
            .request_json(
                Method::POST,
                &self.gateway("/transactions/p2ps"),
                Some(&RequestEnvelope { request: req }),
                Some("3"),
            )
            .await?;
        Ok(envelope.transaction)
    }

    /// Check whether a P2P transfer is possible and get commission details.
    /// Uses the `POST /p2p-restrictions` endpoint (flat response, no envelope).
    pub async fn verify_p2p(&self, req: P2pRequest) -> Result<VerifyP2pResponse, BepaidError> {
        self.request_json(
            Method::POST,
            &self.gateway("/p2p-restrictions"),
            Some(&RequestEnvelope { request: req }),
            None,
        )
        .await
    }
}
