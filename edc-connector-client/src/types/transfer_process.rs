use crate::{BuilderError, ConversionError, DATASPACE_PROTOCOL};
use serde::{Deserialize, Serialize};
use serde_with::{formats::PreferMany, serde_as, OneOrMany};

use super::{
    callback_address::CallbackAddress,
    data_address::DataAddress,
    properties::{FromValue, Properties},
};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferRequest {
    protocol: String,
    counter_party_address: String,
    contract_id: String,
    asset_id: String,
    transfer_type: String,
    data_destination: Option<DataAddress>,
    callback_addresses: Vec<CallbackAddress>,
}

impl TransferRequest {
    pub fn builder() -> TransferRequestBuilder {
        TransferRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct TransferRequestBuilder {
    protocol: Option<String>,
    counter_party_address: Option<String>,
    contract_id: Option<String>,
    transfer_type: Option<String>,
    asset_id: Option<String>,
    data_destination: Option<DataAddress>,
    callback_addresses: Vec<CallbackAddress>,
}

impl TransferRequestBuilder {
    pub fn protocol(mut self, protocol: &str) -> Self {
        self.protocol = Some(protocol.to_string());
        self
    }

    pub fn counter_party_address(mut self, counter_party_address: &str) -> Self {
        self.counter_party_address = Some(counter_party_address.to_string());
        self
    }

    pub fn contract_id(mut self, contract_id: &str) -> Self {
        self.contract_id = Some(contract_id.to_string());
        self
    }

    pub fn transfer_type(mut self, transfer_type: &str) -> Self {
        self.transfer_type = Some(transfer_type.to_string());
        self
    }

    pub fn asset_id(mut self, asset_id: &str) -> Self {
        self.asset_id = Some(asset_id.to_string());
        self
    }

    pub fn destination(mut self, destination: DataAddress) -> Self {
        self.data_destination = Some(destination);
        self
    }

    pub fn callback_address(mut self, callback_address: CallbackAddress) -> Self {
        self.callback_addresses.push(callback_address);
        self
    }

    pub fn build(self) -> Result<TransferRequest, BuilderError> {
        Ok(TransferRequest {
            protocol: self
                .protocol
                .unwrap_or_else(|| DATASPACE_PROTOCOL.to_string()),
            counter_party_address: self
                .counter_party_address
                .ok_or_else(|| BuilderError::missing_property("counter_party_address"))?,
            contract_id: self
                .contract_id
                .ok_or_else(|| BuilderError::missing_property("contract_id"))?,
            asset_id: self
                .asset_id
                .ok_or_else(|| BuilderError::missing_property("asset_id"))?,
            transfer_type: self
                .transfer_type
                .ok_or_else(|| BuilderError::missing_property("transfer_type"))?,
            data_destination: self.data_destination,
            callback_addresses: self.callback_addresses,
        })
    }
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferProcess {
    #[serde(rename = "@id")]
    id: String,
    #[serde(default)]
    private_properties: Properties,
    state: TransferProcessState,
    state_timestamp: i64,
    asset_id: String,
    contract_id: String,
    correlation_id: Option<String>,
    data_destination: Option<DataAddress>,
    #[serde_as(deserialize_as = "OneOrMany<_, PreferMany>")]
    callback_addresses: Vec<CallbackAddress>,
    transfer_type: String,
    #[serde(rename = "type")]
    kind: TransferProcessKind,
}

impl TransferProcess {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn state(&self) -> &TransferProcessState {
        &self.state
    }

    pub fn private_property<T>(&self, property: &str) -> Result<Option<T>, ConversionError>
    where
        T: FromValue,
    {
        self.private_properties.get(property)
    }

    pub fn kind(&self) -> &TransferProcessKind {
        &self.kind
    }

    pub fn asset_id(&self) -> &str {
        &self.asset_id
    }

    pub fn contract_id(&self) -> &str {
        &self.contract_id
    }

    pub fn correlation_id(&self) -> Option<&String> {
        self.correlation_id.as_ref()
    }

    pub fn data_destination(&self) -> Option<&DataAddress> {
        self.data_destination.as_ref()
    }

    pub fn transfer_type(&self) -> &str {
        &self.transfer_type
    }

    pub fn state_timestamp(&self) -> i64 {
        self.state_timestamp
    }

    pub fn callback_addresses(&self) -> &[CallbackAddress] {
        &self.callback_addresses
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransferProcessState {
    Initial,
    Provisioning,
    ProvisioningRequested,
    Provisioned,
    Requesting,
    Requested,
    Starting,
    Started,
    Suspending,
    Suspended,
    Resuming,
    Resumed,
    Completing,
    Completed,
    Terminating,
    Terminated,
    Deprovisioning,
    DeprovisioningRequested,
    Deprovisioned,
    #[serde(untagged)]
    Other(String),
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransferProcessKind {
    Consumer,
    Provider,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferState {
    state: TransferProcessState,
}

impl TransferState {
    pub fn state(&self) -> &TransferProcessState {
        &self.state
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TerminateTransfer {
    #[serde(rename = "@id")]
    pub(crate) id: String,
    pub(crate) reason: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SuspendTransfer {
    #[serde(rename = "@id")]
    pub(crate) id: String,
    pub(crate) reason: String,
}
