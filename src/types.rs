use serde::{Deserialize, Serialize};

// ── shared value objects ──────────────────────────────────────────────────────

/// Billing address of the cardholder or customer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingAddress {
    /// First name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// Last name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// ISO 3166-1 alpha-2 country code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// City.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// State / region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip: Option<String>,
    /// Street address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// Phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}

/// Customer metadata attached to a transaction or checkout.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Customer {
    /// First name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// Last name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// IP address of the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    /// Email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Device identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    /// Birth date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birth_date: Option<String>,
}

/// Browser data used for 3-D Secure / risk scoring.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserInfo {
    /// Screen width in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub screen_width: Option<i64>,
    /// Screen height in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub screen_height: Option<i64>,
    /// Screen color depth in bits.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub screen_color_depth: Option<i64>,
    /// Browser language tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// Whether Java is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub java_enabled: Option<bool>,
    /// Full user-agent string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
    /// Time zone offset in minutes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<i64>,
    /// IANA time zone name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone_name: Option<String>,
    /// `Accept` request header of the browser.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_header: Option<String>,
    /// Window height in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_height: Option<i64>,
    /// Window width in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_width: Option<i64>,
}

// ── credit card variants ──────────────────────────────────────────────────────

/// Raw card data for initial payment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditCardRaw {
    /// Card number (PAN).
    pub number: String,
    /// Card verification value (CVV/CVC).
    pub verification_value: String,
    /// Cardholder name.
    pub holder: String,
    /// Expiration month (1-12).
    pub exp_month: u8,
    /// Expiration year.
    pub exp_year: u16,
    /// Whether to save the card. When `Some(true)` with `additional_data.contract`,
    /// a payment token is returned for later use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub save_card: Option<bool>,
    /// Card token to pay without the PAN (mutually exclusive with `number`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

/// Credit card in API responses (masked, with brand info).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditCardInfo {
    /// Cardholder name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub holder: Option<String>,
    /// Card stamp (stable card identifier).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stamp: Option<String>,
    /// Card brand, e.g. `visa`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand: Option<String>,
    /// Last 4 digits of the card number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_4: Option<String>,
    /// First digit of the card number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_1: Option<String>,
    /// Bank identification number (first 6 digits).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bin: Option<String>,
    /// First 8 digits of the card number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bin_8: Option<String>,
    /// Issuer country code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer_country: Option<String>,
    /// Issuer bank name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer_name: Option<String>,
    /// Card product name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
    /// Expiration month.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp_month: Option<u8>,
    /// Expiration year.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp_year: Option<u16>,
    /// Token provider, e.g. Apple Pay.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_provider: Option<String>,
    /// Card token for repeat payments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

// ── additional data ───────────────────────────────────────────────────────────

/// Extra per-request data: 3-D Secure browser info, contracts, P2P etc.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdditionalData {
    /// Browser fingerprint for 3-D Secure verification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser: Option<BrowserInfo>,
    /// Contracts to grant, e.g. `["recurring"]` or `["card_on_file"]`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract: Option<Vec<String>>,
    /// Referer URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referer: Option<String>,
    /// Any other method-specific fields.
    #[serde(flatten)]
    pub extra: Option<serde_json::Value>,
}

// ── payment info in responses ─────────────────────────────────────────────────

/// Payment / acquirer details returned with a transaction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentInfo {
    /// Authorization code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_code: Option<String>,
    /// Bank code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_code: Option<String>,
    /// Retrieval reference number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rrn: Option<String>,
    /// Reference id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_id: Option<String>,
    /// Acquiring message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Amount in minor units.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    /// ISO 4217 currency code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Merchant billing descriptor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_descriptor: Option<String>,
    /// Gateway transaction id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_id: Option<i64>,
    /// Payment status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// 3-D Secure verification flow data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreeDSecureVerification {
    /// Status of the verification, e.g. `incomplete` or `successful`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Human-readable message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Visa `ve_status` field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ve_status: Option<String>,
    /// URL to redirect the customer to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pa_res_url: Option<String>,
    /// ECI indicator.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eci: Option<String>,
    /// Mastercard `pa_status` field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pa_status: Option<String>,
    /// Transaction id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xid: Option<String>,
    /// Cardholder authentication value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cavv: Option<String>,
    /// CAVV algorithm.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cavv_algorithm: Option<String>,
    /// Failure reason.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fail_reason: Option<String>,
    /// 3DS challenge request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creq: Option<String>,
}

