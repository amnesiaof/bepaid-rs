use bepaid::{
    BepaidClient, BepaidError,
    types::{
        ApmConfirmRequest, ApmPaymentRequest, ApmPayoutRequest, AuthorizationRequest,
        BalanceRequest, CancelSubscriptionRequest, CaptureRequest, ChargeCreditCard, ChargeRequest,
        CheckoutRequest, CheckupRequest, CreateTokenRequest, CurrencyQueryRequest, CustomerRecord,
        EripDevice, Fiscalization, FiscalizationPosition, FiscalizationTax, P2pRequest,
        PaymentRequest, PayoutCreditCard, PayoutRequest, ProductCreateRequest,
        ProductUpdateRequest, ProofDocument, ProofRequest, RecipientTokenizationRequest,
        RefundRequest, ReportListRequest, ReportParams, SubscriptionCreateRequest, VoidRequest,
    },
    webhook::{
        parse_subscription_webhook, parse_webhook, verify_webhook_auth, verify_webhook_signature,
    },
};
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{body_partial_json, method, path},
};

const SHOP_ID: &str = "363";
const SECRET: &str = "45454e083434aa37rfdfd";
const AUTH: &str = "Basic MzYzOjQ1NDU0ZTA4MzQzNGFhMzdyZmRmZA==";

fn client(server: &MockServer) -> BepaidClient {
    let base = server.uri();
    BepaidClient::with_urls(SHOP_ID, SECRET, &base, &base, &base, &base)
}

#[tokio::test]
async fn create_payment_happy_path() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/transactions/payments"))
        .and(wiremock::matchers::header("authorization", AUTH))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "transaction": {"tracking_id": "tracking_id_000", "uid": "some_uid"}
        })))
        .expect(1)
        .mount(&server)
        .await;

    let c = client(&server);
    let resp = c
        .create_payment(PaymentRequest {
            amount: "700".into(),
            currency: "USD".into(),
            test: true,
            description: "Test transaction".into(),
            tracking_id: "tracking_id_000".into(),
            language: None,
            notification_url: None,
            verification_url: None,
            return_url: None,
            billing_address: None,
            credit_card: None,
            customer: None,
            additional_data: None,
            encrypted_data: None,
            fiscalization: None,
            custom_fields: None,
        })
        .await;

    let t = resp.expect("payment should succeed");
    assert_eq!(t.uid, "some_uid");
    assert_eq!(t.tracking_id.as_deref(), Some("tracking_id_000"));
}

#[tokio::test]
async fn create_payment_serializes_h2h_fields() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/transactions/payments"))
        .and(body_partial_json(serde_json::json!({
            "request": {
                "return_url": "https://example.com/return",
                "verification_url": "https://example.com/verify",
            }
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "transaction": {"tracking_id": "tid", "uid": "uid1"}
        })))
        .expect(1)
        .mount(&server)
        .await;

    let c = client(&server);
    c.create_payment(PaymentRequest {
        amount: "700".into(),
        currency: "USD".into(),
        test: true,
        description: "Test transaction".into(),
        tracking_id: "tid".into(),
        language: None,
        notification_url: None,
        verification_url: Some("https://example.com/verify".into()),
        return_url: Some("https://example.com/return".into()),
        billing_address: None,
        credit_card: None,
        customer: None,
        additional_data: None,
        encrypted_data: None,
        fiscalization: None,
        custom_fields: None,
    })
    .await
    .expect("payment should succeed");
}

#[tokio::test]
async fn create_payment_serializes_fiscalization_and_encrypted_data() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/transactions/payments"))
        .and(body_partial_json(serde_json::json!({
            "request": {
                "encrypted_data": "jwe-blob",
                "fiscalization": {
                    "external_id": "fisc-1",
                    "positions": [{
                        "name": "Product",
                        "type": "service",
                        "amount": 100,
                        "quantity": 1.0,
                        "measure_unit_code": 796,
                        "description": "Desc",
                        "untaxed": false,
                        "nomenclature_code": "code-1",
                        "taxes": [{"id": "vat-12", "percent": "12", "type": "vat", "inclusive": true}]
                    }]
                }
            }
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "transaction": {"tracking_id": "tid", "uid": "uid1"}
        })))
        .expect(1)
        .mount(&server)
        .await;

    let c = client(&server);
    c.create_payment(PaymentRequest {
        amount: "700".into(),
        currency: "USD".into(),
        test: true,
        description: "Test transaction".into(),
        tracking_id: "tid".into(),
        language: None,
        notification_url: None,
        verification_url: None,
        return_url: None,
        billing_address: None,
        credit_card: None,
        customer: None,
        additional_data: None,
        encrypted_data: Some("jwe-blob".into()),
        fiscalization: Some(Fiscalization {
            external_id: "fisc-1".into(),
            positions: vec![FiscalizationPosition {
                name: "Product".into(),
                position_type: "service".into(),
                amount: 100,
                quantity: 1.0,
                measure_unit_code: 796,
                description: Some("Desc".into()),
                untaxed: false,
                nomenclature_code: Some("code-1".into()),
                taxes: Some(vec![FiscalizationTax {
                    id: "vat-12".into(),
                    percent: "12".into(),
                    tax_type: "vat".into(),
                    description: None,
                    inclusive: true,
                }]),
            }],
        }),
        custom_fields: None,
    })
    .await
    .expect("payment should succeed");
}

#[tokio::test]
async fn create_payment_rejects_invalid_amount() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/transactions/payments"))
        .respond_with(ResponseTemplate::new(400).set_body_json(serde_json::json!({
            "response": {
                "message": "Validation failed",
                "errors": {"amount": ["can't be blank"]}
            }
        })))
        .mount(&server)
        .await;

    let c = client(&server);
    let err = c
        .create_payment(PaymentRequest {
            amount: "".into(),
            currency: "USD".into(),
            test: true,
            description: "d".into(),
            tracking_id: "t".into(),
            language: None,
            notification_url: None,
            verification_url: None,
            return_url: None,
            billing_address: None,
            credit_card: None,
            customer: None,
            additional_data: None,
            encrypted_data: None,
            fiscalization: None,
            custom_fields: None,
        })
        .await
        .expect_err("should error");

    match err {
        BepaidError::Api(e) => {
            assert_eq!(e.status, 400);
            assert_eq!(e.message, "Validation failed");
            assert!(e.errors.is_some());
        }
        other => panic!("expected Api, got {other:?}"),
    }
}

