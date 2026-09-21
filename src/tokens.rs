use reqwest::Method;

use crate::client::{BepaidClient, RequestEnvelope};
use crate::error::BepaidError;
use crate::types::{CreateTokenRequest, TokenResponse, UpdateTokenRequest};

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

    /// Update a tokenized card's data (cardholder and/or expiration).
    pub async fn update_token(
        &self,
        token: &str,
        req: UpdateTokenRequest,
    ) -> Result<TokenResponse, BepaidError> {
        self.request_json(
            Method::POST,
            &self.gateway(&format!("/credit_cards/{token}")),
            Some(&RequestEnvelope { request: req }),
            Some("3"),
        )
        .await
    }
}
