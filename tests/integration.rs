use bepaid::{
    BepaidClient, BepaidError,
    types::{
        AdditionalData, ApmConfirmRequest, ApmPaymentRequest, ApmPayoutRequest,
        AuthorizationRequest, BalanceRequest, CancelSubscriptionRequest, CaptureRequest,
        ChargeCreditCard, ChargeRequest, CheckoutRequest, CheckupRequest, CreateTokenRequest,
        CreditCardRaw, CurrencyQueryRequest, CustomerRecord, EripDevice, Fiscalization,
        FiscalizationPosition, FiscalizationTax, MasterpassData, MasterpassDeleteCardRequest,
        MasterpassGetCardRequest, MasterpassGetCardsRequest, MasterpassGetSavedCardRequest,
        MasterpassLoginRequest, MasterpassParams, P2pRequest, PaymentRequest, PayoutCreditCard,
        PayoutRequest, ProductCreateRequest, ProductUpdateRequest, ProofDocument, ProofRequest,
        RecipientTokenizationRequest, RefundRequest, ReportListRequest, ReportListV3Params,
        ReportListV3Request, ReportParams, SmartRoutingOptions, SplitRecipient,
        SubscriptionCreateRequest, TokenizationRequest, VoidRequest,
    },
    webhook::{
        parse_checkout_webhook, parse_subscription_webhook, parse_webhook, verify_webhook_auth,
        verify_webhook_signature,
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
        .create_payment(
            PaymentRequest {
                amount: "700".into(),
                currency: "USD".into(),
                test: true,
                description: "Test transaction".into(),
                tracking_id: "tid".into(),
                expired_at: None,
                dynamic_billing_descriptor: None,
                duplicate_check: None,
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
            },
            None,
        )
        .await;

    let t = resp.expect("payment should succeed");
    assert_eq!(t.uid, "some_uid");
    assert_eq!(t.tracking_id.as_deref(), Some("tracking_id_000"));
}

#[tokio::test]
async fn create_payment_serializes_card() {
    for card in [
        serde_json::json!({"token": "saved-card-token"}),
        serde_json::json!({"token": "$begateway_google_pay_1_0_0$eyJ0ZXN0Ijp0cnVlfQ=="}),
        serde_json::json!({"token": "$begateway_google_pay_decrypted_1_0_0$eyJ0ZXN0Ijp0cnVlfQ=="}),
        serde_json::json!({"token": "$begateway_samsung_pay_decrypted_1_0_0$eyJ0ZXN0Ijp0cnVlfQ=="}),
        serde_json::json!({
            "number": "4242424242424242",
            "verification_value": "123",
            "holder": "John Smith",
            "exp_month": 10,
            "exp_year": 2030,
        }),
    ] {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/transactions/payments"))
            .and(wiremock::matchers::body_json(serde_json::json!({
                "request": {
                    "amount": "700",
                    "currency": "USD",
                    "test": true,
                    "description": "Test transaction",
                    "tracking_id": "tracking_id_000",
                    "credit_card": card,
                }
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "transaction": {"uid": "u1"}
            })))
            .expect(1)
            .mount(&server)
            .await;

        let req = PaymentRequest {
            amount: "700".into(),
            currency: "USD".into(),
            test: true,
            description: "Test transaction".into(),
            tracking_id: "tracking_id_000".into(),
            expired_at: None,
            dynamic_billing_descriptor: None,
            language: None,
            notification_url: None,
            verification_url: None,
            return_url: None,
            duplicate_check: None,
            billing_address: None,
            credit_card: Some(serde_json::from_value(card).unwrap()),
            customer: None,
            additional_data: None,
            encrypted_data: None,
            fiscalization: None,
            custom_fields: None,
        };
        assert_eq!(
            client(&server).create_payment(req, None).await.unwrap().uid,
            "u1"
        );
    }
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
                "duplicate_check": false,
            }
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "transaction": {"tracking_id": "tid", "uid": "uid1"}
        })))
        .expect(1)
        .mount(&server)
        .await;

    let c = client(&server);
    c.create_payment(
        PaymentRequest {
            amount: "700".into(),
            currency: "USD".into(),
            test: true,
            description: "Test transaction".into(),
            tracking_id: "tid".into(),
            expired_at: None,
            dynamic_billing_descriptor: None,
            duplicate_check: Some(false),
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
        },
        None,
    )
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
    c.create_payment(
        PaymentRequest {
            amount: "700".into(),
            currency: "USD".into(),
            test: true,
            description: "Test transaction".into(),
            tracking_id: "tid".into(),
            expired_at: None,
            dynamic_billing_descriptor: None,
            duplicate_check: None,
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
        },
        None,
    )
    .await
    .expect("payment should succeed");
}

#[tokio::test]
async fn masterpass_login_happy_path() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/masterpass/login"))
        .and(wiremock::matchers::header("authorization", AUTH))
        .and(wiremock::matchers::header("x-api-version", "3"))
        .and(wiremock::matchers::body_json(serde_json::json!({
            "phone": "375291112233",
            "fingerprint": "fp-1",
            "phone_check_date": "2026-09-17T10:00:00Z",
            "channel": 1,
            "test": true
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "success",
            "is_otp_required": true,
            "session": "mp-session-1",
            "user_status": 2
        })))
        .expect(1)
        .mount(&server)
        .await;

    let c = client(&server);
    let r = c
        .masterpass_login(MasterpassLoginRequest {
            phone: "375291112233".into(),
            fingerprint: "fp-1".into(),
            phone_check_date: Some("2026-09-17T10:00:00Z".into()),
            channel: Some(1),
            test: Some(true),
        })
        .await
        .expect("masterpass login should succeed");

    assert_eq!(r.status.as_deref(), Some("success"));
    assert_eq!(r.is_otp_required, Some(true));
    assert_eq!(r.session.as_deref(), Some("mp-session-1"));
    assert_eq!(r.user_status, Some(2));
}

#[tokio::test]
async fn masterpass_get_cards_happy_path() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/masterpass/get_cards"))
        .and(wiremock::matchers::header("authorization", AUTH))
        .and(wiremock::matchers::header("x-api-version", "3"))
        .and(wiremock::matchers::body_json(serde_json::json!({
            "session": "mp-session-1",
            "test": true
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "success",
            "card_list": [{
                "card_holder": "John Smith",
                "date": "2026-01-15",
                "expiry_date": "2030-10",
                "pan_mask": "424242****4242",
                "card_name": "Visa Classic",
                "token": "mp-token-1",
                "card_status": 1,
                "is_recurring": true,
                "comment1": "c1",
                "comment2": "c2",
                "comment3": "c3"
            }]
        })))
        .expect(1)
        .mount(&server)
        .await;

    let c = client(&server);
    let r = c
        .masterpass_get_cards(MasterpassGetCardsRequest {
            session: "mp-session-1".into(),
            test: Some(true),
        })
        .await
        .expect("masterpass get_cards should succeed");

    assert_eq!(r.status.as_deref(), Some("success"));
    let cards = r.card_list.expect("card list present");
    assert_eq!(cards.len(), 1);
    let card = &cards[0];
    assert_eq!(card.card_holder.as_deref(), Some("John Smith"));
    assert_eq!(card.date.as_deref(), Some("2026-01-15"));
    assert_eq!(card.expiry_date.as_deref(), Some("2030-10"));
    assert_eq!(card.pan_mask.as_deref(), Some("424242****4242"));
    assert_eq!(card.card_name.as_deref(), Some("Visa Classic"));
    assert_eq!(card.token.as_deref(), Some("mp-token-1"));
    assert_eq!(card.card_status, Some(1));
    assert_eq!(card.is_recurring, Some(true));
    assert_eq!(card.comment1.as_deref(), Some("c1"));
    assert_eq!(card.comment2.as_deref(), Some("c2"));
    assert_eq!(card.comment3.as_deref(), Some("c3"));
}

#[tokio::test]
async fn masterpass_get_card_happy_path() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/masterpass/get_card"))
        .and(wiremock::matchers::header("authorization", AUTH))
        .and(wiremock::matchers::header("x-api-version", "3"))
        .and(wiremock::matchers::body_json(serde_json::json!({
            "token": "mp-token-1",
            "amount": 700,
            "currency": "BYN"
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "success",
            "message": "ok",
            "credit_card": {
                "brand": "visa",
                "last_4": "1097",
                "token": "be1e89ab-1d18-4d0f-83a1-7009b333dce0"
            },
            "recommendation": 2,
            "required": 1
        })))
        .expect(1)
        .mount(&server)
        .await;

    let c = client(&server);
    let r = c
        .masterpass_get_card(MasterpassGetCardRequest {
            token: "mp-token-1".into(),
            amount: 700,
            currency: "BYN".into(),
            session: None,
            test: None,
        })
        .await
        .expect("masterpass get_card should succeed");

    assert_eq!(r.status.as_deref(), Some("success"));
    let card = r.credit_card.expect("credit card present");
    assert_eq!(card.brand.as_deref(), Some("visa"));
    assert_eq!(card.last_4.as_deref(), Some("1097"));
    assert_eq!(
        card.token.as_deref(),
        Some("be1e89ab-1d18-4d0f-83a1-7009b333dce0")
    );
    assert_eq!(r.recommendation, Some(2));
    assert_eq!(r.required, Some(1));
}

#[tokio::test]
async fn masterpass_get_saved_card_happy_path() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/masterpass/get_saved_card"))
        .and(wiremock::matchers::header("authorization", AUTH))
        .and(wiremock::matchers::header("x-api-version", "3"))
        .and(wiremock::matchers::body_json(serde_json::json!({
            "credit_card_token": "be1e89ab-1d18-4d0f-83a1-7009b333dce0",
            "amount": 700,
            "currency": "BYN"
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "success",
            "credit_card": {
                "brand": "mastercard",
                "last_4": "5454"
            },
            "recommendation": 0,
            "required": 3
        })))
        .expect(1)
        .mount(&server)
        .await;

    let c = client(&server);
    let r = c
        .masterpass_get_saved_card(MasterpassGetSavedCardRequest {
            credit_card_token: "be1e89ab-1d18-4d0f-83a1-7009b333dce0".into(),
            amount: 700,
            currency: "BYN".into(),
            session: None,
            test: None,
        })
        .await
        .expect("masterpass get_saved_card should succeed");

    assert_eq!(r.status.as_deref(), Some("success"));
    let card = r.credit_card.expect("credit card present");
    assert_eq!(card.brand.as_deref(), Some("mastercard"));
    assert_eq!(card.last_4.as_deref(), Some("5454"));
    assert_eq!(r.recommendation, Some(0));
    assert_eq!(r.required, Some(3));
}

#[tokio::test]
async fn masterpass_delete_card_happy_path() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/masterpass/delete_card"))
        .and(wiremock::matchers::header("authorization", AUTH))
        .and(wiremock::matchers::header("x-api-version", "3"))
        .and(wiremock::matchers::body_json(serde_json::json!({
            "session": "mp-session-1",
            "token": "mp-token-1",
            "test": true
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "success"
        })))
        .expect(1)
        .mount(&server)
        .await;

    let c = client(&server);
    let r = c
        .masterpass_delete_card(MasterpassDeleteCardRequest {
            session: "mp-session-1".into(),
            token: "mp-token-1".into(),
            test: Some(true),
        })
        .await
        .expect("masterpass delete_card should succeed");

    assert_eq!(r.status.as_deref(), Some("success"));
}

#[tokio::test]
async fn masterpass_error_shape() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/masterpass/get_cards"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "failed",
            "error": "Session expired",
            "error_code": 8
        })))
        .expect(1)
        .mount(&server)
        .await;

    let c = client(&server);
    let r = c
        .masterpass_get_cards(MasterpassGetCardsRequest {
            session: "expired".into(),
            test: None,
        })
        .await
        .expect("masterpass error is a 200 with status failed");

    assert_eq!(r.status.as_deref(), Some("failed"));
    assert_eq!(r.error.as_deref(), Some("Session expired"));
    assert_eq!(r.error_code, Some(8));
}

#[tokio::test]
async fn masterpass_requests_have_no_request_envelope() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/masterpass/login"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "success",
            "session": "mp-session-1"
        })))
        .expect(1)
        .mount(&server)
        .await;

    let c = client(&server);
    c.masterpass_login(MasterpassLoginRequest {
        phone: "375291112233".into(),
        fingerprint: "fp-1".into(),
        phone_check_date: None,
        channel: None,
        test: None,
    })
    .await
    .expect("masterpass login should succeed");

    let received = server.received_requests().await.expect("requests recorded");
    assert_eq!(received.len(), 1);
    let body: serde_json::Value = received[0].body_json().expect("json body");
    assert_eq!(body["phone"], "375291112233");
    assert_eq!(body["fingerprint"], "fp-1");
    assert!(body.get("request").is_none());
}

