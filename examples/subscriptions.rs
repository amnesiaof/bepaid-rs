//! Recurring subscription flow: customer → plan → subscription.
//!
//! Run with `SHOP_ID` and `SECRET_KEY` environment variables set:
//!
//! ```sh
//! SHOP_ID=363 SECRET_KEY=secret cargo run --example subscriptions
//! ```

use std::env;

use bepaid::BepaidClient;
use bepaid::types::{
    CustomerRecord, PlanInterval, PlanItem, Subscription, SubscriptionCreateRequest,
    SubscriptionCustomer, SubscriptionPlan,
};

#[tokio::main]
async fn main() -> Result<(), bepaid::BepaidError> {
    let shop_id = env::var("SHOP_ID").expect("SHOP_ID env var");
    let secret_key = env::var("SECRET_KEY").expect("SECRET_KEY env var");
    let client = BepaidClient::new(&shop_id, &secret_key);

    // 1. Create a customer (or pass an existing `id` to the subscription).
    let customer = client
        .create_customer(CustomerRecord {
            id: None,
            first_name: Some("John".to_owned()),
            last_name: Some("Smith".to_owned()),
            address: None,
            city: None,
            country: None,
            zip: None,
            state: None,
            phone: None,
            email: Some("john@example.com".to_owned()),
            ip: Some("127.0.0.1".to_owned()),
            external_id: None,
        })
        .await?;
    println!("customer: {}", customer.id.as_deref().unwrap_or("?"));

    // 2. Create a plan (or pass an existing `id` to the subscription).
    let plan = client
        .create_plan(PlanItem {
            id: None,
            test: Some(true),
            title: Some("Premium".to_owned()),
            currency: Some("EUR".to_owned()),
            language: None,
            plan: Some(PlanInterval {
                amount: Some(990), // minor units
                interval: Some(1),
                interval_unit: Some("month".to_owned()),
                visible_fields: None,
            }),
            trial: None,
            infinite: Some(true),
            billing_cycles: None,
            number_payment_attempts: None,
            prevent_payments_at_night: None,
            created_at: None,
            updated_at: None,
            pay_url: None,
        })
        .await?;
    println!("plan: {}", plan.title.as_deref().unwrap_or("?"));

    // 3. Create the subscription. For the hosted flow leave `card`/`customer`
    //    unset and a `redirect_url` on the response is returned.
    let subscription: Subscription = client
        .create_subscription(SubscriptionCreateRequest {
            card: None,
            customer: Some(SubscriptionCustomer {
                id: customer.id,
                first_name: None,
                last_name: None,
                email: None,
            }),
            plan: SubscriptionPlan {
                id: plan.id,
                title: None,
                currency: None,
                plan: None,
                trial: None,
            },
            tracking_id: Some("sub-1".to_owned()),
            device_id: None,
            return_url: Some("https://example.com/return".to_owned()),
            notification_url: Some("https://example.com/webhook".to_owned()),
            dynamic_billing_descriptor: None,
            additional_data: None,
            settings: None,
        })
        .await?;

    if let Some(url) = subscription.redirect_url {
        println!("3-D Secure required, redirect customer: {url}");
    }
    println!(
        "subscription: {:?} ({:?})",
        subscription.id, subscription.state
    );

    Ok(())
}
