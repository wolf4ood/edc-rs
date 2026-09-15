//! OAuth2 Token Exchange ([RFC 8693](https://datatracker.ietf.org/doc/html/rfc8693)) authentication.
//!
//! A workload credential (typically a projected Kubernetes ServiceAccount JWT) is presented to a
//! token exchange broker, which mints a short-lived, scoped access token that is then sent to the
//! connector as `Authorization: Bearer <token>`. The workload credential never reaches the
//! connector. See the
//! [JAD token exchange spec](https://github.com/eclipse-dataspace-hub/jad/blob/main/docs/token-exchange.md).

use std::{
    path::PathBuf,
    sync::Arc,
    time::{Duration, Instant},
};

use bon::Builder;
use reqwest::{header, Client, StatusCode, Url};
use serde::Deserialize;
use tokio::sync::Mutex;

use crate::{EdcResult, Error};

pub const GRANT_TYPE_TOKEN_EXCHANGE: &str = "urn:ietf:params:oauth:grant-type:token-exchange";
pub const TOKEN_TYPE_JWT: &str = "urn:ietf:params:oauth:token-type:jwt";

const DEFAULT_AUDIENCE: &str = "edcv";
const DEFAULT_EXPIRES_IN: Duration = Duration::from_secs(3600);
const EXPIRY_SKEW: Duration = Duration::from_secs(30);

/// Where the subject token (the workload credential) comes from.
#[derive(Clone, Debug)]
pub enum SubjectToken {
    /// A file holding the credential, e.g. a projected ServiceAccount token.
    /// It is re-read before every exchange so that rotation is picked up.
    File(PathBuf),
    /// A fixed credential.
    Static(String),
}

impl SubjectToken {
    pub fn file(path: impl Into<PathBuf>) -> SubjectToken {
        SubjectToken::File(path.into())
    }

    pub fn static_token(token: impl Into<String>) -> SubjectToken {
        SubjectToken::Static(token.into())
    }

    async fn read(&self) -> EdcResult<String> {
        match self {
            SubjectToken::File(path) => tokio::fs::read_to_string(path)
                .await
                .map(|token| token.trim().to_string())
                .map_err(|source| {
                    auth_error(TokenExchangeError::SubjectToken {
                        path: path.clone(),
                        source,
                    })
                }),
            SubjectToken::Static(token) => Ok(token.clone()),
        }
    }
}

/// Configuration of the token exchange flow.
#[derive(Builder)]
pub struct TokenExchangeConfig {
    /// The broker's token exchange endpoint, e.g. `http://jwtlet:8080/token`.
    #[builder(into)]
    token_exchange_url: String,
    /// The workload credential presented as `subject_token`.
    subject_token: SubjectToken,
    /// The `resource` parameter: the participant context the exchanged token speaks for,
    /// it becomes the `sub` claim of the minted token.
    #[builder(into)]
    resource: String,
    /// The `audience` parameter, must match the broker's configured audience.
    #[builder(into, default = DEFAULT_AUDIENCE.to_string())]
    audience: String,
    /// The scopes requested for the exchanged token.
    #[builder(default = vec!["management-api:read".to_string(), "management-api:write".to_string()])]
    scopes: Vec<String>,
}

#[derive(Debug, thiserror::Error)]
pub enum TokenExchangeError {
    #[error("failed to read the subject token from '{path}': {source}")]
    SubjectToken {
        path: PathBuf,
        source: std::io::Error,
    },
    #[error("token exchange broker returned {status}: {error}{}", description.as_deref().map(|d| format!(" ({d})")).unwrap_or_default())]
    Broker {
        status: StatusCode,
        error: String,
        description: Option<String>,
    },
    #[error("invalid token exchange response: {0}")]
    InvalidResponse(String),
}

fn auth_error(error: TokenExchangeError) -> Error {
    Error::Auth(Box::new(error))
}

#[derive(Clone)]
pub struct TokenExchange(Arc<TokenExchangeInternal>);

struct TokenExchangeInternal {
    cfg: TokenExchangeConfig,
    http_client: Client,
    session: Mutex<Option<TokenSession>>,
}

struct TokenSession {
    access_token: String,
    expires_at: Instant,
}

impl TokenSession {
    fn is_expired(&self) -> bool {
        Instant::now() + EXPIRY_SKEW >= self.expires_at
    }
}

#[derive(Deserialize)]
struct TokenExchangeResponse {
    access_token: String,
    expires_in: Option<u64>,
}

#[derive(Deserialize)]
struct BrokerErrorResponse {
    error: String,
    error_description: Option<String>,
}

