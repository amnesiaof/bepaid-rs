//! bepaid — async client for the bePaid payment API (bepaid.by).

pub mod checkout;
pub mod client;
pub mod direct;
pub mod error;
pub mod gateway;
pub mod p2p;
pub mod subscriptions;
pub mod tokens;
pub mod types;
pub mod webhook;

pub use client::{BepaidClient, DEFAULT_API_URL, DEFAULT_CHECKOUT_URL, DEFAULT_GATEWAY_URL};
pub use error::{ApiError, BepaidError};