#[tokio::test]
async fn create_payment_with_masterpass_additional_data() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/transactions/payments"))
        .and(wiremock::matchers::body_json(serde_json::json!({
            "request": {
                "amount": "700",
                "currency": "USD",
                "test": true,
                "description": "Masterpass payment",
                "tracking_id": "tid",
                "additional_data": {
                    "contract": ["recurring"],
                    "masterpass": {"params": {"session": "mp-session-1"}}
                }
            }
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "transaction": {
                "uid": "u1",
                "tracking_id": "tid",
                "additional_data": {
                    "masterpass": {"result": "success"}
                }
            }
        })))
        .expect(1)
        .mount(&server)
        .await;

    let c = client(&server);
    let t = c
        .create_payment(
            PaymentRequest {
                amount: "700".into(),
                currency: "USD".into(),
                test: true,
                description: "Masterpass payment".into(),
                tracking_id: "tid".into(),
                expired_at: None,
                dynamic_billing_descriptor: None,
                language: None,
                notification_url: None,
                verification_url: None,
                return_url: None,
                duplicate_check: None,
                billing_address: None,
                credit_card: None,
                customer: None,
                additional_data: Some(AdditionalData {
                    browser: None,
                    contract: Some(vec!["recurring".into()]),
                    referer: None,
                    masterpass: Some(MasterpassData {
                        params: Some(MasterpassParams {
                            session: Some("mp-session-1".into()),
                        }),
                    }),
                    split: None,
                    smart_routing_options: None,
                    excluded_gateways: None,
                    extra: None,
                }),
                encrypted_data: None,
                fiscalization: None,
                custom_fields: None,
            },
            None,
        )
        .await
        .expect("masterpass payment should succeed");

    assert_eq!(t.uid, "u1");
    Mock::given(method("GET"))
        .and(path("/transactions/u1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "transaction": {
                "uid": "u1",
                "additional_data": {"masterpass": {"result": "success"}}
            }
        })))
        .expect(1)
        .mount(&server)
        .await;
    let transaction = c.get_transaction(&t.uid).await.unwrap();
    let ad = transaction
        .additional_data
        .expect("additional data present");
    assert_eq!(ad["masterpass"]["result"], "success");
}

#[tokio::test]
async fn create_authorization_with_masterpass_additional_data() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/transactions/authorizations"))
        .and(wiremock::matchers::header("authorization", AUTH))
        .and(wiremock::matchers::header("x-api-version", "3"))
        .and(wiremock::matchers::body_json(serde_json::json!({
            "request": {
                "amount": 700,
                "currency": "USD",
                "description": "Masterpass authorization",
                "tracking_id": "tid",
                "credit_card": {"token": "saved-card-token"},
                "additional_data": {
                    "contract": ["recurring"],
                    "masterpass": {"params": {"session": "mp-session-1"}},
                    "custom_data": "preserved"
                }
            }
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "transaction": {"uid": "u1", "status": "successful"}
        })))
        .expect(1)
        .mount(&server)
        .await;

    let response = client(&server)
        .create_authorization(
            AuthorizationRequest {
                amount: 700,
                currency: "USD".into(),
                description: "Masterpass authorization".into(),
                payment_method_type: None,
                tracking_id: "tid".into(),
                test: None,
                duplicate_check: None,
                language: None,
                notification_url: None,
                return_url: None,
                expired_at: None,
                dynamic_billing_descriptor: None,
                credit_card: Some(
                    serde_json::from_value(serde_json::json!({
                        "token": "saved-card-token"
                    }))
                    .unwrap(),
                ),
                customer: None,
                billing_address: None,
                additional_data: Some(AdditionalData {
                    browser: None,
                    contract: Some(vec!["recurring".into()]),
                    referer: None,
                    masterpass: Some(MasterpassData {
                        params: Some(MasterpassParams {
                            session: Some("mp-session-1".into()),
                        }),
                    }),
                    split: None,
                    smart_routing_options: None,
                    excluded_gateways: None,
                    extra: Some(serde_json::json!({"custom_data": "preserved"})),
                }),
                verification_url: None,
                custom_fields: None,
            },
            None,
        )
        .await
        .unwrap();
    assert_eq!(response.uid, "u1");
    assert_eq!(response.status.as_deref(), Some("successful"));
}

#[test]
fn masterpass_optional_params_are_omitted() {
    assert_eq!(
        serde_json::to_value(MasterpassData { params: None }).unwrap(),
        serde_json::json!({})
    );
    assert_eq!(
        serde_json::to_value(MasterpassData {
            params: Some(MasterpassParams { session: None }),
        })
        .unwrap(),
        serde_json::json!({"params": {}})
    );
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
        .create_payment(
            PaymentRequest {
                amount: "".into(),
                currency: "USD".into(),
                test: true,
                description: "d".into(),
                tracking_id: "t".into(),
                expired_at: None,
                dynamic_billing_descriptor: None,
                duplicate_check: None,
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
            },
            None,
        )
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
    for token in [
        "saved-card-token",
        "$begateway_google_pay_1_0_0$eyJ0ZXN0Ijp0cnVlfQ==",
        "$begateway_google_pay_decrypted_1_0_0$eyJ0ZXN0Ijp0cnVlfQ==",
        "$begateway_samsung_pay_decrypted_1_0_0$eyJ0ZXN0Ijp0cnVlfQ==",
    ] {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/transactions/authorizations"))
            .and(wiremock::matchers::body_json(serde_json::json!({
                "request": {
                    "amount": 100,
                    "currency": "USD",
                    "description": "Test",
                    "tracking_id": "x",
                    "test": true,
                    "duplicate_check": false,
                    "credit_card": {"token": token},
                    "additional_data": {"contract": ["recurring"]},
                }
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "transaction": {
                    "uid": "b6c446e4-b8a8-496f-bbc8-497d34644c7b",
                    "status": "incomplete",
                    "credit_card": {"token": token},
                    "code": "S.0000",
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
            .create_authorization(
                AuthorizationRequest {
                    amount: 100,
                    currency: "USD".into(),
                    description: "Test".into(),
                    payment_method_type: None,
                    tracking_id: "x".into(),
                    test: Some(true),
                    duplicate_check: Some(false),
                    language: None,
                    notification_url: None,
                    return_url: None,
                    expired_at: None,
                    dynamic_billing_descriptor: None,
                    credit_card: Some(CreditCardRaw {
                        number: None,
                        verification_value: None,
                        holder: None,
                        exp_month: None,
                        exp_year: None,
                        save_card: None,
                        token: Some(token.into()),
                        skip_three_d_secure_verification: None,
                        force_three_d_secure_verification: None,
                    }),
                    customer: None,
                    billing_address: None,
                    additional_data: Some(bepaid::types::AdditionalData {
                        browser: None,
                        contract: Some(vec!["recurring".into()]),
                        referer: None,
                        masterpass: None,
                        split: None,
                        smart_routing_options: None,
                        excluded_gateways: None,
                        extra: None,
                    }),
                    verification_url: None,
                    custom_fields: None,
                },
                None,
            )
            .await
            .expect("auth should succeed");

        assert_eq!(t.uid, "b6c446e4-b8a8-496f-bbc8-497d34644c7b");
        assert_eq!(t.status.as_deref(), Some("incomplete"));
        assert_eq!(t.credit_card.unwrap().token.as_deref(), Some(token));
        assert_eq!(t.code.as_deref(), Some("S.0000"));
        assert!(
            t.redirect_url
                .as_deref()
                .unwrap()
                .ends_with("/process/b6c446e4")
        );
    }
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
        .capture(
            CaptureRequest {
                parent_uid: "4298aabd".into(),
                amount: 460,
                tracking_id: None,
                additional_data: None,
            },
            None,
        )
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
        .void(
            VoidRequest {
                parent_uid: "auth-uid".into(),
                amount: 50,
                tracking_id: Some("tracking_id_1".into()),
                additional_data: None,
            },
            None,
        )
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
        .refund(
            RefundRequest {
                parent_uid: "1-310b0da80b".into(),
                amount: 50,
                reason: "Client request".into(),
                tracking_id: None,
                additional_data: None,
                fiscalization: None,
            },
            None,
        )
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
        .and(wiremock::matchers::header("x-api-version", "2"))
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
                expired_at: None,
                additional_data: None,
                custom_fields: None,
            },
            customer: None,
            dynamic_billing_descriptor: None,
            travel: None,
            fiscalization: None,
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
                expired_at: None,
                additional_data: Some(bepaid::types::CheckoutAdditionalData {
                    extra: None,
                    contract: Some(vec!["recurring".into()]),
                }),
                custom_fields: None,
            },
            customer: None,
            dynamic_billing_descriptor: None,
            travel: None,
            fiscalization: None,
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
            iframe: None,
            verification_url: None,
            customer: None,
            payment_method: serde_json::json!({"type": "mts_money", "confirm_agreement": "accept"}),
            additional_data: None,
            custom_fields: None,
        }, None)
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
                    "method": method_payload,
                }
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "transaction": { "uid": "u", "type": "payment", "status": "pending" }
            })))
            .expect(1)
            .mount(&server)
            .await;
        let t = c
            .create_apm_payment(req.clone(), None)
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
                "method": {
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
    let t = c
        .create_apm_payment(req, None)
        .await
        .expect("apm should succeed");
    assert_eq!(t.status.as_deref(), Some("pending"));
}

fn erip_payment_payload() -> serde_json::Value {
    serde_json::from_str(r#"{
        "uid": "erip-1", "id": "erip-1", "order_id": "123456789012",
        "type": "payment", "status": "successful", "payment_method_type": "erip",
        "amount": 1000, "currency": "BYN", "description": "Order 123",
        "tracking_id": "order-123", "test": true, "language": "ru", "version": 2,
        "created_at": "2026-09-01T10:00:00Z", "updated_at": "2026-09-02T10:00:00Z",
        "expired_at": "2026-10-01T10:00:00Z", "paid_at": "2026-09-02T10:00:00Z",
        "closed_at": "2026-09-02T11:00:00Z", "settled_at": "2026-09-03T10:00:00Z",
        "manually_corrected_at": "2026-09-04T10:00:00Z", "psp_settled_at": null,
        "registry_id": null, "smart_routing_verification": {"status": "successful"},
        "payment": {"gateway_id": 3483, "status": "successful"},
        "customer": {"email": "buyer@example.com", "ip": "127.0.0.1"},
        "billing_address": {"middle_name": "Ivanovich", "country": "BY"},
        "additional_data": {"notifications": ["email"], "receipt_text": ["Thanks"]},
        "erip": {
            "request_id": "00000001", "service_no": 99999999, "account_number": "123",
            "transaction_id": 123, "service_no_erip": "12345678", "agent_code": null,
            "instruction": ["Payments -> Shop"], "service_info": ["Order 123"],
            "receipt": ["Thanks"], "qr_code_raw": "cXI=", "qr_code": "data:image/png;base64,cXI=",
            "banks": [{"name": "Bank", "icon": "data:image/svg+xml;base64,cXI=",
                "platform_urls": {"ios": "bank://pay#", "android": "bank://pay#", "huaweiapp": null}}]
        }
    }"#).unwrap()
}

#[tokio::test]
async fn erip_create_uses_dedicated_route_and_preserves_response() {
    let server = MockServer::start().await;
    let other = MockServer::start().await;
    let payload = erip_payment_payload();
    let customer = serde_json::json!({
        "first_name": "Ivan", "middle_name": "Ivanovich", "last_name": "Petrov",
        "country": "BY", "city": "Minsk", "zip": "220000", "address": "Street 1"
    });
    let req = ApmPaymentRequest {
        description: Some("Order 123".into()),
        ip: Some("127.0.0.1".into()),
        customer: Some(serde_json::from_value(customer.clone()).unwrap()),
        ..ApmPaymentRequest::erip(1000, "BYN", "123", "99999999")
    };
    for (route, method_key) in [
        ("/beyag/payments", "payment_method"),
        ("/beyag/transactions/payments", "method"),
    ] {
        Mock::given(method("POST"))
            .and(path(route))
            .and(wiremock::matchers::header("authorization", AUTH))
            .and(wiremock::matchers::header("content-type", "application/json"))
            .and(wiremock::matchers::header("accept", "application/json"))
            .and(wiremock::matchers::body_json(serde_json::json!({"request": {
                "amount": 1000, "currency": "BYN", "description": "Order 123", "ip": "127.0.0.1",
                "customer": customer,
                (method_key): {"type": "erip", "account_number": "123", "service_no": "99999999"}
            }})))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({"transaction": payload})))
            .expect(1)
            .mount(&server)
            .await;
    }
    let c = BepaidClient::with_urls(
        SHOP_ID,
        SECRET,
        &other.uri(),
        &other.uri(),
        &server.uri(),
        &other.uri(),
    );
    let t = c.create_erip_payment(req.clone()).await.unwrap();
    assert_eq!(t.uid.as_deref(), Some("erip-1"));
    assert_eq!(t.id.as_deref(), Some("erip-1"));
    assert_eq!(t.order_id.as_deref(), Some("123456789012"));
    assert_eq!(t.payment_method_type.as_deref(), Some("erip"));
    assert_eq!(t.description.as_deref(), Some("Order 123"));
    assert_eq!(t.erip, Some(payload["erip"].clone()));
    assert_eq!(t.version, Some(2));
    assert_eq!(t.expired_at.as_deref(), payload["expired_at"].as_str());
    assert_eq!(t.paid_at.as_deref(), payload["paid_at"].as_str());
    assert_eq!(t.updated_at.as_deref(), payload["updated_at"].as_str());
    assert_eq!(t.closed_at.as_deref(), payload["closed_at"].as_str());
    assert_eq!(t.settled_at.as_deref(), payload["settled_at"].as_str());
    assert_eq!(
        t.manually_corrected_at.as_deref(),
        payload["manually_corrected_at"].as_str()
    );
    assert!(t.psp_settled_at.is_none());
    assert!(t.registry_id.is_none());
    assert_eq!(
        t.smart_routing_verification.unwrap().status.as_deref(),
        Some("successful")
    );
    assert_eq!(t.billing_address, Some(payload["billing_address"].clone()));
    assert_eq!(t.customer, Some(payload["customer"].clone()));
    assert_eq!(t.additional_data, Some(payload["additional_data"].clone()));
    assert_eq!(
        c.create_apm_payment(req, None)
            .await
            .unwrap()
            .uid
            .as_deref(),
        Some("erip-1")
    );
    let received = server.received_requests().await.unwrap();
    assert_eq!(received.len(), 2);
    assert!(
        received
            .iter()
            .all(|r| !r.headers.contains_key("x-api-version"))
    );
    assert!(other.received_requests().await.unwrap().is_empty());
}