#[tokio::test]
async fn create_authorization_returns_redirect() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/transactions/authorizations"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "transaction": {
                "uid": "b6c446e4-b8a8-496f-bbc8-497d34644c7b",
                "status": "incomplete",
                "redirect_url": "https://gateway.bepaid.by/process/b6c446e4",
                "three_d_secure_verification": {
                    "status": "incomplete",
                    "message": "Authentication Available",
                    "pa_res_url": "https://gateway.bepaid.by/process/b6c446e4"
                }
            }
        })))
        .expect(1)
        .mount(&server)
        .await;

    let c = client(&server);
    let t = c
        .create_authorization(AuthorizationRequest {
            amount: 100,
            currency: "USD".into(),
            description: "Test".into(),
            payment_method_type: None,
            tracking_id: "x".into(),
            test: Some(true),
            credit_card: None,
            customer: None,
            billing_address: None,
            custom_fields: None,
        })
        .await
        .expect("auth should succeed");

    assert_eq!(t.uid, "b6c446e4-b8a8-496f-bbc8-497d34644c7b");
    assert_eq!(t.status.as_deref(), Some("incomplete"));
    assert!(
        t.redirect_url
            .as_deref()
            .unwrap()
            .ends_with("/process/b6c446e4")
    );
}

#[tokio::test]
async fn capture_happy_path() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/transactions/captures"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "transaction": {
                "uid": "52bfc29a-2c2f-408a-a7d5-bd8e84a320a9",
                "status": "successful",
                "type": "capture",
                "parent_uid": "4298aabd",
                "amount": 460,
                "currency": "EUR",
                "code": "S.0000"
            }
        })))
        .expect(1)
        .mount(&server)
        .await;

    let c = client(&server);
    let t = c
        .capture(CaptureRequest {
            parent_uid: "4298aabd".into(),
            amount: 460,
            tracking_id: None,
            additional_data: None,
        })
        .await
        .expect("capture should succeed");

    assert_eq!(t.status.as_deref(), Some("successful"));
    assert_eq!(t.tx_type.as_deref(), Some("capture"));
}

#[tokio::test]
async fn void_happy_path() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/transactions/voids"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "transaction": {
                "uid": "22e47158",
                "status": "successful",
                "type": "void",
                "message": "Successfully processed",
                "tracking_id": "tracking_id_1"
            }
        })))
        .expect(1)
        .mount(&server)
        .await;

    let c = client(&server);
    let t = c
        .void(VoidRequest {
            parent_uid: "auth-uid".into(),
            amount: 50,
            tracking_id: Some("tracking_id_1".into()),
            additional_data: None,
        })
        .await
        .expect("void should succeed");

    assert_eq!(t.tx_type.as_deref(), Some("void"));
    assert_eq!(t.status.as_deref(), Some("successful"));
}

#[tokio::test]
async fn refund_happy_path() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/transactions/refunds"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "transaction": {
                "uid": "2-310b0da80b",
                "parent_uid": "1-310b0da80b",
                "type": "refund",
                "status": "successful",
                "message": "The operation was successfully processed"
            }
        })))
        .expect(1)
        .mount(&server)
        .await;

    let c = client(&server);
    let t = c
        .refund(RefundRequest {
            parent_uid: "1-310b0da80b".into(),
            amount: 50,
            reason: "Client request".into(),
            tracking_id: None,
            additional_data: None,
        })
        .await
        .expect("refund should succeed");

    assert_eq!(t.tx_type.as_deref(), Some("refund"));
    assert_eq!(t.parent_uid.as_deref(), Some("1-310b0da80b"));
}

#[tokio::test]
async fn get_transaction_parses_full_object() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/transactions/2f4d67ff"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "transaction": {
                "uid": "2f4d67ff",
                "status": "successful",
                "amount": 104,
                "currency": "EUR",
                "type": "payment",
                "payment_method_type": "credit_card",
                "credit_card": {
                    "holder": "John Doe",
                    "stamp": "bb58cad9c1204ca2287b3e1006cc1a2c0fb8f062dde9e5232c8be5498bd0e62a",
                    "brand": "visa",
                    "last_4": "1097",
                    "first_1": "4",
                    "exp_month": 10,
                    "exp_year": 2025
                },
                "code": "S.0000",
                "friendly_message": "The operation is successful.",
                "payment": {
                    "auth_code": "654321",
                    "status": "successful",
                    "gateway_id": 4318
                },
                "avs_cvc_verification": {
                    "avs_verification": {"r": ""}
                }
            }
        })))
        .expect(1)
        .mount(&server)
        .await;

    let c = client(&server);
    let t = c
        .get_transaction("2f4d67ff")
        .await
        .expect("get should succeed");

    assert_eq!(t.status.as_deref(), Some("successful"));
    assert_eq!(t.code.as_deref(), Some("S.0000"));
    assert_eq!(
        t.credit_card.as_ref().and_then(|cc| cc.brand.as_deref()),
        Some("visa")
    );
    assert!(t.payment.is_some());
}

#[tokio::test]
async fn transaction_status_by_tracking_id_happy_path() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v2/transactions/tracking_id/order-123"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "uid": "54c70f9b-e6e5-4b5a-bda2-fe6980e44bf0",
            "transaction_status": "approved",
            "result_code": "0",
            "cvc_verification": {"result_code": "1"},
            "customer": {"ip": null, "email": null},
            "billing_address": {
                "first_name": "John",
                "last_name": "Doe",
                "address": "1st Street",
                "country": "UA",
                "city": "Denver",
                "zip": "96002",
                "state": "12",
                "phone": "4567898765467"
            }
        })))
        .expect(1)
        .mount(&server)
        .await;

    let c = client(&server);
    let s = c
        .get_transaction_by_tracking_id("order-123")
        .await
        .expect("query should succeed");

    assert_eq!(s.uid, "54c70f9b-e6e5-4b5a-bda2-fe6980e44bf0");
    assert_eq!(s.transaction_status.as_deref(), Some("approved"));
    assert_eq!(
        s.billing_address.as_ref().and_then(|b| b.city.as_deref()),
        Some("Denver")
    );
}

#[tokio::test]
async fn create_token_happy_path() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/credit_cards"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "holder": "John Doe",
            "stamp": "a825df7faba8804619aef7a6d5a5821ec",
            "brand": "visa",
            "last_4": "0000",
            "first_1": "4",
            "token": "7ba647e7013b5cb9df39f17c375783aef",
            "exp_month": 1,
            "exp_year": 2028
        })))
        .expect(1)
        .mount(&server)
        .await;

    let c = client(&server);
    let t = c
        .create_token(CreateTokenRequest {
            number: "4200000000000000".into(),
            holder: "John Doe".into(),
            exp_month: "05".into(),
            exp_year: "2028".into(),
            contract: Some(vec!["recurring".into()]),
        })
        .await
        .expect("token should succeed");

    assert_eq!(t.brand.as_deref(), Some("visa"));
    assert_eq!(
        t.token.as_deref(),
        Some("7ba647e7013b5cb9df39f17c375783aef")
    );
}

