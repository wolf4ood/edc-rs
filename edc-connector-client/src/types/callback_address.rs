use crate::BuilderError;
use serde::{Deserialize, Serialize};
use serde_with::{formats::PreferMany, serde_as, OneOrMany};

#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct CallbackAddress {
    transactional: bool,
    uri: String,
    #[serde_as(deserialize_as = "OneOrMany<_, PreferMany>")]
    events: Vec<String>,
    #[serde(flatten)]
    auth: Option<CallbackAddressAuth>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CallbackAddressAuth {
    auth_key: String,
    auth_code_id: String,
}

impl CallbackAddress {
    pub fn builder() -> CallbackAddressBuilder {
        CallbackAddressBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct CallbackAddressBuilder {
    transactional: bool,
    uri: Option<String>,
    events: Vec<String>,
    auth: Option<CallbackAddressAuth>,
}

impl CallbackAddressBuilder {
    pub fn transactional(mut self, transactional: bool) -> Self {
        self.transactional = transactional;
        self
    }

    pub fn uri(mut self, uri: &str) -> Self {
        self.uri = Some(uri.to_string());
        self
    }

    pub fn events(mut self, events: Vec<String>) -> Self {
        self.events = events;
        self
    }

    pub fn auth(mut self, auth_key: &str, auth_code_id: &str) -> Self {
        self.auth = Some(CallbackAddressAuth {
            auth_key: auth_key.to_string(),
            auth_code_id: auth_code_id.to_string(),
        });
        self
    }

    pub fn build(self) -> Result<CallbackAddress, BuilderError> {
        Ok(CallbackAddress {
            transactional: self.transactional,
            uri: self
                .uri
                .ok_or_else(|| BuilderError::missing_property("uri"))?,
            events: self.events,
            auth: self.auth,
        })
    }
}
