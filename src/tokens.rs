use reqwest::Method;

use crate::client::{BepaidClient, RequestEnvelope};
use crate::error::BepaidError;
use crate::types::{CreateTokenRequest, TokenResponse};

impl BepaidClient {
    /// Tokenize a card (PCI-DSS certified, merchant stores the token, not the PAN).
    pub async fn create_token(
        &self,
        req: CreateTokenRequest,
    ) -> Result<TokenResponse, BepaidError> {
        self.request_json(
            Method::POST,
            &self.gateway("/credit_cards"),
            Some(&RequestEnvelope { request: req }),
            Some("3"),
        )
        .await
    }
}