#[tokio::test]
async fn checkout_happy_path() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/ctp/api/checkouts"))
        .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
            "checkout": {
                "token": "f5f9610b097fb742bf58e9be35bd007f",
                "redirect_url": "https://checkout.bepaid.by/widget/hpp.html?token=f5f9610b"
            }
        })))
        .expect(1)
        .mount(&server)
        .await;

    let c = client(&server);
    let t = c
        .create_checkout(&CheckoutRequest {
            test: Some(true),
            transaction_type: "payment".into(),
            attempts: None,
            iframe: None,
            settings: None,
            payment_method: None,
            credit_card: None,
            order: bepaid::types::CheckoutOrder {
                currency: "USD".into(),
                amount: 7000,
                description: Some("Test".into()),
                tracking_id: None,
                additional_data: None,
                custom_fields: None,
            },
            customer: None,
        })
        .await
        .expect("checkout should succeed");

    assert_eq!(t.token, "f5f9610b097fb742bf58e9be35bd007f");
    assert!(
        t.redirect_url
            .unwrap()
            .contains("checkout.bepaid.by/widget/hpp.html")
    );
}

#[tokio::test]
async fn payment_token_happy_path() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/payments/tokens"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "checkout": {
                "token": "3241e439f8c87d941d92621a4bdc030d",
                "redirect_url": "https://checkout.bepaid.by/v2/checkout?token=3241e439f8c87d941d92621a4bdc030d"
            }
        })))
        .expect(1)
        .mount(&server)
        .await;

    let c = client(&server);
    let t = c
        .create_payment_token(&CheckoutRequest {
            test: Some(true),
            transaction_type: "payment".into(),
            attempts: None,
            iframe: None,
            settings: None,
            payment_method: None,
            credit_card: None,
            order: bepaid::types::CheckoutOrder {
                currency: "USD".into(),
                amount: 7000,
                description: Some("Widget order".into()),
                tracking_id: None,
                additional_data: Some(bepaid::types::CheckoutAdditionalData {
                    contract: Some(vec!["recurring".into()]),
                }),
                custom_fields: None,
            },
            customer: None,
        })
        .await
        .expect("token creation should succeed");

    assert_eq!(t.token, "3241e439f8c87d941d92621a4bdc030d");
    assert!(
        t.redirect_url
            .unwrap()
            .contains("checkout.bepaid.by/v2/checkout")
    );
}

#[tokio::test]
async fn checkout_status_happy_path() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/ctp/api/checkouts/tok123"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "checkout": {
                "token": "tok123",
                "shop_id": 160,
                "transaction_type": "payment",
                "settings": {"success_url": "http://example.com/success"}
            }
        })))
        .expect(1)
        .mount(&server)
        .await;

    let c = client(&server);
    let t = c
        .get_checkout_status("tok123")
        .await
        .expect("status should succeed");

    assert_eq!(t.token.as_deref(), Some("tok123"));
    assert_eq!(t.shop_id, Some(160));
}

#[tokio::test]
async fn apm_payment_happy_path() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/beyag/transactions/payments"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "transaction": {
                "uid": "d5ac59da-09bf-4f82-826c-f9f96d3fb4ba",
                "type": "payment",
                "status": "pending",
                "amount": 100,
                "currency": "BYN",
                "method_type": "mts_money"
            }
        })))
        .expect(1)
        .mount(&server)
        .await;

    let c = client(&server);
    let t = c
        .create_apm_payment(ApmPaymentRequest {
            amount: 100,
            currency: "BYN".into(),
            description: Some("desc".into()),
            email: None,
            ip: Some("127.0.0.1".into()),
            success_url: None,
            order_id: None,
            tracking_id: None,
            notification_url: None,
            expired_at: None,
            test: None,
            language: None,
            return_url: None,
            customer: None,
            payment_method: serde_json::json!({"type": "mts_money", "confirm_agreement": "accept"}),
            additional_data: None,
            custom_fields: None,
        })
        .await
        .expect("apm should succeed");

    assert_eq!(t.status.as_deref(), Some("pending"));
    assert_eq!(t.tx_type.as_deref(), Some("payment"));
}

#[tokio::test]
async fn apm_payment_constructors_serialize() {
    let requests = [
        ApmPaymentRequest::erip(1000, "BYN", "123", "99999999"),
        ApmPaymentRequest::mts_money(100, "BYN", "375295222222", "accept"),
        ApmPaymentRequest::krok(220, "BYN", "https://example.com/return"),
        ApmPaymentRequest::qiwi_terminal(1000, "RUB", "test_account_123"),
    ];
    let methods = [
        serde_json::json!({ "type": "erip", "account_number": "123", "service_no": "99999999" }),
        serde_json::json!({ "type": "mts_money", "confirm_agreement": "accept" }),
        serde_json::json!({ "type": "krok" }),
        serde_json::json!({ "type": "qiwi_terminal", "account": "test_account_123" }),
    ];

    let server = MockServer::start().await;
    let c = client(&server);
    for (req, method_payload) in requests.iter().zip(methods.iter()) {
        Mock::given(method("POST"))
            .and(path("/beyag/transactions/payments"))
            .and(body_partial_json(serde_json::json!({
                "request": {
                    "amount": req.amount,
                    "currency": req.currency,
                    "payment_method": method_payload,
                }
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "transaction": { "uid": "u", "type": "payment", "status": "pending" }
            })))
            .expect(1)
            .mount(&server)
            .await;
        let t = c
            .create_apm_payment(req.clone())
            .await
            .expect("apm should succeed");
        assert_eq!(t.status.as_deref(), Some("pending"));
    }

    let req = ApmPaymentRequest {
        payment_method: serde_json::json!({
            "type": "erip",
            "account_number": "123",
            "service_no": "99999999",
            "erip_devices": [EripDevice {
                name: "Холодная вода".into(),
                item_unit: "м3".into(),
                rank: "4".into(),
                value: "1234".into(),
                rate: "0.4392".into(),
            }],
        }),
        ..ApmPaymentRequest::erip(2000, "BYN", "123", "99999999")
    };
    Mock::given(method("POST"))
        .and(path("/beyag/transactions/payments"))
        .and(body_partial_json(serde_json::json!({
            "request": {
                "amount": 2000,
                "currency": "BYN",
                "payment_method": {
                    "type": "erip",
                    "account_number": "123",
                    "service_no": "99999999",
                    "erip_devices": [
                        {
                            "name": "Холодная вода",
                            "item_unit": "м3",
                            "rank": "4",
                            "value": "1234",
                            "rate": "0.4392",
                        }
                    ],
                },
            }
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "transaction": { "uid": "u", "type": "payment", "status": "pending" }
        })))
        .expect(1)
        .mount(&server)
        .await;
    let t = c.create_apm_payment(req).await.expect("apm should succeed");
    assert_eq!(t.status.as_deref(), Some("pending"));
}

