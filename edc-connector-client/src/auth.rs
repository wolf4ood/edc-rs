use crate::EdcResult;
use oauth::OAuth2;
pub use oauth::OAuth2Config;
use token_exchange::TokenExchange;
pub use token_exchange::{SubjectToken, TokenExchangeConfig, TokenExchangeError};

mod oauth;
mod token_exchange;

#[derive(Clone)]
pub enum Auth {
    NoAuth,
    ApiToken(String),
    OAuth2(OAuth2),
    BearerToken(String),
    TokenExchange(TokenExchange),
}

impl Auth {
    pub fn api_token(token: impl Into<String>) -> Auth {
        Auth::ApiToken(token.into())
    }

    pub fn oauth(cfg: OAuth2Config) -> EdcResult<Auth> {
        Ok(Auth::OAuth2(OAuth2::init(cfg)?))
    }

    pub fn bearer_token(token: impl Into<String>) -> Auth {
        Auth::BearerToken(token.into())
    }

    /// OAuth2 Token Exchange (RFC 8693): a workload credential is exchanged at a broker for a
    /// short-lived scoped token, which is sent as `Authorization: Bearer <token>`.
    pub fn token_exchange(cfg: TokenExchangeConfig) -> EdcResult<Auth> {
        Ok(Auth::TokenExchange(TokenExchange::init(cfg)?))
    }
}