#[tokio::test]
async fn erip_refund_create_and_lookup_preserve_metadata() {
    let server = MockServer::start().await;
    let payload = serde_json::json!({
        "type": "refund", "uid": "refund-1", "id": "refund-1", "parent_uid": "erip-1",
        "status": "pending", "message": "Created", "amount": 50, "currency": "BYN",
        "reason": "Client request", "paid_at": "2026-09-17T10:00:00Z", "language": "ru",
        "created_at": "2026-09-17T10:00:00Z", "version": 2, "test": true,
        "settled_at": "2026-09-18T10:00:00Z", "psp_settled_at": null, "registry_id": null,
        "payment_method_type": "erip", "erip": {"service_no": "6777"},
        "refund": {"ref_id": "144", "rrn": null, "message": "Created", "status": "pending"}
    });
    Mock::given(method("POST"))
        .and(path("/beyag/refunds"))
        .and(wiremock::matchers::header("authorization", AUTH))
        .and(wiremock::matchers::header("RequestID", "refund-request"))
        .and(wiremock::matchers::body_json(
            serde_json::json!({"request": {
                "parent_uid": "erip-1", "reason": "Client request", "amount": 50
            }}),
        ))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(serde_json::json!({"transaction": payload})),
        )
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/beyag/refunds/refund-1"))
        .and(wiremock::matchers::header("authorization", AUTH))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(serde_json::json!({"transaction": payload})),
        )
        .expect(1)
        .mount(&server)
        .await;
    let c = client(&server);
    let created = c
        .apm_full_refund("erip-1", "Client request", Some(50), Some("refund-request"))
        .await
        .unwrap();
    let found = c.get_apm_refund("refund-1").await.unwrap();
    for t in [created, found] {
        assert_eq!(t.uid.as_deref(), Some("refund-1"));
        assert_eq!(t.id.as_deref(), Some("refund-1"));
        assert_eq!(t.parent_uid.as_deref(), Some("erip-1"));
        assert_eq!(t.reason.as_deref(), Some("Client request"));
        assert_eq!(t.amount, Some(50));
        assert_eq!(t.paid_at.as_deref(), payload["paid_at"].as_str());
        assert_eq!(t.language.as_deref(), Some("ru"));
        assert_eq!(t.version, Some(2));
        assert_eq!(t.payment_method_type.as_deref(), Some("erip"));
        assert_eq!(t.erip, Some(payload["erip"].clone()));
        assert_eq!(t.refund, Some(payload["refund"].clone()));
        assert_eq!(t.settled_at.as_deref(), payload["settled_at"].as_str());
        assert!(t.psp_settled_at.is_none());
        assert!(t.registry_id.is_none());
    }
    let received = server.received_requests().await.unwrap();
    assert_eq!(received.len(), 2);
    assert!(
        received
            .iter()
            .all(|r| !r.headers.contains_key("x-api-version"))
    );
    assert!(received[1].body.is_empty());
}

#[tokio::test]
async fn erip_tree_sends_flat_requests_and_returns_bare_array_or_object() {
    use bepaid::types::{EripPayListCustomer, EripPayListRequest};
    let server = MockServer::start().await;
    for (req, expected, response) in [
        (
            EripPayListRequest {
                terminal_id: "10000002".into(),
                pay_code: "11000000000".into(),
                di_type: "9191".into(),
                test: None,
                erip_session_id: None,
                attributes: None,
                customer: None,
            },
            serde_json::json!({"terminal_id": "10000002", "pay_code": "11000000000", "di_type": "9191"}),
            serde_json::json!([{"code": "10004372291", "name": "Utilities", "di_type": "9191"}]),
        ),
        (
            EripPayListRequest {
                terminal_id: "10000002".into(),
                pay_code: "10004372291".into(),
                di_type: "9191".into(),
                test: Some(true),
                erip_session_id: Some("session-1".into()),
                attributes: Some(serde_json::json!({"1001": "0291234567"})),
                customer: None,
            },
            serde_json::json!({"terminal_id": "10000002", "pay_code": "10004372291", "di_type": "9191",
            "test": true, "erip_session_id": "session-1", "attributes": {"1001": "0291234567"}}),
            serde_json::json!({"billed_amount": "122.43", "fixed_amount": true, "erip_session_id": "session-2",
            "customer_name": {"middle_name": null}, "information_attributes": [], "erip_commision": "0.00"}),
        ),
        (
            EripPayListRequest {
                terminal_id: "10000002".into(),
                pay_code: "10004345361".into(),
                di_type: "9191".into(),
                test: Some(false),
                erip_session_id: None,
                attributes: None,
                customer: Some(EripPayListCustomer {
                    personal_account: Some("112233".into()),
                    erip_account: Some("445566".into()),
                }),
            },
            serde_json::json!({"terminal_id": "10000002", "pay_code": "10004345361", "di_type": "9191",
            "test": false, "customer": {"personal_account": "112233", "erip_account": "445566"}}),
            serde_json::json!({"billed_amount": "0.00", "fixed_amount": false, "erip_session_id": "session-3"}),
        ),
    ] {
        Mock::given(method("POST"))
            .and(path("/beyag/gateways/komplat/get_pay_list"))
            .and(wiremock::matchers::header("authorization", AUTH))
            .and(wiremock::matchers::body_json(expected))
            .respond_with(ResponseTemplate::new(200).set_body_json(response.clone()))
            .expect(1)
            .mount(&server)
            .await;
        assert_eq!(
            client(&server).get_erip_pay_list(req).await.unwrap(),
            response
        );
    }
    let received = server.received_requests().await.unwrap();
    assert_eq!(received.len(), 3);
    assert!(
        received
            .iter()
            .all(|r| !r.headers.contains_key("x-api-version"))
    );
    assert_eq!(
        serde_json::to_value(EripPayListCustomer {
            personal_account: None,
            erip_account: None
        })
        .unwrap(),
        serde_json::json!({})
    );
}

#[tokio::test]
async fn erip_komplat_uses_existing_authorization_extra() {
    let server = MockServer::start().await;
    let komplat = serde_json::json!({"komplat": {
        "pay_code": "10000156731", "di_type": "9191", "erip_session_id": "session-1"
    }});
    Mock::given(method("POST"))
        .and(path("/transactions/authorizations"))
        .and(wiremock::matchers::header("authorization", AUTH))
        .and(wiremock::matchers::header("x-api-version", "3"))
        .and(wiremock::matchers::body_json(
            serde_json::json!({"request": {
                "amount": 100, "currency": "BYN", "description": "ERIP", "tracking_id": "tree-1",
                "credit_card": {"token": "card-token"}, "additional_data": komplat
            }}),
        ))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!({"transaction": {"uid": "auth-1"}})),
        )
        .expect(1)
        .mount(&server)
        .await;
    let t = client(&server)
        .create_authorization(
            AuthorizationRequest {
                amount: 100,
                currency: "BYN".into(),
                description: "ERIP".into(),
                tracking_id: "tree-1".into(),
                payment_method_type: None,
                test: None,
                duplicate_check: None,
                language: None,
                notification_url: None,
                return_url: None,
                expired_at: None,
                dynamic_billing_descriptor: None,
                credit_card: Some(
                    serde_json::from_value(serde_json::json!({"token": "card-token"})).unwrap(),
                ),
                customer: None,
                billing_address: None,
                verification_url: None,
                custom_fields: None,
                additional_data: Some(AdditionalData {
                    browser: None,
                    contract: None,
                    referer: None,
                    masterpass: None,
                    split: None,
                    smart_routing_options: None,
                    excluded_gateways: None,
                    extra: Some(komplat),
                }),
            },
            None,
        )
        .await
        .unwrap();
    assert_eq!(t.uid, "auth-1");
}

#[test]
fn erip_webhooks_preserve_standard_refund_and_external_fields() {
    let payload = erip_payment_payload();
    let t = parse_webhook(&serde_json::json!({"transaction": payload}).to_string())
        .unwrap()
        .transaction;
    assert_eq!(t.erip, Some(payload["erip"].clone()));
    assert_eq!(t.payment_method_type.as_deref(), Some("erip"));
    assert_eq!(t.id.as_deref(), Some("erip-1"));
    assert_eq!(t.order_id.as_deref(), Some("123456789012"));
    assert_eq!(t.expired_at.as_deref(), payload["expired_at"].as_str());
    assert_eq!(t.paid_at.as_deref(), payload["paid_at"].as_str());
    assert_eq!(t.updated_at.as_deref(), payload["updated_at"].as_str());
    assert_eq!(t.closed_at.as_deref(), payload["closed_at"].as_str());
    assert_eq!(t.settled_at.as_deref(), payload["settled_at"].as_str());
    assert_eq!(
        t.manually_corrected_at.as_deref(),
        payload["manually_corrected_at"].as_str()
    );
    assert_eq!(t.version, Some(2));
    assert!(t.psp_settled_at.is_none());
    assert!(t.registry_id.is_none());
    assert_eq!(t.billing_address, Some(payload["billing_address"].clone()));
    for status in ["successful", "failed"] {
        let refund = serde_json::json!({"ref_id": null, "message": "Processed", "status": status});
        let t = parse_webhook(
            &serde_json::json!({"transaction": {
                "uid": "refund-1", "id": "refund-1", "parent_uid": "erip-1", "type": "refund",
                "status": status, "reason": "Client request", "refund": refund, "version": 9,
                "paid_at": "2026-09-17T10:00:00Z", "language": "ru"
            }})
            .to_string(),
        )
        .unwrap()
        .transaction;
        assert_eq!(t.id.as_deref(), Some("refund-1"));
        assert_eq!(t.parent_uid.as_deref(), Some("erip-1"));
        assert_eq!(t.reason.as_deref(), Some("Client request"));
        assert_eq!(t.refund, Some(refund));
        assert_eq!(t.version, Some(9));
        assert_eq!(t.status, status);
        assert_eq!(t.language.as_deref(), Some("ru"));
        assert_eq!(t.paid_at.as_deref(), Some("2026-09-17T10:00:00Z"));
    }
    let external =
        serde_json::json!({"account": "112233", "service_no": "6740", "transaction_id": 42});
    let t = parse_webhook(
        &serde_json::json!({"transaction": {
            "uid": "external-1", "type": "payment", "status": "successful",
            "method_type": "erip_external", "erip_external": external
        }})
        .to_string(),
    )
    .unwrap()
    .transaction;
    assert_eq!(t.method_type.as_deref(), Some("erip_external"));
    assert_eq!(
        t.extra.unwrap(),
        serde_json::json!({"erip_external": external})
    );
    assert!(t.payment_method_type.is_none());
}

#[tokio::test]
async fn erip_widget_metadata_stays_flat_with_typed_fields() {
    use bepaid::types::{BillingAddress, CheckoutAdditionalData, CheckoutOrder, PaymentMethod};
    let server = MockServer::start().await;
    for route in ["/ctp/api/checkouts", "/payments/tokens"] {
        Mock::given(method("POST"))
            .and(path(route))
            .and(wiremock::matchers::body_json(serde_json::json!({"checkout": {
                "transaction_type": "payment",
                "payment_method": {"types": ["bank_transfer"], "excluded_types": ["erip"],
                    "excluded_brands": ["visa"], "bank_transfer": {"account": "112233"}},
                "order": {"amount": 100, "currency": "BYN", "additional_data": {
                    "contract": ["recurring"], "receipt_text": ["Thanks"], "notifications": ["email"]}}
            }})))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({"checkout": {"token": "widget-1"}})))
            .expect(1).mount(&server).await;
    }
    let method_data = serde_json::json!({"types": ["bank_transfer"], "excluded_types": ["erip"],
        "excluded_brands": ["visa"], "bank_transfer": {"account": "112233"}});
    let payment_method: PaymentMethod = serde_json::from_value(method_data.clone()).unwrap();
    assert_eq!(serde_json::to_value(&payment_method).unwrap(), method_data);
    let req = CheckoutRequest {
        transaction_type: "payment".into(),
        test: None,
        attempts: None,
        iframe: None,
        settings: None,
        payment_method: Some(payment_method),
        credit_card: None,
        customer: None,
        dynamic_billing_descriptor: None,
        travel: None,
        fiscalization: None,
        order: CheckoutOrder {
            amount: 100,
            currency: "BYN".into(),
            description: None,
            tracking_id: None,
            expired_at: None,
            custom_fields: None,
            additional_data: Some(CheckoutAdditionalData {
                contract: Some(vec!["recurring".into()]),
                extra: Some(
                    serde_json::json!({"receipt_text": ["Thanks"], "notifications": ["email"]}),
                ),
            }),
        },
    };
    let c = client(&server);
    assert_eq!(c.create_checkout(&req).await.unwrap().token, "widget-1");
    assert_eq!(
        c.create_payment_token(&req).await.unwrap().token,
        "widget-1"
    );
    assert_eq!(
        serde_json::to_value(CheckoutAdditionalData {
            contract: None,
            extra: None
        })
        .unwrap(),
        serde_json::json!({})
    );
    assert_eq!(
        serde_json::to_value(PaymentMethod {
            types: None,
            excluded_types: None,
            excluded_brands: None,
            extra: None
        })
        .unwrap(),
        serde_json::json!({})
    );
    let address: BillingAddress =
        serde_json::from_value(serde_json::json!({"middle_name": "Ivanovich"})).unwrap();
    assert_eq!(address.middle_name.as_deref(), Some("Ivanovich"));
    assert_eq!(
        serde_json::to_value(address).unwrap(),
        serde_json::json!({"middle_name": "Ivanovich"})
    );
}

