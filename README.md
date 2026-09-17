# bepaid

Async Rust client for the [bePaid payment API](https://docs.bepaid.by) (bepaid.by).

Covers the Gateway API (card payments, tokenization, saved-card charges,
capture, void, refunds, payouts, recipient tokenization), the hosted Checkout
API (checkout, payment token, Apple Pay), pay-by-link, the Direct/APM API
(alternative payment methods, balance and currency queries, split payments),
and the Merchant API (reports, channel balances).

## Install

```toml
[dependencies]
bepaid = "0.8"
```

Also needs a Tokio runtime (the client is async):

```toml
[dependencies]
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
```

## Usage

```rust
use bepaid::{
    BepaidClient,
    types::{BillingAddress, CreditCardRaw, Customer, PaymentRequest},
};

let client = BepaidClient::new("shop_id", "secret_key");

// Card payment (amounts are strings in minor units, e.g. "700" = 7.00 BYN)
let payment = client.create_payment(PaymentRequest {
    amount: "700".into(),
    currency: "BYN".into(),
    test: true,
    description: "Order #123".into(),
    tracking_id: "order-123".into(),
    expired_at: None,
    dynamic_billing_descriptor: None,
    language: None,
    notification_url: None,
    verification_url: None,
    return_url: None,
    duplicate_check: None,
    billing_address: Some(BillingAddress {
        middle_name: None,
        first_name: Some("John".into()),
        last_name: Some("Smith".into()),
        country: Some("BY".into()),
        city: Some("Minsk".into()),
        state: None,
        zip: Some("220000".into()),
        address: Some("Nezavisimosti 1".into()),
        phone: Some("+375291234567".into()),
    }),
    credit_card: Some(CreditCardRaw {
        number: Some("4242424242424242".into()),
        verification_value: Some("123".into()),
        holder: Some("John Smith".into()),
        exp_month: Some(10),
        exp_year: Some(2030),
        save_card: Some(true),
        token: None,
        skip_three_d_secure_verification: None,
        force_three_d_secure_verification: None,
    }),
    customer: Some(Customer {
        id: None,
        id_number: None,
        gender: None,
        street: None,
        state: None,
        middle_name: None,
        country: None,
        city: None,
        zip: None,
        address: None,
        first_name: Some("John".into()),
        last_name: Some("Smith".into()),
        ip: None,
        email: Some("john@example.com".into()),
        device_id: None,
        birth_date: None,
        phone: None,
        external_id: None,
        taxpayer_id: None,
    }),
    additional_data: None,
    encrypted_data: None,
    fiscalization: None,
    custom_fields: None,
}, None)
.await?;
```

### Authorization with 3-D Secure

```rust
let auth = client.create_authorization(AuthorizationRequest {
    amount: 700,
    currency: "BYN".into(),
    description: "Order #123".into(),
    payment_method_type: Some("credit_card".into()),
    tracking_id: "order-123".into(),
    test: Some(true),
    duplicate_check: None,
    language: None,
    notification_url: None,
    return_url: None,
    expired_at: None,
    dynamic_billing_descriptor: None,
    credit_card: Some(CreditCardRaw {
        number: Some("4242424242424242".into()),
        verification_value: Some("123".into()),
        holder: Some("John Smith".into()),
        exp_month: Some(10),
        exp_year: Some(2030),
        save_card: Some(true),
        token: None,
        skip_three_d_secure_verification: None,
        force_three_d_secure_verification: None,
    }),
    customer: None,
    billing_address: None,
    additional_data: None,
    verification_url: None,
    custom_fields: None,
}, None)
.await?;
// redirect the customer to auth.redirect_url, then poll for the result:
let tx = client.get_transaction(&auth.uid).await?;
```

### Hosted checkout

```rust
let checkout = client.create_checkout(&CheckoutRequest {
    dynamic_billing_descriptor: None,
    travel: None,
    test: Some(true),
    transaction_type: "payment".into(),
    attempts: None,
    iframe: None,
    settings: Some(CheckoutSettings {
        return_url: Some("https://example.com/return".into()),
        success_url: None,
        decline_url: None,
        fail_url: None,
        cancel_url: None,
        notification_url: Some("https://example.com/webhook".into()),
        button_next_text: None,
        auto_pay: None,
        language: None,
        style: None,
        widget_version: None,
        require: None,
        customer: None,
        agreed: None,
        agreement_toggle: None,
        customer_fields: None,
        credit_card_fields: None,
        verification_url: None,
        auto_return: None,
        card_notification_url: None,
        save_card_toggle: None,
        another_card_toggle: None,
    }),
    payment_method: Some(PaymentMethod {
        extra: None,
        types: Some(vec!["credit_card".into()]),
        excluded_types: None,
        excluded_brands: None,
    }),
    credit_card: None,
    order: CheckoutOrder {
        currency: "BYN".into(),
        amount: 700,
        description: Some("Order #123".into()),
        tracking_id: None,
        expired_at: None,
        additional_data: None,
        custom_fields: None,
    },
    customer: Some(Customer {
        id: None,
        id_number: None,
        gender: None,
        street: None,
        state: None,
        middle_name: None,
        country: None,
        city: None,
        zip: None,
        address: None,
        first_name: Some("John".into()),
        last_name: Some("Smith".into()),
        ip: None,
        email: Some("john@example.com".into()),
        device_id: None,
        birth_date: None,
        phone: None,
        external_id: None,
        taxpayer_id: None,
    }),
})
.await?;
// redirect the customer to checkout.redirect_url
```

### Tokenization

```rust
let token = client.create_token(CreateTokenRequest {
    number: "4200000000000000".into(),
    holder: "John Smith".into(),
    exp_month: "05".into(),
    exp_year: "2028".into(),
    contract: Some(vec!["recurring".into()]),
}).await?;
// store token.token, use it later without re-entering card details
```

### APM payments (Direct API)

```rust
let apm = client.create_apm_payment(ApmPaymentRequest {
    amount: 700,
    currency: "BYN".into(),
    description: None,
    email: Some("john@example.com".into()),
    ip: None,
    success_url: None,
    order_id: None,
    tracking_id: None,
    notification_url: None,
    expired_at: None,
    test: Some(true),
    language: None,
    return_url: None,
    iframe: None,
    verification_url: None,
    customer: None,
    payment_method: serde_json::json!({"type": "erip", "service_no": "0000000001"}),
    additional_data: None,
    custom_fields: None,
}, None).await?;
```

### Webhooks

```rust
use bepaid::webhook::{parse_webhook, verify_webhook_auth};

let ok = verify_webhook_auth(auth_header, "shop_id", "secret_key");
let notification = parse_webhook(body)?;
```

## Examples

Ready-to-run programs under [`examples/`](examples/):

| Example | Run |
|---------|-----|
| [`payment`](examples/payment.rs) | `SHOP_ID=363 SECRET_KEY=secret cargo run --example payment` |
| [`subscriptions`](examples/subscriptions.rs) | `SHOP_ID=363 SECRET_KEY=secret cargo run --example subscriptions` |

## API coverage

| Group         | Operations |
|---------------|------------|
| Gateway       | `create_payment`, `create_authorization`, `capture`, `void`, `refund`, `get_transaction`, `get_transaction_by_tracking_id`, `create_p2p`, `create_payout`, `charge_saved_card`, `tokenize_recipient_card` |
| Tokens        | `create_token` |
| Checkout      | `create_checkout`, `get_checkout_status`, `validate_apple_pay`, `create_payment_token`, `apple_pay_payment` |
| Direct        | `create_apm_payment`, `apm_refund`, `apm_full_refund`, `confirm_apm_payment`, `get_balance`, `get_currencies`, `create_split_payment` |
| Pay-by-link   | `create_product`, `list_products`, `get_product`, `update_product` |
| Subscriptions | `create_customer`, `get_customer`, `list_customers`, `create_plan`, `get_plan`, `list_plans`, `create_subscription`, `get_subscription`, `cancel_subscription`, `get_plan_payment_link` |
| Merchant      | `get_reports`, `get_report_count`, `get_channel_balances` |
| Webhooks      | verification (Basic auth + RSA `Content-Signature`) + payload parsing (transaction & subscription) |

## License

MIT