#[tokio::test]
async fn apm_refund_happy_path() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/beyag/transactions/refunds"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "transaction": {
                "uid": "2-310b0da80b",
                "type": "refund",
                "status": "successful",
                "refund": {"ref_id": "8889999", "message": "ok", "status": "successful"}
            }
        })))
        .expect(1)
        .mount(&server)
        .await;

    let c = client(&server);
    let t = c
        .apm_refund(bepaid::types::ApmRefundRequest {
            parent_uid: "1-310b0da80b".into(),
            reason: "reason".into(),
            amount: Some(50),
            tracking_id: None,
            additional_data: None,
        })
        .await
        .expect("refund should succeed");

    assert_eq!(t.status.as_deref(), Some("successful"));
    assert_eq!(t.tx_type.as_deref(), Some("refund"));
}

#[tokio::test]
async fn webhook_verify_and_parse() {
    assert!(verify_webhook_auth(AUTH, SHOP_ID, SECRET));
    assert!(!verify_webhook_auth("Basic bm90OnJpZ2h0", SHOP_ID, SECRET));

    let body = r#"{
        "transaction": {
            "uid": "566fd40a-2379-46d6-aecd-67779afcf883",
            "type": "payment",
            "status": "pending",
            "amount": 1234,
            "currency": "EUR",
            "description": "Description",
            "created_at": "2018-08-08T13:30:54Z",
            "method_type": "method_name",
            "payment": {"status": "pending", "gateway_id": 1},
            "customer": {},
            "tracking_id": "abc",
            "test": true
        }
    }"#;

    let n = parse_webhook(body).expect("should parse");
    assert_eq!(n.transaction.uid, "566fd40a-2379-46d6-aecd-67779afcf883");
    assert_eq!(n.transaction.status, "pending");
    assert_eq!(n.transaction.tx_type, "payment");
}

#[tokio::test]
async fn webhook_signature_roundtrip() {
    use base64::{Engine as _, engine::general_purpose::STANDARD};
    use rsa::RsaPrivateKey;
    use rsa::pkcs1v15::SigningKey;
    use rsa::pkcs8::EncodePublicKey;
    use rsa::rand_core::OsRng;
    use rsa::sha2::Sha256;
    use rsa::signature::{SignatureEncoding, Signer};

    let mut rng = OsRng;
    let private_key = RsaPrivateKey::new(&mut rng, 2048).expect("keygen");
    let public_key_pem = private_key
        .to_public_key()
        .to_public_key_pem(rsa::pkcs8::LineEnding::LF)
        .expect("pem");
    let body = br#"{"transaction":{"uid":"123"}}"#;
    let signature = SigningKey::<Sha256>::new(private_key).sign(body).to_bytes();
    let signature = STANDARD.encode(signature);

    assert!(verify_webhook_signature(&public_key_pem, &signature, body).expect("verify ok"));
    assert!(
        !verify_webhook_signature(&public_key_pem, &signature, b"tampered").expect("verify ok")
    );
}

#[tokio::test]
async fn subscriptions_crud() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/customers"))
        .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
            "id": "cst_7aee5afb954c7ef7",
            "first_name": "John",
            "last_name": "Doe",
            "email": "customer@example.com"
        })))
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/customers/cst_7aee5afb954c7ef7"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "cst_7aee5afb954c7ef7",
            "email": "customer@example.com"
        })))
        .expect(1)
        .mount(&server)
        .await;

    let c = client(&server);
    let created = c
        .create_customer(CustomerRecord {
            id: None,
            first_name: Some("John".into()),
            last_name: Some("Doe".into()),
            address: None,
            city: None,
            country: None,
            zip: None,
            state: None,
            phone: None,
            email: Some("customer@example.com".into()),
            ip: Some("127.0.0.1".into()),
            external_id: None,
        })
        .await
        .expect("create should succeed");
    assert_eq!(created.id.as_deref(), Some("cst_7aee5afb954c7ef7"));

    let fetched = c
        .get_customer("cst_7aee5afb954c7ef7")
        .await
        .expect("get should succeed");
    assert_eq!(fetched.email.as_deref(), Some("customer@example.com"));
}

#[tokio::test]
async fn create_subscription_with_token() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/subscriptions"))
        .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
            "id": "sbs_cce60e7f2d661bc0",
            "state": "active",
            "tracking_id": "my_tracking_id",
            "card": {"brand": "master", "last_4": "5003", "token": "tok_1"},
            "customer": {"id": "cst_ec240ca02bac424b"},
            "plan": {"id": "pln_f5ee5ebd04e39daa", "title": "Basic plan"},
            "last_transaction": {"uid": "f0eee433", "status": "successful"}
        })))
        .expect(1)
        .mount(&server)
        .await;

    let c = client(&server);
    let s = c
        .create_subscription(SubscriptionCreateRequest {
            card: Some(bepaid::types::SubscriptionCard {
                token: Some("tok_1".into()),
                number: None,
                holder: None,
                verification_value: None,
                exp_month: None,
                exp_year: None,
            }),
            customer: Some(bepaid::types::SubscriptionCustomer {
                id: Some("cst_ec240ca02bac424b".into()),
                first_name: None,
                last_name: None,
                email: None,
            }),
            plan: bepaid::types::SubscriptionPlan {
                id: Some("pln_f5ee5ebd04e39daa".into()),
                title: None,
                currency: None,
                plan: None,
                trial: None,
            },
            tracking_id: Some("my_tracking_id".into()),
            device_id: None,
            return_url: None,
            notification_url: None,
            dynamic_billing_descriptor: None,
            additional_data: None,
            settings: None,
        })
        .await
        .expect("create subscription should succeed");

    assert_eq!(s.state.as_deref(), Some("active"));
    assert_eq!(
        s.card.as_ref().and_then(|c| c.brand.as_deref()),
        Some("master")
    );
}

#[tokio::test]
async fn cancel_subscription_happy_path() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/subscriptions/sbs_1/cancel"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(serde_json::json!({"state": "canceled"})),
        )
        .expect(1)
        .mount(&server)
        .await;

    let c = client(&server);
    let r = c
        .cancel_subscription(
            "sbs_1",
            CancelSubscriptionRequest {
                cancel_reason: "Customer's request".into(),
            },
        )
        .await
        .expect("cancel should succeed");
    assert_eq!(r["state"], "canceled");
}