#[tokio::test]
async fn erip_refund_requires_amount_before_request() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/beyag/refunds"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "transaction": {"uid": "refund-1", "status": "pending"}
        })))
        .expect(0)
        .mount(&server)
        .await;
    let result = client(&server)
        .apm_full_refund("payment-1", "Client request", None, None)
        .await;
    let BepaidError::InvalidRequest(message) = result.unwrap_err() else {
        panic!("expected local validation error")
    };
    assert_eq!(message, "amount is required for /beyag/refunds");
    assert!(server.received_requests().await.unwrap().is_empty());
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
        .apm_refund(
            bepaid::types::ApmRefundRequest {
                parent_uid: "1-310b0da80b".into(),
                reason: "reason".into(),
                amount: Some(50),
                tracking_id: None,
                additional_data: None,
            },
            None,
        )
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
async fn checkout_webhook_parses() {
    let body = r#"{
        "token": "311300d08dc7f22ae37272fac6513921d4c99ca24dcaccf4392a2606fe8f1877",
        "shop_id": 363,
        "transaction_type": "payment",
        "gateway_response": null,
        "order": {"currency": "USD", "amount": 4299},
        "settings": {"language": "en"},
        "customer": {"email": "jake@example.com"},
        "finished": false,
        "expired": true,
        "shop": {"name": "Shop"},
        "test": false,
        "status": "error",
        "message": "Token is expired.",
        "payment_method": {"types": ["erip"]}
    }"#;

    let s = parse_checkout_webhook(body).expect("should parse");
    assert_eq!(
        s.token.as_deref(),
        Some("311300d08dc7f22ae37272fac6513921d4c99ca24dcaccf4392a2606fe8f1877")
    );
    assert_eq!(s.expired, Some(true));
    assert_eq!(s.finished, Some(false));
    assert_eq!(s.test, Some(false));
    assert_eq!(s.status.as_deref(), Some("error"));
    assert_eq!(s.message.as_deref(), Some("Token is expired."));
    assert!(s.customer.is_some());
    assert!(s.payment_method.is_some());
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

#[test]
fn batch7_customer_fields_round_trip() {
    let payload = serde_json::json!({"gender": "male", "street": "Main", "state": "Minsk"});
    let customer: bepaid::types::Customer = serde_json::from_value(payload.clone()).unwrap();
    assert_eq!(serde_json::to_value(customer).unwrap(), payload);
    let empty: bepaid::types::Customer = serde_json::from_value(serde_json::json!({})).unwrap();
    assert_eq!(serde_json::to_value(empty).unwrap(), serde_json::json!({}));
}

#[test]
fn batch7_apm_response_preserves_method_data() {
    for form in [
        serde_json::json!({"action": "https://example.com/pay", "fields": {"token": "t"}}),
        serde_json::json!("<form>pay</form>"),
    ] {
        let extra = serde_json::json!({
            "krok": {"qr_code": "data:image/png;base64,cXI=", "banks": [{"name": "Bank", "platform_urls": {"ios": "bank://pay"}}]},
            "pix": {"qr_code": "pix-code"},
            "crypto_currency": {"amount": "0.000000123456789", "address": "wallet"},
            "sbp": {"token": "sbp-token"},
            "sberpay_qr_deeplink": {"deeplink": "bank://pay"},
            "form": form
        });
        let mut payload = extra.clone();
        payload["uid"] = serde_json::json!("apm-7");
        let payment: bepaid::types::ApmPaymentResponse =
            serde_json::from_value(payload.clone()).unwrap();
        let transaction: bepaid::types::Transaction = serde_json::from_value(payload).unwrap();
        assert_eq!(payment.extra, Some(extra.clone()));
        assert_eq!(transaction.extra, Some(extra));
    }
}

#[tokio::test]
async fn batch7_confirm_modes_preserve_metadata() {
    for (confirm_type, phone, reference, skip, body, envelope) in [
        (
            Some("confirm"),
            None,
            None,
            None,
            serde_json::json!({"request": {"confirm_type": "confirm"}}),
            "transaction",
        ),
        (
            Some("cancel"),
            None,
            None,
            None,
            serde_json::json!({"request": {"confirm_type": "cancel"}}),
            "transaction",
        ),
        (
            None,
            Some("+79991234567"),
            None,
            None,
            serde_json::json!({"phone": "+79991234567"}),
            "response",
        ),
        (
            None,
            None,
            Some("receipt-7"),
            Some(false),
            serde_json::json!({"request": {"transaction_reference": "receipt-7", "skip_duplicate_check": false}}),
            "response",
        ),
        (
            None,
            None,
            Some("receipt-7"),
            None,
            serde_json::json!({"request": {"transaction_reference": "receipt-7"}}),
            "response",
        ),
    ] {
        let server = MockServer::start().await;
        let payload = serde_json::json!({"parent_uid": "apm-7", "type": confirm_type.unwrap_or("confirm"),
            "status": "successful", "message": "Processed", "created_at": "2026-09-17T10:00:00Z", "amount": 700, "currency": "BYN"});
        Mock::given(method("POST"))
            .and(path("/beyag/transactions/apm-7/confirm"))
            .and(wiremock::matchers::header("authorization", AUTH))
            .and(wiremock::matchers::header("RequestID", "batch7"))
            .and(wiremock::matchers::body_json(body))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(serde_json::json!({(envelope): payload})),
            )
            .expect(1)
            .mount(&server)
            .await;
        let response = client(&server)
            .confirm_apm_payment(
                "apm-7",
                ApmConfirmRequest {
                    confirm_type: confirm_type.map(String::from),
                    phone: phone.map(String::from),
                    transaction_reference: reference.map(String::from),
                    skip_duplicate_check: skip,
                },
                Some("batch7"),
            )
            .await
            .unwrap();
        assert_eq!(response.parent_uid.as_deref(), Some("apm-7"));
        assert_eq!(
            response.tx_type.as_deref(),
            Some(confirm_type.unwrap_or("confirm"))
        );
        assert_eq!(response.status.as_deref(), Some("successful"));
        assert_eq!(response.message.as_deref(), Some("Processed"));
        assert_eq!(response.created_at.as_deref(), Some("2026-09-17T10:00:00Z"));
        assert_eq!(response.amount, Some(700));
        assert_eq!(response.currency.as_deref(), Some("BYN"));
    }
}

#[tokio::test]
async fn batch7_confirm_rejects_mixed_and_invalid_modes_locally() {
    let server = MockServer::start().await;
    let c = client(&server);
    for mask in [3_u8, 5, 6, 7] {
        let error = c
            .confirm_apm_payment(
                "apm-7",
                ApmConfirmRequest {
                    confirm_type: (mask & 1 != 0).then(|| "confirm".into()),
                    phone: (mask & 2 != 0).then(|| "+79991234567".into()),
                    transaction_reference: (mask & 4 != 0).then(|| "receipt-7".into()),
                    skip_duplicate_check: None,
                },
                None,
            )
            .await
            .unwrap_err();
        assert!(matches!(error, BepaidError::InvalidRequest(_)), "{error:?}");
    }
    for (confirm_type, phone, skip) in [
        (Some("other"), None, None),
        (Some("confirm"), None, Some(false)),
        (None, Some("+79991234567"), Some(true)),
    ] {
        let error = c
            .confirm_apm_payment(
                "apm-7",
                ApmConfirmRequest {
                    confirm_type: confirm_type.map(String::from),
                    phone: phone.map(String::from),
                    transaction_reference: None,
                    skip_duplicate_check: skip,
                },
                None,
            )
            .await
            .unwrap_err();
        assert!(matches!(error, BepaidError::InvalidRequest(_)), "{error:?}");
    }
    assert!(server.received_requests().await.unwrap().is_empty());
}

#[tokio::test]
async fn batch7_mts_v2_and_widget_v3_contracts() {
    for (test, activated, error_code) in [
        (Some(true), Some(true), serde_json::Value::Null),
        (Some(false), Some(false), serde_json::json!(42)),
        (None, None, serde_json::json!("unavailable")),
    ] {
        for version in ["2", "3"] {
            let server = MockServer::start().await;
            let mut request = serde_json::json!({"customer": {"phone": "375295222222"}});
            if let Some(test) = test {
                request["test"] = serde_json::json!(test);
            }
            Mock::given(method("POST"))
                .and(path(if version == "2" { "/beyag/gateways/mts_money/check_service" } else { "/beyag/gateways/mts_money_widget/check_service" }))
                .and(wiremock::matchers::header("authorization", AUTH))
                .and(wiremock::matchers::header("X-API-Version", version))
                .and(wiremock::matchers::body_json(serde_json::json!({"request": request})))
                .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                    "service_activated": activated, "message": "Checked", "validation": {"operator": "mts", "message": "Provider"}, "error_code": error_code
                }))).expect(1).mount(&server).await;
            let c = client(&server);
            let response = if version == "2" {
                c.check_mts_service_v2("375295222222", test).await.unwrap()
            } else {
                c.check_mts_service(bepaid::types::CheckServiceRequest {
                    test,
                    customer: serde_json::from_value(request["customer"].clone()).unwrap(),
                })
                .await
                .unwrap()
            };
            assert_eq!(response.service_activated, activated);
            assert_eq!(response.message.as_deref(), Some("Checked"));
            let validation = response.validation.unwrap();
            assert_eq!(validation.operator.as_deref(), Some("mts"));
            assert_eq!(validation.message.as_deref(), Some("Provider"));
            assert_eq!(
                response.error_code,
                if error_code.is_null() {
                    None
                } else {
                    Some(error_code.clone())
                }
            );
        }
    }
}

#[tokio::test]
async fn batch7_qiwi_terminal_success_and_errors() {
    for (status, body, expected) in [
        (
            200,
            r#"{"transaction":{"uid":"qiwi-7"}}"#,
            Some(serde_json::json!({"transaction": {"uid": "qiwi-7"}})),
        ),
        (200, "[]", Some(serde_json::json!([]))),
        (200, "", Some(serde_json::json!({}))),
        (204, "", Some(serde_json::json!({}))),
        (200, "not json", None),
        (200, " ", None),
        (
            400,
            r#"{"message":"Rejected","error_code":"bad_account"}"#,
            None,
        ),
        (500, "", None),
    ] {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/beyag/testing/payment"))
            .and(wiremock::matchers::header("authorization", AUTH))
            .and(wiremock::matchers::header("content-type", "application/json"))
            .and(wiremock::matchers::header("accept", "application/json"))
            .and(wiremock::matchers::body_json(serde_json::json!({"request": {
                "amount": 700, "currency": "RUB", "method": {"type": "qiwi_terminal", "account": "test-account"}, "test": true
            }})))
            .respond_with(ResponseTemplate::new(status).set_body_string(body))
            .expect(1).mount(&server).await;
        let result = client(&server)
            .test_qiwi_terminal_payment(700, "RUB", "test-account")
            .await;
        if let Some(expected) = expected {
            assert_eq!(result.unwrap(), expected);
        } else if status >= 400 {
            let BepaidError::Api(error) = result.unwrap_err() else {
                panic!("expected API error")
            };
            assert_eq!(error.status, status);
            if status == 400 {
                assert_eq!(error.message, "Rejected");
                assert_eq!(error.error_code.as_deref(), Some("bad_account"));
            }
        } else {
            assert!(matches!(result, Err(BepaidError::Json(_))), "{result:?}");
        }
        let received = server.received_requests().await.unwrap();
        assert!(!received[0].headers.contains_key("x-api-version"));
    }
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
                confirm_type: None,
                skip_duplicate_check: Some(false),
                transaction_reference: Some("receipt-123".into()),
                phone: None,
            },
            None,
        )
        .await
        .expect("confirm should succeed");

    assert_eq!(r.status.as_deref(), Some("successful"));
    assert_eq!(r.tx_type.as_deref(), Some("confirm"));
}

#[tokio::test]
async fn flat_http_error_preserves_message_and_errors() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/transactions/missing"))
        .respond_with(ResponseTemplate::new(400).set_body_json(serde_json::json!({
            "error_code": "request_validation_error",
            "message": "Validation failed",
            "errors": {"amount": ["must be positive"]}
        })))
        .expect(1)
        .mount(&server)
        .await;
    let err = client(&server)
        .get_transaction("missing")
        .await
        .unwrap_err();
    let BepaidError::Api(e) = err else {
        panic!("expected API error")
    };
    assert_eq!(e.status, 400);
    assert_eq!(e.message, "Validation failed");
    assert_eq!(e.error_code.as_deref(), Some("request_validation_error"));
    assert_eq!(
        e.errors,
        Some(serde_json::json!({"amount": ["must be positive"]}))
    );
}

#[tokio::test]
async fn legacy_http_error_shapes_are_preserved() {
    for (body, message, errors) in [
        (serde_json::json!({"response": {"message": "Response error", "errors": {"amount": ["invalid"]}}}).to_string(), "Response error", Some(serde_json::json!({"amount": ["invalid"]}))),
        (serde_json::json!({"error": {"message": "Nested error", "errors": {"card": ["invalid"]}}}).to_string(), "Nested error", Some(serde_json::json!({"card": ["invalid"]}))),
        (serde_json::json!({"error": "legacy", "message": "Flat message", "errors": {"card": ["invalid"]}}).to_string(), "Flat message", Some(serde_json::json!({"card": ["invalid"]}))),
        (serde_json::json!({"response": {"message": {"card": ["invalid"]}, "errors": {"amount": ["invalid"]}}}).to_string(), "{\"card\":[\"invalid\"]}", Some(serde_json::json!({"amount": ["invalid"]}))),
        ("upstream unavailable".into(), "upstream unavailable", None),
        ("{invalid json".into(), "{invalid json", None),
        ("".into(), "", None),
    ] {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/transactions/missing"))
            .respond_with(ResponseTemplate::new(502).set_body_string(body))
            .expect(1)
            .mount(&server)
            .await;
        let err = client(&server).get_transaction("missing").await.unwrap_err();
        assert_eq!(err.to_string(), format!("API error 502: {message}"));
        let BepaidError::Api(e) = err else { panic!("expected API error") };
        assert_eq!(e.status, 502);
        assert_eq!(e.message, message);
        assert_eq!(e.errors, errors);
    }
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
            description: None,
            expired_at: None,
            duplicate_check: None,
            language: None,
            notification_url: None,
            return_url: None,
            customer: None,
            sender_billing_address: None,
            recipient_billing_address: None,
            billing_address: None,
        })
        .await
        .expect("p2p should succeed");

    assert_eq!(t.status.as_deref(), Some("successful"));
    assert_eq!(t.tx_type.as_deref(), Some("p2p"));
    assert!(t.verify_p2p.is_some());
}

