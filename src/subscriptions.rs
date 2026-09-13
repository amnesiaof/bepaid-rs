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
        self.request_json(Method::POST, &self.api("/customers"), Some(&req), false)
            .await
    }

    /// Get a customer by id.
    pub async fn get_customer(&self, id: &str) -> Result<CustomerRecord, BepaidError> {
        self.request_json(
            Method::GET,
            &self.api(&format!("/customers/{id}")),
            None::<&u8>,
            false,
        )
        .await
    }

    /// List all customers.
    pub async fn list_customers(&self) -> Result<Vec<CustomerRecord>, BepaidError> {
        self.request_json(Method::GET, &self.api("/customers"), None::<&u8>, false)
            .await
    }

    /// Create a subscription plan.
    pub async fn create_plan(&self, req: PlanItem) -> Result<PlanItem, BepaidError> {
        self.request_json(Method::POST, &self.api("/plans"), Some(&req), false)
            .await
    }

    /// Get a plan by id.
    pub async fn get_plan(&self, id: &str) -> Result<PlanItem, BepaidError> {
        self.request_json(
            Method::GET,
            &self.api(&format!("/plans/{id}")),
            None::<&u8>,
            false,
        )
        .await
    }

    /// List all plans.
    pub async fn list_plans(&self) -> Result<Vec<PlanItem>, BepaidError> {
        self.request_json(Method::GET, &self.api("/plans"), None::<&u8>, false)
            .await
    }

    /// Create a subscription. Customer must then be redirected to
    /// `redirect_url` to enter card details and complete the first payment.
    pub async fn create_subscription(
        &self,
        req: SubscriptionCreateRequest,
    ) -> Result<Subscription, BepaidError> {
        self.request_json(Method::POST, &self.api("/subscriptions"), Some(&req), false)
            .await
    }

    /// Get subscription details by id.
    pub async fn get_subscription(&self, id: &str) -> Result<Subscription, BepaidError> {
        self.request_json(
            Method::GET,
            &self.api(&format!("/subscriptions/{id}")),
            None::<&u8>,
            false,
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
            false,
        )
        .await
    }
}
