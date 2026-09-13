use bepaid::{
    BepaidClient, BepaidError,
    types::{
        ApmPaymentRequest, AuthorizationRequest, CaptureRequest, CheckoutRequest,
        CreateTokenRequest, PaymentRequest, RefundRequest, VoidRequest,
    },
    webhook::{parse_webhook, verify_webhook_auth},
};
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{method, path},
};

const SHOP_ID: &str = "363";
const SECRET: &str = "45454e083434aa37rfdfd";
const AUTH: &str = "Basic MzYzOjQ1NDU0ZTA4MzQzNGFhMzdyZmRmZA==";

fn client(server: &MockServer) -> BepaidClient {
    let base = server.uri();
    BepaidClient::with_urls(SHOP_ID, SECRET, &base, &base, &base)
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
            billing_address: None,
            credit_card: None,
            customer: None,
            additional_data: None,
        })
        .await;

    let t = resp.expect("payment should succeed");
    assert_eq!(t.uid, "some_uid");
    assert_eq!(t.tracking_id.as_deref(), Some("tracking_id_000"));
}

#[tokio::test]
async fn create_payment_400_returns_api_error() {
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
            billing_address: None,
            credit_card: None,
            customer: None,
            additional_data: None,
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
        })
        .await
        .expect("apm should succeed");

    assert_eq!(t.status.as_deref(), Some("pending"));
    assert_eq!(t.tx_type.as_deref(), Some("payment"));
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
