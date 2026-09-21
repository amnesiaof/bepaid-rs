# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.9.0] - 2026-09-21

### Added
- Split payment v2 recipients: `AdditionalData.split` now accepts `Vec<SplitRecipient>`
  (amount, tax_id, company_name, bank_account, bank_bic, description, legal_address,
  mailing_address, country, city, postal_code, contact_email, contact_phone, memo) and
  `AdditionalData.smart_routing_options.allow_halva`.
- Fiscalization on saved-card charges (`ChargeRequest.fiscalization`), refunds
  (`RefundRequest.fiscalization: true`) and payment-token creation
  (`CheckoutRequest.fiscalization`); transaction responses now decode the
  `fiscalization` block with `FiscalizationInfo`, `FiscalizationReceipt` and
  `FiscalizationReceiptInfo`.
- Paginated merchant reports: `ReportListV3Params`/`ReportListV3Request` and
  `get_reports_v3` (`POST /api/reports` with `X-Api-Version: 3`, `from`/`to`,
  `starting_after`/`ending_before`/`manual_correction_from`/`manual_correction_to`);
  `ReportListResponse` gains `has_more`, `first_object_id` and `last_object_id`.
- Cascading: `AdditionalData.excluded_gateways` (renamed to `Vec<i64>` on
  `ChargeAdditionalData`) for retries with `duplicate_check: false`.

### Changed
- **Breaking:** new struct fields on `AdditionalData` (`split`,
  `smart_routing_options`, `excluded_gateways`), `ChargeRequest.fiscalization`,
  `RefundRequest.fiscalization` and `CheckoutRequest.fiscalization` — existing
  literals must supply them (`None` when unused); `ChargeAdditionalData.excluded_gateways`
  type is now `Vec<i64>` (was `Vec<String>`).

## [0.8.0] - 2026-09-17

### Added
- Card gateway account balance query: `get_card_balance` (`POST /balance`,
  X-API-Version 2) with new `CardBalanceRequest`, `CardBalanceResponse` and
  `CardBalanceResult` types (camelCase `gatewayId`/`bankInfo` decoded).
- Async processing mode: `create_payment_async`, `create_authorization_async`
  (`POST /async/transactions/payments|authorizations`, API v3), `get_async_status`
  and `get_async_result` (absolute `status_url`/`response_url` polling), with new
  `AsyncAck` and `AsyncStatus` types.
- `ApiError.code` and `ApiError.friendly_message` extracted from error bodies
  alongside the existing `error_code`/`message`.
- Request fields: `PaymentRequest.expired_at`/`dynamic_billing_descriptor`,
  `AuthorizationRequest.language`/`notification_url`/`return_url`/`expired_at`/
  `dynamic_billing_descriptor`, `CheckoutRequest.dynamic_billing_descriptor`/
  `travel`, `CheckoutSettings.style`/`widget_version`/`require`/`customer`,
  `ApmPaymentRequest.iframe`/`verification_url`, `Customer.id`/`id_number`,
  and eleven update fields on `ProductUpdateRequest` (name, description,
  currency, visible_fields, test, immortal, expired_at, return_url, shop_id,
  language, transaction_type). **Breaking:** existing literals must supply the
  new fields (`None` when unused).
- `CheckoutResponse.extra` retains additional echo fields from checkout
  creation; `CheckoutStatus` decodes `merchant`, `version` (int or string),
  `card_info`, `job_id`, `attempts`, `iframe`, `dynamic_billing_descriptor`
  and `travel`.
- `check_mts_service_v2(phone, test)` uses the MTS Money API v2 route; the
  existing widget-v3 method is unchanged. `CheckServiceResponse.error_code`
  preserves string or numeric provider codes.
- `test_qiwi_terminal_payment(amount, currency, account)` sends a wrapped test
  request and returns raw JSON, or `{}` for an empty successful response.
- `Customer` gains optional `gender`, `street` and `state` fields. **Breaking:**
  existing literals must supply these fields (`None` when unused).
- `create_erip_payment` uses `/beyag/payments`; generic `create_apm_payment`
  keeps `/beyag/transactions/payments`. `get_apm_refund` queries `/beyag/refunds/{uid}`.
- `get_erip_pay_list` sends a flat `EripPayListRequest` and returns raw JSON
  arrays or objects. Komplat authorization reuses `AdditionalData.extra`.
- ERIP payment/refund response metadata and raw ERIP data; webhook ERIP/refund
  fields and localized `extra` retain External method-specific payloads.
- Checkout `PaymentMethod.extra` and `CheckoutAdditionalData.extra` accept
  method-specific objects alongside existing typed fields. `Customer` gains
  middle name, country, city, zip and address; `BillingAddress` gains middle name.
  **Breaking:** affected struct literals must supply the new optional fields
  (`None` when unused).
