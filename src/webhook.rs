use base64::{Engine, engine::general_purpose::STANDARD};
use rsa::RsaPublicKey;
use rsa::pkcs1v15::{Signature as RsaSignature, VerifyingKey};
use rsa::pkcs8::DecodePublicKey;
use rsa::signature::hazmat::PrehashVerifier;
use sha2::{Digest, Sha256};

use crate::error::BepaidError;
use crate::types::{CardNotification, CheckoutStatus, Subscription, WebhookNotification};

/// Verify the Basic-auth credentials of a bePaid webhook sender.
/// Always do this before trusting a notification payload.
///
/// # Example
///
/// ```
/// use bepaid::webhook::verify_webhook_auth;
///
/// let ok = verify_webhook_auth(
///     "Basic MzYzOjQ1NDU0ZTA4MzQzNGFhMzdyZmRmZA==",
///     "363",
///     "45454e083434aa37rfdfd",
/// );
/// assert!(ok);
/// ```
pub fn verify_webhook_auth(authorization_header: &str, shop_id: &str, secret_key: &str) -> bool {
    let expected = format!(
        "Basic {}",
        STANDARD.encode(format!("{shop_id}:{secret_key}"))
    );
    authorization_header == expected
}

/// Verify the RSA-SHA256 `Content-Signature` of a bePaid webhook
/// notification. `public_key_pem` is the shop's public key from the bePaid
/// dashboard, `signature` is the base64 `Content-Signature` header value, and
/// `raw_body` is the raw UTF-8 bytes of the notification body (unmodified).
///
/// Returns `Ok(true)` when the signature is genuine, `Ok(false)` when it does
/// not match, and `Err` when the key or signature are malformed.
///
/// # Example
///
/// ```no_run
/// use bepaid::webhook::verify_webhook_signature;
///
/// // Read the raw request body once, keep the bytes for verification.
/// let raw_body = br#"{"transaction":{"uid":"123"}}"#;
/// let signature = "base64 signature from the Content-Signature header";
/// let public_key = r#"-----BEGIN PUBLIC KEY-----
/// ...-----END PUBLIC KEY-----"#;
/// if verify_webhook_signature(public_key, signature, raw_body).unwrap_or(false) {
///     // process the notification
/// }
/// ```
pub fn verify_webhook_signature(
    public_key_pem: &str,
    signature: &str,
    raw_body: &[u8],
) -> Result<bool, BepaidError> {
    let public_key =
        RsaPublicKey::from_public_key_pem(public_key_pem).map_err(BepaidError::RsaKey)?;
    let signature = STANDARD.decode(signature).map_err(BepaidError::Base64)?;
    let signature =
        RsaSignature::try_from(signature.as_slice()).map_err(BepaidError::RsaSignature)?;
    let hash = Sha256::digest(raw_body);
    Ok(VerifyingKey::<Sha256>::new(public_key)
        .verify_prehash(&hash, &signature)
        .is_ok())
}

/// Parse a webhook payload into a typed notification.
pub fn parse_webhook(body: &str) -> Result<WebhookNotification, serde_json::Error> {
    serde_json::from_str(body)
}

/// Parse a subscription-service webhook payload (`event`, e.g.
/// `created.subscription`).
pub fn parse_subscription_webhook(body: &str) -> Result<Subscription, serde_json::Error> {
    serde_json::from_str(body)
}

/// Parse a payment-widget webhook payload (flat checkout-shaped
/// notification, e.g. a token-expiry notice without a `transaction`
/// envelope).
pub fn parse_checkout_webhook(body: &str) -> Result<CheckoutStatus, serde_json::Error> {
    serde_json::from_str(body)
}

/// Parse a card-notification webhook payload (flat card object, sent when
/// `credit_card.notification_url` or `settings.card_notification_url` is set).
pub fn parse_card_notification(body: &str) -> Result<CardNotification, serde_json::Error> {
    serde_json::from_str(body)
}
