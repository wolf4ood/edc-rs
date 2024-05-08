use serde::{Deserialize, Serialize};

use crate::{BuilderError, ConversionError};

use super::{
    properties::{FromValue, Properties, ToValue},
    query::Criterion,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContractDefinition {
    #[serde(rename = "@id")]
    id: String,
    access_policy_id: String,
    contract_policy_id: String,
    #[serde(default)]
    assets_selector: Vec<Criterion>,
    #[serde(default)]
    private_properties: Properties,
}

impl ContractDefinition {
    pub fn builder() -> ContractDefinitionBuilder {
        ContractDefinitionBuilder::default()
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn access_policy_id(&self) -> &str {
        &self.access_policy_id
    }

    pub fn contract_policy_id(&self) -> &str {
        &self.contract_policy_id
    }

    pub fn assets_selector(&self) -> &[Criterion] {
        &self.assets_selector
    }

    pub fn private_property<T>(&self, property: &str) -> Result<Option<T>, ConversionError>
    where
        T: FromValue,
    {
        self.private_properties.get(property)
    }
}

#[derive(Default)]
pub struct ContractDefinitionBuilder {
    id: Option<String>,
    access_policy_id: Option<String>,
    contract_policy_id: Option<String>,
    assets_selector: Vec<Criterion>,
    private_properties: Properties,
}

impl ContractDefinitionBuilder {
    pub fn id(mut self, id: &str) -> Self {
        self.id = Some(id.to_string());
        self
    }

    pub fn access_policy_id(mut self, access_policy_id: &str) -> Self {
        self.access_policy_id = Some(access_policy_id.to_string());
        self
    }
    pub fn contract_policy_id(mut self, contract_policy_id: &str) -> Self {
        self.contract_policy_id = Some(contract_policy_id.to_string());
        self
    }
    pub fn private_property<T>(mut self, property: &str, value: T) -> Self
    where
        T: ToValue,
    {
        self.private_properties.set(property, value);
        self
    }

    pub fn asset_selector(mut self, selector: Criterion) -> Self {
        self.assets_selector.push(selector);
        self
    }

    pub fn build(self) -> Result<ContractDefinition, BuilderError> {
        Ok(ContractDefinition {
            id: self
                .id
                .ok_or_else(|| BuilderError::missing_property("id"))?,
            access_policy_id: self
                .access_policy_id
                .ok_or_else(|| BuilderError::missing_property("access_policy_id"))?,
            contract_policy_id: self
                .contract_policy_id
                .ok_or_else(|| BuilderError::missing_property("contract_policy_id"))?,

            assets_selector: self.assets_selector,
            private_properties: self.private_properties,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewContractDefinition {
    #[serde(rename = "@id")]
    id: Option<String>,
    access_policy_id: String,
    contract_policy_id: String,
    assets_selector: Vec<Criterion>,
    #[serde(default)]
    private_properties: Properties,
}

impl NewContractDefinition {
    pub fn builder() -> NewContractDefinitionBuilder {
        NewContractDefinitionBuilder::default()
    }
}

#[derive(Default)]
pub struct NewContractDefinitionBuilder {
    id: Option<String>,
    access_policy_id: Option<String>,
    contract_policy_id: Option<String>,
    asset_selector: Vec<Criterion>,
    private_properties: Properties,
}

impl NewContractDefinitionBuilder {
    pub fn id(mut self, id: &str) -> Self {
        self.id = Some(id.to_string());
        self
    }

    pub fn private_property<T>(mut self, property: &str, value: T) -> Self
    where
        T: ToValue,
    {
        self.private_properties.set(property, value);
        self
    }

    pub fn access_policy_id(mut self, access_policy_id: &str) -> Self {
        self.access_policy_id = Some(access_policy_id.to_string());
        self
    }
    pub fn contract_policy_id(mut self, contract_policy_id: &str) -> Self {
        self.contract_policy_id = Some(contract_policy_id.to_string());
        self
    }
    pub fn asset_selector(mut self, selector: Criterion) -> Self {
        self.asset_selector.push(selector);
        self
    }

    pub fn build(self) -> Result<NewContractDefinition, BuilderError> {
        Ok(NewContractDefinition {
            id: self.id,
            access_policy_id: self
                .access_policy_id
                .ok_or_else(|| BuilderError::missing_property("access_policy_id"))?,
            contract_policy_id: self
                .contract_policy_id
                .ok_or_else(|| BuilderError::missing_property("contract_policy_id"))?,

            assets_selector: self.asset_selector,
            private_properties: self.private_properties,
        })
    }
}
