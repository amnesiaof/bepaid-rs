use serde::{Deserialize, Serialize};

// ── shared value objects ──────────────────────────────────────────────────────

/// Billing address of the cardholder or customer.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct BillingAddress {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub middle_name: Option<String>,
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
#[allow(missing_docs)]
pub struct Customer {
    /// Customer identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Customer identity document number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub middle_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
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
    /// Phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// Customer identifier in the merchant's system.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    /// Customer taxpayer ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taxpayer_id: Option<String>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,
    /// Card verification value (CVV/CVC).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_value: Option<String>,
    /// Cardholder name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub holder: Option<String>,
    /// Expiration month (1-12).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp_month: Option<u8>,
    /// Expiration year.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp_year: Option<u16>,
    /// Whether to save the card. When `Some(true)` with `additional_data.contract`,
    /// a payment token is returned for later use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub save_card: Option<bool>,
    /// Card token to pay without the PAN (mutually exclusive with `number`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    /// Skip 3-D Secure verification for the card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_three_d_secure_verification: Option<bool>,
    /// Force 3-D Secure verification for the card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_three_d_secure_verification: Option<bool>,
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
    /// Masterpass parameters for saving cards and payments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub masterpass: Option<MasterpassData>,
    /// v2 split distribution: one entry per recipient. The sum of the split
    /// amounts must equal the request `amount` (else error `E.1025`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub split: Option<Vec<SplitRecipient>>,
    /// Smart-routing options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smart_routing_options: Option<SmartRoutingOptions>,
    /// Gateway ids excluded from cascading.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_gateways: Option<Vec<i64>>,
    /// Any other method-specific fields.
    #[serde(flatten)]
    pub extra: Option<serde_json::Value>,
}

/// Smart-routing options of a payment (`additional_data.smart_routing_options`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmartRoutingOptions {
    /// `true` disables the halva co-brand on the payment page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_halva: Option<bool>,
}

/// One recipient of a v2 split payment (`additional_data.split[]`).
///
/// `amount` and `tax_id` are required; the sum of all `amount`s must equal the
/// request `amount`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SplitRecipient {
    /// Amount (minor units) routed to this recipient.
    pub amount: i64,
    /// Recipient UNP (taxpayer id).
    pub tax_id: String,
    /// Recipient company name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_name: Option<String>,
    /// Recipient bank account (IBAN).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_account: Option<String>,
    /// Recipient bank BIC.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_bic: Option<String>,
    /// Payment description for this recipient.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Recipient legal address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legal_address: Option<String>,
    /// Recipient mailing address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mailing_address: Option<String>,
    /// Recipient country.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Recipient city.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Recipient postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// Recipient contact email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_email: Option<String>,
    /// Recipient contact phone.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_phone: Option<String>,
    /// Free-form memo.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
}

/// Masterpass data attached to a payment or authorization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MasterpassData {
    /// Masterpass request parameters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<MasterpassParams>,
}

/// Masterpass parameters for saving a card or paying with it.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MasterpassParams {
    /// Session returned by Masterpass login.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<String>,
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

/// Tax line of a fiscalized position (KZT fiscalization).
#[derive(Debug, Clone, Serialize)]
pub struct FiscalizationTax {
    /// Tax internal id.
    pub id: String,
    /// Tax rate, e.g. `"12"`.
    pub percent: String,
    /// Tax type, e.g. `vat`.
    #[serde(rename = "type")]
    pub tax_type: String,
    /// Free-form tax description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Whether the tax is included in the position amount.
    pub inclusive: bool,
}

/// One itemized position of a fiscalized KZT payment.
#[derive(Debug, Clone, Serialize)]
pub struct FiscalizationPosition {
    /// Product name.
    pub name: String,
    /// Position type, e.g. `service`.
    #[serde(rename = "type")]
    pub position_type: String,
    /// Position amount in minor units.
    pub amount: i64,
    /// Quantity.
    pub quantity: f64,
    /// OKEI measure unit code, e.g. `796`.
    pub measure_unit_code: i64,
    /// Free-form position description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Whether the position is untaxed.
    pub untaxed: bool,
    /// Product nomenclature code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nomenclature_code: Option<String>,
    /// Applied taxes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taxes: Option<Vec<FiscalizationTax>>,
}

/// KZT fiscalization payload sent alongside the `credit_card` data.
#[derive(Debug, Clone, Serialize)]
pub struct Fiscalization {
    /// Merchant-side fiscalization id.
    pub external_id: String,
    /// Itemized positions.
    pub positions: Vec<FiscalizationPosition>,
}

/// Fiscal receipt info nested in a transaction fiscalization.
#[derive(Debug, Clone, Deserialize)]
#[allow(missing_docs)]
pub struct FiscalizationReceiptInfo {
    pub kkm_id: Option<String>,
    pub id: Option<String>,
    pub shift_id: Option<String>,
    pub serial_id: Option<String>,
    pub serial_shift_id: Option<String>,
    pub issue_time: Option<String>,
    pub operation_type: Option<String>,
    pub payment_type: Option<String>,
    pub currency_code: Option<String>,
    pub subtotal_amount: Option<i64>,
    pub total_amount: Option<i64>,
    pub cashier_code: Option<String>,
    pub cashier_name: Option<String>,
    pub receipt_num: Option<String>,
    pub ofd_receipt_id: Option<String>,
    pub ofd_qr_code: Option<String>,
}

/// One fiscal receipt of a transaction (`fiscalization.receipts[]`).
#[derive(Debug, Clone, Deserialize)]
#[allow(missing_docs)]
pub struct FiscalizationReceipt {
    pub id: Option<String>,
    pub serial_id: Option<String>,
    pub receipt_num: Option<String>,
    pub created_at: Option<String>,
    pub ofd_id: Option<String>,
    pub ofd_link: Option<String>,
    pub ofd_qr_code: Option<String>,
    pub total_amount: Option<i64>,
    pub receipt_info: Option<FiscalizationReceiptInfo>,
}

/// Fiscalization data returned with a transaction.
#[derive(Debug, Clone, Deserialize)]
#[allow(missing_docs)]
pub struct FiscalizationInfo {
    pub id: Option<String>,
    pub external_id: Option<String>,
    pub status: Option<String>,
    pub code: Option<String>,
    pub message: Option<String>,
    pub friendly_message: Option<String>,
    pub receipts: Option<Vec<FiscalizationReceipt>>,
}

/// A single custom field attached to a transaction (`custom_field_N` entry).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CustomField {
    /// Field label, displayed in reports.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// Field value, displayed in reports.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// Hint shown in the field on the confirmation page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
    /// If `true`, displays the field on the confirmation page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible: Option<bool>,
    /// If `true`, requires the field on the confirmation page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    /// If `true`, prevents editing the field value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
}

/// Up to 3 custom fields (`order.custom_fields` / request `custom_fields`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CustomFields {
    /// First custom field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_field_1: Option<CustomField>,
    /// Second custom field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_field_2: Option<CustomField>,
    /// Third custom field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_field_3: Option<CustomField>,
}

/// Card payment request. `amount` is a string in minor units, e.g. `"700"`.
#[derive(Debug, Clone, Serialize)]
pub struct PaymentRequest {
    /// Payment expiration timestamp (ISO 8601).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expired_at: Option<String>,
    /// Descriptor shown on the cardholder statement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_billing_descriptor: Option<String>,
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
    /// URL for 3-D Secure verification redirect.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_url: Option<String>,
    /// URL the customer is returned to after 3-D Secure (required for
    /// server-to-server H2H payments).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
    /// `false` to allow duplicate transactions (same amount and card within
    /// 30 seconds) instead of rejecting them.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duplicate_check: Option<bool>,
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
    /// Encrypted card data (JWE) for client-side encryption — mutually
    /// exclusive with `credit_card`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted_data: Option<String>,
    /// KZT fiscalization payload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fiscalization: Option<Fiscalization>,
    /// Up to 3 custom fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<CustomFields>,
}

// ── gateway: authorization ────────────────────────────────────────────────────