- `verify_visa_alias` sends a flat, authenticated API v3 request to
  `/services/visa-alias/verify-phone`; typed responses reuse root-level
  `CreditCardInfo` and decode camelCase `service_info` fields.
- P2P request metadata, customer and billing addresses; additional-data referer,
  receipt lines and contracts. **Breaking:** `P2pRequest` and `P2pAdditionalData`
  literals must supply the new optional fields (`None` when unused).
- Masterpass service support: `masterpass_login`, `masterpass_get_cards`,
  `masterpass_get_card`, `masterpass_get_saved_card`, and `masterpass_delete_card`
  via `POST /masterpass/*`, with new request/response types and
  `AdditionalData.masterpass` for save-card/payments. **Breaking:** existing
  `AdditionalData` struct literals must supply `masterpass` (`None` when unused).

### Changed
- **Breaking:** `create_payment` and `create_authorization` return the full
  `Transaction`; removed unused `PaymentResponse` and `AuthorizationResponse`.
- **Breaking:** `BepaidError::Api` now holds `Box<ApiError>` to keep the error
  variant small after adding fields. Field access through the box is unchanged.

### Fixed
- Async polling validates the configured gateway origin (scheme, host and effective
  port) and rejects malformed URLs, foreign origins and userinfo before HTTP.
  Polling redirects are disabled without changing other endpoints' redirect behavior.
- Universal confirmation accepts the documented empty `{"request": {}}` body
  as well as skip-only requests, preserving BelVEB and SberPay modes.
- README examples include all required struct fields and current method arguments.
- `create_checkout` now sends `X-API-Version: 2` as the Checkout API requires.
- Universal `confirm_apm_payment` reference confirmations send a wrapped
  `{"request": {...}}` body, and `skip_duplicate_check` alone (without
  `transaction_reference`) is accepted; confirm_type and phone modes are
  unchanged.
- `create_apm_payment` sends `request.method`; `create_erip_payment` continues
  sending `request.payment_method`, preserving the public request field.
- `ApmPaymentResponse.extra` and `Transaction.extra` retain method-specific
  response JSON, including QR/bank data, string crypto amounts and object/string
  forms. **Breaking:** literals must supply the new `extra` field.
- `confirm_apm_payment` supports wrapped BelVEB confirm/cancel requests with a
  `transaction` response and flat SberPay phone requests with a `response` envelope.
  Reference confirmations use a wrapped body and retain the response envelope;
  mixed modes, invalid confirmation types and misplaced duplicate flags
  are rejected locally. **Breaking:** `ApmConfirmRequest.transaction_reference`
  is optional; literals must also supply `phone` (`None` for legacy callers).
- `apm_full_refund` rejects missing amounts locally with `BepaidError::InvalidRequest`
  before sending a request; its `Option<i64>` signature and valid callers are preserved.
  Full ERIP refunds require the explicit full amount, not `None`.
- Flat API error bodies (e.g. Visa Alias, Verify P2P) preserve `error_code`,
  `message` and `errors`; legacy `response`/`error` shapes unchanged. New
  `ApiError.error_code`.
- `P2pResponse` and `VerifyP2pResponse` now decode updated-at/paid-at, language,
  payment-method type, message, status code, id, addresses, raw customer and
  additional data, and validation `errors`.
- `CreditCardRaw` supports token-only payments, omitting absent PAN, CVC, holder and expiry fields while preserving raw-card serialization. **Breaking:** `number`, `verification_value`, `holder`, `exp_month` and `exp_year` now use `Option`; wrap raw-card values in `Some`.
- `AuthorizationRequest.additional_data` accepts `Option<AdditionalData>`, including `contract`. **Breaking:** existing struct literals must supply `additional_data` (`None` when unused).

## [0.7.0] - 2026-09-17

### Added
- `AuthorizationRequest` now has a `verification_url` field to enable transaction verification.
- `webhook::parse_checkout_webhook()` to parse flat payment-widget webhook payloads (e.g. token-expiry notices) into `CheckoutStatus`.
- `CheckoutStatus` now deserializes `customer`, `finished`, `expired`, `shop`, `test`, `status`, `message`, `payment_method`.
- `CheckoutCustomerFields.hidden` array.
- New `create_tokenization()` method (H2H): tokenize a card via the
  `POST /transactions/tokenizations` endpoint (3-D Secure supported). New
  `TokenizationRequest`, `ThreeDSecureAdvanced`, `TokenizationInfo` types.
  `CreditCardRaw` now has `skip_three_d_secure_verification` and
  `force_three_d_secure_verification` fields. `Transaction` now deserializes
  the `tokenization` object.
- `PaymentRequest` and `AuthorizationRequest` now have a `duplicate_check`
  field (set `false` to allow a repeat request within 30 seconds instead of
  getting a `Duplicate transaction` error).
