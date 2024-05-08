use serde::{Deserialize, Serialize};
use serde_with::{formats::PreferMany, serde_as, OneOrMany};

use crate::{BuilderError, ConversionError, DATASPACE_PROTOCOL};

use super::{
    callback_address::CallbackAddress,
    policy::Policy,
    properties::{FromValue, Properties},
};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContractRequest {
    protocol: String,
    counter_party_id: String,
    counter_party_address: String,
    policy: Policy,
    callback_addresses: Vec<CallbackAddress>,
}

impl ContractRequest {
    pub fn builder() -> ContractRequestBuilder {
        ContractRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct ContractRequestBuilder {
    protocol: Option<String>,
    counter_party_id: Option<String>,
    counter_party_address: Option<String>,
    policy: Option<Policy>,
    callback_addresses: Vec<CallbackAddress>,
}

impl ContractRequestBuilder {
    pub fn protocol(mut self, protocol: &str) -> Self {
        self.protocol = Some(protocol.to_string());
        self
    }

    pub fn counter_party_address(mut self, counter_party_address: &str) -> Self {
        self.counter_party_address = Some(counter_party_address.to_string());
        self
    }

    pub fn counter_party_id(mut self, counter_party_id: &str) -> Self {
        self.counter_party_id = Some(counter_party_id.to_string());
        self
    }

    pub fn policy(mut self, policy: Policy) -> Self {
        self.policy = Some(policy);
        self
    }

    pub fn callback_address(mut self, callback_address: CallbackAddress) -> Self {
        self.callback_addresses.push(callback_address);
        self
    }

    pub fn build(self) -> Result<ContractRequest, BuilderError> {
        Ok(ContractRequest {
            protocol: self
                .protocol
                .unwrap_or_else(|| DATASPACE_PROTOCOL.to_string()),
            counter_party_address: self
                .counter_party_address
                .ok_or_else(|| BuilderError::missing_property("counter_party_address"))?,
            counter_party_id: self
                .counter_party_id
                .ok_or_else(|| BuilderError::missing_property("counter_party_id"))?,
            policy: self
                .policy
                .ok_or_else(|| BuilderError::missing_property("policy"))?,
            callback_addresses: self.callback_addresses,
        })
    }
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContractNegotiation {
    #[serde(rename = "@id")]
    id: String,
    #[serde(default)]
    private_properties: Properties,
    state: ContractNegotiationState,
    contract_agreement_id: Option<String>,
    counter_party_id: String,
    counter_party_address: String,
    protocol: String,
    created_at: i64,
    #[serde_as(deserialize_as = "OneOrMany<_, PreferMany>")]
    callback_addresses: Vec<CallbackAddress>,
    #[serde(rename = "type")]
    kind: ContractNegotiationKind,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ContractNegotiationKind {
    Consumer,
    Provider,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ContractNegotiationState {
    Initial,
    Requesting,
    Requested,
    Offering,
    Offered,
    Accepting,
    Accepted,
    Agreeing,
    Agreed,
    Verifying,
    Verified,
    Finalizing,
    Finalized,
    Terminating,
    Terminated,
    #[serde(untagged)]
    Other(String),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NegotiationState {
    state: ContractNegotiationState,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TerminateNegotiation {
    #[serde(rename = "@id")]
    pub(crate) id: String,
    pub(crate) reason: String,
}

impl NegotiationState {
    pub fn state(&self) -> &ContractNegotiationState {
        &self.state
    }
}

impl ContractNegotiation {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn state(&self) -> &ContractNegotiationState {
        &self.state
    }

    pub fn private_property<T>(&self, property: &str) -> Result<Option<T>, ConversionError>
    where
        T: FromValue,
    {
        self.private_properties.get(property)
    }

    pub fn contract_agreement_id(&self) -> Option<&String> {
        self.contract_agreement_id.as_ref()
    }

    pub fn counter_party_id(&self) -> &str {
        &self.counter_party_id
    }

    pub fn counter_party_address(&self) -> &str {
        &self.counter_party_address
    }

    pub fn kind(&self) -> &ContractNegotiationKind {
        &self.kind
    }

    pub fn created_at(&self) -> i64 {
        self.created_at
    }

    pub fn callback_addresses(&self) -> &[CallbackAddress] {
        &self.callback_addresses
    }

    pub fn protocol(&self) -> &str {
        &self.protocol
    }
}
