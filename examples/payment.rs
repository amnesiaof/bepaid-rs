//! Card payment flow with 3-D Secure handling.
//!
//! Run with `SHOP_ID` and `SECRET_KEY` environment variables set:
//!
//! ```sh
//! SHOP_ID=363 SECRET_KEY=secret cargo run --example payment
//! ```

use std::env;

use bepaid::BepaidClient;
use bepaid::types::{AuthorizationRequest, CreditCardRaw, Customer};

#[tokio::main]
async fn main() -> Result<(), bepaid::BepaidError> {
    let shop_id = env::var("SHOP_ID").expect("SHOP_ID env var");
    let secret_key = env::var("SECRET_KEY").expect("SECRET_KEY env var");
    let client = BepaidClient::new(&shop_id, &secret_key);

    // 1. Create an authorization: the gateway returns a redirect_url when 3-D
    //    Secure is required for the card.
    let auth = client
        .create_authorization(
            AuthorizationRequest {
                amount: 104, // minor units
                currency: "EUR".to_owned(),
                description: "Test order".to_owned(),
                payment_method_type: None,
                tracking_id: format!("order-{}", time_now()),
                test: Some(true),
                duplicate_check: None,
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
                billing_address: None,
                additional_data: None,
                verification_url: None,
                custom_fields: None,
                language: None,
                notification_url: None,
                return_url: None,
                expired_at: None,
                dynamic_billing_descriptor: None,
                encrypted_credit_card: None,
                three_d_secure: None,
                travel: None,
            },
            None,
        )
        .await?;

    match auth.redirect_url {
        Some(url) => {
            // 2. Send the customer to `url`, then poll the transaction status
            //    until it leaves the `incomplete` state.
            println!("3-D Secure required: {url}");
            println!("uid: {}", auth.uid);
        }
        None => {
            // Card did not require 3-D Secure; the authorization is complete.
            let tx = client.get_transaction(&auth.uid).await?;
            println!("authorized: {:?}", tx.status);
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