- `Transaction` now deserializes the full v3 response format: `psp_settled_at`,
  `parent_uid`, `reason`, `errors`, `three_d_secure_verification`.
  `CaptureResponse` gains `psp_settled_at`.

### Changed
- Host-to-host methods (`create_payment`, `create_authorization`, `capture`, `void`, `refund`, `create_payout`, `charge_saved_card`, `create_apm_payment`, `apm_refund`, `apm_full_refund`, `confirm_apm_payment`, `apm_payout`, `apm_proof`, `checkup`) now accept an optional `request_id: Option<&str>` argument that sets the `RequestID` header to make requests idempotent (per bePaid idempotent requests docs). **Breaking:** these method signatures changed.
- `Transaction.status_code` and `CaptureResponse.status_code` are now integers (`Option<i64>`) to match the API.

## [0.6.9] - 2026-09-16

### Added
- `CheckoutSettings` fields: `agreed`, `agreement_toggle`, `customer_fields`, `credit_card_fields`, `verification_url`, `auto_return`, `card_notification_url`, `save_card_toggle`, `another_card_toggle`.
- `PaymentMethod` fields: `excluded_types`, `excluded_brands`.
- `CheckoutOrder.expired_at`.
- `Customer` fields: `external_id`, `taxpayer_id`.

## [0.6.8] - 2026-09-16

### Added

- `confirm_type` field to `ApmConfirmRequest` for BelВеб-кредит
  confirm/cancel.

## [0.6.7] - 2026-09-16

### Added

- `custom_fields` support (bePaid API change of 2026-04-22): new
  `CustomField` and `CustomFields` structs. `custom_fields` field added to
  `PaymentRequest`, `AuthorizationRequest`, `CheckoutOrder`,
  `ApmPaymentRequest`, `ApmPayoutRequest`, `PayoutRequest` and the response
  structs `Transaction`, `ApmPaymentResponse`, `ApmPayoutResponse`,
  `PayoutResponse`.

## [0.6.6] - 2026-09-15

### Fixed

- `RefundResponse.uid` is now required (`String`, not `Option<String>`).
- `ApmPaymentRequest.order_id` changed from `serde_json::Value` to
  `Option<String>` (docs: string "12-digit order number").
- `validate_apple_pay` now sends `X-API-Version: 2` as required by docs.

### Changed

- Centralized `RequestEnvelope<T>` into `client.rs` — removed 4 duplicate
  definitions from direct/gateway/p2p/tokens modules.
- `Subscription.plan` changed from `Option<serde_json::Value>` to
  `Option<PlanItem>`. New `SubscriptionLastTransaction` struct for
  `Subscription.last_transaction` (uid/status/message/created_at).

## [0.6.5] - 2026-09-15

### Added

- `X-Api-Version: 3` on all gateway API calls (payments, authorizations,
  captures, voids, refunds, status, tokens, payouts, P2P, checkups).
- SberPay push constructor: `ApmPaymentRequest::sberpay` with optional
  `phone` on the customer.
- APM constructors: `alfaclick`, `webpay`, `rccard`, `byncard`, `halva`.
- MTS Money service check: `check_mts_service` (`API v3`).
- P2P restrictions check: `verify_p2p` (`POST /p2p-restrictions`).
- ERIP payment endpoints: `get_erip_payment`, `get_erip_payment_by_order_id`,
  `delete_erip_payment` (only `pending`/`permanent` requirements can be
  deleted).
- `Transaction.order_id` and `Transaction.erip` fields.

### Changed

- `charge_saved_card` now sends `X-Api-Version: 3`.

## [0.6.4] - 2026-09-14

### Added

- APM transaction status queries: `get_apm_transaction` (by uid) and
  `get_apm_transactions_by_tracking_id`.
- APM payouts: `apm_payout` with `ApmPayoutRequest` / `ApmPayoutResponse`.
- APM payment proof: `apm_proof` with `ProofRequest` / `ProofDocument` /
  `ProofResponse`.
- Card checkup (risk management rules check): `checkup` with `CheckupRequest`.

## [0.6.3] - 2026-09-14

### Added

- ERIP and alternative payment method (APM) constructors on
  `ApmPaymentRequest`: `erip`, `mts_money`, `krok`, `qiwi_terminal`.
- `Customer.phone` and `EripDevice`.

## [0.6.2] - 2026-09-14

### Added

- Client-side encrypted card data (CSE): `PaymentRequest.encrypted_data`.
- Fiscalization support (KZT): `PaymentRequest.fiscalization` with positions
  and taxes.

## [0.6.1] - 2026-09-14

### Changed

- `PaymentRequest` gains `verification_url` and `return_url` for 3-D Secure
  handling in server-to-server (H2H) payments.

### Added

- Runnable example: `examples/h2h.rs` (server-to-server card payment).