#[tokio::test]
async fn p2p_request_serializes_all_fields_for_create_and_verify() {
    for verify in [false, true] {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(if verify {
                "/p2p-restrictions"
            } else {
                "/transactions/p2ps"
            }))
            .and(wiremock::matchers::header("authorization", AUTH))
            .and(wiremock::matchers::body_json(serde_json::json!({
                "request": {
                    "amount": 100,
                    "currency": "EUR",
                    "credit_card": {"token": "tok-from"},
                    "recipient_card": {"token": "tok-to"},
                    "test": true,
                    "tracking_id": "p2p-1",
                    "description": "Transfer",
                    "expired_at": "2026-10-01T00:00:00Z",
                    "duplicate_check": false,
                    "language": "ru",
                    "notification_url": "https://example.com/notify",
                    "return_url": "https://example.com/return",
                    "customer": {"email": "john@example.com"},
                    "sender_billing_address": {"country": "BY"},
                    "recipient_billing_address": {"country": "RU"},
                    "billing_address": {"country": "US"},
                    "additional_data": {
                        "referer": "https://example.com",
                        "receipt_text": ["r1", "r2"],
                        "contract": ["p2p_on"],
                        "p2p": {"type": "C2C"}
                    }
                }
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(if verify {
                serde_json::json!({"status": "successful"})
            } else {
                serde_json::json!({"transaction": {"uid": "u1"}})
            }))
            .expect(1)
            .mount(&server)
            .await;
        let req = P2pRequest {
            amount: 100,
            currency: "EUR".into(),
            credit_card: p2p_card_token("tok-from"),
            recipient_card: p2p_card_token("tok-to"),
            description: Some("Transfer".into()),
            expired_at: Some("2026-10-01T00:00:00Z".into()),
            duplicate_check: Some(false),
            language: Some("ru".into()),
            notification_url: Some("https://example.com/notify".into()),
            return_url: Some("https://example.com/return".into()),
            customer: Some(
                serde_json::from_value(serde_json::json!({"email": "john@example.com"})).unwrap(),
            ),
            sender_billing_address: Some(
                serde_json::from_value(serde_json::json!({"country": "BY"})).unwrap(),
            ),
            recipient_billing_address: Some(
                serde_json::from_value(serde_json::json!({"country": "RU"})).unwrap(),
            ),
            billing_address: Some(
                serde_json::from_value(serde_json::json!({"country": "US"})).unwrap(),
            ),
            test: Some(true),
            tracking_id: Some("p2p-1".into()),
            additional_data: Some(bepaid::types::P2pAdditionalData {
                referer: Some("https://example.com".into()),
                receipt_text: Some(vec!["r1".into(), "r2".into()]),
                contract: Some(vec!["p2p_on".into()]),
                p2p: Some(bepaid::types::P2pInfo {
                    p2p_type: Some("C2C".into()),
                }),
            }),
        };
        if verify {
            assert_eq!(
                client(&server)
                    .verify_p2p(req)
                    .await
                    .unwrap()
                    .status
                    .as_deref(),
                Some("successful")
            );
        } else {
            assert_eq!(
                client(&server)
                    .create_p2p(req)
                    .await
                    .unwrap()
                    .uid
                    .as_deref(),
                Some("u1")
            );
        }
        let received = server.received_requests().await.unwrap();
        assert_eq!(received.len(), 1);
        if verify {
            assert!(!received[0].headers.contains_key("x-api-version"));
        } else {
            assert_eq!(received[0].headers.get("x-api-version").unwrap(), "3");
        }
    }
}

#[test]
fn p2p_request_omits_absent_optional_fields() {
    let req = P2pRequest {
        amount: 100,
        currency: "EUR".into(),
        credit_card: p2p_card_token("tok-from"),
        recipient_card: p2p_card_token("tok-to"),
        description: None,
        expired_at: None,
        duplicate_check: None,
        language: None,
        notification_url: None,
        return_url: None,
        customer: None,
        sender_billing_address: None,
        recipient_billing_address: None,
        billing_address: None,
        test: None,
        tracking_id: None,
        additional_data: None,
    };
    let json = serde_json::to_value(&req).unwrap();
    assert_eq!(
        json,
        serde_json::json!({
            "amount": 100, "currency": "EUR",
            "credit_card": {"token": "tok-from"},
            "recipient_card": {"token": "tok-to"}
        })
    );
    assert_eq!(
        serde_json::to_value(bepaid::types::P2pAdditionalData {
            p2p: None,
            referer: None,
            receipt_text: None,
            contract: None,
        })
        .unwrap(),
        serde_json::json!({})
    );
    for key in [
        "description",
        "expired_at",
        "duplicate_check",
        "language",
        "notification_url",
        "return_url",
        "customer",
        "sender_billing_address",
        "recipient_billing_address",
        "billing_address",
        "additional_data",
    ] {
        assert!(json.get(key).is_none(), "{key} must be omitted");
    }
}

#[tokio::test]
async fn p2p_response_parses_all_documented_fields() {
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
                "description": "Transfer",
                "tracking_id": "t-1",
                "test": true,
                "created_at": "2026-09-17T10:00:00Z",
                "updated_at": "2026-09-17T10:00:05Z",
                "paid_at": "2026-09-17T10:00:06Z",
                "language": "ru",
                "payment_method_type": "credit_card",
                "message": "Successfully processed",
                "status_code": 1,
                "id": "b6c446e4",
                "redirect_url": "https://gateway.bepaid.by/process/1",
                "additional_data": {"p2p": {"type": "vis2vis"}},
                "customer": {"email": "john@example.com"},
                "billing_address": {"country": "BY"},
                "sender_billing_address": {"country": "BY"},
                "recipient_billing_address": {"country": "RU"},
                "credit_card": {"brand": "visa", "last_4": "1112"},
                "recipient_card": {"brand": "visa", "last_4": "0000"},
                "receipt_url": "https://example.com/receipt",
                "verify_p2p": {"status": "successful"},
                "p2p": {"type": "vis2vis"}
            }
        })))
        .expect(1)
        .mount(&server)
        .await;

    let t = client(&server)
        .create_p2p(P2pRequest {
            amount: 100,
            currency: "EUR".into(),
            credit_card: p2p_card_token("tok-from"),
            recipient_card: p2p_card_token("tok-to"),
            description: None,
            expired_at: None,
            duplicate_check: None,
            language: None,
            notification_url: None,
            return_url: None,
            customer: None,
            sender_billing_address: None,
            recipient_billing_address: None,
            billing_address: None,
            test: None,
            tracking_id: None,
            additional_data: None,
        })
        .await
        .expect("p2p should succeed");

    assert_eq!(t.uid.as_deref(), Some("1-82cc07d15d"));
    assert_eq!(t.status.as_deref(), Some("successful"));
    assert_eq!(t.description.as_deref(), Some("Transfer"));
    assert_eq!(t.tracking_id.as_deref(), Some("t-1"));
    assert_eq!(t.test, Some(true));
    assert_eq!(t.created_at.as_deref(), Some("2026-09-17T10:00:00Z"));
    assert_eq!(t.updated_at.as_deref(), Some("2026-09-17T10:00:05Z"));
    assert_eq!(t.paid_at.as_deref(), Some("2026-09-17T10:00:06Z"));
    assert_eq!(t.language.as_deref(), Some("ru"));
    assert_eq!(t.payment_method_type.as_deref(), Some("credit_card"));
    assert_eq!(t.message.as_deref(), Some("Successfully processed"));
    assert_eq!(t.status_code, Some(1));
    assert_eq!(t.id.as_deref(), Some("b6c446e4"));
    assert_eq!(t.amount, Some(100));
    assert_eq!(t.currency.as_deref(), Some("EUR"));
    assert_eq!(t.tx_type.as_deref(), Some("p2p"));
    assert_eq!(
        t.redirect_url.as_deref(),
        Some("https://gateway.bepaid.by/process/1")
    );
    assert_eq!(
        t.additional_data,
        Some(serde_json::json!({"p2p": {"type": "vis2vis"}}))
    );
    assert_eq!(
        t.customer,
        Some(serde_json::json!({"email": "john@example.com"}))
    );
    assert_eq!(
        t.credit_card.as_ref().unwrap().last_4.as_deref(),
        Some("1112")
    );
    assert_eq!(
        t.recipient_card.as_ref().unwrap().last_4.as_deref(),
        Some("0000")
    );
    assert_eq!(
        t.billing_address
            .as_ref()
            .and_then(|b| b.country.as_deref()),
        Some("BY")
    );
    assert_eq!(
        t.sender_billing_address
            .as_ref()
            .and_then(|b| b.country.as_deref()),
        Some("BY")
    );
    assert_eq!(
        t.recipient_billing_address
            .as_ref()
            .and_then(|b| b.country.as_deref()),
        Some("RU")
    );
    assert_eq!(
        t.receipt_url.as_deref(),
        Some("https://example.com/receipt")
    );
    assert_eq!(
        t.verify_p2p,
        Some(serde_json::json!({"status": "successful"}))
    );
    assert_eq!(t.p2p, Some(serde_json::json!({"type": "vis2vis"})));
}

#[tokio::test]
async fn verify_p2p_happy_path() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/p2p-restrictions"))
        .and(body_partial_json(serde_json::json!({
            "request": {
                "amount": 100,
                "currency": "EUR",
                "credit_card": {"token": "tok-from"},
                "recipient_card": {"token": "tok-to"}
            }
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "successful",
            "message": "Transaction is possible",
            "commission": {"minimum": 1.5, "percent": 2.0, "bank_fee": 0.5, "currency": "EUR"},
            "test": true,
            "required_fields": {"credit_card": ["holder"]}
        })))
        .expect(1)
        .mount(&server)
        .await;

    let r = client(&server)
        .verify_p2p(P2pRequest {
            amount: 100,
            currency: "EUR".into(),
            credit_card: p2p_card_token("tok-from"),
            recipient_card: p2p_card_token("tok-to"),
            description: None,
            expired_at: None,
            duplicate_check: None,
            language: None,
            notification_url: None,
            return_url: None,
            customer: None,
            sender_billing_address: None,
            recipient_billing_address: None,
            billing_address: None,
            test: None,
            tracking_id: None,
            additional_data: None,
        })
        .await
        .expect("verify_p2p should succeed");

    assert_eq!(r.status.as_deref(), Some("successful"));
    assert_eq!(r.message.as_deref(), Some("Transaction is possible"));
    let commission = r.commission.expect("commission present");
    assert_eq!(commission.minimum, Some(1.5));
    assert_eq!(commission.percent, Some(2.0));
    assert_eq!(commission.bank_fee, Some(0.5));
    assert_eq!(commission.currency.as_deref(), Some("EUR"));
    assert_eq!(r.test, Some(true));
    let required = r.required_fields.expect("required fields present");
    assert_eq!(
        required.credit_card.as_deref(),
        Some(&["holder".to_owned()][..])
    );
}

#[tokio::test]
async fn verify_p2p_required_fields_and_errors() {
    for status in [200, 400, 422] {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/p2p-restrictions"))
            .and(wiremock::matchers::header("authorization", AUTH))
            .respond_with(ResponseTemplate::new(status).set_body_json(serde_json::json!({
                "status": "failed",
                "message": "Additional card details required",
                "error_code": "request_validation_error",
                "errors": {"recipient_card": {"holder": ["can't be blank"]}},
                "required_fields": {"credit_card": ["holder"], "recipient_card": ["holder", "number"]}
            })))
            .expect(1)
            .mount(&server)
            .await;
        let result = client(&server)
            .verify_p2p(P2pRequest {
                amount: 100,
                currency: "EUR".into(),
                credit_card: p2p_card_token("tok-from"),
                recipient_card: p2p_card_token("tok-to"),
                description: None,
                expired_at: None,
                duplicate_check: None,
                language: None,
                notification_url: None,
                return_url: None,
                customer: None,
                sender_billing_address: None,
                recipient_billing_address: None,
                billing_address: None,
                test: None,
                tracking_id: None,
                additional_data: None,
            })
            .await;
        let expected_errors =
            Some(serde_json::json!({"recipient_card": {"holder": ["can't be blank"]}}));
        if status == 200 {
            let r = result.unwrap();
            assert_eq!(r.status.as_deref(), Some("failed"));
            assert_eq!(
                r.message.as_deref(),
                Some("Additional card details required")
            );
            assert_eq!(r.error_code.as_deref(), Some("request_validation_error"));
            assert_eq!(r.errors, expected_errors);
            assert!(r.commission.is_none());
            let required = r.required_fields.unwrap();
            assert_eq!(required.credit_card, Some(vec!["holder".into()]));
            assert_eq!(
                required.recipient_card,
                Some(vec!["holder".into(), "number".into()])
            );
        } else {
            let BepaidError::Api(e) = result.unwrap_err() else {
                panic!("expected API error")
            };
            assert_eq!(e.status, status);
            assert_eq!(e.message, "Additional card details required");
            assert_eq!(e.error_code.as_deref(), Some("request_validation_error"));
            assert_eq!(e.errors, expected_errors);
        }
        let received = server.received_requests().await.unwrap();
        assert!(!received[0].headers.contains_key("x-api-version"));
        assert_eq!(
            received[0].body_json::<serde_json::Value>().unwrap(),
            serde_json::json!({
                "request": {"amount": 100, "currency": "EUR", "credit_card": {"token": "tok-from"}, "recipient_card": {"token": "tok-to"}}
            })
        );
    }
}