#[tokio::test]
async fn create_plan_and_list() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/plans"))
        .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
            "id": "pln_2b0c211f50deb72c",
            "title": "Basic plan",
            "currency": "USD",
            "plan": {"amount": 20, "interval": 7, "interval_unit": "day"},
            "number_payment_attempts": 3,
            "test": true
        })))
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/plans"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
            {"id": "pln_2b0c211f50deb72c", "title": "Basic plan"}
        ])))
        .expect(1)
        .mount(&server)
        .await;

    let c = client(&server);
    let p = c
        .create_plan(bepaid::types::PlanItem {
            id: None,
            test: Some(true),
            title: Some("Basic plan".into()),
            currency: Some("USD".into()),
            language: None,
            plan: Some(bepaid::types::PlanInterval {
                amount: Some(20),
                interval: Some(7),
                interval_unit: Some("day".into()),
                visible_fields: None,
            }),
            trial: None,
            infinite: None,
            billing_cycles: None,
            number_payment_attempts: Some(3),
            prevent_payments_at_night: None,
            created_at: None,
            updated_at: None,
            pay_url: None,
        })
        .await
        .expect("create plan should succeed");
    assert_eq!(p.id.as_deref(), Some("pln_2b0c211f50deb72c"));

    let plans = c.list_plans().await.expect("list should succeed");
    assert_eq!(plans.len(), 1);
    assert_eq!(plans[0].title.as_deref(), Some("Basic plan"));
}

#[tokio::test]
async fn apm_confirm_happy_path() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/beyag/transactions/1-310b0da80b/confirm"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "response": {
                "parent_uid": "1-310b0da80b",
                "type": "confirm",
                "status": "successful",
                "message": "Confirm was successfully processed",
                "amount": 332400,
                "currency": "USD"
            }
        })))
        .expect(1)
        .mount(&server)
        .await;

    let c = client(&server);
    let r = c
        .confirm_apm_payment(
            "1-310b0da80b",
            ApmConfirmRequest {
                skip_duplicate_check: Some(false),
                transaction_reference: "receipt-123".into(),
            },
        )
        .await
        .expect("confirm should succeed");

    assert_eq!(r.status.as_deref(), Some("successful"));
    assert_eq!(r.tx_type.as_deref(), Some("confirm"));
}

#[tokio::test]
async fn p2p_happy_path() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/transactions/p2ps"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "transaction": {
                "uid": "1-82cc07d15d",
                "status": "successful",
                "type": "p2p",
                "amount": 100,
                "currency": "EUR",
                "credit_card": {"brand": "visa", "last_4": "1112"},
                "recipient_card": {"brand": "visa", "last_4": "0000"},
                "verify_p2p": {"status": "successful", "amount": 100, "currency": "EUR"}
            }
        })))
        .expect(1)
        .mount(&server)
        .await;

    let c = client(&server);
    let t = c
        .create_p2p(P2pRequest {
            amount: 100,
            currency: "EUR".into(),
            credit_card: bepaid::types::P2pCard {
                number: Some("4012001037141112".into()),
                holder: Some("John Doe".into()),
                verification_value: Some("123".into()),
                exp_month: Some("12".into()),
                exp_year: Some("2028".into()),
                token: None,
            },
            recipient_card: bepaid::types::P2pCard {
                number: Some("4200000000000000".into()),
                holder: None,
                verification_value: None,
                exp_month: None,
                exp_year: None,
                token: None,
            },
            test: Some(true),
            tracking_id: None,
            additional_data: None,
        })
        .await
        .expect("p2p should succeed");

    assert_eq!(t.status.as_deref(), Some("successful"));
    assert_eq!(t.tx_type.as_deref(), Some("p2p"));
    assert!(t.verify_p2p.is_some());
}

#[tokio::test]
async fn subscription_webhook_parses() {
    let body = r#"{
        "id": "sbs_962f994ca74420d3",
        "state": "trial",
        "event": "created.subscription",
        "card": {"brand": "visa", "last_4": "1006", "token": "tok"},
        "plan": {"id": "pln_7f2e3edfbca72afc", "title": "Test plan"}
    }"#;

    let s = parse_subscription_webhook(body).expect("should parse");
    assert_eq!(s.id.as_deref(), Some("sbs_962f994ca74420d3"));
    assert_eq!(s.state.as_deref(), Some("trial"));
    assert_eq!(s.event.as_deref(), Some("created.subscription"));
}

#[tokio::test]
async fn payout_happy_path() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/transactions/payouts"))
        .and(wiremock::matchers::header("x-api-version", "3"))
        .and(wiremock::matchers::header("authorization", AUTH))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "transaction": {
                "uid": "1-310b0da80b",
                "type": "payout",
                "status": "successful",
                "amount": 100,
                "currency": "USD",
                "description": "Payout",
                "test": true,
                "tracking_id": "payout-1",
                "payout": {"status": "successful", "gateway_id": 1345, "rrn": "1234"},
                "customer": {"ip": "127.0.0.1", "email": "john@example.com"}
            }
        })))
        .expect(1)
        .mount(&server)
        .await;

    let c = client(&server);
    let p = c
        .create_payout(PayoutRequest {
            test: Some(true),
            amount: 100,
            currency: "USD".into(),
            description: Some("Payout".into()),
            tracking_id: Some("payout-1".into()),
            recipient: bepaid::types::Customer {
                first_name: None,
                last_name: None,
                ip: Some("127.0.0.1".into()),
                email: Some("john@example.com".into()),
                device_id: None,
                birth_date: Some("1990-10-20".into()),
                phone: None,
            },
            sender: bepaid::types::Customer {
                first_name: None,
                last_name: None,
                ip: Some("127.0.0.1".into()),
                email: Some("john@example.com".into()),
                device_id: None,
                birth_date: Some("1990-10-20".into()),
                phone: None,
            },
            recipient_billing_address: bepaid::types::BillingAddress {
                first_name: None,
                last_name: None,
                country: Some("US".into()),
                city: Some("Denver".into()),
                state: Some("CO".into()),
                zip: Some("96002".into()),
                address: Some("1st Street".into()),
                phone: None,
            },
            sender_billing_address: bepaid::types::BillingAddress {
                first_name: None,
                last_name: None,
                country: Some("US".into()),
                city: Some("Denver".into()),
                state: Some("CO".into()),
                zip: Some("96002".into()),
                address: Some("1st Street".into()),
                phone: None,
            },
            recipient_credit_card: None,
            additional_data: None,
            custom_fields: None,
        })
        .await
        .expect("payout should succeed");

    assert_eq!(p.status.as_deref(), Some("successful"));
    assert_eq!(p.tx_type.as_deref(), Some("payout"));
    assert_eq!(p.payout.as_ref().unwrap().rrn.as_deref(), Some("1234"));
}

