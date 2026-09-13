use serde::{Deserialize, Serialize};

// ── shared value objects ──────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingAddress {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Customer {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birth_date: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub screen_width: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub screen_height: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub screen_color_depth: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub java_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_header: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_height: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_width: Option<i64>,
}

// ── credit card variants ──────────────────────────────────────────────────────

/// Raw card data for initial payment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditCardRaw {
    pub number: String,
    pub verification_value: String,
    pub holder: String,
    pub exp_month: u8,
    pub exp_year: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub save_card: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

/// Credit card in API responses (masked, with brand info).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditCardInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub holder: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stamp: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_4: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bin_8: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer_country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp_month: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp_year: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

// ── additional data ───────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdditionalData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser: Option<BrowserInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referer: Option<String>,
    #[serde(flatten)]
    pub extra: Option<serde_json::Value>,
}

// ── payment info in responses ─────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rrn: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_descriptor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreeDSecureVerification {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ve_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pa_res_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eci: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pa_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cavv: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cavv_algorithm: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fail_reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creq: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmartRoutingVerification {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvsCvcVerification {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avs_verification: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cvc_verification: Option<serde_json::Value>,
}

// ── gateway: create payment ───────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize)]
pub struct PaymentRequest {
    pub amount: String,
    pub currency: String,
    pub test: bool,
    pub description: String,
    pub tracking_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<BillingAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_card: Option<CreditCardRaw>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<Customer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<AdditionalData>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PaymentResponse {
    pub tracking_id: Option<String>,
    pub uid: String,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct TransactionEnvelope {
    pub transaction: PaymentResponse,
}

// ── gateway: authorization ────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize)]
pub struct AuthorizationRequest {
    pub amount: i64,
    pub currency: String,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_type: Option<String>,
    pub tracking_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_card: Option<CreditCardRaw>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<Customer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<BillingAddress>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AuthorizationResponse {
    pub uid: String,
    pub status: Option<String>,
    pub amount: Option<i64>,
    pub currency: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "type")]
    pub tx_type: Option<String>,
    pub payment_method_type: Option<String>,
    pub tracking_id: Option<String>,
    pub test: Option<bool>,
    pub credit_card: Option<CreditCardInfo>,
    pub redirect_url: Option<String>,
    pub three_d_secure_verification: Option<ThreeDSecureVerification>,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct AuthorizationEnvelope {
    pub transaction: AuthorizationResponse,
}

// ── gateway: transaction status ───────────────────────────────────────────────

#[derive(Debug, Clone, Deserialize)]
pub struct Transaction {
    pub uid: String,
    pub status: Option<String>,
    pub amount: Option<i64>,
    pub currency: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "type")]
    pub tx_type: Option<String>,
    pub payment_method_type: Option<String>,
    pub tracking_id: Option<String>,
    pub message: Option<String>,
    pub test: Option<bool>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub paid_at: Option<String>,
    pub expired_at: Option<String>,
    pub recurring_type: Option<String>,
    pub closed_at: Option<String>,
    pub settled_at: Option<String>,
    pub manually_corrected_at: Option<String>,
    pub language: Option<String>,
    pub credit_card: Option<CreditCardInfo>,
    pub receipt_url: Option<String>,
    pub status_code: Option<String>,
    pub gateway: Option<serde_json::Value>,
    pub mute_notifications: Option<serde_json::Value>,
    pub version: Option<i64>,
    pub id: Option<String>,
    pub redirect_url: Option<String>,
    pub code: Option<String>,
    pub friendly_message: Option<String>,
    pub smart_routing_verification: Option<SmartRoutingVerification>,
    pub payment: Option<PaymentInfo>,
    pub avs_cvc_verification: Option<AvsCvcVerification>,
    pub additional_data: Option<serde_json::Value>,
    pub customer: Option<serde_json::Value>,
    pub billing_address: Option<BillingAddress>,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct TransactionEnvelopeFull {
    pub transaction: Transaction,
}

// ── gateway: capture / void / refund ──────────────────────────────────────────

#[derive(Debug, Clone, Serialize)]
pub struct CaptureRequest {
    pub parent_uid: String,
    pub amount: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<AdditionalData>,
}

#[derive(Debug, Clone, Serialize)]
pub struct VoidRequest {
    pub parent_uid: String,
    pub amount: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<AdditionalData>,
}

#[derive(Debug, Clone, Serialize)]
pub struct RefundRequest {
    pub parent_uid: String,
    pub amount: i64,
    pub reason: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<AdditionalData>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CaptureResponse {
    pub uid: String,
    pub status: Option<String>,
    pub amount: Option<i64>,
    pub currency: Option<String>,
    #[serde(rename = "type")]
    pub tx_type: Option<String>,
    pub message: Option<String>,
    pub test: Option<bool>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub paid_at: Option<String>,
    pub closed_at: Option<String>,
    pub settled_at: Option<String>,
    pub manually_corrected_at: Option<String>,
    pub parent_uid: Option<String>,
    pub receipt_url: Option<String>,
    pub status_code: Option<String>,
    pub mute_notifications: Option<serde_json::Value>,
    pub version: Option<i64>,
    pub id: Option<String>,
    pub code: Option<String>,
    pub friendly_message: Option<String>,
    pub tracking_id: Option<String>,
    pub smart_routing_verification: Option<SmartRoutingVerification>,
    pub capture: Option<serde_json::Value>,
    pub payment: Option<PaymentInfo>,
    pub additional_data: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct CaptureEnvelope {
    pub transaction: CaptureResponse,
}

#[derive(Debug, Clone, Deserialize)]
pub struct VoidResponse {
    pub uid: String,
    pub status: Option<String>,
    pub message: Option<String>,
    pub code: Option<String>,
    #[serde(rename = "type")]
    pub tx_type: Option<String>,
    pub tracking_id: Option<String>,
    pub receipt_url: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct VoidEnvelope {
    pub transaction: VoidResponse,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RefundResponse {
    pub uid: Option<String>,
    pub parent_uid: Option<String>,
    pub status: Option<String>,
    pub message: Option<String>,
    pub code: Option<String>,
    #[serde(rename = "type")]
    pub tx_type: Option<String>,
    pub amount: Option<i64>,
    pub currency: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub tracking_id: Option<String>,
    pub receipt_url: Option<String>,
    pub refund: Option<serde_json::Value>,
    pub smart_routing_verification: Option<SmartRoutingVerification>,
    pub additional_data: Option<serde_json::Value>,
    pub test: Option<bool>,
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct RefundEnvelope {
    pub transaction: RefundResponse,
}

// ── token API ─────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize)]
pub struct CreateTokenRequest {
    pub number: String,
    pub holder: String,
    pub exp_month: String,
    pub exp_year: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TokenResponse {
    pub holder: Option<String>,
    pub stamp: Option<String>,
    pub brand: Option<String>,
    pub last_4: Option<String>,
    pub first_1: Option<String>,
    pub token: Option<String>,
    pub exp_month: Option<i64>,
    pub exp_year: Option<i64>,
}

// ── checkout API ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize)]
pub struct CheckoutRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test: Option<bool>,
    #[serde(rename = "transaction_type")]
    pub transaction_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attempts: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iframe: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<CheckoutSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<PaymentMethod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_card: Option<CheckoutCreditCard>,
    pub order: CheckoutOrder,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<Customer>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckoutSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decline_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fail_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub button_next_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_pay: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentMethod {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckoutCreditCard {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CheckoutOrder {
    pub currency: String,
    pub amount: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CheckoutResponse {
    pub token: String,
    pub redirect_url: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct CheckoutEnvelope {
    pub checkout: CheckoutResponse,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CheckoutStatus {
    pub token: Option<String>,
    pub shop_id: Option<i64>,
    pub transaction_type: Option<String>,
    pub gateway_response: Option<serde_json::Value>,
    pub order: Option<serde_json::Value>,
    pub settings: Option<CheckoutSettings>,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct CheckoutStatusEnvelope {
    pub checkout: CheckoutStatus,
}

// ── direct / APM API ─────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize)]
pub struct ApmPaymentRequest {
    pub amount: i64,
    pub currency: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expired_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<Customer>,
    pub payment_method: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ApmPaymentResponse {
    pub uid: Option<String>,
    pub status: Option<String>,
    pub amount: Option<i64>,
    pub currency: Option<String>,
    #[serde(rename = "type")]
    pub tx_type: Option<String>,
    pub message: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub method_type: Option<String>,
    pub receipt_url: Option<String>,
    pub tracking_id: Option<String>,
    pub test: Option<bool>,
    pub language: Option<String>,
    pub payment: Option<PaymentInfo>,
    pub customer: Option<serde_json::Value>,
    pub billing_address: Option<serde_json::Value>,
    pub additional_data: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct ApmPaymentEnvelope {
    pub transaction: ApmPaymentResponse,
}

#[derive(Debug, Clone, Serialize)]
pub struct ApmRefundRequest {
    pub parent_uid: String,
    pub reason: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ApmRefundResponse {
    pub uid: Option<String>,
    pub parent_uid: Option<String>,
    pub status: Option<String>,
    pub message: Option<String>,
    #[serde(rename = "type")]
    pub tx_type: Option<String>,
    pub amount: Option<i64>,
    pub currency: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub method_type: Option<String>,
    pub receipt_url: Option<String>,
    pub refund: Option<serde_json::Value>,
    pub smart_routing_verification: Option<SmartRoutingVerification>,
    pub additional_data: Option<serde_json::Value>,
    pub test: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct ApmRefundEnvelope {
    pub transaction: ApmRefundResponse,
}

// ── subscriptions API (api.bepaid.by) ────────────────────────────────────────

/// A customer resource in the subscription service. Used for both create
/// requests (server assigns `id`) and responses. All fields are optional
/// except `email` when creating.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    #[serde(rename = "external_id", skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
}

/// Plan interval / amount configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanInterval {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_unit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible_fields: Option<Vec<String>>,
}

/// Trial period settings of a plan.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanTrial {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_unit: Option<String>,
    #[serde(rename = "as_first_payment", skip_serializing_if = "Option::is_none")]
    pub as_first_payment: Option<bool>,
}

/// A subscription plan. Used for both create requests and responses.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<PlanInterval>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial: Option<PlanTrial>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infinite: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_cycles: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_payment_attempts: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prevent_payments_at_night: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_url: Option<String>,
}

/// Card section of a subscription create request. Either a `token` or the
/// full card details (expiration as zero-padded strings, per the docs).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionCard {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub holder: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_value: Option<String>,
    #[serde(rename = "exp_month", skip_serializing_if = "Option::is_none")]
    pub exp_month: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp_year: Option<String>,
}

/// Customer section of a subscription request: either an existing customer
/// `id` or inline customer details.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionCustomer {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

/// Plan section of a subscription request: either a plan `id` or a full plan.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionPlan {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<PlanInterval>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial: Option<PlanTrial>,
}

/// Create a subscription. `plan` is required; `card` + `customer` optional
/// for the hosted flow (customer is redirected to `redirect_url`).
#[derive(Debug, Clone, Serialize)]
pub struct SubscriptionCreateRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<SubscriptionCard>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<SubscriptionCustomer>,
    pub plan: SubscriptionPlan,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_url: Option<String>,
    #[serde(
        rename = "dynamic_billing_descriptor",
        skip_serializing_if = "Option::is_none"
    )]
    pub dynamic_billing_descriptor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<serde_json::Value>,
}

/// A subscription as returned by the API or delivered by a webhook.
#[derive(Debug, Clone, Deserialize)]
pub struct Subscription {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renew_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_to: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<CreditCardInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<SubscriptionCustomer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_transaction: Option<serde_json::Value>,
    #[serde(
        rename = "paid_billing_cycles",
        skip_serializing_if = "Option::is_none"
    )]
    pub paid_billing_cycles: Option<i64>,
    #[serde(
        rename = "number_failed_payment_attempts",
        skip_serializing_if = "Option::is_none"
    )]
    pub number_failed_payment_attempts: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<serde_json::Value>,
    #[serde(rename = "redirect_url", skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<String>,
    /// Event name, present in webhook payloads (e.g. `created.subscription`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CancelSubscriptionRequest {
    #[serde(rename = "cancel_reason")]
    pub cancel_reason: String,
}

// ── APM: confirm transaction ──────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize)]
pub struct ApmConfirmRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_duplicate_check: Option<bool>,
    pub transaction_reference: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ApmConfirmResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_uid: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub tx_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct ApmConfirmEnvelope {
    pub response: ApmConfirmResponse,
}

// ── P2P transfer ──────────────────────────────────────────────────────────────

/// Sender or recipient card of a P2P transfer. Either a `token` or a card
/// number (expiration/holder/CVV required for the source card).
#[derive(Debug, Clone, Serialize)]
pub struct P2pCard {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub holder: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_value: Option<String>,
    #[serde(rename = "exp_month", skip_serializing_if = "Option::is_none")]
    pub exp_month: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp_year: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct P2pAdditionalData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p2p: Option<P2pInfo>,
}

#[derive(Debug, Clone, Serialize)]
pub struct P2pInfo {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub p2p_type: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct P2pRequest {
    pub amount: i64,
    pub currency: String,
    pub credit_card: P2pCard,
    pub recipient_card: P2pCard,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<P2pAdditionalData>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct P2pResponse {
    pub uid: Option<String>,
    pub status: Option<String>,
    pub amount: Option<i64>,
    pub currency: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "type")]
    pub tx_type: Option<String>,
    pub tracking_id: Option<String>,
    pub test: Option<bool>,
    pub created_at: Option<String>,
    pub redirect_url: Option<String>,
    pub credit_card: Option<CreditCardInfo>,
    pub recipient_card: Option<CreditCardInfo>,
    pub receipt_url: Option<String>,
    pub verify_p2p: Option<serde_json::Value>,
    pub p2p: Option<serde_json::Value>,
    pub sender_billing_address: Option<BillingAddress>,
    pub recipient_billing_address: Option<BillingAddress>,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct P2pEnvelope {
    pub transaction: P2pResponse,
}

// ── webhook ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Deserialize)]
pub struct WebhookNotification {
    pub transaction: WebhookTransaction,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WebhookTransaction {
    pub uid: String,
    #[serde(rename = "type")]
    pub tx_type: String,
    pub status: String,
    pub amount: Option<i64>,
    pub currency: Option<String>,
    pub description: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub method_type: Option<String>,
    pub payment: Option<PaymentInfo>,
    pub customer: Option<serde_json::Value>,
    pub message: Option<String>,
    pub tracking_id: Option<String>,
    pub test: Option<bool>,
    pub language: Option<String>,
    pub paid_at: Option<String>,
    pub billing_address: Option<serde_json::Value>,
    pub additional_data: Option<serde_json::Value>,
}