fn p2p_card_token(token: &str) -> bepaid::types::P2pCard {
    bepaid::types::P2pCard {
        number: None,
        holder: None,
        verification_value: None,
        exp_month: None,
        exp_year: None,
        token: Some(token.into()),
    }
}

#[tokio::test]
async fn visa_alias_verify_phone_success() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/services/visa-alias/verify-phone"))
        .and(wiremock::matchers::header("x-api-version", "3"))
        .and(wiremock::matchers::header("authorization", AUTH))
        .and(wiremock::matchers::body_json(serde_json::json!({
            "recipient_info": {"phone_number": "375291112233"}
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "holder": "John Doe",
            "stamp": "card-stamp",
            "brand": "visa",
            "last_4": "0002",
            "first_1": "4",
            "bin": "411111",
            "bin_8": "41111111",
            "issuer_country": "BY",
            "issuer_name": "Card issuer",
            "product": "Classic",
            "exp_month": 12,
            "exp_year": 2030,
            "token_provider": "visa",
            "token": "visa-alias-token",
            "service_info": {
                "recipientName": "John Doe",
                "issuerName": "Priorbank",
                "cardType": "credit",
                "address1": "Nezavisimosti 1",
                "address2": "Apartment 2",
                "city": "Minsk",
                "country": "BY",
                "postalCode": "220000"
            }
        })))
        .expect(1)
        .mount(&server)
        .await;

    let r = client(&server)
        .verify_visa_alias(bepaid::types::VisaAliasPhoneRequest {
            recipient_info: bepaid::types::VisaAliasPhoneInfo {
                phone_number: "375291112233".into(),
            },
        })
        .await
        .expect("visa alias verify should succeed");

    let card = &r.credit_card;
    assert_eq!(card.token.as_deref(), Some("visa-alias-token"));
    assert_eq!(card.brand.as_deref(), Some("visa"));
    assert_eq!(card.last_4.as_deref(), Some("0002"));
    assert_eq!(card.holder.as_deref(), Some("John Doe"));
    assert_eq!(card.stamp.as_deref(), Some("card-stamp"));
    assert_eq!(card.first_1.as_deref(), Some("4"));
    assert_eq!(card.bin.as_deref(), Some("411111"));
    assert_eq!(card.bin_8.as_deref(), Some("41111111"));
    assert_eq!(card.issuer_country.as_deref(), Some("BY"));
    assert_eq!(card.issuer_name.as_deref(), Some("Card issuer"));
    assert_eq!(card.product.as_deref(), Some("Classic"));
    assert_eq!(card.exp_month, Some(12));
    assert_eq!(card.exp_year, Some(2030));
    assert_eq!(card.token_provider.as_deref(), Some("visa"));
    let info = r.service_info.expect("service_info present");
    assert_eq!(info.recipient_name.as_deref(), Some("John Doe"));
    assert_eq!(info.issuer_name.as_deref(), Some("Priorbank"));
    assert_eq!(info.card_type.as_deref(), Some("credit"));
    assert_eq!(info.address1.as_deref(), Some("Nezavisimosti 1"));
    assert_eq!(info.address2.as_deref(), Some("Apartment 2"));
    assert_eq!(info.city.as_deref(), Some("Minsk"));
    assert_eq!(info.country.as_deref(), Some("BY"));
    assert_eq!(info.postal_code.as_deref(), Some("220000"));
}

#[tokio::test]
async fn visa_alias_verify_phone_validation_error() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/services/visa-alias/verify-phone"))
        .respond_with(ResponseTemplate::new(400).set_body_json(serde_json::json!({
            "error_code": "request_validation_error",
            "message": {"recipient_info": {"phone_number": ["is in invalid format"]}},
            "status": "error",
            "code": "E.1025",
            "friendly_message": "Invalid request params"
        })))
        .expect(1)
        .mount(&server)
        .await;

    let err = client(&server)
        .verify_visa_alias(bepaid::types::VisaAliasPhoneRequest {
            recipient_info: bepaid::types::VisaAliasPhoneInfo {
                phone_number: "bad".into(),
            },
        })
        .await
        .unwrap_err();

    let BepaidError::Api(e) = err else {
        panic!("expected API error")
    };
    assert_eq!(e.status, 400);
    assert_eq!(e.error_code.as_deref(), Some("request_validation_error"));
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(&e.message).unwrap(),
        serde_json::json!({"recipient_info": {"phone_number": ["is in invalid format"]}})
    );
    assert_eq!(
        e.errors,
        Some(serde_json::json!({"recipient_info": {"phone_number": ["is in invalid format"]}}))
    );
}

#[tokio::test]
async fn visa_alias_verify_phone_not_found_error() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/services/visa-alias/verify-phone"))
        .respond_with(ResponseTemplate::new(404).set_body_json(serde_json::json!({
            "status": "error",
            "code": "E.1037",
            "message": "Card Not Found",
            "friendly_message": "Card Not Found"
        })))
        .expect(1)
        .mount(&server)
        .await;

    let err = client(&server)
        .verify_visa_alias(bepaid::types::VisaAliasPhoneRequest {
            recipient_info: bepaid::types::VisaAliasPhoneInfo {
                phone_number: "375291112233".into(),
            },
        })
        .await
        .unwrap_err();

    let BepaidError::Api(e) = err else {
        panic!("expected API error")
    };
    assert_eq!(e.status, 404);
    assert_eq!(e.message, "Card Not Found");
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
        .create_payout(
            PayoutRequest {
                test: Some(true),
                amount: 100,
                currency: "USD".into(),
                description: Some("Payout".into()),
                tracking_id: Some("payout-1".into()),
                recipient: bepaid::types::Customer {
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
                    ip: Some("127.0.0.1".into()),
                    email: Some("john@example.com".into()),
                    device_id: None,
                    birth_date: Some("1990-10-20".into()),
                    phone: None,
                    external_id: None,
                    taxpayer_id: None,
                },
                sender: bepaid::types::Customer {
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
                    ip: Some("127.0.0.1".into()),
                    email: Some("john@example.com".into()),
                    device_id: None,
                    birth_date: Some("1990-10-20".into()),
                    phone: None,
                    external_id: None,
                    taxpayer_id: None,
                },
                recipient_billing_address: bepaid::types::BillingAddress {
                    middle_name: None,
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
                    middle_name: None,
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
            },
            None,
        )
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
            name: None,
            description: None,
            currency: None,
            amount: Some(950),
            visible_fields: None,
            infinite: Some(false),
            quantity: Some("5".to_owned()),
            test: None,
            immortal: None,
            expired_at: None,
            return_url: None,
            shop_id: None,
            language: None,
            transaction_type: None,
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
        .charge_saved_card(
            ChargeRequest {
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
                fiscalization: None,
            },
            None,
        )
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
async fn create_tokenization_happy_path() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/transactions/tokenizations"))
        .and(wiremock::matchers::header("x-api-version", "3"))
        .and(wiremock::matchers::header("authorization", AUTH))
        .and(body_partial_json(serde_json::json!({
            "request": {
                "credit_card": {
                    "number": "4242424242424242",
                    "skip_three_d_secure_verification": false,
                },
            }
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "transaction": {
                "uid": "e89abc1a-1d18-4d0f-83a1-7009b333dce0",
                "status": "successful",
                "type": "tokenization",
                "credit_card": {
                    "brand": "visa",
                    "last_4": "1097",
                    "token": "e3ba5977-8705-4496-bf90-a6a93d3d31cc"
                },
                "tokenization": {
                    "gateway_id": 3483,
                    "status": "successful"
                }
            }
        })))
        .expect(1)
        .mount(&server)
        .await;

    let c = client(&server);
    let t = c
        .create_tokenization(
            TokenizationRequest {
                amount: 100,
                currency: "USD".into(),
                description: "Test transaction".into(),
                tracking_id: None,
                duplicate_check: None,
                dynamic_billing_descriptor: None,
                language: None,
                notification_url: None,
                verification_url: None,
                return_url: None,
                test: Some(true),
                billing_address: None,
                credit_card: Some(CreditCardRaw {
                    number: Some("4242424242424242".into()),
                    verification_value: Some("123".into()),
                    holder: Some("John Smith".into()),
                    exp_month: Some(10),
                    exp_year: Some(2030),
                    save_card: None,
                    token: None,
                    skip_three_d_secure_verification: Some(false),
                    force_three_d_secure_verification: None,
                }),
                three_d_secure: None,
                travel: None,
                customer: None,
                additional_data: None,
            },
            None,
        )
        .await
        .expect("tokenization should succeed");

    assert_eq!(t.uid, "e89abc1a-1d18-4d0f-83a1-7009b333dce0");
    assert_eq!(
        t.credit_card.and_then(|c| c.token).as_deref(),
        Some("e3ba5977-8705-4496-bf90-a6a93d3d31cc")
    );
    let tokenization = t.tokenization.expect("tokenization info present");
    assert_eq!(tokenization.gateway_id, Some(3483));
    assert_eq!(tokenization.status.as_deref(), Some("successful"));
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
        .apm_payout(
            ApmPayoutRequest {
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
            },
            None,
        )
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
            None,
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
        .checkup(
            CheckupRequest {
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
            },
            None,
        )
        .await
        .expect("checkup should succeed");

    assert_eq!(t.status.as_deref(), Some("successful"));
    assert_eq!(t.amount, Some(100));
}

#[tokio::test]
async fn create_payment_sends_request_id_header() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/transactions/payments"))
        .and(wiremock::matchers::header("requestid", "uuid-request-1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "transaction": {"tracking_id": "t1", "uid": "u1"}
        })))
        .expect(1)
        .mount(&server)
        .await;

    let c = client(&server);
    c.create_payment(
        PaymentRequest {
            amount: "700".into(),
            currency: "USD".into(),
            test: true,
            description: "Test".into(),
            tracking_id: "t1".into(),
            expired_at: None,
            dynamic_billing_descriptor: None,
            duplicate_check: None,
            language: None,
            notification_url: None,
            verification_url: None,
            return_url: None,
            billing_address: None,
            credit_card: None,
            customer: None,
            additional_data: None,
            custom_fields: None,
            encrypted_data: None,
            fiscalization: None,
        },
        Some("uuid-request-1"),
    )
    .await
    .expect("payment with request id should succeed");
}

#[tokio::test]
async fn checkout_response_preserves_extra_fields() {
    let server = MockServer::start().await;
    let settings = serde_json::json!({"style": {"button_color": "#fff"},
        "widget_version": "2", "require": ["email"], "customer": {"email": "buyer@example.com"}});
    Mock::given(method("POST"))
        .and(path("/ctp/api/checkouts"))
        .and(wiremock::matchers::header("x-api-version", "2"))
        .and(wiremock::matchers::header("authorization", AUTH))
        .and(wiremock::matchers::body_json(serde_json::json!({"checkout": {
            "transaction_type": "payment", "order": {"currency": "USD", "amount": 100},
            "settings": settings, "dynamic_billing_descriptor": "Shop", "travel": {"flight": "AB123"}
        }})))
        .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
            "checkout": {
                "token": "tok-extra",
                "redirect_url": "https://checkout.bepaid.by/widget/hpp.html?token=tok-extra",
                "brand_id": "b1",
                "payment_status": "success"
            }
        })))
        .expect(1)
        .mount(&server)
        .await;

    let t = client(&server)
        .create_checkout(&CheckoutRequest {
            test: None,
            transaction_type: "payment".into(),
            attempts: None,
            iframe: None,
            settings: Some(serde_json::from_value(settings).unwrap()),
            payment_method: None,
            credit_card: None,
            order: bepaid::types::CheckoutOrder {
                currency: "USD".into(),
                amount: 100,
                description: None,
                tracking_id: None,
                expired_at: None,
                additional_data: None,
                custom_fields: None,
            },
            customer: None,
            dynamic_billing_descriptor: Some("Shop".into()),
            travel: Some(serde_json::json!({"flight": "AB123"})),
            fiscalization: None,
        })
        .await
        .unwrap();

    assert_eq!(t.token, "tok-extra");
    assert_eq!(t.extra.as_ref().unwrap()["brand_id"], "b1");
    assert_eq!(t.extra.as_ref().unwrap()["payment_status"], "success");
}

#[tokio::test]
async fn checkout_status_decodes_full_payload() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/ctp/api/checkouts/tok9"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "checkout": {
                "token": "tok9",
                "merchant": {"id": 160, "name": "Shop"},
                "version": 2,
                "card_info": {"brand": "visa", "last_4": "0000"},
                "job_id": "job-1",
                "attempts": 3,
                "iframe": false,
                "dynamic_billing_descriptor": "descriptor-1",
                "travel": {"departure_country": "BY"},
                "settings": {"success_url": "https://example.com/ok",
                    "style": {"button_color": "#fff"}, "widget_version": "2",
                    "require": ["email"], "customer": {"email": "buyer@example.com"}}
            }
        })))
        .expect(1)
        .mount(&server)
        .await;

    let t = client(&server).get_checkout_status("tok9").await.unwrap();
    assert_eq!(t.merchant.unwrap()["id"], 160);
    assert_eq!(t.version, Some(serde_json::json!(2)));
    assert_eq!(t.card_info.unwrap()["last_4"], "0000");
    assert_eq!(t.job_id.as_deref(), Some("job-1"));
    assert_eq!(t.attempts, Some(3));
    assert_eq!(t.iframe, Some(false));
    assert_eq!(
        t.dynamic_billing_descriptor.as_deref(),
        Some("descriptor-1")
    );
    assert_eq!(t.travel.unwrap()["departure_country"], "BY");
    let settings = t.settings.unwrap();
    assert_eq!(settings.style.unwrap()["button_color"], "#fff");
    assert_eq!(settings.widget_version.as_deref(), Some("2"));
    assert_eq!(settings.require, Some(serde_json::json!(["email"])));
    assert_eq!(settings.customer.unwrap()["email"], "buyer@example.com");
    let string_version: bepaid::types::CheckoutStatus =
        serde_json::from_value(serde_json::json!({"version": "2"})).unwrap();
    assert_eq!(string_version.version, Some(serde_json::json!("2")));
}