/// Result of the smart-routing verification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmartRoutingVerification {
    /// Status of the check.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// AVS / CVC verification results.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvsCvcVerification {
    /// Address verification result.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avs_verification: Option<serde_json::Value>,
    /// CVC verification result.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cvc_verification: Option<serde_json::Value>,
}

// ── gateway: create payment ───────────────────────────────────────────────────

/// Card payment request. `amount` is a string in minor units, e.g. `"700"`.
#[derive(Debug, Clone, Serialize)]
pub struct PaymentRequest {
    /// Amount in minor units as a string, e.g. `"700"`.
    pub amount: String,
    /// ISO 4217 currency code, e.g. `USD`.
    pub currency: String,
    /// `true` to run in test mode.
    pub test: bool,
    /// Free-form purchase description.
    pub description: String,
    /// Merchant tracking id (max 255 chars).
    pub tracking_id: String,
    /// ISO 639-1 language code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// URL bePaid POSTs status notifications to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_url: Option<String>,
    /// Cardholder billing address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<BillingAddress>,
    /// Card data (or token) to charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_card: Option<CreditCardRaw>,
    /// Customer metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<Customer>,
    /// Extra data (browser, contracts for tokenization).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<AdditionalData>,
}

/// Minimal response of a successful payment creation.
#[derive(Debug, Clone, Deserialize)]
pub struct PaymentResponse {
    /// Echo of the merchant tracking id.
    pub tracking_id: Option<String>,
    /// Transaction uid, used for status lookup and follow-ups.
    pub uid: String,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct TransactionEnvelope {
    pub transaction: PaymentResponse,
}

// ── gateway: authorization ────────────────────────────────────────────────────

/// Card authorization request. Amounts are integers in minor units.
#[derive(Debug, Clone, Serialize)]
pub struct AuthorizationRequest {
    /// Amount in minor units.
    pub amount: i64,
    /// ISO 4217 currency code.
    pub currency: String,
    /// Free-form description.
    pub description: String,
    /// Payment method, defaults to `credit_card`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_type: Option<String>,
    /// Merchant tracking id.
    pub tracking_id: String,
    /// Set to `true` to run in test mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test: Option<bool>,
    /// Card data or token.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_card: Option<CreditCardRaw>,
    /// Customer metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<Customer>,
    /// Cardholder billing address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<BillingAddress>,
}

/// Response of an authorization. When 3-D Secure is required, `redirect_url`
/// tells you where to send the customer; poll the transaction afterwards.
#[derive(Debug, Clone, Deserialize)]
pub struct AuthorizationResponse {
    /// Transaction uid.
    pub uid: String,
    /// Status, e.g. `incomplete`.
    pub status: Option<String>,
    /// Amount in minor units.
    pub amount: Option<i64>,
    /// ISO 4217 currency code.
    pub currency: Option<String>,
    /// Free-form description.
    pub description: Option<String>,
    /// Transaction type (`authorization`).
    #[serde(rename = "type")]
    pub tx_type: Option<String>,
    /// Payment method type.
    pub payment_method_type: Option<String>,
    /// Merchant tracking id.
    pub tracking_id: Option<String>,
    /// Whether the transaction is a test.
    pub test: Option<bool>,
    /// Masked card details.
    pub credit_card: Option<CreditCardInfo>,
    /// Redirect URL for the 3-D Secure flow.
    pub redirect_url: Option<String>,
    /// 3-D Secure verification state.
    pub three_d_secure_verification: Option<ThreeDSecureVerification>,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct AuthorizationEnvelope {
    pub transaction: AuthorizationResponse,
}

// ── gateway: transaction status ───────────────────────────────────────────────

/// Full transaction status returned by `get_transaction`.
#[derive(Debug, Clone, Deserialize)]
pub struct Transaction {
    /// Transaction uid.
    pub uid: String,
    /// Status, e.g. `successful`, `failed`, `pending`.
    pub status: Option<String>,
    /// Amount in minor units.
    pub amount: Option<i64>,
    /// ISO 4217 currency code.
    pub currency: Option<String>,
    /// Free-form description.
    pub description: Option<String>,
    /// Transaction type (payment, authorization, ...).
    #[serde(rename = "type")]
    pub tx_type: Option<String>,
    /// Payment method type.
    pub payment_method_type: Option<String>,
    /// Merchant tracking id.
    pub tracking_id: Option<String>,
    /// Human-readable message.
    pub message: Option<String>,
    /// Whether it was a test transaction.
    pub test: Option<bool>,
    /// Creation timestamp (ISO 8601).
    pub created_at: Option<String>,
    /// Last update timestamp.
    pub updated_at: Option<String>,
    /// Payment timestamp.
    pub paid_at: Option<String>,
    /// Expiration timestamp.
    pub expired_at: Option<String>,
    /// Recurring type if any.
    pub recurring_type: Option<String>,
    /// Closure timestamp.
    pub closed_at: Option<String>,
    /// Settlement timestamp.
    pub settled_at: Option<String>,
    /// Manual correction timestamp.
    pub manually_corrected_at: Option<String>,
    /// ISO 639-1 language code.
    pub language: Option<String>,
    /// Masked card details.
    pub credit_card: Option<CreditCardInfo>,
    /// Receipt (check) URL.
    pub receipt_url: Option<String>,
    /// Status code of the gateway.
    pub status_code: Option<String>,
    /// Gateway-specific response.
    pub gateway: Option<serde_json::Value>,
    /// Whether merchant notifications are muted.
    pub mute_notifications: Option<serde_json::Value>,
    /// Transaction version.
    pub version: Option<i64>,
    /// Gateway internal id.
    pub id: Option<String>,
    /// Redirect URL when customer action is required.
    pub redirect_url: Option<String>,
    /// bePaid result code, e.g. `S.0000`.
    pub code: Option<String>,
    /// Human-readable result message.
    pub friendly_message: Option<String>,
    /// Smart routing verification result.
    pub smart_routing_verification: Option<SmartRoutingVerification>,
    /// Acquirer payment details.
    pub payment: Option<PaymentInfo>,
    /// AVS / CVC verification results.
    pub avs_cvc_verification: Option<AvsCvcVerification>,
    /// Additional data echoed back.
    pub additional_data: Option<serde_json::Value>,
    /// Customer data echoed back.
    pub customer: Option<serde_json::Value>,
    /// Billing address.
    pub billing_address: Option<BillingAddress>,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct TransactionEnvelopeFull {
    pub transaction: Transaction,
}

// ── gateway: capture / void / refund ──────────────────────────────────────────

/// Capture previously authorized funds.
#[derive(Debug, Clone, Serialize)]
pub struct CaptureRequest {
    /// Uid of the parent authorization.
    pub parent_uid: String,
    /// Amount to capture in minor units.
    pub amount: i64,
    /// Merchant tracking id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_id: Option<String>,
    /// Extra data (e.g. referer).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<AdditionalData>,
}

/// Void a previously authorized transaction.
#[derive(Debug, Clone, Serialize)]
pub struct VoidRequest {
    /// Uid of the parent authorization.
    pub parent_uid: String,
    /// Amount to void in minor units.
    pub amount: i64,
    /// Merchant tracking id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_id: Option<String>,
    /// Extra data (e.g. referer).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<AdditionalData>,
}

/// Refund a payment, fully or partially.
#[derive(Debug, Clone, Serialize)]
pub struct RefundRequest {
    /// Uid of the parent payment.
    pub parent_uid: String,
    /// Amount to refund in minor units.
    pub amount: i64,
    /// Reason for the refund.
    pub reason: String,
    /// Merchant tracking id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_id: Option<String>,
    /// Extra data (e.g. referer).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<AdditionalData>,
}

/// Response of a capture operation.
#[derive(Debug, Clone, Deserialize)]
pub struct CaptureResponse {
    /// New transaction uid.
    pub uid: String,
    /// Status of the capture.
    pub status: Option<String>,
    /// Captured amount in minor units.
    pub amount: Option<i64>,
    /// ISO 4217 currency code.
    pub currency: Option<String>,
    /// Transaction type (`capture`).
    #[serde(rename = "type")]
    pub tx_type: Option<String>,
    /// Human-readable message.
    pub message: Option<String>,
    /// Whether it was a test transaction.
    pub test: Option<bool>,
    /// Creation timestamp.
    pub created_at: Option<String>,
    /// Last update timestamp.
    pub updated_at: Option<String>,
    /// Payment timestamp.
    pub paid_at: Option<String>,
    /// Closure timestamp.
    pub closed_at: Option<String>,
    /// Settlement timestamp.
    pub settled_at: Option<String>,
    /// Manual correction timestamp.
    pub manually_corrected_at: Option<String>,
    /// Uid of the parent authorization.
    pub parent_uid: Option<String>,
    /// Receipt URL.
    pub receipt_url: Option<String>,
    /// Status code.
    pub status_code: Option<String>,
    /// Whether notifications are muted.
    pub mute_notifications: Option<serde_json::Value>,
    /// Transaction version.
    pub version: Option<i64>,
    /// Gateway internal id.
    pub id: Option<String>,
    /// bePaid result code.
    pub code: Option<String>,
    /// Human-readable result message.
    pub friendly_message: Option<String>,
    /// Merchant tracking id.
    pub tracking_id: Option<String>,
    /// Smart routing verification result.
    pub smart_routing_verification: Option<SmartRoutingVerification>,
    /// Capture-specific details.
    pub capture: Option<serde_json::Value>,
    /// Acquirer payment details.
    pub payment: Option<PaymentInfo>,
    /// Additional data echoed back.
    pub additional_data: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct CaptureEnvelope {
    pub transaction: CaptureResponse,
}

/// Response of a void operation.
#[derive(Debug, Clone, Deserialize)]
pub struct VoidResponse {
    /// New transaction uid.
    pub uid: String,
    /// Status of the void.
    pub status: Option<String>,
    /// Human-readable message.
    pub message: Option<String>,
    /// bePaid result code.
    pub code: Option<String>,
    /// Transaction type (`void`).
    #[serde(rename = "type")]
    pub tx_type: Option<String>,
    /// Merchant tracking id.
    pub tracking_id: Option<String>,
    /// Receipt URL.
    pub receipt_url: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct VoidEnvelope {
    pub transaction: VoidResponse,
}

/// Response of a refund operation.
#[derive(Debug, Clone, Deserialize)]
pub struct RefundResponse {
    /// New transaction uid.
    pub uid: Option<String>,
    /// Uid of the parent payment.
    pub parent_uid: Option<String>,
    /// Status of the refund.
    pub status: Option<String>,
    /// Human-readable message.
    pub message: Option<String>,
    /// bePaid result code.
    pub code: Option<String>,
    /// Transaction type (`refund`).
    #[serde(rename = "type")]
    pub tx_type: Option<String>,
    /// Refunded amount in minor units.
    pub amount: Option<i64>,
    /// ISO 4217 currency code.
    pub currency: Option<String>,
    /// Creation timestamp.
    pub created_at: Option<String>,
    /// Last update timestamp.
    pub updated_at: Option<String>,
    /// Merchant tracking id.
    pub tracking_id: Option<String>,
    /// Receipt URL.
    pub receipt_url: Option<String>,
    /// Refund-specific details.
    pub refund: Option<serde_json::Value>,
    /// Smart routing verification result.
    pub smart_routing_verification: Option<SmartRoutingVerification>,
    /// Additional data echoed back.
    pub additional_data: Option<serde_json::Value>,
    /// Whether it was a test transaction.
    pub test: Option<bool>,
    /// Refund reason.
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct RefundEnvelope {
    pub transaction: RefundResponse,
}

// ── token API ─────────────────────────────────────────────────────────────────

/// Card tokenization request. Expiration is expected as zero-padded strings.
#[derive(Debug, Clone, Serialize)]
pub struct CreateTokenRequest {
    /// Card number (PAN).
    pub number: String,
    /// Cardholder name.
    pub holder: String,
    /// Expiration month, e.g. `"05"`.
    pub exp_month: String,
    /// Expiration year, e.g. `"2028"`.
    pub exp_year: String,
    /// Contracts, e.g. `["recurring"]`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract: Option<Vec<String>>,
}

/// Token API response: masked card plus the reusable token.
#[derive(Debug, Clone, Deserialize)]
pub struct TokenResponse {
    /// Cardholder name.
    pub holder: Option<String>,
    /// Card stamp.
    pub stamp: Option<String>,
    /// Card brand.
    pub brand: Option<String>,
    /// Last 4 digits.
    pub last_4: Option<String>,
    /// First digit.
    pub first_1: Option<String>,
    /// Card token for future payments.
    pub token: Option<String>,
    /// Expiration month.
    pub exp_month: Option<i64>,
    /// Expiration year.
    pub exp_year: Option<i64>,
}

// ── checkout API ──────────────────────────────────────────────────────────────

/// Hosted checkout page request.
#[derive(Debug, Clone, Serialize)]
pub struct CheckoutRequest {
    /// `true` to run in test mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test: Option<bool>,
    /// Transaction type of the checkout, e.g. `payment`.
    #[serde(rename = "transaction_type")]
    pub transaction_type: String,
    /// Max payment attempts inside the checkout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attempts: Option<i64>,
    /// Whether the checkout is embedded in an iframe.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iframe: Option<bool>,
    /// Redirection and page settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<CheckoutSettings>,
    /// Allowed payment methods.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<PaymentMethod>,
    /// Prefilled card token.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_card: Option<CheckoutCreditCard>,
    /// Order to be paid.
    pub order: CheckoutOrder,
    /// Customer metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<Customer>,
}

/// Redirection and page settings of a checkout.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckoutSettings {
    /// Base return URL for the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
    /// URL for a successful payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_url: Option<String>,
    /// URL for a declined payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decline_url: Option<String>,
    /// URL for a failed payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fail_url: Option<String>,
    /// URL for a cancelled payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_url: Option<String>,
    /// Webhook notification URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_url: Option<String>,
    /// Text of the "next" button.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub button_next_text: Option<String>,
    /// Whether to auto-pay without a button.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_pay: Option<bool>,
    /// Checkout page language.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
}

/// Allowed payment method types in a checkout.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentMethod {
    /// Method types, e.g. `["credit_card"]`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<String>>,
}

/// Prefilled card in a checkout.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckoutCreditCard {
    /// Card token.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

/// Order section of a checkout.
#[derive(Debug, Clone, Serialize)]
pub struct CheckoutOrder {
    /// ISO 4217 currency code.
    pub currency: String,
    /// Amount in minor units.
    pub amount: i64,
    /// Order description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Response of checkout creation: the hosted page and its token.
#[derive(Debug, Clone, Deserialize)]
pub struct CheckoutResponse {
    /// Checkout token, used for status queries.
    pub token: String,
    /// Redirect the customer to this hosted-payment URL.
    pub redirect_url: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct CheckoutEnvelope {
    pub checkout: CheckoutResponse,
}

/// Checkout status / payment state by token.
#[derive(Debug, Clone, Deserialize)]
pub struct CheckoutStatus {
    /// Checkout token.
    pub token: Option<String>,
    /// Shop id.
    pub shop_id: Option<i64>,
    /// Transaction type.
    pub transaction_type: Option<String>,
    /// Gateway response of the underlying transaction.
    pub gateway_response: Option<serde_json::Value>,
    /// Order section.
    pub order: Option<serde_json::Value>,
    /// Checkout settings echoed back.
    pub settings: Option<CheckoutSettings>,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct CheckoutStatusEnvelope {
    pub checkout: CheckoutStatus,
}

// ── direct / APM API ─────────────────────────────────────────────────────────

/// Alternative payment method (APM) request, e.g. ERIP, Alfabank, MTS Money.
#[derive(Debug, Clone, Serialize)]
pub struct ApmPaymentRequest {
    /// Amount in minor units.
    pub amount: i64,
    /// ISO 4217 currency code.
    pub currency: String,
    /// Free-form description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Customer email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Customer IP.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    /// Merchant success URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_url: Option<String>,
    /// Merchant order id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<serde_json::Value>,
    /// Merchant tracking id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_id: Option<String>,
    /// Webhook notification URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_url: Option<String>,
    /// Expiration timestamp (ISO 8601).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expired_at: Option<String>,
    /// `true` to run in test mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test: Option<bool>,
    /// ISO 639-1 language code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// Merchant return URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
    /// Customer metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<Customer>,
    /// Method-specific parameters, e.g. `{"type": "erip", ...}`.
    pub payment_method: serde_json::Value,
    /// Additional method-specific data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<serde_json::Value>,
}

/// Response of an APM payment request.
#[derive(Debug, Clone, Deserialize)]
pub struct ApmPaymentResponse {
    /// Transaction uid.
    pub uid: Option<String>,
    /// Status, e.g. `pending`, `successful`.
    pub status: Option<String>,
    /// Amount in minor units.
    pub amount: Option<i64>,
    /// ISO 4217 currency code.
    pub currency: Option<String>,
    /// Transaction type.
    #[serde(rename = "type")]
    pub tx_type: Option<String>,
    /// Human-readable message.
    pub message: Option<String>,
    /// Creation timestamp.
    pub created_at: Option<String>,
    /// Last update timestamp.
    pub updated_at: Option<String>,
    /// Payment method type.
    pub method_type: Option<String>,
    /// Receipt URL.
    pub receipt_url: Option<String>,
    /// Merchant tracking id.
    pub tracking_id: Option<String>,
    /// Whether it was a test transaction.
    pub test: Option<bool>,
    /// ISO 639-1 language code.
    pub language: Option<String>,
    /// Acquirer payment details.
    pub payment: Option<PaymentInfo>,
    /// Customer data echoed back.
    pub customer: Option<serde_json::Value>,
    /// Billing address.
    pub billing_address: Option<serde_json::Value>,
    /// Additional data echoed back.
    pub additional_data: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct ApmPaymentEnvelope {
    pub transaction: ApmPaymentResponse,
}

/// Refund of an APM transaction. `amount: None` refunds the full amount.
#[derive(Debug, Clone, Serialize)]
pub struct ApmRefundRequest {
    /// Uid of the parent transaction.
    pub parent_uid: String,
    /// Reason for the refund.
    pub reason: String,
    /// Amount to refund in minor units; omit for full refund.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    /// Merchant tracking id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_id: Option<String>,
    /// Additional data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<serde_json::Value>,
}

/// Response of an APM refund.
#[derive(Debug, Clone, Deserialize)]
pub struct ApmRefundResponse {
    /// New transaction uid.
    pub uid: Option<String>,
    /// Uid of the parent transaction.
    pub parent_uid: Option<String>,
    /// Status of the refund.
    pub status: Option<String>,
    /// Human-readable message.
    pub message: Option<String>,
    /// Transaction type (`refund`).
    #[serde(rename = "type")]
    pub tx_type: Option<String>,
    /// Refunded amount in minor units.
    pub amount: Option<i64>,
    /// ISO 4217 currency code.
    pub currency: Option<String>,
    /// Creation timestamp.
    pub created_at: Option<String>,
    /// Last update timestamp.
    pub updated_at: Option<String>,
    /// Payment method type.
    pub method_type: Option<String>,
    /// Receipt URL.
    pub receipt_url: Option<String>,
    /// Refund-specific details.
    pub refund: Option<serde_json::Value>,
    /// Smart routing verification result.
    pub smart_routing_verification: Option<SmartRoutingVerification>,
    /// Additional data echoed back.
    pub additional_data: Option<serde_json::Value>,
    /// Whether it was a test transaction.
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
    /// Server-assigned customer id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// First name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// Last name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Street address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// City.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// ISO 3166-1 alpha-2 country code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip: Option<String>,
    /// State / region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// Email address (required when creating).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// IP address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    /// Merchant-side external id (max 255 chars).
    #[serde(rename = "external_id", skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
}

/// Plan interval / amount configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanInterval {
    /// Billing amount in minor units.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    /// Interval length in `interval_unit`s.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<i64>,
    /// Unit of the interval: `hour`, `day` or `month`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_unit: Option<String>,
    /// Billing fields shown to the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible_fields: Option<Vec<String>>,
}

/// Trial period settings of a plan.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanTrial {
    /// Trial amount in minor units.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    /// Trial length in `interval_unit`s.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<i64>,
    /// Unit of the interval.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_unit: Option<String>,
    /// Whether the trial payment is charged as the first payment.
    #[serde(rename = "as_first_payment", skip_serializing_if = "Option::is_none")]
    pub as_first_payment: Option<bool>,
}

/// A subscription plan. Used for both create requests and responses.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanItem {
    /// Server-assigned plan id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Whether the plan is a test plan.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test: Option<bool>,
    /// Plan title (required when creating).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// ISO 4217 currency code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// ISO 639-1 language code (default `en`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// Billing configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<PlanInterval>,
    /// Trial period configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial: Option<PlanTrial>,
    /// Whether the plan runs forever (default `true`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infinite: Option<bool>,
    /// Number of billing cycles (ignored when `infinite`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_cycles: Option<i64>,
    /// Payment retry attempts (default 3).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_payment_attempts: Option<i64>,
    /// Whether night charges are prevented.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prevent_payments_at_night: Option<bool>,
    /// Creation timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Last update timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    /// Payment page URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_url: Option<String>,
}

/// Card section of a subscription create request. Either a `token` or the
/// full card details (expiration as zero-padded strings, per the docs).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionCard {
    /// Card token.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    /// Card number (PAN).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,
    /// Cardholder name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub holder: Option<String>,
    /// Card verification value (CVV/CVC).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_value: Option<String>,
    /// Expiration month, e.g. `"01"`.
    #[serde(rename = "exp_month", skip_serializing_if = "Option::is_none")]
    pub exp_month: Option<String>,
    /// Expiration year, e.g. `"2028"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp_year: Option<String>,
}

/// Customer section of a subscription request: either an existing customer
/// `id` or inline customer details.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionCustomer {
    /// Existing customer id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// First name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// Last name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

/// Plan section of a subscription request: either a plan `id` or a full plan.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionPlan {
    /// Existing plan id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Plan title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// ISO 4217 currency code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Billing configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<PlanInterval>,
    /// Trial configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial: Option<PlanTrial>,
}

/// Create a subscription. `plan` is required; `card` + `customer` optional
/// for the hosted flow (customer is redirected to `redirect_url`).
#[derive(Debug, Clone, Serialize)]
pub struct SubscriptionCreateRequest {
    /// Card token or raw card details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<SubscriptionCard>,
    /// Existing customer id or inline customer data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<SubscriptionCustomer>,
    /// Plan id or full plan data (required).
    pub plan: SubscriptionPlan,
    /// Merchant tracking id (max 255 chars).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_id: Option<String>,
    /// Device id of the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    /// Return URL after card entry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
    /// Webhook notification URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_url: Option<String>,
    /// Dynamic billing descriptor for the charges.
    #[serde(
        rename = "dynamic_billing_descriptor",
        skip_serializing_if = "Option::is_none"
    )]
    pub dynamic_billing_descriptor: Option<String>,
    /// Additional data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<serde_json::Value>,
    /// Settings (e.g. `{"language": "en"}`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<serde_json::Value>,
}

/// A subscription as returned by the API or delivered by a webhook.
#[derive(Debug, Clone, Deserialize)]
pub struct Subscription {
    /// Subscription id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// State, e.g. `active`, `trial`, `cancelled`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Merchant tracking id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_id: Option<String>,
    /// Device id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    /// Creation timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Next renewal timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renew_at: Option<String>,
    /// Active-until timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_to: Option<String>,
    /// Masked card details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<CreditCardInfo>,
    /// Customer reference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<SubscriptionCustomer>,
    /// Plan data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<serde_json::Value>,
    /// Last charge transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_transaction: Option<serde_json::Value>,
    /// Number of billing cycles already paid.
    #[serde(
        rename = "paid_billing_cycles",
        skip_serializing_if = "Option::is_none"
    )]
    pub paid_billing_cycles: Option<i64>,
    /// Consecutive failed payment attempts.
    #[serde(
        rename = "number_failed_payment_attempts",
        skip_serializing_if = "Option::is_none"
    )]
    pub number_failed_payment_attempts: Option<i64>,
    /// Additional data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<serde_json::Value>,
    /// Redirect URL for entering card details.
    #[serde(rename = "redirect_url", skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<String>,
    /// Event name, present in webhook payloads (e.g. `created.subscription`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event: Option<String>,
}