#[tokio::test]
async fn apm_balance_happy_path() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/beyag/balance"))
        .and(wiremock::matchers::header("authorization", AUTH))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "Successful",
            "code": "S.0000",
            "friendly_message": "Successfully processed",
            "gateway_id": 1234,
            "account": "40701810842020395221",
            "amount": 1290092162,
            "currency": "USD",
            "provider_info": {"BankCode": 33}
        })))
        .expect(1)
        .mount(&server)
        .await;

    let c = client(&server);
    let b = c
        .get_balance(BalanceRequest {
            gateway_id: 1234,
            account: Some("40701810842020395221".into()),
            currency: Some("USD".into()),
        })
        .await
        .expect("balance should succeed");

    assert_eq!(b.status.as_deref(), Some("Successful"));
    assert_eq!(b.amount, Some(1290092162));
}

#[tokio::test]
async fn currency_query_happy_path() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/beyag/currencies"))
        .and(wiremock::matchers::header("authorization", AUTH))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "Successful",
            "code": "S.0000",
            "friendly_message": "Successfully processed",
            "gateway_id": 1234,
            "account": "40701810842020395221",
            "country": "GB",
            "currency": "TRX",
            "provider_info": {
                "currency": "TRX",
                "alias": "Tron",
                "allowDeposit": true,
                "allowWithdrawal": true,
                "priceUSD": "0.05963000",
                "networks": [{"name": "tron"}]
            }
        })))
        .expect(1)
        .mount(&server)
        .await;

    let c = client(&server);
    let info = c
        .get_currencies(CurrencyQueryRequest {
            gateway_id: 1234,
            account: Some("40701810842020395221".into()),
            country: Some("GB".into()),
        })
        .await
        .expect("currency query should succeed");

    assert_eq!(info.status.as_deref(), Some("Successful"));
    assert_eq!(info.currency.as_deref(), Some("TRX"));
    assert_eq!(
        info.provider_info.as_ref().unwrap()["networks"][0]["name"],
        serde_json::json!("tron")
    );
}

#[tokio::test]
async fn merchant_reports_list_happy_path() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/api/reports"))
        .and(wiremock::matchers::header("x-api-version", "2"))
        .and(wiremock::matchers::header("authorization", AUTH))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "transactions": [{
                "shop": {"id": 1},
                "uid": "20527-b7ea8c95f4",
                "id": 28859,
                "payment_method_type": "credit_card",
                "type": "authorization",
                "status": "failed",
                "amount": 1000,
                "currency": "USD",
                "test": false,
                "created_at": "2022-01-27T13:48:51Z",
                "credit_card": {"brand": "visa", "last_4": "1006"}
            }],
            "count": 1
        })))
        .expect(1)
        .mount(&server)
        .await;

    let c = client(&server);
    let r = c
        .get_reports(ReportListRequest {
            report_params: ReportParams {
                date_type: "created_at".into(),
                date: "2022-01-27".into(),
                status: "failed".into(),
                payment_method_type: "credit_card".into(),
                time_zone: "Europe/London".into(),
            },
        })
        .await
        .expect("reports should succeed");

    assert_eq!(r.count, Some(1));
    assert_eq!(r.transactions[0].uid.as_deref(), Some("20527-b7ea8c95f4"));
    assert_eq!(r.transactions[0].status.as_deref(), Some("failed"));
}

#[tokio::test]
async fn merchant_report_count_happy_path() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/api/reports/count"))
        .and(wiremock::matchers::header("x-api-version", "3"))
        .and(wiremock::matchers::header("authorization", AUTH))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "transactions": {"count": 2}
        })))
        .expect(1)
        .mount(&server)
        .await;

    let c = client(&server);
    let r = c
        .get_report_count(bepaid::types::ReportCountRequest {
            report_params: bepaid::types::ReportCountParams {
                date_type: "created_at".into(),
                from: "2022-01-25 00:00:00".into(),
                to: "2022-01-27 23:59:59".into(),
                status: "incomplete".into(),
                payment_method_type: "credit_card".into(),
                time_zone: "Etc/UTC".into(),
            },
        })
        .await
        .expect("report count should succeed");

    assert_eq!(r.transactions.count, 2);
}

#[tokio::test]
async fn merchant_channel_balances_happy_path() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/shop/channel_balances/"))
        .and(wiremock::matchers::header("authorization", AUTH))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
            {"gateway_id": 3405, "currency": "USD", "amount": 100}
        ])))
        .expect(1)
        .mount(&server)
        .await;

    let c = client(&server);
    let bals = c
        .get_channel_balances(3405, Some("USD"))
        .await
        .expect("channel balances should succeed");

    assert_eq!(bals.len(), 1);
    assert_eq!(bals[0].gateway_id, Some(3405));
    assert_eq!(bals[0].amount, Some(100));
}

#[tokio::test]
async fn split_payment_happy_path() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/splits/payment"))
        .and(wiremock::matchers::header("authorization", AUTH))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "splits": [
                {
                    "uid": "21-99834feb0b",
                    "amount": 70,
                    "status": "successful",
                    "message": "Successfully processed",
                    "shop_id": 91,
                    "parent": true,
                    "parent_uid": null
                },
                {
                    "uid": "22-56784ffecd",
                    "amount": 30,
                    "status": "successful",
                    "message": "Successfully processed",
                    "shop_id": 1111,
                    "parent": false,
                    "parent_uid": "21-99834feb0b"
                }
            ]
        })))
        .expect(1)
        .mount(&server)
        .await;

    use std::collections::HashMap;

    let c = client(&server);
    let mut split = HashMap::new();
    split.insert("1111".to_owned(), 30);
    let resp = c
        .create_split_payment(bepaid::types::SplitPaymentRequest {
            amount: 100,
            currency: "USD".to_owned(),
            description: "Test transaction".to_owned(),
            tracking_id: "tracking_id_000".to_owned(),
            billing_address: None,
            credit_card: bepaid::types::SplitCreditCard {
                token: "credit-card-token".to_owned(),
            },
            customer: None,
            additional_data: Some(bepaid::types::SplitAdditionalData {
                contract: None,
                split,
            }),
        })
        .await
        .expect("split payment should succeed");

    assert_eq!(resp.splits.len(), 2);
    assert_eq!(resp.splits[0].uid, "21-99834feb0b");
    assert!(resp.splits[0].parent);
    assert_eq!(resp.splits[1].parent_uid.as_deref(), Some("21-99834feb0b"));
}

