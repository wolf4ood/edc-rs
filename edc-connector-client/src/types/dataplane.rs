use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::BuilderError;

use super::properties::{Properties, ToValue};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DataPlaneInstance {
    #[serde(rename = "@id")]
    id: String,
    url: String,
    allowed_source_types: HashSet<String>,
    allowed_dest_types: HashSet<String>,
    allowed_transfer_types: HashSet<String>,
    properties: Properties,
}

impl DataPlaneInstance {
    pub fn builder() -> DataPlaneInstanceBuilder {
        DataPlaneInstanceBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct DataPlaneInstanceBuilder {
    id: Option<String>,
    url: Option<String>,
    properties: Properties,
    allowed_source_types: HashSet<String>,
    allowed_dest_types: HashSet<String>,
    allowed_transfer_types: HashSet<String>,
}

impl DataPlaneInstanceBuilder {
    pub fn id(mut self, id: &str) -> Self {
        self.id = Some(id.to_string());
        self
    }

    pub fn url(mut self, url: &str) -> Self {
        self.url = Some(url.to_string());
        self
    }

    pub fn property<T>(mut self, property: &str, value: T) -> Self
    where
        T: ToValue,
    {
        self.properties.set(property, value);
        self
    }

    pub fn allowed_destination_type(mut self, dest_type: &str) -> Self {
        self.allowed_dest_types.insert(dest_type.to_string());
        self
    }

    pub fn allowed_source_type(mut self, source_type: &str) -> Self {
        self.allowed_source_types.insert(source_type.to_string());
        self
    }

    pub fn allowed_transfer_type(mut self, transfer_type: &str) -> Self {
        self.allowed_transfer_types
            .insert(transfer_type.to_string());
        self
    }

    pub fn build(self) -> Result<DataPlaneInstance, BuilderError> {
        Ok(DataPlaneInstance {
            id: self
                .id
                .ok_or_else(|| BuilderError::missing_property("id"))?,
            url: self
                .url
                .ok_or_else(|| BuilderError::missing_property("url"))?,
            allowed_source_types: self.allowed_source_types,
            allowed_dest_types: self.allowed_dest_types,
            allowed_transfer_types: self.allowed_transfer_types,
            properties: self.properties,
        })
    }
}