/// Cancel a subscription and stop all further charges.
#[derive(Debug, Clone, Serialize)]
pub struct CancelSubscriptionRequest {
    /// Reason for cancellation (required).
    #[serde(rename = "cancel_reason")]
    pub cancel_reason: String,
}

// ── APM: confirm transaction ──────────────────────────────────────────────────

/// Confirm a payment from a third-party application.
#[derive(Debug, Clone, Serialize)]
pub struct ApmConfirmRequest {
    /// Skip check for duplicate confirmations (default `false`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_duplicate_check: Option<bool>,
    /// Payment receipt id (required).
    pub transaction_reference: String,
}

/// Response of a payment confirmation.
#[derive(Debug, Clone, Deserialize)]
pub struct ApmConfirmResponse {
    /// Uid of the parent transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_uid: Option<String>,
    /// Transaction type (`confirm`).
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub tx_type: Option<String>,
    /// Status, e.g. `successful` or `failed`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Human-readable message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Creation timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Amount in minor units.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    /// ISO 4217 currency code.
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
    /// Card number (PAN).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,
    /// Cardholder name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub holder: Option<String>,
    /// Card verification value (CVV/CVC).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_value: Option<String>,
    /// Expiration month.
    #[serde(rename = "exp_month", skip_serializing_if = "Option::is_none")]
    pub exp_month: Option<String>,
    /// Expiration year.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp_year: Option<String>,
    /// Card token.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

/// Additional data of a P2P transfer.
#[derive(Debug, Clone, Serialize)]
pub struct P2pAdditionalData {
    /// P2P-specific data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p2p: Option<P2pInfo>,
}

/// P2P transfer metadata.
#[derive(Debug, Clone, Serialize)]
pub struct P2pInfo {
    /// P2P method type (3-char code).
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub p2p_type: Option<String>,
}

/// P2P card-to-card transfer request.
#[derive(Debug, Clone, Serialize)]
pub struct P2pRequest {
    /// Amount in minor units.
    pub amount: i64,
    /// ISO 4217 currency code.
    pub currency: String,
    /// Sender card (token or raw details).
    pub credit_card: P2pCard,
    /// Recipient card (token or number).
    pub recipient_card: P2pCard,
    /// `true` to run in test mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test: Option<bool>,
    /// Merchant tracking id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_id: Option<String>,
    /// Additional data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<P2pAdditionalData>,
}

/// Response of a P2P transfer.
#[derive(Debug, Clone, Deserialize)]
pub struct P2pResponse {
    /// Transaction uid.
    pub uid: Option<String>,
    /// Status.
    pub status: Option<String>,
    /// Amount in minor units.
    pub amount: Option<i64>,
    /// ISO 4217 currency code.
    pub currency: Option<String>,
    /// Free-form description.
    pub description: Option<String>,
    /// Transaction type (`p2p`).
    #[serde(rename = "type")]
    pub tx_type: Option<String>,
    /// Merchant tracking id.
    pub tracking_id: Option<String>,
    /// Whether it was a test transaction.
    pub test: Option<bool>,
    /// Creation timestamp.
    pub created_at: Option<String>,
    /// Redirect URL for 3-D Secure (when required).
    pub redirect_url: Option<String>,
    /// Masked sender card.
    pub credit_card: Option<CreditCardInfo>,
    /// Masked recipient card.
    pub recipient_card: Option<CreditCardInfo>,
    /// Receipt URL.
    pub receipt_url: Option<String>,
    /// P2P verification state.
    pub verify_p2p: Option<serde_json::Value>,
    /// P2P transfer details.
    pub p2p: Option<serde_json::Value>,
    /// Sender billing address.
    pub sender_billing_address: Option<BillingAddress>,
    /// Recipient billing address.
    pub recipient_billing_address: Option<BillingAddress>,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct P2pEnvelope {
    pub transaction: P2pResponse,
}

// ── webhook ───────────────────────────────────────────────────────────────────

/// A bePaid webhook notification: one `transaction` object.
#[derive(Debug, Clone, Deserialize)]
pub struct WebhookNotification {
    /// The transaction this notification is about.
    pub transaction: WebhookTransaction,
}

/// Transaction payload inside a webhook notification.
#[derive(Debug, Clone, Deserialize)]
pub struct WebhookTransaction {
    /// Transaction uid.
    pub uid: String,
    /// Transaction type.
    #[serde(rename = "type")]
    pub tx_type: String,
    /// Status, e.g. `successful`, `failed`, `pending`, `expired`.
    pub status: String,
    /// Amount in minor units.
    pub amount: Option<i64>,
    /// ISO 4217 currency code.
    pub currency: Option<String>,
    /// Free-form description.
    pub description: Option<String>,
    /// Creation timestamp.
    pub created_at: Option<String>,
    /// Last update timestamp.
    pub updated_at: Option<String>,
    /// Payment method type.
    pub method_type: Option<String>,
    /// Acquirer payment details.
    pub payment: Option<PaymentInfo>,
    /// Customer data.
    pub customer: Option<serde_json::Value>,
    /// Human-readable message.
    pub message: Option<String>,
    /// Merchant tracking id.
    pub tracking_id: Option<String>,
    /// Whether it was a test transaction.
    pub test: Option<bool>,
    /// ISO 639-1 language code.
    pub language: Option<String>,
    /// Payment timestamp.
    pub paid_at: Option<String>,
    /// Billing address.
    pub billing_address: Option<serde_json::Value>,
    /// Additional data.
    pub additional_data: Option<serde_json::Value>,
}