#[tokio::test]
async fn card_balance_happy_path_and_error() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/balance"))
        .and(wiremock::matchers::header("x-api-version", "2"))
        .and(wiremock::matchers::header("authorization", AUTH))
        .and(wiremock::matchers::body_json(
            serde_json::json!({"request": {
                "account": "40701810842020395221", "currency": "BYN", "gateway_id": 3483
            }}),
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "success",
            "result": {
                "gatewayId": 3483,
                "account": "40701810842020395221",
                "amount": 12345,
                "currency": "BYN",
                "bankInfo": {"name": "Priorbank"}
            }
        })))
        .expect(1)
        .mount(&server)
        .await;

    let t = client(&server)
        .get_card_balance(bepaid::types::CardBalanceRequest {
            account: "40701810842020395221".into(),
            currency: "BYN".into(),
            gateway_id: Some(3483),
        })
        .await
        .unwrap();
    assert_eq!(t.status.as_deref(), Some("success"));
    let result = t.result.unwrap();
    assert_eq!(result.gateway_id, 3483);
    assert_eq!(result.account.as_deref(), Some("40701810842020395221"));
    assert_eq!(result.amount, Some(12345));
    assert_eq!(result.currency.as_deref(), Some("BYN"));
    assert_eq!(result.bank_info.unwrap()["name"], "Priorbank");

    Mock::given(method("POST"))
        .and(path("/balance"))
        .respond_with(ResponseTemplate::new(400).set_body_json(serde_json::json!({
            "response": {
                "message": "Account not found",
                "code": "E.42",
                "friendly_message": "Счёт не найден"
            }
        })))
        .mount(&server)
        .await;
    let err = client(&server)
        .get_card_balance(bepaid::types::CardBalanceRequest {
            account: "404".into(),
            currency: "BYN".into(),
            gateway_id: None,
        })
        .await
        .unwrap_err();
    let BepaidError::Api(e) = err else {
        panic!("expected Api error");
    };
    assert_eq!(e.code.as_deref(), Some("E.42"));
    assert_eq!(e.friendly_message.as_deref(), Some("Счёт не найден"));
}

#[tokio::test]
async fn create_payment_decodes_full_transaction() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/transactions/payments"))
        .and(wiremock::matchers::header("x-api-version", "3"))
        .and(body_partial_json(serde_json::json!({"request": {
            "expired_at": "2026-10-01T10:00:00Z", "dynamic_billing_descriptor": "Shop"
        }})))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "transaction": {
                "uid": "pay-full",
                "status": "failed",
                "message": "Card declined",
                "credit_card": {"token": "tok-1", "brand": "visa", "last_4": "4242"},
                "redirect_url": "https://gateway.bepaid.by/process/pay-full",
                "code": "H.0001",
                "payment": {"gateway_id": 42, "status": "failed", "message": "Insufficient funds"}
            }
        })))
        .expect(1)
        .mount(&server)
        .await;

    let t = client(&server)
        .create_payment(
            PaymentRequest {
                amount: "700".into(),
                currency: "USD".into(),
                test: true,
                description: "d".into(),
                tracking_id: "t".into(),
                expired_at: Some("2026-10-01T10:00:00Z".into()),
                dynamic_billing_descriptor: Some("Shop".into()),
                duplicate_check: None,
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
            },
            None,
        )
        .await
        .unwrap();
    assert_eq!(t.uid, "pay-full");
    assert_eq!(t.status.as_deref(), Some("failed"));
    assert_eq!(t.message.as_deref(), Some("Card declined"));
    assert_eq!(t.code.as_deref(), Some("H.0001"));
    assert_eq!(
        t.redirect_url.as_deref(),
        Some("https://gateway.bepaid.by/process/pay-full")
    );
    let card = t.credit_card.unwrap();
    assert_eq!(card.token.as_deref(), Some("tok-1"));
    assert_eq!(card.last_4.as_deref(), Some("4242"));
    let payment = t.payment.unwrap();
    assert_eq!(payment.gateway_id, Some(42));
    assert_eq!(payment.message.as_deref(), Some("Insufficient funds"));
}

#[tokio::test]
async fn async_payment_flow_happy_path() {
    let server = MockServer::start().await;
    let base = server.uri();
    Mock::given(method("POST"))
        .and(path("/async/transactions/payments"))
        .and(wiremock::matchers::header("x-api-version", "3"))
        .and(body_partial_json(serde_json::json!({
            "request": {"amount": "700", "currency": "USD", "tracking_id": "t1"}
        })))
        .respond_with(ResponseTemplate::new(202).set_body_json(serde_json::json!({
            "status": "pending",
            "request_id": "req-1",
            "status_url": format!("{base}/async/status/req-1"),
            "response_url": format!("{base}/async/result/req-1")
        })))
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/async/status/req-1"))
        .and(wiremock::matchers::header("authorization", AUTH))
        .and(wiremock::matchers::header("x-api-version", "3"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "completed",
            "request_id": "req-1",
            "response_url": format!("{base}/async/result/req-1")
        })))
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/async/result/req-1"))
        .and(wiremock::matchers::header("authorization", AUTH))
        .and(wiremock::matchers::header("x-api-version", "3"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "transaction": {"uid": "async-1", "status": "successful", "type": "payment"}
        })))
        .expect(1)
        .mount(&server)
        .await;

    let c = client(&server);
    let ack = c
        .create_payment_async(
            PaymentRequest {
                amount: "700".into(),
                currency: "USD".into(),
                test: true,
                description: "d".into(),
                tracking_id: "t1".into(),
                expired_at: None,
                dynamic_billing_descriptor: None,
                duplicate_check: None,
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
            },
            None,
        )
        .await
        .unwrap();
    assert_eq!(ack.status.as_deref(), Some("pending"));
    assert_eq!(ack.request_id.as_deref(), Some("req-1"));

    let status = c.get_async_status(&ack.status_url.unwrap()).await.unwrap();
    assert_eq!(status.status.as_deref(), Some("completed"));
    let tx = c
        .get_async_result(&status.response_url.unwrap())
        .await
        .unwrap();
    assert_eq!(tx.uid, "async-1");
    assert_eq!(tx.status.as_deref(), Some("successful"));
}

#[tokio::test]
async fn async_polling_rejects_invalid_destinations_before_http() {
    let server = MockServer::start().await;
    let other = MockServer::start().await;
    let base = server.uri();
    let c = client(&server);
    for url in [
        String::new(),
        "not a URL".into(),
        "/async/status/request".into(),
        "http://[".into(),
        base.replacen("http:", "https:", 1),
        base.replacen("127.0.0.1", "localhost", 1),
        other.uri(),
        base.replacen("://", "://user@", 1),
        base.replacen("://", "://:password@", 1),
        base.replacen("://", "://@", 1),
        "file:///async/status/request".into(),
    ] {
        assert!(matches!(
            c.get_async_status(&url).await,
            Err(BepaidError::InvalidRequest(_))
        ));
        assert!(matches!(
            c.get_async_result(&url).await,
            Err(BepaidError::InvalidRequest(_))
        ));
    }
    let invalid_gateway = BepaidClient::with_urls("shop", "test", "invalid", &base, &base, &base);
    assert!(matches!(
        invalid_gateway.get_async_status(&base).await,
        Err(BepaidError::InvalidRequest(_))
    ));
    assert!(matches!(
        invalid_gateway.get_async_result(&base).await,
        Err(BepaidError::InvalidRequest(_))
    ));
    assert!(server.received_requests().await.unwrap().is_empty());
    assert!(other.received_requests().await.unwrap().is_empty());
}

#[tokio::test]
async fn async_polling_disables_redirects_without_changing_other_requests() {
    let server = MockServer::start().await;
    let c = client(&server);
    for status in [301, 302, 303, 307, 308] {
        Mock::given(method("GET"))
            .and(path(format!("/poll/{status}")))
            .and(wiremock::matchers::header("authorization", AUTH))
            .respond_with(ResponseTemplate::new(status).insert_header("location", "/not-followed"))
            .expect(2)
            .mount(&server)
            .await;
        let url = format!("{}/poll/{status}", server.uri());
        let BepaidError::Api(error) = c.get_async_status(&url).await.unwrap_err() else {
            panic!("expected redirect status error")
        };
        assert_eq!(error.status, status);
        let BepaidError::Api(error) = c.get_async_result(&url).await.unwrap_err() else {
            panic!("expected redirect status error")
        };
        assert_eq!(error.status, status);
    }
    let requests = server.received_requests().await.unwrap();
    assert_eq!(requests.len(), 10);
    assert!(
        requests
            .iter()
            .all(|request| request.url.path().starts_with("/poll/"))
    );
    Mock::given(method("GET"))
        .and(path("/transactions/redirect"))
        .respond_with(ResponseTemplate::new(302).insert_header("location", "/transactions/final"))
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/transactions/final"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!({"transaction": {"uid": "final"}})),
        )
        .expect(1)
        .mount(&server)
        .await;
    assert_eq!(c.get_transaction("redirect").await.unwrap().uid, "final");
}

#[tokio::test]
async fn async_authorization_ack() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/async/transactions/authorizations"))
        .and(wiremock::matchers::header("x-api-version", "3"))
        .and(wiremock::matchers::header("authorization", AUTH))
        .and(wiremock::matchers::header("requestid", "async-auth"))
        .and(wiremock::matchers::body_json(serde_json::json!({"request": {
            "amount": 100, "currency": "USD", "description": "d", "tracking_id": "t2",
            "credit_card": {"token": "tok"}, "language": "ru",
            "notification_url": "https://example.com/notify", "return_url": "https://example.com/return",
            "expired_at": "2026-10-01T10:00:00Z", "dynamic_billing_descriptor": "Shop"
        }})))
        .respond_with(ResponseTemplate::new(202).set_body_json(serde_json::json!({
            "status": "pending", "request_id": "req-2"
        })))
        .expect(1)
        .mount(&server)
        .await;

    let c = client(&server);
    let ack = c
        .create_authorization_async(
            AuthorizationRequest {
                amount: 100,
                currency: "USD".into(),
                description: "d".into(),
                payment_method_type: None,
                tracking_id: "t2".into(),
                test: None,
                duplicate_check: None,
                language: Some("ru".into()),
                notification_url: Some("https://example.com/notify".into()),
                return_url: Some("https://example.com/return".into()),
                expired_at: Some("2026-10-01T10:00:00Z".into()),
                dynamic_billing_descriptor: Some("Shop".into()),
                credit_card: Some(
                    serde_json::from_value(serde_json::json!({"token": "tok"})).unwrap(),
                ),
                customer: None,
                billing_address: None,
                additional_data: None,
                verification_url: None,
                custom_fields: None,
            },
            Some("async-auth"),
        )
        .await
        .unwrap();
    assert_eq!(ack.status.as_deref(), Some("pending"));
    assert_eq!(ack.request_id.as_deref(), Some("req-2"));
}

#[tokio::test]
async fn api_error_preserves_code_and_friendly_message() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/transactions/err-1"))
        .respond_with(ResponseTemplate::new(422).set_body_json(serde_json::json!({
            "response": {
                "message": "Processing error",
                "code": "H.0002",
                "friendly_message": "Платёж отклонён банком"
            }
        })))
        .expect(1)
        .mount(&server)
        .await;

    let err = client(&server).get_transaction("err-1").await.unwrap_err();
    let BepaidError::Api(e) = err else {
        panic!("expected Api error");
    };
    assert_eq!(e.status, 422);
    assert_eq!(e.message, "Processing error");
    assert_eq!(e.code.as_deref(), Some("H.0002"));
    assert_eq!(
        e.friendly_message.as_deref(),
        Some("Платёж отклонён банком")
    );
}

#[tokio::test]
async fn apm_request_serializes_iframe_and_verification_url() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/beyag/transactions/payments"))
        .and(body_partial_json(serde_json::json!({"request": {
            "iframe": true, "verification_url": "https://example.com/verify"
        }})))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "transaction": {"uid": "u", "status": "pending"}
        })))
        .expect(1)
        .mount(&server)
        .await;

    client(&server)
        .create_apm_payment(
            ApmPaymentRequest {
                iframe: Some(true),
                verification_url: Some("https://example.com/verify".into()),
                ..ApmPaymentRequest::alfaclick(100, "BYN")
            },
            None,
        )
        .await
        .unwrap();
}

#[test]
fn customer_serializes_id_and_id_number() {
    let customer: bepaid::types::Customer = serde_json::from_value(serde_json::json!({
        "id": "cust-1", "id_number": "3751234A001PB2"
    }))
    .unwrap();
    assert_eq!(
        serde_json::to_value(&customer).unwrap(),
        serde_json::json!({"id": "cust-1", "id_number": "3751234A001PB2"})
    );
}