#[tokio::test]
async fn create_product_happy_path() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/products"))
        .and(wiremock::matchers::header("authorization", AUTH))
        .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
            "id": "prd_ed27b047d3ccd1a6",
            "name": "product",
            "description": "description of product",
            "currency": "USD",
            "amount": 990,
            "quantity": 10,
            "infinite": false,
            "language": "en",
            "transaction_type": "payment",
            "created_at": "2022-12-20T18:54:42.033Z",
            "updated_at": "2022-12-20T18:54:42.033Z",
            "test": false,
            "additional_data": {},
            "pay_url": "https://api.bepaid.by/products/prd_ed27b047d3ccd1a6/pay",
            "payment_url": "https://api.bepaid.by/products/prd_ed27b047d3ccd1a6/pay",
            "confirm_url": "https://checkout.bepaid.by/v2/confirm_order/prd_ed27b047d3ccd1a6/1"
        })))
        .expect(1)
        .mount(&server)
        .await;

    let c = client(&server);
    let resp = c
        .create_product(ProductCreateRequest {
            name: "product".to_owned(),
            description: "description of product".to_owned(),
            currency: "USD".to_owned(),
            amount: 990,
            quantity: Some("10".to_owned()),
            infinite: Some(false),
            visible_fields: None,
            test: Some(false),
            immortal: Some(false),
            expired_at: None,
            return_url: None,
            shop_id: None,
            language: Some("en".to_owned()),
            transaction_type: Some("payment".to_owned()),
        })
        .await
        .expect("product creation should succeed");

    assert_eq!(resp.id, "prd_ed27b047d3ccd1a6");
    assert_eq!(resp.amount, 990);
    assert_eq!(
        resp.pay_url,
        "https://api.bepaid.by/products/prd_ed27b047d3ccd1a6/pay"
    );
}

#[tokio::test]
async fn list_products_happy_path() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/products"))
        .and(wiremock::matchers::header("authorization", AUTH))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
            {
                "id": "prd_1",
                "name": "product",
                "description": "description",
                "amount": 100,
                "currency": "USD",
                "language": "en",
                "infinite": true,
                "quantity": null,
                "transaction_type": "payment",
                "created_at": "2023-05-02T17:35:14.950Z",
                "updated_at": "2023-05-02T17:35:14.950Z",
                "additional_data": {},
                "test": false,
                "pay_url": "https://api.bepaid.by/products/prd_1/pay",
                "payment_url": "https://api.bepaid.by/products/prd_1/pay",
                "confirm_url": "https://checkout.bepaid.by/v2/confirm_order/prd_1/1"
            }
        ])))
        .expect(1)
        .mount(&server)
        .await;

    let c = client(&server);
    let resp = c.list_products().await.expect("listing should succeed");
    assert_eq!(resp.len(), 1);
    assert_eq!(resp[0].id, "prd_1");
}

#[tokio::test]
async fn get_product_happy_path() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/products/prd_1"))
        .and(wiremock::matchers::header("authorization", AUTH))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "prd_1",
            "name": "product",
            "description": "description",
            "amount": 100,
            "currency": "USD",
            "language": "en",
            "infinite": true,
            "quantity": null,
            "transaction_type": "payment",
            "created_at": "2023-05-02T17:35:14.950Z",
            "updated_at": "2023-05-02T17:35:14.950Z",
            "additional_data": {},
            "test": false,
            "pay_url": "https://api.bepaid.by/products/prd_1/pay",
            "payment_url": "https://api.bepaid.by/products/prd_1/pay",
            "confirm_url": "https://checkout.bepaid.by/v2/confirm_order/prd_1/1"
        })))
        .expect(1)
        .mount(&server)
        .await;

    let c = client(&server);
    let resp = c.get_product("prd_1").await.expect("fetch should succeed");
    assert_eq!(resp.id, "prd_1");
}

#[tokio::test]
async fn update_product_happy_path() {
    let server = MockServer::start().await;
    Mock::given(method("PUT"))
        .and(path("/products/prd_1"))
        .and(wiremock::matchers::header("authorization", AUTH))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&server)
        .await;

    let c = client(&server);
    c.update_product(
        "prd_1",
        ProductUpdateRequest {
            amount: Some(950),
            infinite: Some(false),
            quantity: Some("5".to_owned()),
        },
    )
    .await
    .expect("update should succeed");
}

#[tokio::test]
async fn plan_payment_link_happy_path() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/plans/pln_a134847c902551de/pay"))
        .and(wiremock::matchers::header("authorization", AUTH))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "redirect_url": "https://checkout.bepaid.by/pay?token=abc"
        })))
        .expect(1)
        .mount(&server)
        .await;

    let c = client(&server);
    let resp = c
        .get_plan_payment_link("pln_a134847c902551de")
        .await
        .expect("link should succeed");
    assert_eq!(
        resp["redirect_url"],
        "https://checkout.bepaid.by/pay?token=abc"
    );
}

#[tokio::test]
async fn charge_saved_card_happy_path() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/services/credit_cards/charges"))
        .and(wiremock::matchers::header("x-api-version", "3"))
        .and(wiremock::matchers::header("authorization", AUTH))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "transaction": {
                "uid": "1-310b0da80b",
                "type": "payment",
                "status": "successful",
                "amount": 700,
                "currency": "USD",
                "description": "Recurring charge",
                "test": true,
                "credit_card": {"last_4": "1006", "brand": "visa"}
            }
        })))
        .expect(1)
        .mount(&server)
        .await;

    let c = client(&server);
    let t = c
        .charge_saved_card(ChargeRequest {
            amount: 700,
            currency: "USD".into(),
            description: "Recurring charge".into(),
            tracking_id: Some("order-7000".into()),
            expired_at: None,
            duplicate_check: None,
            dynamic_billing_descriptor: None,
            language: None,
            notification_url: None,
            verification_url: None,
            return_url: None,
            test: Some(true),
            force_three_d_secure_verification: None,
            credit_card: ChargeCreditCard {
                number: None,
                verification_value: None,
                holder: None,
                exp_month: None,
                exp_year: None,
                token: Some("tok_123".into()),
                skip_three_d_secure_verification: None,
            },
            customer: None,
            additional_data: None,
        })
        .await
        .expect("charge should succeed");
    assert_eq!(t.uid, "1-310b0da80b");
    assert_eq!(t.status.as_deref(), Some("successful"));
}

