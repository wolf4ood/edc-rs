//! End-to-end test of the RFC 8693 token exchange auth against a mock broker and a mock
//! management API. Does not need the docker compose stack.

#![allow(clippy::unwrap_used)]

use edc_connector_client::{
    Auth, EdcConnectorApiVersion, EdcConnectorClient, SubjectToken, TokenExchangeConfig,
};
use wiremock::{
    matchers::{body_string_contains, header, method, path},
    Mock, MockServer, ResponseTemplate,
};

#[tokio::test]
async fn should_call_management_api_with_exchanged_token() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/token"))
        .and(body_string_contains(
            "grant_type=urn%3Aietf%3Aparams%3Aoauth%3Agrant-type%3Atoken-exchange",
        ))
        .and(body_string_contains("subject_token=my-sa-jwt"))
        .and(body_string_contains("resource=ctx-1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "access_token": "minted",
            "token_type": "Bearer",
            "expires_in": 3600
        })))
        .expect(1)
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/management/v3/assets/1"))
        .and(header("authorization", "Bearer minted"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "@context": {},
            "@id": "1",
            "@type": "Asset",
            "properties": {}
        })))
        .expect(2)
        .mount(&server)
        .await;

    let client = EdcConnectorClient::builder()
        .management_url(format!("{}/management", server.uri()))
        .with_auth(
            Auth::token_exchange(
                TokenExchangeConfig::builder()
                    .token_exchange_url(format!("{}/token", server.uri()))
                    .subject_token(SubjectToken::static_token("my-sa-jwt"))
                    .resource("ctx-1")
                    .build(),
            )
            .unwrap(),
        )
        .build()
        .unwrap();

    let asset = client
        .assets(EdcConnectorApiVersion::V3)
        .get("1")
        .await
        .unwrap();
    assert_eq!(asset.id(), "1");

    // Second call reuses the cached token: the broker is hit only once.
    let asset = client
        .assets(EdcConnectorApiVersion::V3)
        .get("1")
        .await
        .unwrap();
    assert_eq!(asset.id(), "1");
}