## [0.6.0] - 2026-09-14

### Added

- Split payments: `create_split_payment` (Direct API).
- Pay-by-link products: `create_product`, `list_products`, `get_product`,
  `update_product`.
- Payment token for the payment page/widget: `create_payment_token`.
- Webhook `Content-Signature` verification: `verify_webhook_signature`
  (RSA-SHA256 over the raw body).
- Plan payment link: `get_plan_payment_link`.
- Saved-card charges: `charge_saved_card` (Gateway API).
- Recipient tokenization for payouts: `tokenize_recipient_card`.
- Apple Pay payment: `apple_pay_payment` (Checkout API).
- APM currency query: `get_currencies` (Direct API).
- Transaction status by tracking id: `get_transaction_by_tracking_id`.

## [0.5.0] - 2026-09-13

### Added

- Payout transactions: `create_payout` (Gateway API, `X-API-Version: 3`).
- APM balance query: `get_balance` (Direct API).
- Merchant reports: `get_reports`, `get_report_count`, `get_channel_balances`
  (Merchant API with configurable `merchant_url` and mixed `X-API-Version: 2/3`).
- `DEFAULT_MERCHANT_URL` constant and `merchant_url` field on `BepaidClient`.
- Runnable examples: `examples/payment.rs` (card flow with 3-D Secure),
  `examples/subscriptions.rs` (customer → plan → subscription).

## [0.4.0] - 2026-09-13

### Added

- Full rustdoc coverage: `#![warn(missing_docs)]` and doc comments on all
  public items, crate-level docs and doctests.

## [0.3.0] - 2026-09-13

### Changed

- `reqwest` 0.12 → 0.13 (default TLS backend is now rustls with
  `rustls-platform-verifier`).
- `base64` 0.22 → 0.23.

## [0.2.0] - 2026-09-13

### Added

- Subscriptions service: `create_customer`, `get_customer`, `list_customers`,
  `create_plan`, `get_plan`, `list_plans`, `create_subscription`,
  `get_subscription`, `cancel_subscription`.
- P2P transfers: `create_p2p`.
- APM payment confirmation: `confirm_apm_payment`.
- Subscription webhook parsing: `parse_subscription_webhook`.

## [0.1.0] - 2026-09-13

### Added

- Async client (`BepaidClient`) for the bePaid API (bepaid.by) with HTTP Basic
  authentication and optional `X-API-Version: 3` header.
- Gateway operations: `create_payment`, `create_authorization`, `capture`,
  `void`, `refund`, `get_transaction`.
- Token API: `create_token`.
- Checkout API: `create_checkout`, `get_checkout_status`,
  `validate_apple_pay`.
- Direct (APM) API: `create_apm_payment`, `apm_refund`, `apm_full_refund`.
- Webhooks: `verify_webhook_auth`, `parse_webhook`.

[Unreleased]: https://github.com/amnesiaof/bepaid-rs/compare/v0.9.0...HEAD
[0.9.0]: https://github.com/amnesiaof/bepaid-rs/compare/v0.8.0...v0.9.0
[0.8.0]: https://github.com/amnesiaof/bepaid-rs/compare/v0.7.0...v0.8.0
[0.7.0]: https://github.com/amnesiaof/bepaid-rs/compare/v0.6.9...v0.7.0
[0.6.9]: https://github.com/amnesiaof/bepaid-rs/compare/v0.6.8...v0.6.9
[0.6.8]: https://github.com/amnesiaof/bepaid-rs/compare/v0.6.7...v0.6.8
[0.6.7]: https://github.com/amnesiaof/bepaid-rs/compare/v0.6.6...v0.6.7
[0.6.6]: https://github.com/amnesiaof/bepaid-rs/compare/v0.6.5...v0.6.6
[0.6.5]: https://github.com/amnesiaof/bepaid-rs/compare/v0.6.4...v0.6.5
[0.6.4]: https://github.com/amnesiaof/bepaid-rs/compare/v0.6.3...v0.6.4
[0.6.3]: https://github.com/amnesiaof/bepaid-rs/compare/v0.6.2...v0.6.3
[0.6.2]: https://github.com/amnesiaof/bepaid-rs/compare/v0.6.1...v0.6.2
[0.6.1]: https://github.com/amnesiaof/bepaid-rs/compare/v0.6.0...v0.6.1
[0.6.0]: https://github.com/amnesiaof/bepaid-rs/compare/v0.5.0...v0.6.0
[0.5.0]: https://github.com/amnesiaof/bepaid-rs/compare/v0.4.0...v0.5.0
[0.4.0]: https://github.com/amnesiaof/bepaid-rs/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/amnesiaof/bepaid-rs/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/amnesiaof/bepaid-rs/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/amnesiaof/bepaid-rs/releases/tag/v0.1.0