/// Card authorization request. Amounts are integers in minor units.
#[derive(Debug, Clone, Serialize)]
pub struct AuthorizationRequest {
    /// ISO 639-1 language code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// URL for transaction notifications.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_url: Option<String>,
    /// Customer return URL after verification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
    /// Authorization expiration timestamp (ISO 8601).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expired_at: Option<String>,
    /// Descriptor shown on the cardholder statement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_billing_descriptor: Option<String>,
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
    /// `false` to allow duplicate transactions (same amount and card within
    /// 30 seconds) instead of rejecting them.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duplicate_check: Option<bool>,
    /// Card data or token.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_card: Option<CreditCardRaw>,
    /// Customer metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<Customer>,
    /// Cardholder billing address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<BillingAddress>,
    /// Extra data (browser, contracts for tokenization).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<AdditionalData>,
    /// URL bePaid POSTs the transaction verification request to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_url: Option<String>,
    /// Up to 3 custom fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<CustomFields>,
}

// ── gateway: transaction status ───────────────────────────────────────────────

/// Full transaction status returned by `get_transaction`.
#[derive(Debug, Clone, Deserialize)]
pub struct Transaction {
    #[allow(missing_docs)]
    #[serde(flatten)]
    pub extra: Option<serde_json::Value>,
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
    /// PSP settlement timestamp.
    pub psp_settled_at: Option<String>,
    /// ISO 639-1 language code.
    pub language: Option<String>,
    /// Masked card details.
    pub credit_card: Option<CreditCardInfo>,
    /// Receipt (check) URL.
    pub receipt_url: Option<String>,
    /// Status code of the 3-D Secure check.
    pub status_code: Option<i64>,
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
    /// 3-D Secure verification result.
    pub three_d_secure_verification: Option<ThreeDSecureVerification>,
    /// Uid of the parent transaction, if any.
    pub parent_uid: Option<String>,
    /// Reason of a disputed transaction (chargeback).
    pub reason: Option<String>,
    /// Error data of transaction processing.
    pub errors: Option<serde_json::Value>,
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
    /// Merchant order id (ЕРИП).
    pub order_id: Option<String>,
    /// ЕРИП payment data (QR code, instruction, etc.).
    pub erip: Option<serde_json::Value>,
    /// Up to 3 custom fields.
    pub custom_fields: Option<CustomFields>,
    /// Tokenization result (present on `tokenization` transactions).
    pub tokenization: Option<TokenizationInfo>,
    /// Fiscalization result (present on fiscalized transactions).
    pub fiscalization: Option<FiscalizationInfo>,
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
    /// Set to `true` to fiscalize the full amount of the parent transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fiscalization: Option<bool>,
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
    /// PSP settlement timestamp.
    pub psp_settled_at: Option<String>,
    /// Uid of the parent authorization.
    pub parent_uid: Option<String>,
    /// Receipt URL.
    pub receipt_url: Option<String>,
    /// Status code.
    pub status_code: Option<i64>,
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
    pub uid: String,
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

/// Result of a card tokenization transaction.
#[derive(Debug, Clone, Deserialize)]
pub struct TokenizationInfo {
    /// Gateway internal tokenization id.
    pub gateway_id: Option<i64>,
    /// Tokenization status, e.g. `successful`.
    pub status: Option<String>,
    /// Human-readable message.
    pub message: Option<String>,
}

/// 3-D Secure advanced-control flow settings.
#[derive(Debug, Clone, Serialize)]
pub struct ThreeDSecureAdvanced {
    /// Enable the advanced 3-D Secure flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advanced: Option<bool>,
}

/// Card tokenization (3-D Secure) request.
#[derive(Debug, Clone, Serialize)]
pub struct TokenizationRequest {
    /// Amount in minor units.
    pub amount: i64,
    /// ISO 4217 currency code.
    pub currency: String,
    /// Free-form description.
    pub description: String,
    /// Merchant tracking id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_id: Option<String>,
    /// Block identical concurrent requests.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duplicate_check: Option<bool>,
    /// Dynamic descriptor shown in the statement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_billing_descriptor: Option<String>,
    /// ISO 639-1 language code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// URL bePaid POSTs the transaction notification to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_url: Option<String>,
    /// URL bePaid POSTs the transaction verification request to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_url: Option<String>,
    /// URL to redirect the customer to after payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
    /// Set to `true` to run in test mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test: Option<bool>,
    /// Cardholder billing address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<BillingAddress>,
    /// Card data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_card: Option<CreditCardRaw>,
    /// 3-D Secure flow control.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub three_d_secure: Option<ThreeDSecureAdvanced>,
    /// Travel data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub travel: Option<serde_json::Value>,
    /// Customer metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<Customer>,
    /// Extra per-request data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<AdditionalData>,
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
    /// Descriptor shown on the cardholder statement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_billing_descriptor: Option<String>,
    /// Travel industry data (flights, car rentals).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub travel: Option<serde_json::Value>,
    /// KZT fiscalization payload (payment-token creation).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fiscalization: Option<Fiscalization>,
}

/// User-consent toggle settings on the confirmation page.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgreementToggle {
    /// Default toggle position (on/off).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<bool>,
    /// URL of the merchant's User Agreement text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Text shown next to the toggle.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

/// Customer detail fields shown on the payment widget.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckoutCustomerFields {
    /// Fields displayed non-editable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<Vec<String>>,
    /// Fields displayed editable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible: Option<Vec<String>>,
    /// Fields hidden on the widget.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hidden: Option<Vec<String>>,
}

/// Cardholder name prefilling settings on the widget.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditCardFields {
    /// Cardholder name to prefill.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub holder: Option<String>,
    /// Fields blocked from editing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<Vec<String>>,
}

/// Save-card toggle configuration on the widget.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveCardToggle {
    /// Whether the toggle is displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<bool>,
    /// `true` – toggle means consent to provide card data to the merchant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_contract: Option<bool>,
    /// Text replacing the standard toggle name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Hint text for the toggle.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hint: Option<String>,
}

/// "Pay with another card" toggle configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnotherCardToggle {
    /// Whether the toggle is displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<bool>,
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
    /// Whether to skip the user-agreement page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agreed: Option<bool>,
    /// User-agreement toggle settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agreement_toggle: Option<AgreementToggle>,
    /// Customer detail fields shown on the widget.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_fields: Option<CheckoutCustomerFields>,
    /// Cardholder name prefilling on the widget.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_card_fields: Option<CreditCardFields>,
    /// URL used for payout verification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_url: Option<String>,
    /// Seconds before auto-return, `0` for immediate redirect.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_return: Option<String>,
    /// URL for card art notifications.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_notification_url: Option<String>,
    /// Save-card toggle configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub save_card_toggle: Option<SaveCardToggle>,
    /// Another-card toggle configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub another_card_toggle: Option<AnotherCardToggle>,
    /// Widget styling object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<serde_json::Value>,
    /// Widget version, e.g. `2`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub widget_version: Option<String>,
    /// User-consent requirements, e.g. `["terms"]`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require: Option<serde_json::Value>,
    /// Checkout customer section (cardholder details, phone/email).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<serde_json::Value>,
}

/// Allowed payment method types in a checkout.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct PaymentMethod {
    #[serde(flatten)]
    pub extra: Option<serde_json::Value>,
    /// Method types, e.g. `["credit_card"]`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<String>>,
    /// Payment types excluded from the payment page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_types: Option<Vec<String>>,
    /// Card brands and digital wallets excluded from the widget.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_brands: Option<Vec<String>>,
}

/// Prefilled card in a checkout.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckoutCreditCard {
    /// Card token.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

/// Extra order data for a checkout.
#[derive(Debug, Clone, Serialize)]
#[allow(missing_docs)]
pub struct CheckoutAdditionalData {
    #[serde(flatten)]
    pub extra: Option<serde_json::Value>,
    /// Contract types; include `recurring` to receive a card token back.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract: Option<Vec<String>>,
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
    /// Order tracking/reference id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_id: Option<String>,
    /// Date and time till a payment can be done (ISO 8601).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expired_at: Option<String>,
    /// Extra order data, e.g. `contract` for tokenization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<CheckoutAdditionalData>,
    /// Up to 3 custom fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<CustomFields>,
}

