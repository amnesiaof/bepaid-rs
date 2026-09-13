# bepaid

Async Rust client for the [bePaid payment API](https://docs.bepaid.by) (bepaid.by).

Covers the Gateway API (card payments, tokenization, capture, void, refunds),
the hosted Checkout API, and the Direct/APM API (alternative payment methods).

## Install

```toml
[dependencies]
bepaid = "0.2"
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
    language: None,
    notification_url: None,
    billing_address: Some(BillingAddress {
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
        number: "4242424242424242".into(),
        verification_value: "123".into(),
        holder: "John Smith".into(),
        exp_month: 10,
        exp_year: 2030,
        save_card: Some(true),
        token: None,
    }),
    customer: Some(Customer {
        first_name: Some("John".into()),
        last_name: Some("Smith".into()),
        ip: None,
        email: Some("john@example.com".into()),
        device_id: None,
        birth_date: None,
    }),
    additional_data: None,
})
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
    credit_card: Some(CreditCardRaw {
        number: "4242424242424242".into(),
        verification_value: "123".into(),
        holder: "John Smith".into(),
        exp_month: 10,
        exp_year: 2030,
        save_card: Some(true),
        token: None,
    }),
    customer: None,
    billing_address: None,
})
.await?;
// redirect the customer to auth.redirect_url, then poll for the result:
let tx = client.get_transaction(&auth.uid).await?;
```

### Hosted checkout

```rust
let checkout = client.create_checkout(&CheckoutRequest {
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
    }),
    payment_method: Some(PaymentMethod {
        types: Some(vec!["credit_card".into()]),
    }),
    credit_card: None,
    order: CheckoutOrder {
        currency: "BYN".into(),
        amount: 700,
        description: Some("Order #123".into()),
    },
    customer: Some(Customer {
        first_name: Some("John".into()),
        last_name: Some("Smith".into()),
        ip: None,
        email: Some("john@example.com".into()),
        device_id: None,
        birth_date: None,
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
    customer: None,
    payment_method: serde_json::json!({"type": "erip", "service_no": "0000000001"}),
    additional_data: None,
}).await?;
```

### Webhooks

```rust
use bepaid::webhook::{parse_webhook, verify_webhook_auth};

let ok = verify_webhook_auth(auth_header, "shop_id", "secret_key");
let notification = parse_webhook(body)?;
```

## API coverage

| Group         | Operations |
|---------------|------------|
| Gateway       | `create_payment`, `create_authorization`, `capture`, `void`, `refund`, `get_transaction`, `create_p2p` |
| Tokens        | `create_token` |
| Checkout      | `create_checkout`, `get_checkout_status`, `validate_apple_pay` |
| Direct        | `create_apm_payment`, `apm_refund`, `apm_full_refund`, `confirm_apm_payment` |
| Subscriptions | `create_customer`, `get_customer`, `list_customers`, `create_plan`, `get_plan`, `list_plans`, `create_subscription`, `get_subscription`, `cancel_subscription` |
| Webhooks      | verification + payload parsing (transaction & subscription) |

## License

MIT