impl TokenExchange {
    pub fn init(cfg: TokenExchangeConfig) -> EdcResult<TokenExchange> {
        Url::parse(&cfg.token_exchange_url).map_err(|e| Error::Auth(Box::new(e)))?;

        Ok(TokenExchange(Arc::new(TokenExchangeInternal {
            cfg,
            http_client: Client::new(),
            session: Mutex::default(),
        })))
    }

    pub async fn token(&self) -> EdcResult<String> {
        self.0.token().await
    }
}

impl TokenExchangeInternal {
    async fn token(&self) -> EdcResult<String> {
        let mut session = self.session.lock().await;

        match session.as_ref() {
            Some(t) if !t.is_expired() => Ok(t.access_token.clone()),
            _ => {
                let new_session = self.exchange().await?;
                let access_token = new_session.access_token.clone();
                *session = Some(new_session);
                Ok(access_token)
            }
        }
    }

    async fn exchange(&self) -> EdcResult<TokenSession> {
        let subject_token = self.cfg.subject_token.read().await?;
        let scope = self.cfg.scopes.join(" ");

        let params = [
            ("grant_type", GRANT_TYPE_TOKEN_EXCHANGE),
            ("subject_token", subject_token.as_str()),
            ("subject_token_type", TOKEN_TYPE_JWT),
            ("requested_token_type", TOKEN_TYPE_JWT),
            ("resource", self.cfg.resource.as_str()),
            ("scope", scope.as_str()),
            ("audience", self.cfg.audience.as_str()),
        ];

        let response = self
            .http_client
            .post(&self.cfg.token_exchange_url)
            .header(header::ACCEPT, "application/json")
            .form(&params)
            .send()
            .await?;

        let status = response.status();
        let body = response.text().await?;

        if !status.is_success() {
            let (error, description) = match serde_json::from_str::<BrokerErrorResponse>(&body) {
                Ok(err) => (err.error, err.error_description),
                Err(_) => (body, None),
            };
            return Err(auth_error(TokenExchangeError::Broker {
                status,
                error,
                description,
            }));
        }

        let token = serde_json::from_str::<TokenExchangeResponse>(&body)
            .map_err(|e| auth_error(TokenExchangeError::InvalidResponse(e.to_string())))?;

        let expires_in = token
            .expires_in
            .map(Duration::from_secs)
            .unwrap_or(DEFAULT_EXPIRES_IN);

        Ok(TokenSession {
            access_token: token.access_token,
            expires_at: Instant::now() + expires_in,
        })
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use wiremock::{
        matchers::{body_string_contains, header, method, path},
        Mock, MockServer, ResponseTemplate,
    };

    fn token_response(token: &str, expires_in: u64) -> ResponseTemplate {
        ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "access_token": token,
            "issued_token_type": TOKEN_TYPE_JWT,
            "token_type": "Bearer",
            "expires_in": expires_in
        }))
    }

    fn exchange(server: &MockServer, subject_token: SubjectToken) -> TokenExchange {
        TokenExchange::init(
            TokenExchangeConfig::builder()
                .token_exchange_url(format!("{}/token", server.uri()))
                .subject_token(subject_token)
                .resource("ctx-1")
                .build(),
        )
        .unwrap()
    }

    #[tokio::test]
    async fn should_send_rfc8693_form_request() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/token"))
            .and(header("content-type", "application/x-www-form-urlencoded"))
            .and(header("accept", "application/json"))
            .and(body_string_contains(
                "grant_type=urn%3Aietf%3Aparams%3Aoauth%3Agrant-type%3Atoken-exchange",
            ))
            .and(body_string_contains("subject_token=my-sa-jwt"))
            .and(body_string_contains(
                "subject_token_type=urn%3Aietf%3Aparams%3Aoauth%3Atoken-type%3Ajwt",
            ))
            .and(body_string_contains(
                "requested_token_type=urn%3Aietf%3Aparams%3Aoauth%3Atoken-type%3Ajwt",
            ))
            .and(body_string_contains("resource=ctx-1"))
            .and(body_string_contains(
                "scope=management-api%3Aread+management-api%3Awrite",
            ))
            .and(body_string_contains("audience=edcv"))
            .respond_with(token_response("minted", 3600))
            .expect(1)
            .mount(&server)
            .await;

        let exchange = exchange(&server, SubjectToken::static_token("my-sa-jwt"));

        assert_eq!(exchange.token().await.unwrap(), "minted");
    }

    #[tokio::test]
    async fn should_honour_custom_scopes_and_audience() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/token"))
            .and(body_string_contains("scope=management-api%3Aassets%3Aread"))
            .and(body_string_contains("audience=other"))
            .respond_with(token_response("minted", 3600))
            .expect(1)
            .mount(&server)
            .await;

        let exchange = TokenExchange::init(
            TokenExchangeConfig::builder()
                .token_exchange_url(format!("{}/token", server.uri()))
                .subject_token(SubjectToken::static_token("my-sa-jwt"))
                .resource("ctx-1")
                .audience("other")
                .scopes(vec!["management-api:assets:read".to_string()])
                .build(),
        )
        .unwrap();

        assert_eq!(exchange.token().await.unwrap(), "minted");
    }

    #[tokio::test]
    async fn should_cache_token_until_expiry() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/token"))
            .respond_with(token_response("minted", 3600))
            .expect(1)
            .mount(&server)
            .await;

        let exchange = exchange(&server, SubjectToken::static_token("my-sa-jwt"));

        assert_eq!(exchange.token().await.unwrap(), "minted");
        assert_eq!(exchange.token().await.unwrap(), "minted");
    }

    #[tokio::test]
    async fn should_exchange_again_when_token_is_about_to_expire() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/token"))
            .respond_with(token_response("minted", 10))
            .expect(2)
            .mount(&server)
            .await;

        let exchange = exchange(&server, SubjectToken::static_token("my-sa-jwt"));

        assert_eq!(exchange.token().await.unwrap(), "minted");
        assert_eq!(exchange.token().await.unwrap(), "minted");
    }

    #[tokio::test]
    async fn should_reread_subject_token_file_on_every_exchange() {
        let server = MockServer::start().await;
        let token_file =
            std::env::temp_dir().join(format!("edc-rs-subject-token-{}", uuid::Uuid::new_v4()));

        Mock::given(method("POST"))
            .and(path("/token"))
            .and(body_string_contains("subject_token=first"))
            .respond_with(token_response("minted-first", 0))
            .expect(1)
            .mount(&server)
            .await;

        Mock::given(method("POST"))
            .and(path("/token"))
            .and(body_string_contains("subject_token=second"))
            .respond_with(token_response("minted-second", 0))
            .expect(1)
            .mount(&server)
            .await;

        let exchange = exchange(&server, SubjectToken::file(&token_file));

        std::fs::write(&token_file, "first\n").unwrap();
        assert_eq!(exchange.token().await.unwrap(), "minted-first");

        std::fs::write(&token_file, "second\n").unwrap();
        assert_eq!(exchange.token().await.unwrap(), "minted-second");

        std::fs::remove_file(&token_file).unwrap();
    }

    #[tokio::test]
    async fn should_fail_when_subject_token_file_is_missing() {
        let server = MockServer::start().await;

        let exchange = exchange(&server, SubjectToken::file("/does/not/exist/token"));

        let err = exchange.token().await.unwrap_err();

        assert!(matches!(err, Error::Auth(_)));
        assert!(err.to_string().contains("/does/not/exist/token"));
    }

    #[tokio::test]
    async fn should_surface_broker_errors() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/token"))
            .respond_with(ResponseTemplate::new(400).set_body_json(serde_json::json!({
                "error": "invalid_grant",
                "error_description": "subject token rejected"
            })))
            .mount(&server)
            .await;

        let exchange = exchange(&server, SubjectToken::static_token("my-sa-jwt"));

        let err = exchange.token().await.unwrap_err();

        assert!(matches!(err, Error::Auth(_)));
        let message = err.to_string();
        assert!(message.contains("400"), "{message}");
        assert!(message.contains("invalid_grant"), "{message}");
        assert!(message.contains("subject token rejected"), "{message}");
    }

    #[tokio::test]
    async fn should_surface_non_json_broker_errors() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/token"))
            .respond_with(ResponseTemplate::new(503).set_body_string("upstream down"))
            .mount(&server)
            .await;

        let exchange = exchange(&server, SubjectToken::static_token("my-sa-jwt"));

        let err = exchange.token().await.unwrap_err();

        assert!(matches!(err, Error::Auth(_)));
        assert!(err.to_string().contains("upstream down"));
    }

    #[tokio::test]
    async fn should_fail_on_malformed_success_response() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "token_type": "Bearer"
            })))
            .mount(&server)
            .await;

        let exchange = exchange(&server, SubjectToken::static_token("my-sa-jwt"));

        let err = exchange.token().await.unwrap_err();

        assert!(matches!(err, Error::Auth(_)));
        assert!(err.to_string().contains("access_token"));
    }

    #[test]
    fn should_reject_invalid_endpoint_url() {
        let result = TokenExchange::init(
            TokenExchangeConfig::builder()
                .token_exchange_url("not a url")
                .subject_token(SubjectToken::static_token("my-sa-jwt"))
                .resource("ctx-1")
                .build(),
        );

        assert!(matches!(result, Err(Error::Auth(_))));
    }
}