/// Response of checkout creation: the hosted page and its token.
#[derive(Debug, Clone, Deserialize)]
pub struct CheckoutResponse {
    /// Checkout token, used for status queries.
    pub token: String,
    /// Redirect the customer to this hosted-payment URL.
    pub redirect_url: Option<String>,
    /// Any additional checkout fields echoed by the API.
    #[serde(flatten)]
    pub extra: Option<serde_json::Value>,
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
    /// Customer data.
    pub customer: Option<serde_json::Value>,
    /// Whether the checkout finished.
    pub finished: Option<bool>,
    /// Whether the payment token expired.
    pub expired: Option<bool>,
    /// Shop metadata.
    pub shop: Option<serde_json::Value>,
    /// Whether it was a test checkout.
    pub test: Option<bool>,
    /// Status, e.g. `error`, `successful`.
    pub status: Option<String>,
    /// Human-readable message.
    pub message: Option<String>,
    /// Payment method details.
    pub payment_method: Option<serde_json::Value>,
    /// Shop section returned by the API.
    pub merchant: Option<serde_json::Value>,
    /// Checkout version (integer or string).
    pub version: Option<serde_json::Value>,
    /// Masked card info of the attempted payment.
    pub card_info: Option<serde_json::Value>,
    /// Background job id for deferred checkouts.
    pub job_id: Option<String>,
    /// Max payment attempts echoed back.
    pub attempts: Option<i64>,
    /// Whether the checkout runs inside an iframe.
    pub iframe: Option<bool>,
    /// Descriptor echoed back.
    pub dynamic_billing_descriptor: Option<String>,
    /// Travel industry data echoed back.
    pub travel: Option<serde_json::Value>,
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
    /// Whether the payment page is embedded in an iframe.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iframe: Option<bool>,
    /// URL for third-party payment verification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_url: Option<String>,
    /// Merchant success URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_url: Option<String>,
    /// Merchant order id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
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
    /// Up to 3 custom fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<CustomFields>,
}

#[derive(Debug, Clone, Serialize)]
#[allow(missing_docs)]
pub struct EripPayListRequest {
    pub terminal_id: String,
    pub pay_code: String,
    pub di_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub erip_session_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<EripPayListCustomer>,
}

#[derive(Debug, Clone, Serialize)]
#[allow(missing_docs)]
pub struct EripPayListCustomer {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personal_account: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub erip_account: Option<String>,
}

/// Meter/device entry in an ERIP payment (`erip_devices`).
#[derive(Debug, Clone, Serialize)]
pub struct EripDevice {
    /// Device name, e.g. `Холодная вода`.
    pub name: String,
    /// Unit of measurement, e.g. `м3`.
    pub item_unit: String,
    /// Display rank.
    pub rank: String,
    /// Current meter value.
    pub value: String,
    /// Tariff rate.
    pub rate: String,
}

impl ApmPaymentRequest {
    fn base(amount: i64, currency: &str, payment_method: serde_json::Value) -> Self {
        Self {
            amount,
            currency: currency.to_owned(),
            description: None,
            email: None,
            ip: None,
            iframe: None,
            verification_url: None,
            success_url: None,
            order_id: None,
            tracking_id: None,
            notification_url: None,
            expired_at: None,
            test: None,
            language: None,
            return_url: None,
            customer: None,
            payment_method,
            additional_data: None,
            custom_fields: None,
        }
    }

    /// ERIP (ЕРИП) payment. `account_number` is the subscriber account,
    /// `service_no` the ERIP service number. For richer payloads
    /// (`service_info`, `receipt`, `erip_devices`) extend via struct-update
    /// syntax and [`EripDevice`].
    pub fn erip(amount: i64, currency: &str, account_number: &str, service_no: &str) -> Self {
        Self::base(
            amount,
            currency,
            serde_json::json!({
                "type": "erip",
                "account_number": account_number,
                "service_no": service_no,
            }),
        )
    }

    /// MTS Money payment. `phone` must include the country code, e.g.
    /// `375295222222`. `confirm_agreement` is one of `already_accepted`,
    /// `accept`, `decline`.
    pub fn mts_money(amount: i64, currency: &str, phone: &str, confirm_agreement: &str) -> Self {
        let mut req = Self::base(
            amount,
            currency,
            serde_json::json!({
                "type": "mts_money",
                "confirm_agreement": confirm_agreement,
            }),
        );
        req.customer = Some(Customer {
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
            ip: None,
            email: None,
            device_id: None,
            birth_date: None,
            phone: Some(phone.to_owned()),
            external_id: None,
            taxpayer_id: None,
        });
        req
    }

    /// KROK payment. `return_url` is where the customer lands after paying.
    pub fn krok(amount: i64, currency: &str, return_url: &str) -> Self {
        let mut req = Self::base(amount, currency, serde_json::json!({ "type": "krok" }));
        req.return_url = Some(return_url.to_owned());
        req
    }

    /// QIWI terminal payment. `account` is the buyer's id in the merchant
    /// system (phone, email or customer id).
    pub fn qiwi_terminal(amount: i64, currency: &str, account: &str) -> Self {
        Self::base(
            amount,
            currency,
            serde_json::json!({
                "type": "qiwi_terminal",
                "account": account,
            }),
        )
    }

    /// SberPay (QR + deeplink) payment. `return_url` is required; `phone`
    /// enables the desktop push-notification flow.
    pub fn sberpay(amount: i64, currency: &str, return_url: &str, phone: Option<&str>) -> Self {
        let mut req = Self::base(
            amount,
            currency,
            serde_json::json!({ "type": "sberpay_qr_deeplink" }),
        );
        req.return_url = Some(return_url.to_owned());
        if let Some(phone) = phone {
            req.customer = Some(Customer {
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
                ip: None,
                email: None,
                device_id: None,
                birth_date: None,
                phone: Some(phone.to_owned()),
                external_id: None,
                taxpayer_id: None,
            });
        }
        req
    }

    /// Alfa-Click payment (Belarus).
    pub fn alfaclick(amount: i64, currency: &str) -> Self {
        Self::base(amount, currency, serde_json::json!({ "type": "alfaclick" }))
    }

    /// WebPay payment (Belarus).
    pub fn webpay(amount: i64, currency: &str) -> Self {
        Self::base(amount, currency, serde_json::json!({ "type": "webpay" }))
    }

    /// RC Card payment (Russia).
    pub fn rccard(amount: i64, currency: &str) -> Self {
        Self::base(amount, currency, serde_json::json!({ "type": "rccard" }))
    }

    /// BYN Card payment (Belarus).
    pub fn byncard(amount: i64, currency: &str) -> Self {
        Self::base(amount, currency, serde_json::json!({ "type": "byncard" }))
    }

    /// Halva payment (Belarus).
    pub fn halva(amount: i64, currency: &str) -> Self {
        Self::base(amount, currency, serde_json::json!({ "type": "halva" }))
    }
}

