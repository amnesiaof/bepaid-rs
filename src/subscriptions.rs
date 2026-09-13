//! Customers, plans and recurring subscriptions.
//!
//! See the methods on [`BepaidClient`].

use reqwest::Method;

use crate::client::BepaidClient;
use crate::error::BepaidError;
use crate::types::{
    CancelSubscriptionRequest, CustomerRecord, PlanItem, Subscription, SubscriptionCreateRequest,
};

impl BepaidClient {
    /// Create a customer in the subscription service.
    pub async fn create_customer(
        &self,
        req: CustomerRecord,
    ) -> Result<CustomerRecord, BepaidError> {
        self.request_json(Method::POST, &self.api("/customers"), Some(&req), None)
            .await
    }

    /// Get a customer by id.
    pub async fn get_customer(&self, id: &str) -> Result<CustomerRecord, BepaidError> {
        self.request_json(
            Method::GET,
            &self.api(&format!("/customers/{id}")),
            None::<&u8>,
            None,
        )
        .await
    }

    /// List all customers.
    pub async fn list_customers(&self) -> Result<Vec<CustomerRecord>, BepaidError> {
        self.request_json(Method::GET, &self.api("/customers"), None::<&u8>, None)
            .await
    }

    /// Create a subscription plan.
    pub async fn create_plan(&self, req: PlanItem) -> Result<PlanItem, BepaidError> {
        self.request_json(Method::POST, &self.api("/plans"), Some(&req), None)
            .await
    }

    /// Get a plan by id.
    pub async fn get_plan(&self, id: &str) -> Result<PlanItem, BepaidError> {
        self.request_json(
            Method::GET,
            &self.api(&format!("/plans/{id}")),
            None::<&u8>,
            None,
        )
        .await
    }

    /// List all plans.
    pub async fn list_plans(&self) -> Result<Vec<PlanItem>, BepaidError> {
        self.request_json(Method::GET, &self.api("/plans"), None::<&u8>, None)
            .await
    }

    /// Create a subscription. Customer must then be redirected to
    /// `redirect_url` to enter card details and complete the first payment.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use bepaid::BepaidClient;
    /// use bepaid::types::{
    ///     CustomerRecord, PlanInterval, PlanItem, SubscriptionCreateRequest,
    ///     SubscriptionCustomer, SubscriptionPlan,
    /// };
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), bepaid::BepaidError> {
    ///     let client = BepaidClient::new("shop_id", "secret_key");
    ///     let customer = client
    ///         .create_customer(CustomerRecord {
    ///             id: None,
    ///             first_name: Some("John".to_owned()),
    ///             last_name: Some("Smith".to_owned()),
    ///             address: None,
    ///             city: None,
    ///             country: None,
    ///             zip: None,
    ///             state: None,
    ///             phone: None,
    ///             email: Some("john@example.com".to_owned()),
    ///             ip: Some("8.8.8.8".to_owned()),
    ///             external_id: None,
    ///         })
    ///         .await?;
    ///     let plan = client
    ///         .create_plan(PlanItem {
    ///             id: None,
    ///             test: Some(true),
    ///             title: Some("Pro".to_owned()),
    ///             currency: Some("USD".to_owned()),
    ///             language: None,
    ///             plan: Some(PlanInterval {
    ///                 amount: Some(990),
    ///                 interval: Some(1),
    ///                 interval_unit: Some("month".to_owned()),
    ///                 visible_fields: None,
    ///             }),
    ///             trial: None,
    ///             infinite: None,
    ///             billing_cycles: None,
    ///             number_payment_attempts: None,
    ///             prevent_payments_at_night: None,
    ///             created_at: None,
    ///             updated_at: None,
    ///             pay_url: None,
    ///         })
    ///         .await?;
    ///     let subscription = client
    ///         .create_subscription(SubscriptionCreateRequest {
    ///             card: None,
    ///             customer: Some(SubscriptionCustomer {
    ///                 id: customer.id,
    ///                 first_name: None,
    ///                 last_name: None,
    ///                 email: None,
    ///             }),
    ///             plan: SubscriptionPlan {
    ///                 id: plan.id,
    ///                 title: None,
    ///                 currency: None,
    ///                 plan: None,
    ///                 trial: None,
    ///             },
    ///             tracking_id: Some("sub-1".to_owned()),
    ///             device_id: None,
    ///             return_url: Some("https://example.com/return".to_owned()),
    ///             notification_url: None,
    ///             dynamic_billing_descriptor: None,
    ///             additional_data: None,
    ///             settings: None,
    ///         })
    ///         .await?;
    ///     if let Some(url) = subscription.redirect_url {
    ///         println!("redirect customer to {url}");
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub async fn create_subscription(
        &self,
        req: SubscriptionCreateRequest,
    ) -> Result<Subscription, BepaidError> {
        self.request_json(Method::POST, &self.api("/subscriptions"), Some(&req), None)
            .await
    }

    /// Get subscription details by id.
    pub async fn get_subscription(&self, id: &str) -> Result<Subscription, BepaidError> {
        self.request_json(
            Method::GET,
            &self.api(&format!("/subscriptions/{id}")),
            None::<&u8>,
            None,
        )
        .await
    }

    /// Cancel a subscription and stop all further charges.
    pub async fn cancel_subscription(
        &self,
        id: &str,
        req: CancelSubscriptionRequest,
    ) -> Result<serde_json::Value, BepaidError> {
        self.request_json(
            Method::POST,
            &self.api(&format!("/subscriptions/{id}/cancel")),
            Some(&req),
            None,
        )
        .await
    }
}
