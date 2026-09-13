//! Pay-by-link products: create items, get payment links, manage stock.
//!
//! See the methods on [`BepaidClient`].

use reqwest::Method;

use crate::client::BepaidClient;
use crate::error::BepaidError;
use crate::types::{Product, ProductCreateRequest, ProductUpdateRequest};

impl BepaidClient {
    /// Create a product and obtain its payment links.
    pub async fn create_product(&self, req: ProductCreateRequest) -> Result<Product, BepaidError> {
        self.request_json(Method::POST, &self.api("/products"), Some(&req), None)
            .await
    }

    /// Get all products of the shop.
    pub async fn list_products(&self) -> Result<Vec<Product>, BepaidError> {
        self.request_json(Method::GET, &self.api("/products"), None::<&u8>, None)
            .await
    }

    /// Get a single product by id.
    pub async fn get_product(&self, id: &str) -> Result<Product, BepaidError> {
        self.request_json(
            Method::GET,
            &self.api(&format!("/products/{id}")),
            None::<&u8>,
            None,
        )
        .await
    }

    /// Update a product's price or stock (204 No Content on success).
    pub async fn update_product(
        &self,
        id: &str,
        req: ProductUpdateRequest,
    ) -> Result<(), BepaidError> {
        self.request_status(
            Method::PUT,
            &self.api(&format!("/products/{id}")),
            Some(&req),
            None,
        )
        .await
    }
}