/// Response of an APM payment request.
#[derive(Debug, Clone, Deserialize)]
#[allow(missing_docs)]
pub struct ApmPaymentResponse {
    #[serde(flatten)]
    pub extra: Option<serde_json::Value>,
    pub erip: Option<serde_json::Value>,
    pub payment_method_type: Option<String>,
    pub id: Option<String>,
    pub order_id: Option<String>,
    pub description: Option<String>,
    pub expired_at: Option<String>,
    pub paid_at: Option<String>,
    pub closed_at: Option<String>,
    pub settled_at: Option<String>,
    pub manually_corrected_at: Option<String>,
    pub psp_settled_at: Option<String>,
    pub registry_id: Option<serde_json::Value>,
    pub version: Option<i64>,
    pub smart_routing_verification: Option<SmartRoutingVerification>,
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
    /// Up to 3 custom fields.
    pub custom_fields: Option<CustomFields>,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct ApmPaymentEnvelope {
    pub transaction: ApmPaymentResponse,
}

/// Refund of an APM transaction. ERIP `/beyag/refunds` requires an explicit amount.
#[derive(Debug, Clone, Serialize)]
pub struct ApmRefundRequest {
    /// Uid of the parent transaction.
    pub parent_uid: String,
    /// Reason for the refund.
    pub reason: String,
    /// Amount to refund in minor units; required for ERIP `/beyag/refunds`.
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
#[allow(missing_docs)]
pub struct ApmRefundResponse {
    pub id: Option<String>,
    pub reason: Option<String>,
    pub paid_at: Option<String>,
    pub language: Option<String>,
    pub version: Option<i64>,
    pub payment_method_type: Option<String>,
    pub erip: Option<serde_json::Value>,
    pub settled_at: Option<String>,
    pub psp_settled_at: Option<String>,
    pub registry_id: Option<serde_json::Value>,
    pub tracking_id: Option<String>,
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

/// Last charge transaction attached to a subscription.
#[derive(Debug, Clone, Deserialize)]
pub struct SubscriptionLastTransaction {
    /// Transaction uid.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
    /// Transaction status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Human-readable message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Creation timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
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
    pub plan: Option<PlanItem>,
    /// Last charge transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_transaction: Option<SubscriptionLastTransaction>,
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
    /// Confirmation type for BelВеб-кредит: `"confirm"` or `"cancel"` (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirm_type: Option<String>,
    /// Skip check for duplicate confirmations (default `false`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_duplicate_check: Option<bool>,
    #[allow(missing_docs)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_reference: Option<String>,
    #[allow(missing_docs)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
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

// ── MTS Money check_service ──────────────────────────────────────────────────

/// Request to check if a customer is an MTS Money participant.
#[derive(Debug, Clone, Serialize)]
pub struct CheckServiceRequest {
    /// Set `true` to run in test mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test: Option<bool>,
    /// Customer information. `phone` is required (e.g. `375291112233`).
    pub customer: Customer,
}

/// Response from the MTS Money `check_service` endpoint.
#[derive(Debug, Clone, Deserialize)]
pub struct CheckServiceResponse {
    #[allow(missing_docs)]
    pub error_code: Option<serde_json::Value>,
    /// Whether the customer's phone is enrolled in MTS Money.
    pub service_activated: Option<bool>,
    /// System message.
    pub message: Option<String>,
    /// Provider validation details.
    pub validation: Option<CheckServiceValidation>,
}

/// Provider-specific validation info from MTS Money check.
#[derive(Debug, Clone, Deserialize)]
pub struct CheckServiceValidation {
    /// Operator identifier (e.g. `mts`).
    pub operator: Option<String>,
    /// Provider-specific message.
    pub message: Option<String>,
}

// ── P2P transfer ────────────────────────────────────────────────────────────

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
#[allow(missing_docs)]
pub struct P2pAdditionalData {
    /// P2P-specific data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p2p: Option<P2pInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_text: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract: Option<Vec<String>>,
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
#[allow(missing_docs)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expired_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duplicate_check: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<Customer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_billing_address: Option<BillingAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient_billing_address: Option<BillingAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<BillingAddress>,
}

/// Response of a P2P transfer.
#[derive(Debug, Clone, Deserialize)]
#[allow(missing_docs)]
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
    pub updated_at: Option<String>,
    pub paid_at: Option<String>,
    pub language: Option<String>,
    pub payment_method_type: Option<String>,
    pub message: Option<String>,
    pub status_code: Option<i64>,
    pub id: Option<String>,
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
    pub additional_data: Option<serde_json::Value>,
    pub customer: Option<serde_json::Value>,
    pub billing_address: Option<BillingAddress>,
    /// Sender billing address.
    pub sender_billing_address: Option<BillingAddress>,
    /// Recipient billing address.
    pub recipient_billing_address: Option<BillingAddress>,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct P2pEnvelope {
    pub transaction: P2pResponse,
}

/// Response of a P2P-restriction check (flat, no envelope).
#[derive(Debug, Clone, Deserialize)]
pub struct VerifyP2pResponse {
    /// Check status (`successful` / `failed`).
    pub status: Option<String>,
    /// Message from the bank system.
    pub message: Option<String>,
    /// Commission info (present when transfer is possible).
    pub commission: Option<P2pCommission>,
    /// Whether the check used test cards.
    pub test: Option<bool>,
    /// Error code on failure.
    pub error_code: Option<String>,
    /// Fields that must be collected additionally from the cardholder.
    pub required_fields: Option<P2pRequiredFields>,
    #[allow(missing_docs)]
    pub errors: Option<serde_json::Value>,
}

/// Commission details returned by a P2P-restriction check.
#[derive(Debug, Clone, Deserialize)]
pub struct P2pCommission {
    /// Minimum commission in major units.
    pub minimum: Option<f64>,
    /// Commission percentage.
    pub percent: Option<f64>,
    /// Absolute commission computed by the processing bank.
    pub bank_fee: Option<f64>,
    /// Commission currency (ISO-4217).
    pub currency: Option<String>,
}

/// Fields requested additionally about source/recipient cards.
#[derive(Debug, Clone, Deserialize)]
pub struct P2pRequiredFields {
    /// Fields to request about the source card.
    pub credit_card: Option<Vec<String>>,
    /// Fields to request about the recipient card.
    pub recipient_card: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize)]
#[allow(missing_docs)]
pub struct VisaAliasPhoneRequest {
    pub recipient_info: VisaAliasPhoneInfo,
}

#[derive(Debug, Clone, Serialize)]
#[allow(missing_docs)]
pub struct VisaAliasPhoneInfo {
    pub phone_number: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(missing_docs)]
pub struct VisaAliasServiceInfo {
    pub recipient_name: Option<String>,
    pub issuer_name: Option<String>,
    pub card_type: Option<String>,
    pub address1: Option<String>,
    pub address2: Option<String>,
    pub city: Option<String>,
    pub country: Option<String>,
    pub postal_code: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[allow(missing_docs)]
pub struct VisaAliasPhoneResponse {
    #[serde(flatten)]
    pub credit_card: CreditCardInfo,
    pub service_info: Option<VisaAliasServiceInfo>,
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
#[allow(missing_docs)]
pub struct WebhookTransaction {
    pub erip: Option<serde_json::Value>,
    pub payment_method_type: Option<String>,
    pub id: Option<String>,
    pub order_id: Option<String>,
    pub expired_at: Option<String>,
    pub parent_uid: Option<String>,
    pub reason: Option<String>,
    pub refund: Option<serde_json::Value>,
    pub version: Option<i64>,
    pub closed_at: Option<String>,
    pub settled_at: Option<String>,
    pub manually_corrected_at: Option<String>,
    pub psp_settled_at: Option<String>,
    pub registry_id: Option<serde_json::Value>,
    pub receipt_url: Option<String>,
    pub smart_routing_verification: Option<SmartRoutingVerification>,
    pub custom_fields: Option<CustomFields>,
    #[serde(flatten)]
    pub extra: Option<serde_json::Value>,
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

// ── Payout ────────────────────────────────────────────────────────────────────

/// Document data for payouts, e.g. `{"type": "PASSPORT"}`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayoutDocument {
    /// Document type, e.g. `PASSPORT`.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub doc_type: Option<String>,
    /// Issuing authority.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
    /// Document series.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub series: Option<String>,
    /// Document number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,
    /// Issue date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issued_at: Option<String>,
    /// Expiry date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_until: Option<String>,
}

/// Additional data of a payout.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayoutAdditionalData {
    /// Recipient document.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<PayoutDocument>,
}

/// Card payout destination. Card details are sent to bePaid, the merchant
/// should prefer the tokenized flow where available.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayoutCreditCard {
    /// Card number (PAN).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,
    /// Cardholder name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub holder: Option<String>,
    /// Expiration month as `MM` string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp_month: Option<String>,
    /// Expiration year as `YYYY` string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp_year: Option<String>,
}

/// Create a card payout (push money out of the merchant balance).
#[derive(Debug, Clone, Serialize)]
pub struct PayoutRequest {
    /// `true` to run in test mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test: Option<bool>,
    /// Amount in minor units.
    pub amount: i64,
    /// ISO 4217 currency code.
    pub currency: String,
    /// Free-form description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Merchant tracking id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_id: Option<String>,
    /// Payee identification (ip/email/birth_date required).
    pub recipient: Customer,
    /// Payer identification (ip/email/birth_date required).
    pub sender: Customer,
    /// Recipient billing address.
    pub recipient_billing_address: BillingAddress,
    /// Payer billing address.
    pub sender_billing_address: BillingAddress,
    /// Destination card details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient_credit_card: Option<PayoutCreditCard>,
    /// Additional data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<PayoutAdditionalData>,
    /// Up to 3 custom fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<CustomFields>,
}

/// Payout execution details.
#[derive(Debug, Clone, Deserialize)]
pub struct Payout {
    /// Payout status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// bePaid gateway id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_id: Option<i64>,
    /// Reference id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_id: Option<String>,
    /// Bank code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_code: Option<String>,
    /// Retrieval reference number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rrn: Option<String>,
}

/// Response of a payout.
#[derive(Debug, Clone, Deserialize)]
pub struct PayoutResponse {
    /// Transaction uid.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
    /// Transaction type (`payout`).
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub tx_type: Option<String>,
    /// Status, e.g. `successful`, `pending`, `failed`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Amount in minor units.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    /// ISO 4217 currency code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Free-form description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Creation timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Last update timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    /// Payment method type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method_type: Option<String>,
    /// Receipt URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_url: Option<String>,
    /// Human-readable message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Merchant tracking id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_id: Option<String>,
    /// Whether it was a test transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test: Option<bool>,
    /// Payout execution details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payout: Option<Payout>,
    /// Customer data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<Customer>,
    /// Billing address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<BillingAddress>,
    /// Up to 3 custom fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<CustomFields>,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct PayoutEnvelope {
    pub transaction: PayoutResponse,
}

// ── APM balance query ─────────────────────────────────────────────────────────

/// Query the balance of an APM gateway account. Sent without a `request`
/// wrapper.
#[derive(Debug, Clone, Serialize)]
pub struct BalanceRequest {
    /// bePaid gateway id.
    pub gateway_id: i64,
    /// Account number to query.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    /// ISO 4217 currency code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
}

/// Balance of an APM account. Returned without an envelope.
#[derive(Debug, Clone, Deserialize)]
pub struct BalanceResponse {
    /// Response code, e.g. `S.0000`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// Status, e.g. `Successful`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Raw status message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Human-readable message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub friendly_message: Option<String>,
    /// bePaid gateway id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_id: Option<i64>,
    /// Queried account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    /// Balance amount in minor units.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    /// ISO 4217 currency code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Provider-specific balance data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_info: Option<serde_json::Value>,
}

// ── Merchant reports ──────────────────────────────────────────────────────────

/// Filter for the report list (API v2). Uses a single `date`.
#[derive(Debug, Clone, Serialize)]
pub struct ReportParams {
    /// Date basis: `created_at`, `paid_at` or `settled_at`.
    pub date_type: String,
    /// Report date `YYYY-MM-DD`.
    pub date: String,
    /// Transaction status: `all`, `successful`, `failed`, `pending`,
    /// `incomplete`.
    pub status: String,
    /// Payment method type: `credit_card`, `alternative` or `erip`.
    pub payment_method_type: String,
    /// IANA time zone, e.g. `Europe/London`.
    pub time_zone: String,
}

/// Filter for the report count (API v3). Uses a `from`/`to` range.
#[derive(Debug, Clone, Serialize)]
pub struct ReportCountParams {
    /// Date basis: `created_at`, `paid_at` or `settled_at`.
    pub date_type: String,
    /// Range start `YYYY-MM-DD hh:mm:ss`.
    pub from: String,
    /// Range end `YYYY-MM-DD hh:mm:ss`.
    pub to: String,
    /// Transaction status: `all`, `successful`, `failed`, `pending`,
    /// `incomplete`.
    pub status: String,
    /// Payment method type: `credit_card`, `alternative` or `erip`.
    pub payment_method_type: String,
    /// IANA time zone, e.g. `Europe/London`.
    pub time_zone: String,
}

/// One report transaction. Shape varies with `payment_method_type`; the
/// method-specific objects are kept as raw JSON.
#[derive(Debug, Clone, Deserialize)]
pub struct ReportTransaction {
    /// Row id, used for pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// Transaction uid.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
    /// Transaction type, e.g. `authorization`, `p2p`.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub tx_type: Option<String>,
    /// Payment method type: `credit_card`, `alternative` or `erip`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_type: Option<String>,
    /// Status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Human-readable message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Amount in minor units.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    /// Discount rate, percent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount_rate: Option<f64>,
    /// Transaction rate, percent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_rate: Option<f64>,
    /// Transaction fee in minor units.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_fee: Option<f64>,
    /// Amount paid to the merchant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_to_merchant: Option<f64>,
    /// Whether it was a test transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test: Option<bool>,
    /// ISO 4217 currency code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Free-form description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Merchant tracking id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_id: Option<String>,
    /// Order id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<i64>,
    /// Creation timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Payment timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_at: Option<String>,
    /// Settlement timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settled_at: Option<String>,
    /// Manual correction timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manually_corrected_at: Option<String>,
    /// Billing address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<BillingAddress>,
    /// Customer data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<Customer>,
    /// Card details (credit card transactions).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_card: Option<serde_json::Value>,
    /// Method-specific payment data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment: Option<serde_json::Value>,
    /// 3-D Secure verification data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub three_d_secure_verification: Option<serde_json::Value>,
    /// Additional data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<serde_json::Value>,
}

/// Body of the report list request (API v2).
#[derive(Debug, Clone, Serialize)]
pub struct ReportListRequest {
    /// Report filters.
    pub report_params: ReportParams,
}

/// Filter for the paginated report list (API v3). Uses a `from`/`to` range and
/// cursor pagination.
#[derive(Debug, Clone, Serialize)]
pub struct ReportListV3Params {
    /// Date basis: `created_at`, `paid_at` or `settled_at`.
    pub date_type: String,
    /// Range start `YYYY-MM-DD hh:mm:ss`.
    pub from: String,
    /// Range end `YYYY-MM-DD hh:mm:ss`.
    pub to: String,
    /// Transaction status: `all`, `successful`, `failed`, `pending`,
    /// `incomplete`.
    pub status: String,
    /// Payment method type: `credit_card`, `alternative` or `erip`.
    pub payment_method_type: String,
    /// IANA time zone, e.g. `Europe/London`.
    pub time_zone: String,
    /// Return rows after this cursor (`last_object_id` of the previous page).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,
    /// Return rows before this cursor (`first_object_id` of the next page).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,
    /// Manual-correction range start `YYYY-MM-DD hh:mm:ss`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_correction_from: Option<String>,
    /// Manual-correction range end `YYYY-MM-DD hh:mm:ss`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_correction_to: Option<String>,
}

/// Body of the paginated report list request (API v3).
#[derive(Debug, Clone, Serialize)]
pub struct ReportListV3Request {
    /// Report filters.
    pub report_params: ReportListV3Params,
}

/// Response of the report list (API v2/v3).
#[derive(Debug, Clone, Deserialize)]
pub struct ReportListResponse {
    /// Report transactions.
    pub transactions: Vec<ReportTransaction>,
    /// Transaction count.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    /// Whether more rows are available (API v3).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    /// Cursor of the first row in this page (API v3).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_object_id: Option<String>,
    /// Cursor of the last row in this page (API v3).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_object_id: Option<String>,
}

/// Body of the report count request (API v3).
#[derive(Debug, Clone, Serialize)]
pub struct ReportCountRequest {
    /// Report filters.
    pub report_params: ReportCountParams,
}

/// Count result of the report count endpoint.
#[derive(Debug, Clone, Deserialize)]
pub struct ReportCountResult {
    /// Transaction count.
    pub count: i64,
}

/// Response of the report count (API v3). The count sits in
/// `transactions.count`.
#[derive(Debug, Clone, Deserialize)]
pub struct ReportCountResponse {
    /// Count result.
    pub transactions: ReportCountResult,
}

/// Balance of a payout channel (payout control).
#[derive(Debug, Clone, Deserialize)]
pub struct ChannelBalance {
    /// bePaid gateway id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_id: Option<i64>,
    /// ISO 4217 currency code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Balance amount in minor units.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
}

/// Credit card used in a split payment (token only).
#[derive(Debug, Clone, Serialize)]
pub struct SplitCreditCard {
    /// Payment token previously created via tokenization.
    pub token: String,
}

/// Distribution of a split payment among recipient shops.
#[derive(Debug, Clone, Serialize)]
pub struct SplitAdditionalData {
    /// Contract reference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract: Option<String>,
    /// Amount (minor units) per recipient shop id, e.g. `{"241": 40, "242": 50}`.
    pub split: std::collections::HashMap<String, i64>,
}

/// Request body of the split payment endpoint.
#[derive(Debug, Clone, Serialize)]
pub struct SplitPaymentRequest {
    /// Total amount in minor units.
    pub amount: i64,
    /// ISO 4217 currency code.
    pub currency: String,
    /// Transaction description.
    pub description: String,
    /// Merchant tracking id.
    pub tracking_id: String,
    /// Billing address of the cardholder (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<BillingAddress>,
    /// Card token used for the payment.
    pub credit_card: SplitCreditCard,
    /// Customer metadata (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<Customer>,
    /// Split distribution across shops (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<SplitAdditionalData>,
}

/// One transaction produced by a split payment.
#[derive(Debug, Clone, Deserialize)]
pub struct SplitItem {
    /// Transaction uid.
    pub uid: String,
    /// Amount in minor units.
    pub amount: i64,
    /// Transaction status, e.g. `successful`.
    pub status: String,
    /// Result message.
    pub message: String,
    /// Shop this part is paid to.
    pub shop_id: i64,
    /// Whether this is the parent transaction.
    pub parent: bool,
    /// UID of the parent transaction, if any.
    pub parent_uid: Option<String>,
}

/// Response of the split payment endpoint.
#[derive(Debug, Clone, Deserialize)]
pub struct SplitPaymentResponse {
    /// The individual transactions making up the split.
    pub splits: Vec<SplitItem>,
}

/// Request body for creating a pay-by-link product.
#[derive(Debug, Clone, Serialize)]
pub struct ProductCreateRequest {
    /// Product name.
    pub name: String,
    /// Product description.
    pub description: String,
    /// ISO 4217 currency code (or cryptocurrency code).
    pub currency: String,
    /// Price in minor units.
    pub amount: i64,
    /// Number of units in stock; omit when `infinite` is set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<String>,
    /// Whether the stock is unlimited.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infinite: Option<bool>,
    /// Fields shown on the payment page (`first_name`, `last_name`, ...).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible_fields: Option<Vec<String>>,
    /// Whether this is a test product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test: Option<bool>,
    /// Whether the payment time is unlimited.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub immortal: Option<bool>,
    /// ISO 8601 expiry (`YYYY-MM-DDThh:mm:ssTZD`); required unless `immortal`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expired_at: Option<String>,
    /// URL the customer is redirected to after payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
    /// Shop id the payment goes to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shop_id: Option<String>,
    /// Payment page language (default `en`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// `payment` or `authorization`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_type: Option<String>,
}

/// Request body for updating a pay-by-link product.
#[derive(Debug, Clone, Serialize)]
pub struct ProductUpdateRequest {
    /// New product name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// New product description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// New ISO 4217 currency code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// New price in minor units.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    /// Fields shown on the payment page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible_fields: Option<Vec<String>>,
    /// Whether the stock is unlimited.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infinite: Option<bool>,
    /// New available quantity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<String>,
    /// Whether this is a test product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test: Option<bool>,
    /// Whether the payment time is unlimited.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub immortal: Option<bool>,
    /// ISO 8601 expiry of the product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expired_at: Option<String>,
    /// URL the customer is redirected to after payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
    /// Shop id the payment goes to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shop_id: Option<String>,
    /// Payment page language.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// `payment` or `authorization`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_type: Option<String>,
}

/// A pay-by-link product and its payment links.
#[derive(Debug, Clone, Deserialize)]
pub struct Product {
    /// Product id (e.g. `prd_...`).
    pub id: String,
    /// Product name.
    pub name: String,
    /// Product description.
    pub description: String,
    /// ISO 4217 currency code.
    pub currency: String,
    /// Price in minor units.
    pub amount: i64,
    /// Units in stock; `None` when unlimited.
    pub quantity: Option<i64>,
    /// Whether the quantity is unlimited.
    pub infinite: bool,
    /// Payment page language.
    pub language: String,
    /// `payment` or `authorization`.
    pub transaction_type: String,
    /// Creation timestamp.
    pub created_at: String,
    /// Last update timestamp.
    pub updated_at: String,
    /// Whether this is a test product.
    pub test: bool,
    /// Additional product data.
    pub additional_data: serde_json::Value,
    /// Direct payment URL.
    pub pay_url: String,
    /// Payment URL (returns the customer to the payment page).
    pub payment_url: String,
    /// Order confirmation URL.
    pub confirm_url: String,
}

// ── gateway: saved-card charges ───────────────────────────────────────────────

/// Credit card data for saved-card charges.
///
/// Either `token` (from a previous tokenization) or raw card details are
/// required.  When both are omitted the request is rejected by bePaid.
#[derive(Debug, Clone, Serialize)]
pub struct ChargeCreditCard {
    /// Card number (PAN).  Omit when using `token`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,
    /// Card verification value (CVV/CVC).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_value: Option<String>,
    /// Cardholder name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub holder: Option<String>,
    /// Expiration month (1-12).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp_month: Option<i32>,
    /// Expiration year.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp_year: Option<i32>,
    /// Saved card token.  Required when card details are omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    /// Skip 3-D Secure verification (requires prior agreement with bePaid).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_three_d_secure_verification: Option<bool>,
}

/// Additional data for saved-card charges.
///
/// Only the most common fields are typed; the remaining undocumented sub-objects
/// (`p2p`, `masterpass`, `sub_brand`, `card_on_file`) can be added in the
/// future as bePaid documents them.
#[derive(Debug, Clone, Serialize)]
pub struct ChargeAdditionalData {
    /// Contract types, e.g. `["oneclick"]`, `["recurring", "card_on_file"]`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract: Option<Vec<String>>,
    /// Gateway IDs excluded from cascading.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_gateways: Option<Vec<i64>>,
    /// Browser data for 3-D Secure 2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser: Option<BrowserInfo>,
}

/// Request to charge a saved card (oneclick / recurring).
///
/// Uses the same Gateway API family as regular payments (X-API-Version 3).
#[derive(Debug, Clone, Serialize)]
pub struct ChargeRequest {
    /// Amount in minor units (integers).
    pub amount: i64,
    /// ISO 4217 currency code, e.g. `USD`.
    pub currency: String,
    /// Free-form purchase description.
    pub description: String,
    /// Merchant tracking id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_id: Option<String>,
    /// ISO 8601 expiry (`YYYY-MM-DDThh:mm:ssTZD`); payment cannot be completed
    /// after this time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expired_at: Option<String>,
    /// Reject duplicate `tracking_id` values (default `true`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duplicate_check: Option<bool>,
    /// Dynamic billing descriptor shown on the cardholder statement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_billing_descriptor: Option<String>,
    /// ISO 639-1 language code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// URL for status notifications.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_url: Option<String>,
    /// URL for 3-D Secure verification redirect.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_url: Option<String>,
    /// URL the customer is returned to after 3-D Secure (conditional — required
    /// when 3-D Secure may be triggered).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
    /// `true` for test mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test: Option<bool>,
    /// Force 3-D Secure even on tokenized cards (overrides
    /// `credit_card.skip_three_d_secure_verification`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_three_d_secure_verification: Option<bool>,
    /// Credit card data — either a saved `token` or raw card details.
    pub credit_card: ChargeCreditCard,
    /// Customer metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<Customer>,
    /// Additional data (contract types, excluded gateways, 3-D Secure 2 browser).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<ChargeAdditionalData>,
    /// KZT fiscalization payload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fiscalization: Option<Fiscalization>,
}

// ── gateway: recipient tokenization ──────────────────────────────────────────

/// Additional data for recipient tokenization.
#[derive(Debug, Clone, Serialize)]
pub struct RecipientTokenizationAdditionalData {
    /// The URL the request originates from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referer: Option<String>,
}

/// Tokenize a recipient's card for future payouts.
///
/// POST `/transactions/recipient_tokenizations` (X-API-Version 3). No money
/// moves: the returned card token is used in
/// [`PayoutRequest::recipient_credit_card`]. The response shape is not
/// documented by bePaid as a typed object, so the raw JSON is returned.
#[derive(Debug, Clone, Serialize)]
pub struct RecipientTokenizationRequest {
    /// Free-form description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Merchant tracking id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_id: Option<String>,
    /// Billing address of the cardholder.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient_billing_address: Option<BillingAddress>,
    /// The card to tokenize.
    pub recipient_credit_card: PayoutCreditCard,
    /// The customer requesting the tokenization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient: Option<Customer>,
    /// Additional data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<RecipientTokenizationAdditionalData>,
}

// ── gateway: risk checkup ──────────────────────────────────────────────────────

/// Pre-authorization risk check (`POST /transactions/checkups`).
///
/// Validates a transaction against risk-management rules before processing.
#[derive(Debug, Clone, Serialize)]
pub struct CheckupRequest {
    /// Amount in minor units.
    pub amount: i64,
    /// ISO 4217 currency code.
    pub currency: String,
    /// Free-form description.
    pub description: String,
    /// Merchant tracking id.
    pub tracking_id: String,
    /// ISO 639-1 language code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// Webhook notification URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_url: Option<String>,
    /// URL for successful payment verification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_url: Option<String>,
    /// `true` to run in test mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test: Option<bool>,
    /// Card data or token.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_card: Option<ChargeCreditCard>,
    /// Customer metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<Customer>,
    /// Cardholder billing address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<BillingAddress>,
    /// Additional data (e.g. `{"referer": "..."}`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<serde_json::Value>,
}

// ── APM currency query ─────────────────────────────────────────────────────────

/// Query which currencies an APM gateway account supports. Sent without a
/// `request` wrapper.
#[derive(Debug, Clone, Serialize)]
pub struct CurrencyQueryRequest {
    /// bePaid gateway id.
    pub gateway_id: i64,
    /// Account number to query.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    /// ISO 3166-1 alpha-2 country code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
}

/// Currencies an APM gateway account supports. Returned without an envelope.
#[derive(Debug, Clone, Deserialize)]
pub struct CurrencyInfo {
    /// Response code, e.g. `S.0000`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// Status, e.g. `Successful`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Raw status message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Human-readable message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub friendly_message: Option<String>,
    /// bePaid gateway id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_id: Option<i64>,
    /// Queried account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    /// ISO 3166-1 alpha-2 country code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// ISO 4217 currency code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Provider-specific currency data (e.g. deposit/withdrawal allowances).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_info: Option<serde_json::Value>,
}

// ── gateway: tracking-id status query ─────────────────────────────────────────

/// CVC verification flags returned by the tracking-id status query.
#[derive(Debug, Clone, Deserialize)]
pub struct TrackingCvcVerification {
    /// CVC verification result code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_code: Option<String>,
}

/// Card balance request (`POST {gateway}/balance`).
#[derive(Debug, Clone, Serialize)]
pub struct CardBalanceRequest {
    /// Account number to query.
    pub account: String,
    /// ISO 4217 currency code.
    pub currency: String,
    /// bePaid gateway id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_id: Option<i64>,
}

/// Balance of a single gateway account.
#[derive(Debug, Clone, Deserialize)]
pub struct CardBalanceResult {
    /// bePaid gateway id (`gatewayId` in JSON).
    #[serde(rename = "gatewayId")]
    pub gateway_id: i64,
    /// Account number (`account` in JSON).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    /// Balance in minor units.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    /// ISO 4217 currency code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Issuing bank details (`bankInfo` in JSON).
    #[serde(rename = "bankInfo", skip_serializing_if = "Option::is_none")]
    pub bank_info: Option<serde_json::Value>,
}

/// Response of the card balance query.
#[derive(Debug, Clone, Deserialize)]
pub struct CardBalanceResponse {
    /// Query status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Balance of the queried account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<CardBalanceResult>,
}

// ── async processing mode ─────────────────────────────────────────────────────

/// Acknowledgement of an async payment/authorization submission.
#[derive(Debug, Clone, Deserialize)]
pub struct AsyncAck {
    /// Queue status, e.g. `pending`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Async request id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// Poll this URL for the processing status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_url: Option<String>,
    /// Fetch the final transaction from this URL when completed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_url: Option<String>,
}

/// Status of an async processing request.
#[derive(Debug, Clone, Deserialize)]
pub struct AsyncStatus {
    /// Status, e.g. `pending` or `completed`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Async request id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// Fetch the final transaction from this URL when completed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_url: Option<String>,
}

/// Transaction status as returned by the tracking-id status query.
#[derive(Debug, Clone, Deserialize)]
pub struct TrackingIdStatus {
    /// Unique transaction id.
    pub uid: String,
    /// Status, e.g. `approved`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_status: Option<String>,
    /// Transaction result code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_code: Option<String>,
    /// CVC verification flags.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cvc_verification: Option<TrackingCvcVerification>,
    /// Customer metadata of the transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<Customer>,
    /// Billing address of the transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<BillingAddress>,
}

// ── APM: transaction status query ─────────────────────────────────────────────

/// Envelope for `GET /beyag/transactions/tracking_id/{tracking_id}`.
#[derive(Debug, Clone, Deserialize)]
pub(crate) struct TransactionListEnvelope {
    pub transactions: Vec<Transaction>,
}

// ── APM: payout ───────────────────────────────────────────────────────────────

/// APM payout request (`POST /beyag/transactions/payouts`).
///
/// `method` must carry the method-specific parameters, e.g. `{"type": "ad_payments"}`.
#[derive(Debug, Clone, Serialize)]
pub struct ApmPayoutRequest {
    /// Amount in minor units.
    pub amount: i64,
    /// ISO 4217 currency code.
    pub currency: String,
    /// Free-form description (max 255 chars).
    pub description: String,
    /// `true` to run in test mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test: Option<bool>,
    /// Merchant tracking id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_id: Option<String>,
    /// Customer IP.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    /// ISO 639-1 language code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// Webhook notification URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_url: Option<String>,
    /// URL for successful payment verification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_url: Option<String>,
    /// URL the customer is returned to after the transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
    /// Customer metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<Customer>,
    /// Method-specific parameters, e.g. `{"type": "ad_payments"}`.
    pub method: serde_json::Value,
    /// Additional method-specific data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<serde_json::Value>,
    /// Up to 3 custom fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<CustomFields>,
}

/// APM payout response (`POST /beyag/transactions/payouts`).
#[derive(Debug, Clone, Deserialize)]
pub struct ApmPayoutResponse {
    /// Unique transaction id.
    pub uid: Option<String>,
    /// Transaction type, e.g. `payout`.
    #[serde(rename = "type")]
    pub tx_type: Option<String>,
    /// Transaction status, e.g. `successful`.
    pub status: Option<String>,
    /// Amount in minor units.
    pub amount: Option<i64>,
    /// ISO 4217 currency code.
    pub currency: Option<String>,
    /// Free-form description.
    pub description: Option<String>,
    /// Creation timestamp (ISO 8601).
    pub created_at: Option<String>,
    /// Last update timestamp (ISO 8601).
    pub updated_at: Option<String>,
    /// Payout method type.
    pub method_type: Option<String>,
    /// Receipt URL.
    pub receipt_url: Option<String>,
    /// Human-readable message.
    pub message: Option<String>,
    /// Merchant tracking id.
    pub tracking_id: Option<String>,
    /// `true` if the transaction ran in test mode.
    pub test: Option<bool>,
    /// ISO 639-1 language code.
    pub language: Option<String>,
    /// Payment timestamp (ISO 8601).
    pub paid_at: Option<String>,
    /// Payout details (`status`, `gateway_id`, `ref_id`, `bank_code`, `rrn`, `message`).
    pub payout: Option<serde_json::Value>,
    /// Billing address.
    pub billing_address: Option<serde_json::Value>,
    /// Customer metadata.
    pub customer: Option<serde_json::Value>,
    /// Smart-routing verification result.
    pub smart_routing_verification: Option<serde_json::Value>,
    /// Additional data.
    pub additional_data: Option<serde_json::Value>,
    /// Up to 3 custom fields.
    pub custom_fields: Option<CustomFields>,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct ApmPayoutEnvelope {
    pub transaction: ApmPayoutResponse,
}

// ── APM: proof of payment ─────────────────────────────────────────────────────

/// A single document submitted as payment proof.
#[derive(Debug, Clone, Serialize)]
pub struct ProofDocument {
    /// MIME type; one of `application/pdf`, `image/png`, `image/jpeg`.
    pub content_type: String,
    /// File name including extension.
    pub file_name: String,
    /// File size in bytes.
    pub file_size: i64,
    /// Base64-encoded file content (PDF documents must be unscanned originals).
    pub content: String,
    /// SHA-256 checksum of the document.
    pub checksum: String,
}

/// Proof-of-payment request (`POST /beyag/transactions/{uid}/proof`).
#[derive(Debug, Clone, Serialize)]
pub struct ProofRequest {
    /// Skip duplicate-check validation (default `false`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_duplicate_check: Option<bool>,
    /// Amount of the transaction being proven (minor units).
    pub amount: i64,
    /// ISO 4217 currency code.
    pub currency: String,
    /// Gateway transaction reference of the proof.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_reference: Option<String>,
    /// The document itself.
    pub document: ProofDocument,
}

/// Response of a proof submission (`POST /beyag/transactions/{uid}/proof`).
#[derive(Debug, Clone, Deserialize)]
pub struct ProofResponse {
    /// Unique transaction id of the proof.
    pub uid: Option<String>,
    /// Uid of the original transaction.
    pub parent_uid: Option<String>,
    /// Transaction type, e.g. `proof`.
    #[serde(rename = "type")]
    pub tx_type: Option<String>,
    /// Transaction status, e.g. `successful`.
    pub status: Option<String>,
    /// Human-readable message.
    pub message: Option<String>,
    /// Amount in minor units.
    pub amount: Option<i64>,
    /// ISO 4217 currency code.
    pub currency: Option<String>,
    /// Proof details (`message`, `ref_id`, `gateway_id`, `status`).
    pub proof: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct ProofEnvelope {
    pub transaction: ProofResponse,
}

/// Masterpass login request, sent without a `request` wrapper.
#[derive(Debug, Clone, Serialize)]
pub struct MasterpassLoginRequest {
    /// Customer phone number.
    pub phone: String,
    /// Customer device fingerprint.
    pub fingerprint: String,
    /// Date of the last phone verification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_check_date: Option<String>,
    /// Masterpass channel identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<i64>,
    /// Set to `true` to run in test mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test: Option<bool>,
}

/// Response of Masterpass login.
#[derive(Debug, Clone, Deserialize)]
pub struct MasterpassLoginResponse {
    /// Operation status, e.g. `successful` or `failed`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Error message on failure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// Masterpass error code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<i64>,
    /// Whether a one-time password is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_otp_required: Option<bool>,
    /// Session for subsequent Masterpass requests.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<String>,
    /// Masterpass user status code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_status: Option<i64>,
}

/// Request to list Masterpass cards, sent without a `request` wrapper.
#[derive(Debug, Clone, Serialize)]
pub struct MasterpassGetCardsRequest {
    /// Session returned by Masterpass login.
    pub session: String,
    /// Set to `true` to run in test mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test: Option<bool>,
}

/// Card entry returned by Masterpass `get_cards`.
#[derive(Debug, Clone, Deserialize)]
pub struct MasterpassCardEntry {
    /// Cardholder name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_holder: Option<String>,
    /// Card registration date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    /// Card expiration date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_date: Option<String>,
    /// Masked card number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pan_mask: Option<String>,
    /// Card display name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_name: Option<String>,
    /// Masterpass card token.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    /// Masterpass card status code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_status: Option<i64>,
    /// Whether the card supports recurring payments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_recurring: Option<bool>,
    /// First additional card comment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment1: Option<String>,
    /// Second additional card comment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment2: Option<String>,
    /// Third additional card comment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment3: Option<String>,
}

/// Response containing the customer's Masterpass cards.
#[derive(Debug, Clone, Deserialize)]
pub struct MasterpassGetCardsResponse {
    /// Operation status, e.g. `successful` or `failed`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Error message on failure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// Masterpass error code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<i64>,
    /// Cards registered with Masterpass.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_list: Option<Vec<MasterpassCardEntry>>,
}

/// Request to retrieve a Masterpass card, sent without a `request` wrapper.
#[derive(Debug, Clone, Serialize)]
pub struct MasterpassGetCardRequest {
    /// Masterpass card token.
    pub token: String,
    /// Payment amount in minor units.
    pub amount: i64,
    /// ISO 4217 currency code.
    pub currency: String,
    /// Session returned by Masterpass login.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<String>,
    /// Set to `true` to run in test mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test: Option<bool>,
}

/// Request to retrieve a saved Masterpass card, without a `request` wrapper.
#[derive(Debug, Clone, Serialize)]
pub struct MasterpassGetSavedCardRequest {
    /// bePaid credit card token.
    pub credit_card_token: String,
    /// Payment amount in minor units.
    pub amount: i64,
    /// ISO 4217 currency code.
    pub currency: String,
    /// Session returned by Masterpass login.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<String>,
    /// Set to `true` to run in test mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test: Option<bool>,
}

/// Shared response of Masterpass `get_card` and `get_saved_card`.
#[derive(Debug, Clone, Deserialize)]
pub struct MasterpassGetCardResponse {
    /// Operation status, e.g. `successful` or `failed`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Error message on failure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// Masterpass error code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<i64>,
    /// Human-readable message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Masked card details and bePaid payment token.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_card: Option<CreditCardInfo>,
    /// Recommended verification code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation: Option<i64>,
    /// Required verification code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<i64>,
}

/// Request to delete a Masterpass card, sent without a `request` wrapper.
#[derive(Debug, Clone, Serialize)]
pub struct MasterpassDeleteCardRequest {
    /// Session returned by Masterpass login.
    pub session: String,
    /// Masterpass card token to delete.
    pub token: String,
    /// Set to `true` to run in test mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test: Option<bool>,
}

/// Response of Masterpass card deletion.
#[derive(Debug, Clone, Deserialize)]
pub struct MasterpassDeleteCardResponse {
    /// Operation status, e.g. `successful` or `failed`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Error message on failure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// Masterpass error code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<i64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn custom_fields_roundtrip() {
        let cf = CustomFields {
            custom_field_1: Some(CustomField {
                label: Some("Email".into()),
                value: Some("john@example.com".into()),
                placeholder: None,
                visible: Some(true),
                required: Some(true),
                read_only: None,
            }),
            custom_field_2: Some(CustomField {
                label: Some("Agreement number".into()),
                value: Some("12349".into()),
                placeholder: None,
                visible: Some(true),
                required: None,
                read_only: Some(true),
            }),
            custom_field_3: None,
        };
        let json = serde_json::to_value(&cf).unwrap();
        assert_eq!(
            json["custom_field_1"],
            serde_json::json!({
                "label": "Email",
                "value": "john@example.com",
                "visible": true,
                "required": true,
            })
        );
        let back: CustomFields = serde_json::from_value(json).unwrap();
        assert_eq!(
            back.custom_field_2.as_ref().unwrap().value.as_deref(),
            Some("12349")
        );
        assert_eq!(back.custom_field_3, None);
    }
}