#[tokio::test]
async fn recipient_tokenization_happy_path() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/transactions/recipient_tokenizations"))
        .and(wiremock::matchers::header("x-api-version", "3"))
        .and(wiremock::matchers::header("authorization", AUTH))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "transaction": {
                "uid": "1-310b0da80b",
                "status": "pending",
                "recipient_credit_card": {"token": "tok_recipient"}
            }
        })))
        .expect(1)
        .mount(&server)
        .await;

    let c = client(&server);
    let resp = c
        .tokenize_recipient_card(RecipientTokenizationRequest {
            description: Some("Tokenize card".into()),
            tracking_id: None,
            recipient_billing_address: None,
            recipient_credit_card: PayoutCreditCard {
                number: Some("4242424242424242".into()),
                holder: Some("John Smith".into()),
                exp_month: Some("10".into()),
                exp_year: Some("2030".into()),
            },
            recipient: None,
            additional_data: None,
        })
        .await
        .expect("tokenization should succeed");

    assert_eq!(resp["transaction"]["uid"], "1-310b0da80b");
    assert_eq!(resp["transaction"]["status"], "pending");
}

#[tokio::test]
async fn apple_pay_payment_happy_path() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/apple_pay/payment"))
        .and(wiremock::matchers::header("authorization", AUTH))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "Success": true,
            "Model": null
        })))
        .expect(1)
        .mount(&server)
        .await;

    let c = client(&server);
    let resp = c
        .apple_pay_payment("eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9...")
        .await
        .expect("apple pay payment should succeed");

    assert_eq!(resp["Success"], true);
}

#[tokio::test]
async fn get_apm_transaction_happy_path() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/beyag/transactions/apm1"))
        .and(wiremock::matchers::header("authorization", AUTH))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "transaction": {
                "uid": "apm1",
                "type": "payment",
                "status": "successful",
                "amount": 100,
                "currency": "BYN"
            }
        })))
        .expect(1)
        .mount(&server)
        .await;

    let c = client(&server);
    let t = c
        .get_apm_transaction("apm1")
        .await
        .expect("status should succeed");

    assert_eq!(t.status.as_deref(), Some("successful"));
    assert_eq!(t.amount, Some(100));
}

#[tokio::test]
async fn get_apm_transactions_by_tracking_id_happy_path() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/beyag/transactions/tracking_id/tracking_1"))
        .and(wiremock::matchers::header("authorization", AUTH))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "transactions": [
                {"uid": "apm1", "type": "payment", "status": "successful"},
                {"uid": "apm2", "type": "payment", "status": "failed"}
            ]
        })))
        .expect(1)
        .mount(&server)
        .await;

    let c = client(&server);
    let ts = c
        .get_apm_transactions_by_tracking_id("tracking_1")
        .await
        .expect("status should succeed");

    let uids: Vec<_> = ts.iter().map(|t| t.uid.as_str()).collect();
    assert_eq!(uids, vec!["apm1", "apm2"]);
}

#[tokio::test]
async fn apm_payout_happy_path() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/beyag/transactions/payouts"))
        .and(body_partial_json(serde_json::json!({
            "request": {
                "amount": 100,
                "currency": "USD",
                "description": "payout",
                "method": {"type": "ad_payments"}
            }
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "transaction": {
                "uid": "pay1",
                "type": "payout",
                "status": "successful",
                "amount": 100,
                "currency": "USD",
                "payout": {"status": "successful", "gateway_id": 85}
            }
        })))
        .expect(1)
        .mount(&server)
        .await;

    let c = client(&server);
    let p = c
        .apm_payout(ApmPayoutRequest {
            amount: 100,
            currency: "USD".into(),
            description: "payout".into(),
            test: None,
            tracking_id: None,
            ip: None,
            language: None,
            notification_url: None,
            verification_url: None,
            return_url: None,
            customer: None,
            method: serde_json::json!({"type": "ad_payments"}),
            additional_data: None,
            custom_fields: None,
        })
        .await
        .expect("payout should succeed");

    assert_eq!(p.status.as_deref(), Some("successful"));
    let payout = p.payout.as_ref().expect("payout details");
    assert_eq!(payout["gateway_id"], 85);
}

#[tokio::test]
async fn apm_proof_happy_path() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/beyag/transactions/apm1/proof"))
        .and(body_partial_json(serde_json::json!({
            "request": {
                "amount": 71267,
                "currency": "USD",
                "document": {
                    "content_type": "application/pdf",
                    "file_name": "proof.pdf",
                    "file_size": 12345,
                    "content": "base64...",
                    "checksum": "sha256..."
                }
            }
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "transaction": {
                "uid": "pr1",
                "parent_uid": "apm1",
                "type": "proof",
                "status": "successful",
                "amount": 71267,
                "currency": "USD",
                "proof": {"message": "Proof was successfully processed."}
            }
        })))
        .expect(1)
        .mount(&server)
        .await;

    let c = client(&server);
    let r = c
        .apm_proof(
            "apm1",
            ProofRequest {
                skip_duplicate_check: None,
                amount: 71267,
                currency: "USD".into(),
                transaction_reference: None,
                document: ProofDocument {
                    content_type: "application/pdf".into(),
                    file_name: "proof.pdf".into(),
                    file_size: 12345,
                    content: "base64...".into(),
                    checksum: "sha256...".into(),
                },
            },
        )
        .await
        .expect("proof should succeed");

    assert_eq!(r.status.as_deref(), Some("successful"));
    assert_eq!(r.parent_uid.as_deref(), Some("apm1"));
}

#[tokio::test]
async fn checkup_happy_path() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/transactions/checkups"))
        .and(wiremock::matchers::header("x-api-version", "3"))
        .and(body_partial_json(serde_json::json!({
            "request": {
                "amount": 100,
                "currency": "USD",
                "description": "checkup",
                "tracking_id": "tracking_1",
                "credit_card": {"token": "tok1"}
            }
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "transaction": {
                "uid": "c1",
                "type": "payment",
                "status": "successful",
                "amount": 100,
                "currency": "USD"
            }
        })))
        .expect(1)
        .mount(&server)
        .await;

    let c = client(&server);
    let t = c
        .checkup(CheckupRequest {
            amount: 100,
            currency: "USD".into(),
            description: "checkup".into(),
            tracking_id: "tracking_1".into(),
            language: None,
            notification_url: None,
            verification_url: None,
            test: None,
            credit_card: Some(ChargeCreditCard {
                number: None,
                verification_value: None,
                holder: None,
                exp_month: None,
                exp_year: None,
                token: Some("tok1".into()),
                skip_three_d_secure_verification: None,
            }),
            customer: None,
            billing_address: None,
            additional_data: None,
        })
        .await
        .expect("checkup should succeed");

    assert_eq!(t.status.as_deref(), Some("successful"));
    assert_eq!(t.amount, Some(100));
}
