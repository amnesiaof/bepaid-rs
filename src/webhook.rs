use base64::{Engine, engine::general_purpose::STANDARD};

use crate::types::{Subscription, WebhookNotification};

/// Verify the Basic-auth credentials of a bePaid webhook sender.
/// Always do this before trusting a notification payload.
pub fn verify_webhook_auth(authorization_header: &str, shop_id: &str, secret_key: &str) -> bool {
    let expected = format!(
        "Basic {}",
        STANDARD.encode(format!("{shop_id}:{secret_key}"))
    );
    authorization_header == expected
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