#[tokio::test]
async fn universal_confirm_wrapped_body_exact_match() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/beyag/transactions/apm-8/confirm"))
        .and(wiremock::matchers::body_json(
            serde_json::json!({"request": {
                "transaction_reference": "receipt-8", "skip_duplicate_check": true
            }}),
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "response": {"parent_uid": "apm-8", "status": "successful"}
        })))
        .expect(1)
        .mount(&server)
        .await;

    let r = client(&server)
        .confirm_apm_payment(
            "apm-8",
            ApmConfirmRequest {
                confirm_type: None,
                phone: None,
                transaction_reference: Some("receipt-8".into()),
                skip_duplicate_check: Some(true),
            },
            None,
        )
        .await
        .unwrap();
    assert_eq!(r.parent_uid.as_deref(), Some("apm-8"));
    assert_eq!(r.status.as_deref(), Some("successful"));
}

#[tokio::test]
async fn universal_confirm_empty_or_skip_alone_is_allowed() {
    for skip in [None, Some(true), Some(false)] {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/beyag/transactions/skip-only/confirm"))
            .and(wiremock::matchers::header("authorization", AUTH))
            .and(wiremock::matchers::body_json(serde_json::json!({
                "request": skip.map_or_else(|| serde_json::json!({}), |skip| serde_json::json!({"skip_duplicate_check": skip}))
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "response": {"parent_uid": "skip-only", "status": "successful"}
            })))
            .expect(1)
            .mount(&server)
            .await;
        let response = client(&server)
            .confirm_apm_payment(
                "skip-only",
                ApmConfirmRequest {
                    confirm_type: None,
                    phone: None,
                    transaction_reference: None,
                    skip_duplicate_check: skip,
                },
                None,
            )
            .await
            .unwrap();
        assert_eq!(response.status.as_deref(), Some("successful"));
    }
}

#[tokio::test]
async fn update_product_serializes_new_fields() {
    let server = MockServer::start().await;
    Mock::given(method("PUT"))
        .and(path("/products/prd_2"))
        .and(body_partial_json(serde_json::json!({
            "name": "Widget",
            "currency": "EUR"
        })))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&server)
        .await;

    client(&server)
        .update_product(
            "prd_2",
            ProductUpdateRequest {
                name: Some("Widget".into()),
                description: None,
                currency: Some("EUR".into()),
                amount: None,
                visible_fields: None,
                infinite: None,
                quantity: None,
                test: None,
                immortal: None,
                expired_at: None,
                return_url: None,
                shop_id: None,
                language: None,
                transaction_type: None,
            },
        )
        .await
        .unwrap();
}

#[tokio::test]
async fn split_v2_serializes_recipients_array_and_smart_routing() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/transactions/payments"))
        .and(body_partial_json(serde_json::json!({
            "request": {
                "amount": "100",
                "additional_data": {
                    "split": [
                        {
                            "amount": 40,
                            "tax_id": "123456789",
                            "company_name": "Shop A",
                            "bank_account": "BY13NBRB3600900000002Z00AB00",
                            "bank_bic": "NBRBBY2X",
                            "city": "Minsk"
                        },
                        {"amount": 60, "tax_id": "987654321"}
                    ],
                    "smart_routing_options": {"allow_halva": true}
                }
            }
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "transaction": {"uid": "u1"}
        })))
        .expect(1)
        .mount(&server)
        .await;

    let splits = vec![
        SplitRecipient {
            amount: 40,
            tax_id: "123456789".into(),
            company_name: Some("Shop A".into()),
            bank_account: Some("BY13NBRB3600900000002Z00AB00".into()),
            bank_bic: Some("NBRBBY2X".into()),
            description: None,
            legal_address: None,
            mailing_address: None,
            country: None,
            city: Some("Minsk".into()),
            postal_code: None,
            contact_email: None,
            contact_phone: None,
            memo: None,
        },
        SplitRecipient {
            amount: 60,
            tax_id: "987654321".into(),
            company_name: None,
            bank_account: None,
            bank_bic: None,
            description: None,
            legal_address: None,
            mailing_address: None,
            country: None,
            city: None,
            postal_code: None,
            contact_email: None,
            contact_phone: None,
            memo: None,
        },
    ];
    assert_eq!(splits.iter().map(|s| s.amount).sum::<i64>(), 100);

    client(&server)
        .create_payment(
            PaymentRequest {
                amount: "100".into(),
                currency: "BYN".into(),
                test: true,
                description: "Split payment".into(),
                tracking_id: "split-1".into(),
                expired_at: None,
                dynamic_billing_descriptor: None,
                duplicate_check: None,
                language: None,
                notification_url: None,
                verification_url: None,
                return_url: None,
                billing_address: None,
                credit_card: None,
                customer: None,
                additional_data: Some(AdditionalData {
                    browser: None,
                    contract: None,
                    referer: None,
                    masterpass: None,
                    split: Some(splits),
                    smart_routing_options: Some(SmartRoutingOptions {
                        allow_halva: Some(true),
                    }),
                    excluded_gateways: None,
                    extra: None,
                }),
                encrypted_data: None,
                fiscalization: None,
                custom_fields: None,
            },
            None,
        )
        .await
        .expect("split payment should succeed");
}

#[tokio::test]
async fn charge_serializes_fiscalization() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/services/credit_cards/charges"))
        .and(body_partial_json(serde_json::json!({
            "request": {
                "fiscalization": {
                    "external_id": "fisc-1",
                    "positions": [{
                        "name": "Product",
                        "type": "service",
                        "amount": 700,
                        "quantity": 1.0,
                        "measure_unit_code": 796,
                        "untaxed": false
                    }]
                }
            }
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "transaction": {"uid": "u1"}
        })))
        .expect(1)
        .mount(&server)
        .await;

    client(&server)
        .charge_saved_card(
            ChargeRequest {
                amount: 700,
                currency: "BYN".into(),
                description: "Fiscalized charge".into(),
                tracking_id: Some("fisc-charge-1".into()),
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
                fiscalization: Some(Fiscalization {
                    external_id: "fisc-1".into(),
                    positions: vec![FiscalizationPosition {
                        name: "Product".into(),
                        position_type: "service".into(),
                        amount: 700,
                        quantity: 1.0,
                        measure_unit_code: 796,
                        description: None,
                        untaxed: false,
                        nomenclature_code: None,
                        taxes: None,
                    }],
                }),
            },
            None,
        )
        .await
        .expect("fiscalized charge should succeed");
}

#[tokio::test]
async fn refund_serializes_fiscalization_flag() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/transactions/refunds"))
        .and(body_partial_json(serde_json::json!({
            "request": {"fiscalization": true}
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "transaction": {"uid": "2-1", "type": "refund", "status": "successful"}
        })))
        .expect(1)
        .mount(&server)
        .await;

    client(&server)
        .refund(
            RefundRequest {
                parent_uid: "1-1".into(),
                amount: 50,
                reason: "Client request".into(),
                tracking_id: None,
                additional_data: None,
                fiscalization: Some(true),
            },
            None,
        )
        .await
        .expect("fiscalized refund should succeed");
}

#[tokio::test]
async fn checkout_token_create_serializes_fiscalization() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/payments/tokens"))
        .and(body_partial_json(serde_json::json!({
            "checkout": {
                "fiscalization": {
                    "external_id": "fisc-2",
                    "positions": [{
                        "name": "Token product",
                        "type": "service",
                        "amount": 7000,
                        "quantity": 1.0,
                        "measure_unit_code": 796,
                        "untaxed": false
                    }]
                }
            }
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "checkout": {"token": "tok-1"}
        })))
        .expect(1)
        .mount(&server)
        .await;

    client(&server)
        .create_payment_token(&CheckoutRequest {
            test: Some(true),
            transaction_type: "payment".into(),
            attempts: None,
            iframe: None,
            settings: None,
            payment_method: None,
            credit_card: None,
            order: bepaid::types::CheckoutOrder {
                currency: "BYN".into(),
                amount: 7000,
                description: Some("Widget order".into()),
                tracking_id: None,
                expired_at: None,
                additional_data: None,
                custom_fields: None,
            },
            customer: None,
            dynamic_billing_descriptor: None,
            travel: None,
            fiscalization: Some(Fiscalization {
                external_id: "fisc-2".into(),
                positions: vec![FiscalizationPosition {
                    name: "Token product".into(),
                    position_type: "service".into(),
                    amount: 7000,
                    quantity: 1.0,
                    measure_unit_code: 796,
                    description: None,
                    untaxed: false,
                    nomenclature_code: None,
                    taxes: None,
                }],
            }),
        })
        .await
        .expect("fiscalized token creation should succeed");
}

#[tokio::test]
async fn paginated_reports_v3_happy_path() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/api/reports"))
        .and(wiremock::matchers::header("x-api-version", "3"))
        .and(wiremock::matchers::header("authorization", AUTH))
        .and(body_partial_json(serde_json::json!({
            "report_params": {
                "date_type": "created_at",
                "from": "2022-01-25 00:00:00",
                "to": "2022-01-27 23:59:59",
                "status": "all",
                "payment_method_type": "credit_card",
                "time_zone": "Etc/UTC",
                "starting_after": "10",
                "manual_correction_from": "2022-01-26 00:00:00"
            }
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "transactions": [{"uid": "t1", "id": 11, "status": "successful"}],
            "count": 1,
            "has_more": true,
            "first_object_id": "11",
            "last_object_id": "11"
        })))
        .expect(1)
        .mount(&server)
        .await;

    let r = client(&server)
        .get_reports_v3(ReportListV3Request {
            report_params: ReportListV3Params {
                date_type: "created_at".into(),
                from: "2022-01-25 00:00:00".into(),
                to: "2022-01-27 23:59:59".into(),
                status: "all".into(),
                payment_method_type: "credit_card".into(),
                time_zone: "Etc/UTC".into(),
                starting_after: Some("10".into()),
                ending_before: None,
                manual_correction_from: Some("2022-01-26 00:00:00".into()),
                manual_correction_to: None,
            },
        })
        .await
        .expect("paginated reports should succeed");

    assert_eq!(r.count, Some(1));
    assert_eq!(r.has_more, Some(true));
    assert_eq!(r.first_object_id.as_deref(), Some("11"));
    assert_eq!(r.last_object_id.as_deref(), Some("11"));
    assert_eq!(r.transactions[0].uid.as_deref(), Some("t1"));
}

#[tokio::test]
async fn cascading_serializes_excluded_gateways_and_parses_gateway_id() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/transactions/payments"))
        .and(body_partial_json(serde_json::json!({
            "request": {
                "duplicate_check": false,
                "additional_data": {"excluded_gateways": [3483, 3484]}
            }
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "transaction": {
                "uid": "u1",
                "status": "failed",
                "payment": {"gateway_id": 3483, "status": "failed"}
            }
        })))
        .expect(1)
        .mount(&server)
        .await;

    let t = client(&server)
        .create_payment(
            PaymentRequest {
                amount: "100".into(),
                currency: "BYN".into(),
                test: true,
                description: "Cascading".into(),
                tracking_id: "cascade-1".into(),
                expired_at: None,
                dynamic_billing_descriptor: None,
                duplicate_check: Some(false),
                language: None,
                notification_url: None,
                verification_url: None,
                return_url: None,
                billing_address: None,
                credit_card: None,
                customer: None,
                additional_data: Some(AdditionalData {
                    browser: None,
                    contract: None,
                    referer: None,
                    masterpass: None,
                    split: None,
                    smart_routing_options: None,
                    excluded_gateways: Some(vec![3483, 3484]),
                    extra: None,
                }),
                encrypted_data: None,
                fiscalization: None,
                custom_fields: None,
            },
            None,
        )
        .await
        .expect("cascading payment should succeed");

    assert_eq!(t.payment.unwrap().gateway_id, Some(3483));
}

#[tokio::test]
async fn transaction_parses_fiscalization_receipts() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/transactions/fisc-1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "transaction": {
                "uid": "fisc-1",
                "status": "successful",
                "fiscalization": {
                    "id": "f1",
                    "external_id": "ext-1",
                    "status": "successful",
                    "code": "S.0000",
                    "message": "ok",
                    "friendly_message": "ok",
                    "receipts": [{
                        "id": "r1",
                        "serial_id": "s1",
                        "receipt_num": "1",
                        "created_at": "2024-01-01T00:00:00Z",
                        "ofd_id": "ofd1",
                        "ofd_link": "https://ofd.example/receipt",
                        "ofd_qr_code": "qr",
                        "total_amount": 700,
                        "receipt_info": {
                            "kkm_id": "kkm1",
                            "id": "ri1",
                            "shift_id": "sh1",
                            "serial_id": "s1",
                            "serial_shift_id": "ss1",
                            "issue_time": "2024-01-01T00:00:00Z",
                            "operation_type": "sale",
                            "payment_type": "card",
                            "currency_code": "BYN",
                            "subtotal_amount": 700,
                            "total_amount": 700,
                            "cashier_code": "c1",
                            "cashier_name": "John",
                            "receipt_num": "1",
                            "ofd_receipt_id": "ofd-r1",
                            "ofd_qr_code": "qr"
                        }
                    }]
                }
            }
        })))
        .expect(1)
        .mount(&server)
        .await;

    let t = client(&server)
        .get_transaction("fisc-1")
        .await
        .expect("transaction should parse");

    let fisc = t.fiscalization.expect("fiscalization present");
    assert_eq!(fisc.status.as_deref(), Some("successful"));
    assert_eq!(fisc.code.as_deref(), Some("S.0000"));
    let receipt = &fisc.receipts.unwrap()[0];
    assert_eq!(receipt.total_amount, Some(700));
    assert_eq!(
        receipt
            .receipt_info
            .as_ref()
            .unwrap()
            .cashier_name
            .as_deref(),
        Some("John")
    );
}
