//! Server-to-server (H2H) card payment.
//!
//! Unlike the customer-facing flows, H2H charges the card straight from your
//! back end. When 3-D Secure is mandatory the gateway returns a
//! `three_d_secure_verification` block in the transaction and the customer
//! must still verify.
//!
//! Run with `SHOP_ID` and `SECRET_KEY` environment variables set:
//!
//! ```sh
//! SHOP_ID=363 SECRET_KEY=secret cargo run --example h2h
//! ```

use std::env;

use bepaid::BepaidClient;
use bepaid::types::{CreditCardRaw, Customer, PaymentRequest};

#[tokio::main]
async fn main() -> Result<(), bepaid::BepaidError> {
    let shop_id = env::var("SHOP_ID").expect("SHOP_ID env var");
    let secret_key = env::var("SECRET_KEY").expect("SECRET_KEY env var");
    let client = BepaidClient::new(&shop_id, &secret_key);

    // 1. Charge the card server-to-server. `return_url` is where the customer
    //    lands after 3-D Secure; `verification_url` is where the 3-D Secure
    //    request posts the result.
    let payment = client
        .create_payment(
            PaymentRequest {
                amount: "700".to_owned(), // minor units
                currency: "USD".to_owned(),
                test: true,
                description: "H2H test payment".to_owned(),
                tracking_id: format!("h2h-{}", time_now()),
                language: None,
                notification_url: None,
                verification_url: Some("https://example.com/3ds-verify".to_owned()),
                return_url: Some("https://example.com/payment-return".to_owned()),
                duplicate_check: None,
                billing_address: None,
                credit_card: Some(CreditCardRaw {
                    number: Some("4242424242424242".to_owned()),
                    verification_value: Some("123".to_owned()),
                    holder: Some("John Smith".to_owned()),
                    exp_month: Some(10),
                    exp_year: Some(2030),
                    save_card: Some(true),
                    token: None,
                    skip_three_d_secure_verification: None,
                    force_three_d_secure_verification: None,
                    notification_url: None,
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
                    first_name: None,
                    last_name: None,
                    ip: Some("127.0.0.1".to_owned()),
                    email: Some("john@example.com".to_owned()),
                    device_id: None,
                    birth_date: None,
                    phone: None,
                    external_id: None,
                    taxpayer_id: None,
                }),
                additional_data: Some(bepaid::types::AdditionalData {
                    browser: None,
                    contract: Some(vec!["recurring".to_owned()]),
                    referer: None,
                    masterpass: None,
                    split: None,
                    smart_routing_options: None,
                    excluded_gateways: None,
                    extra: None,
                }),
                encrypted_data: None,
                fiscalization: None,
                custom_fields: None,
                expired_at: None,
                dynamic_billing_descriptor: None,
                encrypted_credit_card: None,
                three_d_secure: None,
                travel: None,
            },
            None,
        )
        .await?;

    println!("payment uid: {}", payment.uid);

    // 2. Poll the transaction until it leaves `incomplete`. When 3-D Secure
    //    is triggered, handle `three_d_secure_verification` and follow the
    //    redirect before retrying.
    loop {
        let tx = client.get_transaction(&payment.uid).await?;
        println!("status: {:?}", tx.status);
        match tx.status.as_deref() {
            Some("successful") | Some("failed") => break,
            _ => std::thread::sleep(std::time::Duration::from_secs(2)),
        }
    }

    Ok(())
}

fn time_now() -> u128 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis()
}
