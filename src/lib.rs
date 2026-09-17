//! Async client for the [bePaid payment API](https://docs.bepaid.by/ru/) (bepaid.by).
//!
//! Provides typed operations for the three bePaid APIs:
//! - **Gateway** ([`gateway`]) — card payments, authorizations with 3DS, captures, voids,
//!   refunds and P2P transfers against `https://gateway.bepaid.by`.
//! - **Checkout** ([`checkout`]) — hosted payment pages and Apple Pay validation against
//!   `https://checkout.bepaid.by`.
//! - **Direct** ([`direct`]) — alternative payment methods (APM), their confirmations
//!   and balance queries against `https://api.bepaid.by`.
//! - **Subscriptions** ([`subscriptions`]) — customers, plans and recurring subscriptions.
//! - **Products** ([`products`]) — pay-by-link products and payment link management.
//! - **Tokens** ([`tokens`]) — PCI-DSS-certified card tokenization.
//! - **Merchant** ([`merchant`]) — reports and payout control against
//!   `https://merchant.bepaid.by`.
//!
//! A single [`BepaidClient`] holds your HTTP Basic credentials and exposes every operation.
//!
//! # Example
//!
//! ```no_run
//! use bepaid::BepaidClient;
//! use bepaid::types::PaymentRequest;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), bepaid::BepaidError> {
//!     let client = BepaidClient::new("shop_id", "secret_key");
//!     let request = PaymentRequest {
//!         amount: "700".to_owned(), // minor units
//!         currency: "BYN".to_owned(),
//!         test: true,
//!         description: "Test payment".to_owned(),
//!         tracking_id: "order-123".to_owned(),
//!         language: None,
//!         notification_url: None,
//!         verification_url: None,
//!         return_url: None,
//!         duplicate_check: None,
//!         billing_address: None,
//!         credit_card: None,
//!         customer: None,
//!         additional_data: None,
//!         encrypted_data: None,
//!         fiscalization: None,
//!         custom_fields: None,
//!     };
//!     let payment = client.create_payment(request, None).await?;
//!     println!("uid: {}", payment.uid);
//!     Ok(())
//! }
//! ```
#![warn(missing_docs)]

/// Hosted payment page and Apple Pay validation.
pub mod checkout;
/// HTTP client and the three base URLs.
pub mod client;
/// Alternative payment methods (APM) and their confirmations.
pub mod direct;
/// Error types returned by the whole crate.
pub mod error;
/// Card payments, authorizations, captures, voids, refunds.
pub mod gateway;
/// Merchant reports and payout control.
pub mod merchant;
/// Peer-to-peer card transfers.
pub mod p2p;
/// Pay-by-link products and payment links.
pub mod products;
/// Customers, plans and recurring subscriptions.
pub mod subscriptions;
/// Card tokenization.
pub mod tokens;
/// Shared request and response types.
pub mod types;
/// Webhook payload parsing and notification verification.
pub mod webhook;

pub use client::{
    BepaidClient, DEFAULT_API_URL, DEFAULT_CHECKOUT_URL, DEFAULT_GATEWAY_URL, DEFAULT_MERCHANT_URL,
};
pub use error::{ApiError, BepaidError};
